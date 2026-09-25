//! F3 architecture.

mod id_page;
mod sdbp;

use std::collections::HashMap;

use crate::{
    cpu::{
        VECTOR_TABLE_SIZE,
        arm32::{self, INSTRUCTION_SIZE},
    },
    drive,
    protocol::{
        Transfer,
        ata::{
            self, SECTOR_SIZE,
            command::{Command, CommandRegisters},
            identify,
        },
        scsi::sense,
    },
};
use log::{debug, info};

/// Display name of drive type.
const DISPLAY_NAME: &str = "Seagate F3";
/// Memory address of ARM exception vector table.
const EXCEPTION_VECTOR_TABLE_ADDRESS: u32 = 0;

/// F3 error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// ID Page error.
    IdPage(id_page::Error),
    /// ID Page absent.
    NoIdPage(u8),
    /// SDBP error.
    Sdbp(sdbp::Error),
    /// SDBP packet has unexpected port.
    UnexpectedSdbpPort(sdbp::Port),
    /// Diagnostic Internal Test Service error.
    Dits(sense::SenseKey, sense::asc::AdditionalSenseCode),
    /// Data returned from VUC is too short.
    VucDataTruncated,
    /// Invalid ARM exception vector table in memory.
    InvalidExceptionVectorTable,
    /// System file enumeration found nothing.
    NoSystemFiles,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            Self::IdPage(x) => Some(x),
            Self::Sdbp(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::IdPage(_) => write!(f, "ID Page error"),
            Self::NoIdPage(x) => write!(f, "no ID Page {x}"),
            Self::Sdbp(_) => write!(f, "SDBP error"),
            Self::UnexpectedSdbpPort(x) => write!(f, "unexpected SDBP port {x}"),
            Self::Dits(key, asc) => write!(f, "DITS {key} {asc}"),
            Self::VucDataTruncated => write!(f, "VUC data truncated"),
            Self::InvalidExceptionVectorTable => write!(f, "invalid ARM exception vector table"),
            Self::NoSystemFiles => write!(f, "no system files"),
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

impl From<id_page::Error> for Error {
    fn from(value: id_page::Error) -> Self {
        Self::IdPage(value)
    }
}

impl From<sdbp::Error> for Error {
    fn from(value: sdbp::Error) -> Self {
        Self::Sdbp(value)
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
}

impl<'a> Drive<'a> {
    /// GPL log for VUC read operations.
    const VUC_LOG_READ: ata::log::Log = ata::log::Log::VendorSpecific(0xBF);
    /// GPL log for VUC write operations.
    const VUC_LOG_WRITE: ata::log::Log = ata::log::Log::VendorSpecific(0xBE);

    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<Self>) -> bool {
        const MODEL_PREFIX: &str = "ST";
        const WWN_OUI: u32 = 0xC50;

        let is_hdd = matches!(identify.rotation_rate, Some(identify::RotationRate::Rpm(_)));
        let model_match = identify.model.starts_with(MODEL_PREFIX);
        let wwn_match = identify.wwn.is_some_and(|x| x.oui == WWN_OUI);
        let has_gpl = identify.features_enabled.gpl == Some(true);
        let valid = is_hdd && model_match && wwn_match && has_gpl;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (HDD: {is_hdd}, model: {model_match}, WWN: \
                 {wwn_match}, GPL: {has_gpl})"
            );
        }

        valid
    }

    /// Open drive.
    fn open(ata: &'a ata::Drive) -> Result<Option<Self>, Error> {
        const VUC_LOG_PAGE_COUNT: u16 = u16::MAX;

        let drive = Self { ata };

        // Check identify device fields
        let identify = ata.identify_device()?;
        if !Self::validate_identify(&identify, Some(drive)) {
            return Ok(None);
        }

        let gpl_directory = ata.gpl_directory()?;
        let has_vuc_log_write = gpl_directory[Self::VUC_LOG_WRITE] == VUC_LOG_PAGE_COUNT;
        let has_vuc_log_read = gpl_directory[Self::VUC_LOG_READ] == VUC_LOG_PAGE_COUNT;
        let has_vuc_logs = has_vuc_log_write && has_vuc_log_read;
        debug!(
            "[{drive}] Validate VUC logs: {has_vuc_logs} (write: {has_vuc_log_write}, read: \
             {has_vuc_log_read})"
        );
        if !has_vuc_logs {
            return Ok(None);
        }

        let id_page_result = drive.id_page_11();
        debug!("[{drive}] Validate ID Page 11: {id_page_result:?}");
        let id_page = match id_page_result {
            Ok(_) => true,
            Err(Error::Ata(x)) if x.is_command_error() => false,
            Err(Error::IdPage(_) | Error::NoIdPage(_)) => false,
            Err(x) => return Err(x),
        };
        if !id_page {
            return Ok(None);
        }

        debug!("[{drive}] Validated");

        Ok(Some(drive))
    }

    /// Get vendor-specific ID page data.
    fn id_page(self, page: u8) -> Result<[u8; SECTOR_SIZE], Error> {
        const LBA: u64 = 0x20_97DB;

        let registers = CommandRegisters {
            feature: page.into(),
            count: 1,
            lba: LBA,
            command: Command::IdentifyDevice,
            ..Default::default()
        };

        let mut data = [0u8; SECTOR_SIZE];

        self.ata
            .command(registers, Transfer::Read(&mut data), false, false, None)?;

        Ok(data)
    }

    /// Get vendor-specific ID page 0.
    fn id_page_0(self) -> Result<id_page::Page0, Error> {
        const PAGE: u8 = 0;

        let data = self.id_page(PAGE)?;
        let page = (&data).try_into()?;
        debug!("[{self}] ID page {PAGE}: {page:?}");

        Ok(page)
    }

    /// Get vendor-specific ID page 11.
    fn id_page_11(self) -> Result<id_page::Page11, Error> {
        const PAGE: u8 = 11;

        let page_0 = self.id_page_0()?;
        let Some(version) = page_0.directory[PAGE as usize] else {
            return Err(Error::NoIdPage(PAGE));
        };

        let data = self.id_page(PAGE)?;
        let page = id_page::Page11::parse(&data, version)?;
        debug!("[{self}] ID page {PAGE}: {page:?}");

        Ok(page)
    }

    /// Execute VUC.
    fn vuc(
        self,
        port: sdbp::Port,
        dfb: &sdbp::Dfb,
        max_result_size: usize,
    ) -> Result<Box<[u8]>, Error> {
        const LOG_PAGE: u16 = 0x2459;

        let log_info = format!(
            "function: {:#x}, revision: {:#x}, data size: {}",
            dfb.function,
            dfb.revision,
            dfb.data.len()
        );
        debug!("[{self}] Executing VUC: {port} ({log_info}, max result size: {max_result_size})");

        let dfb = Box::<[_]>::from(dfb);
        let write_packet = sdbp::Packet {
            from: port,
            to: port,
            data: &dfb,
        };
        let write_packet = Box::<[_]>::from(&write_packet);
        self.ata
            .gpl_write(&write_packet, Self::VUC_LOG_WRITE, LOG_PAGE, 0)?;

        let read_size = sdbp::Packet::HEADER_SIZE + max_result_size;
        let mut read_packet = vec![0u8; read_size].into_boxed_slice();
        self.ata
            .gpl_read(&mut read_packet, Self::VUC_LOG_READ, LOG_PAGE, 0)?;
        let read_packet = sdbp::Packet::try_from(read_packet.as_ref())?;

        if read_packet.from != port {
            return Err(Error::UnexpectedSdbpPort(read_packet.from));
        }

        debug!(
            "[{self}] Executed VUC: {port} ({log_info}, result size: {})",
            read_packet.data.len()
        );

        Ok(read_packet.data.into())
    }

    /// Execute Diagnostic Internal Test Service VUC.
    fn vuc_dits(self, dfb: &sdbp::Dfb, result: &mut [u8]) -> Result<(), Error> {
        let max_dsb_size = sdbp::DitsDsb::HEADER_SIZE + result.len();
        let dsb = self.vuc(sdbp::Port::Dits, dfb, max_dsb_size)?;
        let dsb = sdbp::DitsDsb::try_from(dsb.as_ref())?;

        info!(
            "[{self}] Executed DITS VUC: {:#x} {:#x} (sense key: {}, ASC/ASCQ: {}, result size: \
             {})",
            dfb.function,
            dfb.revision,
            dsb.sense_key,
            dsb.asc,
            dsb.data.len()
        );

        // Check error status before data, error only returns DSB header
        if dsb.sense_key.is_error() {
            return Err(Error::Dits(dsb.sense_key, dsb.asc));
        }

        if dsb.data.len() != result.len() {
            return Err(Error::VucDataTruncated);
        }

        result.copy_from_slice(dsb.data);

        Ok(())
    }

    /// VUC enable or disable Diagnostic Internal Test Service.
    fn vuc_dits_set_enabled(self, value: bool) -> Result<(), Error> {
        const FUNCTION: u16 = 0xFFFF;
        const REVISION: u16 = 1;
        const KEY_ENABLE: u32 = 0x34F_329A;
        const KEY_DISABLE: u32 = 0x834F_329A;

        let dfb_data = if value { KEY_ENABLE } else { KEY_DISABLE }.to_le_bytes();
        let function = sdbp::Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: &dfb_data,
        };

        self.vuc_dits(&function, &mut [])?;

        Ok(())
    }

    /// VUC DITS read system file (system area module).
    fn vuc_dits_read_system_file(
        self,
        id: u16,
        sector_count: u16,
        sector_offset: u32,
        get_sector_count: bool,
    ) -> Result<Box<[u8]>, Error> {
        const FUNCTION: u16 = 0x144;
        const REVISION: u16 = 1;

        let dfb_id = id.to_le_bytes();
        let dfb_sector_count = sector_count.to_le_bytes();
        let dfb_sector_offset = sector_offset.to_le_bytes();
        let dfb_flags = u8::from(get_sector_count);
        let dfb_data = [
            dfb_id[0],
            dfb_id[1],
            dfb_sector_count[0],
            dfb_sector_count[1],
            dfb_sector_offset[0],
            dfb_sector_offset[1],
            dfb_sector_offset[2],
            dfb_sector_offset[3],
            dfb_flags,
            0,
            0,
            0,
        ];
        let dfb = sdbp::Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: &dfb_data,
        };

        let result_size = if get_sector_count {
            size_of::<u32>()
        } else {
            sector_count as usize * SECTOR_SIZE
        };

        let mut result = vec![0u8; result_size].into_boxed_slice();

        self.vuc_dits(&dfb, &mut result)?;

        Ok(result)
    }

    /// Get size of a system file in sectors.
    fn system_file_sector_count(self, id: u16) -> Result<u32, Error> {
        let data = self.vuc_dits_read_system_file(id, 0, 0, true)?;
        let sector_count = u32::from_le_bytes(std::array::from_fn(|i| data[i]));

        debug!("[{self}] System file {id:#x} sector count: {sector_count}");

        Ok(sector_count)
    }

    /// Enumerate present system files.
    fn system_file_list(self) -> Result<HashMap<u16, u32>, Error> {
        const MAX_ID: u16 = 0xFF;

        let mut list = HashMap::new();

        for id in 0..=MAX_ID {
            let sector_count = match self.system_file_sector_count(id) {
                Ok(x) => x,
                Err(Error::Dits(
                    sense::SenseKey::IllegalRequest,
                    sense::asc::AdditionalSenseCode::ParameterValueInvalid,
                )) => 0,
                Err(x) => return Err(x),
            };

            if sector_count > 0 {
                list.insert(id, sector_count);
            }
        }

        debug!("[{self}] System file list: {list:?}");
        Ok(list)
    }

    /// VUC DITS read controller memory.
    fn vuc_dits_read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const FUNCTION: u16 = 0x149;
        const REVISION: u16 = 1;

        let dfb_address = address.to_le_bytes();
        let dfb_size = u16::try_from(data.len()).unwrap().to_le_bytes();
        let dfb_data = [
            dfb_address[0],
            dfb_address[1],
            dfb_address[2],
            dfb_address[3],
            0,
            0,
            dfb_size[0],
            dfb_size[1],
            0,
            0,
            0,
            0,
        ];
        let dfb = sdbp::Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: &dfb_data,
        };

        self.vuc_dits(&dfb, data)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const MAX_READ_SIZE: usize = u16::MAX as _;

        for (chunk_index, chunk_data) in data.chunks_mut(MAX_READ_SIZE).enumerate() {
            let read_address = address
                .checked_add(u32::try_from(chunk_index * MAX_READ_SIZE).unwrap())
                .unwrap();
            self.vuc_dits_read_memory(read_address, chunk_data)?;
        }

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

/// F3 vendor type.
pub struct VendorType;

impl VendorType {
    /// Check read system area.
    fn check_read_sa(drive: Drive) -> drive::CheckResult {
        let result = match drive.system_file_list() {
            Ok(x) if x.is_empty() => Err(Error::NoSystemFiles.into()),
            Ok(x) => {
                let sector_count = x.values().fold(0u64, |s, &x| s.saturating_add(x.into()));
                let size = sector_count.saturating_mul(SECTOR_SIZE as _);
                Ok(format!(
                    "{} modules ({})",
                    x.len(),
                    crate::output::format_byte_size_decimal(size)
                ))
            },
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read system area".into(), result)
    }

    /// Check read controller memory.
    fn check_read_memory(drive: Drive) -> drive::CheckResult {
        let read_result = drive.read_exception_vector_table();
        debug!("[{drive}] Read exception vector table (check read memory): {read_result:?}");

        let result = match read_result {
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
        const MOV_R0_R0: [u8; INSTRUCTION_SIZE] = [0x0, 0x0, 0xA0, 0xE1]; // mov r0, r0
        const NOP: [u8; INSTRUCTION_SIZE] = [0x0, 0xF0, 0x20, 0xE3]; // nop

        let result = |x| drive::CheckResult::new("IRATEMONK".into(), x);

        let data = match drive.read_exception_vector_table() {
            Ok(x) => x,
            Err(x) => return result(Err(x.into())),
        };

        let reserved_vector = std::array::from_fn(|x| data[RESERVED_VECTOR_OFFSET + x]);

        if reserved_vector == MOV_R0_R0 || reserved_vector == NOP {
            return result(Ok("Infection not found".into()));
        }

        // Logical address in system area, unknown format
        let data_address = u32::from_le_bytes(reserved_vector);

        result(Ok(format!("INFECTED (data address: {data_address:#x})")))
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

        let mut results = Vec::new();

        let id_page = drive.id_page_11()?;

        // ID Page 11 part of the validation checks, assume it always succeeds
        results.push(drive::CheckResult::new(
            "ID Page firmware".into(),
            Ok(format!(
                "{} (date: {}, controller: {}, servo: {})",
                id_page.package_version,
                id_page.package_timestamp,
                id_page.package_cfw_version,
                id_page.sfw_version
            )),
        ));

        drive.vuc_dits_set_enabled(true)?;
        results.push(Self::check_read_sa(drive));
        results.push(Self::check_read_memory(drive));
        results.push(Self::check_iratemonk(drive));
        drive.vuc_dits_set_enabled(false)?;

        Ok(Some(results.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn validate_identify() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::seagate_momentus5::IDENTIFY,
            test_data::seagate_barracudapro::IDENTIFY,
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::westerndigital_scorpioblack::IDENTIFY,
            test_data::seagate_momentus3::IDENTIFY, // ST-10
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
