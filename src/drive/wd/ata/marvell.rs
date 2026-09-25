//! Marvell controller architecture.

mod file;
mod table;
mod vuc;

use log::{debug, info};

use crate::{
    cpu::{
        VECTOR_TABLE_SIZE,
        arm32::{self, INSTRUCTION_SIZE},
    },
    drive, output,
    protocol::{
        Transfer,
        ata::{
            self, SECTOR_SIZE,
            command::{Command, CommandRegisters},
            identify,
        },
    },
};

/// Display name of drive type.
const DISPLAY_NAME: &str = "Western Digital Marvell";
/// SMART log for VUC sense.
const VUC_SMART_LOG_SENSE: ata::log::Log = ata::log::Log::VendorSpecific(0xBD);
/// SMART log for VUC key and status.
const VUC_SMART_LOG_KEY_STATUS: ata::log::Log = ata::log::Log::VendorSpecific(0xBE);
/// SMART log for VUC data.
const VUC_SMART_LOG_DATA: ata::log::Log = ata::log::Log::VendorSpecific(0xBF);
/// Memory address of ARM exception vector table.
const EXCEPTION_VECTOR_TABLE_ADDRESS: u32 = 0;

/// Marvell error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// VUC error.
    Vuc(vuc::ErrorCode),
    /// VUC sense error.
    VucSense(vuc::sense::Error),
    /// VUC status error.
    VucStatus(vuc::status::Error),
    /// SA error.
    Sa(file::Error),
    /// VUC table error.
    Table(table::Error),
    /// Invalid exception vector table data in memory.
    InvalidExceptionVectorTable,
    /// Invalid VUC transfer size.
    InvalidTransferSize(u64),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            Self::VucSense(x) => x.source(),
            Self::VucStatus(x) => x.source(),
            Self::Sa(x) => x.source(),
            Self::Table(x) => x.source(),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::Vuc(x) => write!(f, "VUC {x}"),
            Self::VucSense(x) => x.fmt(f),
            Self::VucStatus(x) => x.fmt(f),
            Self::Sa(x) => x.fmt(f),
            Self::Table(x) => x.fmt(f),
            Self::InvalidExceptionVectorTable => write!(f, "invalid ARM exception vector table"),
            Self::InvalidTransferSize(x) => write!(f, "invalid transfer size {x}"),
        }
    }
}

impl drive::VendorError for Error {
    fn name(&self) -> &str {
        DISPLAY_NAME
    }
}

impl From<ata::Error> for Error {
    fn from(value: ata::Error) -> Self {
        Self::Ata(value)
    }
}

impl From<vuc::ErrorCode> for Error {
    fn from(value: vuc::ErrorCode) -> Self {
        Self::Vuc(value)
    }
}

impl From<vuc::sense::Error> for Error {
    fn from(value: vuc::sense::Error) -> Self {
        Self::VucSense(value)
    }
}

impl From<vuc::status::Error> for Error {
    fn from(value: vuc::status::Error) -> Self {
        Self::VucStatus(value)
    }
}

impl From<file::Error> for Error {
    fn from(value: file::Error) -> Self {
        Self::Sa(value)
    }
}

impl From<table::Error> for Error {
    fn from(value: table::Error) -> Self {
        Self::Table(value)
    }
}

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Ata(x) => x.into(),
            x => Self::Vendor(Box::new(x)),
        }
    }
}

/// Drive interface.
#[derive(Clone, Copy, Debug)]
struct Drive<'a> {
    /// ATA drive.
    ata: &'a ata::Drive,
    /// VUC sense supported.
    sense: bool,
}

impl<'a> Drive<'a> {
    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<Self>) -> bool {
        const MODEL_PREFIX: &str = "WDC WD";
        const WWN_OUI: u32 = 0x14EE;
        const WORD142: u16 = 4;

        let is_hdd = matches!(identify.rotation_rate, Some(identify::RotationRate::Rpm(_)));
        let model_match = identify.model.starts_with(MODEL_PREFIX);
        let wwn_match = identify.wwn.is_some_and(|x| x.oui == WWN_OUI);
        let sct_support = identify.sct_supported.sct;
        let word142 = u16::from_le_bytes(std::array::from_fn(|i| identify.vendor_specific[26 + i]));
        let word142_match = word142 == WORD142;
        let valid = is_hdd && model_match && wwn_match && sct_support && word142_match;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (HDD: {is_hdd}, model: {model_match}, WWN: \
                 {wwn_match}, SCT: {sct_support}, word 142: {word142_match})"
            );
        }

        valid
    }

    /// Open drive.
    fn open(ata: &'a ata::Drive) -> Result<Option<Self>, Error> {
        let mut drive = Self { ata, sense: false };

        // Check identify device fields
        let identify = ata.identify_device()?;
        if !Self::validate_identify(&identify, Some(drive)) {
            return Ok(None);
        }

        // Check if VUC sense supported
        let sense_supported = match drive.vuc_sense() {
            Ok(_) => true,
            Err(Error::Ata(x)) if x.is_command_error() => false,
            Err(Error::VucSense(_)) => false,
            Err(x) => return Err(x),
        };

        // Check if VUC status supported
        let status_supported = match drive.vuc_status() {
            Ok(_) => true,
            Err(Error::Ata(x)) if x.is_command_error() => false,
            Err(Error::VucStatus(_)) => false,
            Err(x) => return Err(x),
        };

        // Either VUC sense or status must be supported
        let sense_or_status = sense_supported || status_supported;
        debug!(
            "[{drive}] Validate VUC sense/status: {sense_or_status} (sense: {sense_supported}, \
             status: {status_supported})"
        );
        if !sense_or_status {
            return Ok(None);
        }

        debug!("[{drive}] Validated");

        drive.sense = sense_supported;
        Ok(Some(drive))
    }

    /// Enable or disable VUC access.
    fn vuc_set_enabled(self, enable: bool) -> Result<(), Error> {
        const FEATURE_ENABLE: u16 = 'E' as _;
        const FEATURE_DISABLE: u16 = 'D' as _;
        const LBA: u64 = (('W' as u64) << 16) | (('D' as u64) << 8);
        const COMMAND: Command = Command::VendorSpecific(0x80);

        let feature = if enable {
            FEATURE_ENABLE
        } else {
            FEATURE_DISABLE
        };

        let registers = CommandRegisters {
            feature,
            lba: LBA,
            command: COMMAND,
            ..Default::default()
        };

        self.ata
            .command(registers, Transfer::None, false, false, None)?;

        Ok(())
    }

    /// Get VUC sense.
    fn vuc_sense(self) -> Result<vuc::sense::Sense, Error> {
        let mut data = [0u8; vuc::sense::Sense::SIZE];

        self.ata.smart_log_read(&mut data, VUC_SMART_LOG_SENSE)?;

        let sense = vuc::sense::Sense::try_from(&data)?;
        debug!("[{self}] VUC sense: {sense:?}");

        Ok(sense)
    }

    /// Get VUC status.
    fn vuc_status(self) -> Result<vuc::status::Status, Error> {
        let mut data = [0u8; vuc::status::Status::SIZE];

        self.ata
            .smart_log_read(&mut data, VUC_SMART_LOG_KEY_STATUS)?;

        let status = vuc::status::Status::try_from(&data)?;
        debug!("[{self}] VUC status: {status:?}");

        Ok(status)
    }

    /// Get error code if any for last VUC operation.
    fn vuc_error(self) -> Result<Option<vuc::ErrorCode>, Error> {
        Ok(if self.sense {
            self.vuc_sense()?.principal_error
        } else {
            self.vuc_status()?.error
        })
    }

    /// Get VUC pending transfer size in bytes.
    fn vuc_transfer_size(self) -> Result<usize, Error> {
        const MAX_SECTOR_COUNT: u64 = 128 * 1024;

        let sector_count = if self.sense {
            self.vuc_sense()?.sector_count.into()
        } else {
            self.vuc_status()?.sector_count
        };

        if sector_count > MAX_SECTOR_COUNT {
            return Err(Error::InvalidTransferSize(sector_count));
        }

        Ok(usize::try_from(sector_count).unwrap() * SECTOR_SIZE)
    }

    /// Handle an ATA error during a VUC, try retrieve a VUC error code.
    fn handle_vuc_ata_error(self, ata_error: ata::Error) -> Error {
        // Only handle command failed errors
        if !ata_error.is_command_error() {
            return ata_error.into();
        }

        // If VUC sense/status has an error code, use that as the error
        match self.vuc_error() {
            Err(_) | Ok(None) => ata_error.into(),
            Ok(Some(x)) => Error::Vuc(x),
        }
    }

    /// VUC set key.
    fn vuc_set_key(
        self,
        action: vuc::Action,
        function: Option<u16>,
        parameter: &[u8],
    ) -> Result<(), Error> {
        let key_action = action as u16;
        let key_data_size = size_of_val(&key_action) + size_of::<u16>() + parameter.len();
        let mut key_data = Vec::with_capacity(key_data_size);
        key_data.extend_from_slice(&key_action.to_le_bytes());

        if let Some(x) = function {
            key_data.extend_from_slice(&x.to_le_bytes());
        }

        key_data.extend_from_slice(parameter);

        self.ata
            .smart_log_write(&key_data, VUC_SMART_LOG_KEY_STATUS)
            .map_err(|x| self.handle_vuc_ata_error(x))?;

        Ok(())
    }

    /// VUC data transfer.
    fn vuc_transfer(self, transfer: Transfer) -> Result<(), Error> {
        if transfer == Transfer::None {
            return Ok(());
        }

        match transfer {
            Transfer::None => unreachable!(),
            Transfer::Read(data) => data
                .chunks_mut(SECTOR_SIZE)
                .try_for_each(|chunk| self.ata.smart_log_read(chunk, VUC_SMART_LOG_DATA).map(drop)),
            Transfer::Write(data) => data.chunks(SECTOR_SIZE).try_for_each(|chunk| {
                self.ata
                    .smart_log_write(chunk, VUC_SMART_LOG_DATA)
                    .map(drop)
            }),
        }
        .map_err(|x| self.handle_vuc_ata_error(x))
    }

    /// Execute VUC.
    fn vuc(
        self,
        transfer: Transfer,
        action: vuc::Action,
        function: Option<u16>,
        parameter: &[u8],
    ) -> Result<(), Error> {
        let log_info = format!(
            "function: {}, parameter: {}, data: {transfer}",
            match function {
                Some(x) => x.to_string(),
                None => "N/A".into(),
            },
            match parameter {
                &[] => "N/A".into(),
                x => output::format_bytes_hex(x),
            },
        );

        debug!("[{self}] Executing VUC: ({log_info})");
        self.vuc_set_key(action, function, parameter)?;
        self.vuc_transfer(transfer)?;

        info!("[{self}] Executed VUC: ({log_info})");
        Ok(())
    }

    /// Execute VUC with read transfer of dynamic size.
    fn vuc_read_any(
        self,
        action: vuc::Action,
        function: Option<u16>,
        parameter: &[u8],
    ) -> Result<Box<[u8]>, Error> {
        let log_info = format!(
            "function: {}, parameter: {}",
            match function {
                Some(x) => x.to_string(),
                None => "N/A".into(),
            },
            match parameter {
                &[] => "N/A".into(),
                x => output::format_bytes_hex(x),
            }
        );
        debug!("[{self}] Executing VUC: {action} ({log_info}, data: read any)");

        self.vuc_set_key(action, function, parameter)?;

        let data_size = self.vuc_transfer_size()?;
        let mut data = vec![0u8; data_size].into_boxed_slice();
        let transfer = Transfer::Read(&mut data);
        let log_info = format!("{log_info}, data: {transfer}");

        self.vuc_transfer(transfer)?;

        info!("[{self}] Executed VUC: {action} ({log_info})");

        Ok(data)
    }

    /// Read SA file.
    fn read_file(self, id: u16) -> Result<file::File, Error> {
        const FUNCTION: Option<u16> = Some(1);

        let parameter = id.to_le_bytes();
        let data = self.vuc_read_any(vuc::Action::File, FUNCTION, &parameter)?;

        let file = file::File::try_from(data.as_ref())?;
        debug!(
            "[{self}] Read SA file: {id} (size: {}, header: {:?})",
            file.data.len(),
            file.header
        );

        Ok(file)
    }

    /// Read SA directory.
    fn read_sa_directory(self) -> Result<file::Directory, Error> {
        let file = self.read_file(file::FileId::SaDirectory as _)?;

        let directory = file::Directory::try_from(&file)?;
        debug!(
            "[{self}] Read SA directory: {} files",
            directory.valid().count()
        );

        Ok(directory)
    }

    /// Read flash directory.
    fn read_flash_directory(self) -> Result<file::Directory, Error> {
        let mut file = self.read_file(file::FileId::FlashDirectory as _)?;

        // For ROYL-20B tag is a write sequence counter, active copy has the higher tag
        if file.header.version == file::Directory::VERSION_ROYL_ABA_20B_FLASH {
            let file_ext = self.read_file(file::FileId::FlashDirectoryExt as _)?;

            if file_ext
                .header
                .tag
                .wrapping_sub(file.header.tag)
                .cast_signed()
                > 0
            {
                file = file_ext;
            }
        }

        let directory = file::Directory::try_from(&file)?;
        debug!("[{self}] Read flash directory: {directory:?}");

        Ok(directory)
    }

    /// VUC get table.
    fn get_table(self, data: &mut [u8], id: u16) -> Result<(), Error> {
        self.vuc(
            Transfer::Read(data),
            vuc::Action::Table,
            None,
            &id.to_le_bytes(),
        )?;

        Ok(())
    }

    /// VUC physical parameters table.
    fn physical_parameters(self) -> Result<table::PhysicalParameters, Error> {
        const TABLE_ID: u16 = 1;

        let mut data = [0u8; SECTOR_SIZE];
        self.get_table(&mut data, TABLE_ID)?;

        let physical_parameters = (&data).try_into()?;
        debug!("[{self}] Physical parameters: {physical_parameters:?}");

        Ok(physical_parameters)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const FUNCTION: Option<u16> = Some(1);

        let size = u32::try_from(data.len()).unwrap();
        let mut parameter = Vec::with_capacity(size_of_val(&address) + size_of_val(&size));
        parameter.extend_from_slice(&address.to_le_bytes());
        parameter.extend_from_slice(&size.to_le_bytes());

        self.vuc(
            Transfer::Read(data),
            vuc::Action::Memory,
            FUNCTION,
            &parameter,
        )?;

        Ok(())
    }

    /// Read ARM exception vector table from controller memory.
    fn read_exception_vector_table(self) -> Result<[u8; VECTOR_TABLE_SIZE], Error> {
        let mut data = [0; _];
        self.read_memory(EXCEPTION_VECTOR_TABLE_ADDRESS, &mut data)?;

        if !arm32::is_exception_vector_table(&data) {
            return Err(Error::InvalidExceptionVectorTable);
        }

        Ok(data)
    }
}

impl std::fmt::Display for Drive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{DISPLAY_NAME} {}", self.ata.path().display())
    }
}

/// Marvell vendor type.
pub struct VendorType;

impl VendorType {
    /// Check VUC physical parameters table.
    fn check_physical_parameters(drive: Drive) -> drive::CheckResult {
        let result = match drive.physical_parameters() {
            Ok(x) => Ok(format!(
                "(firmware: (controller: {}, overlay: {}), manufacture date: {}, type: {}, \
                 interface: {}, DRAM: {} MB, flash: {})",
                x.controller_firmware,
                x.overlay_firmware,
                x.manufacture_date,
                x.drive_type,
                x.interface,
                x.dram_size,
                output::format_byte_size_binary(x.flash_size.into())
            )),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Physical parameters".into(), result)
    }

    /// Format SA/flash directory for display.
    fn format_directory<'a>(
        entries: impl Iterator<Item = &'a file::DirectoryEntry>,
        flash: bool,
    ) -> String {
        let mut count = 0u64;
        let mut size = 0u64;
        for entry in entries.filter(|x| x.attributes.flash == flash) {
            count += 1;
            size += entry.size();
        }

        format!(
            "{} modules ({})",
            count,
            output::format_byte_size_decimal(size)
        )
    }

    /// Check read system area.
    fn check_read_sa(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_sa_directory() {
            Ok(directory) => Ok(Self::format_directory(directory.valid(), false)),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read system area".into(), result)
    }

    /// Check read flash.
    fn check_read_flash(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_flash_directory() {
            Ok(directory) => Ok(Self::format_directory(directory.valid(), true)),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read flash".into(), result)
    }

    /// Check read controller memory.
    fn check_read_memory(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_exception_vector_table() {
            Ok(_) => Ok(format!(
                "ARM exception vector table at {EXCEPTION_VECTOR_TABLE_ADDRESS:#010x}"
            )),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read memory".into(), result)
    }

    /// Check for IRATEMONK infection, implements the same logic `nls_933w.dll`
    /// uses to locate implant data storage.
    fn check_iratemonk(drive: Drive) -> drive::CheckResult {
        const RESERVED_VECTOR_OFFSET: usize = 5 * INSTRUCTION_SIZE;
        const FIQ_VECTOR_OFFSET: usize = 7 * INSTRUCTION_SIZE;
        const MOV_R0_R0: [u8; INSTRUCTION_SIZE] = [0x0, 0x0, 0xA0, 0xE1]; // mov r0, r0
        const MOV_R1_R1: [u8; INSTRUCTION_SIZE] = [0x1, 0x10, 0xA0, 0xE1]; // mov r1, r1
        const MOV_R2_R2: [u8; INSTRUCTION_SIZE] = [0x2, 0x20, 0xA0, 0xE1]; // mov r2, r2
        const MOV_R3_R3: [u8; INSTRUCTION_SIZE] = [0x3, 0x30, 0xA0, 0xE1]; // mov r3, r3
        const LDR_PC_18H: [u8; INSTRUCTION_SIZE] = [0x18, 0xF0, 0x9F, 0xE5]; // ldr pc, [pc, #0x18]

        let result = |x| drive::CheckResult {
            name: "IRATEMONK".into(),
            result: x,
        };
        let result_not_found = Ok("Infection not found".to_string());

        let data = match drive.read_exception_vector_table() {
            Ok(x) => x,
            Err(x) => return result(Err(x.into())),
        };

        let reserved_vector = std::array::from_fn(|x| data[RESERVED_VECTOR_OFFSET + x]);
        let fiq_vector = std::array::from_fn(|x| data[FIQ_VECTOR_OFFSET + x]);

        let chs_packed_offset = if reserved_vector == MOV_R0_R0 && fiq_vector == LDR_PC_18H {
            52
        } else if reserved_vector == MOV_R1_R1 {
            60
        } else if reserved_vector == MOV_R2_R2 {
            180
        } else if reserved_vector == MOV_R3_R3 {
            92
        } else {
            return result(result_not_found);
        };

        let chs_packed = u32::from_le_bytes(std::array::from_fn(|x| data[chs_packed_offset + x]));
        debug!("[{drive}] IRATEMONK packed CHS {chs_packed:#x} (offset: {chs_packed_offset})");

        let cylinder = chs_packed.cast_signed() >> 15;
        let head = ((chs_packed >> 12) & 0b111) as u8;
        let sector = (chs_packed & 0xFFF) as u16;

        // Valid system area address always has a negative cylinder
        if cylinder >= 0 {
            return result(result_not_found);
        }

        result(Ok(format!(
            "INFECTED (data C/H/S: {cylinder}/{head}/{sector})"
        )))
    }
}

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        DISPLAY_NAME
    }

    fn check(
        &self,
        drive: &drive::Drive,
    ) -> Result<Option<Box<[drive::CheckResult]>>, drive::Error> {
        // Drive must be ATA
        let drive::Drive::Ata(ata_drive) = drive else {
            return Ok(None);
        };

        // Attempt to open drive
        let Some(drive) = Drive::open(ata_drive)? else {
            return Ok(None);
        };

        // Enable VUC access
        drive.vuc_set_enabled(true)?;

        let results = vec![
            Self::check_physical_parameters(drive),
            Self::check_read_sa(drive),
            Self::check_read_flash(drive),
            Self::check_read_memory(drive),
            Self::check_iratemonk(drive),
        ]
        .into();

        // Disable VUC access
        drive.vuc_set_enabled(false)?;

        Ok(Some(results))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn validate_identify() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::westerndigital_scorpioblack::IDENTIFY,
            test_data::westerndigital_bluemobile::IDENTIFY,
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::seagate_momentus5::IDENTIFY,
            test_data::westerndigital_caviarse::IDENTIFY, // WDC MCU (WD70Cxx)
        ];

        for &data in DATA_VALID {
            let identify = identify::IdentifyDevice::try_from(data).unwrap();
            assert!(Drive::validate_identify(&identify, None));
        }

        for &data in DATA_INVALID {
            let identify = identify::IdentifyDevice::try_from(data).unwrap();
            assert!(!Drive::validate_identify(&identify, None));
        }
    }
}
