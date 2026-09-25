//! S12 controller.

mod info_block;
mod system_info;

use log::{debug, info};

use crate::{
    cpu::{VECTOR_TABLE_SIZE, arm32},
    drive,
    drive::phison::{self, VucLockState, nvme::VucOperation},
    protocol::{
        Transfer,
        ata::{self, SECTOR_SIZE, identify},
        nvme,
    },
};

/// Display name of drive type.
const DISPLAY_NAME: &str = "Phison S12";
/// Memory address of ARM exception vector table.
const EXCEPTION_VECTOR_TABLE_ADDRESS: u32 = 0;

/// S12 error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// VUC status represents error.
    VucStatus(nvme::status::StatusField),
    /// Invalid VUC completion status.
    Status(nvme::status::Error),
    /// Invalid system info data.
    InvalidSystemInfo,
    /// Invalid info block data.
    InvalidInfoBlock,
    /// Invalid exception vector table data in memory.
    InvalidExceptionVectorTable,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            Self::Status(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::VucStatus(x) => write!(f, "VUC status {x}"),
            Self::Status(_) => write!(f, "invalid VUC status"),
            Self::InvalidSystemInfo => write!(f, "invalid system info"),
            Self::InvalidInfoBlock => write!(f, "invalid info block"),
            Self::InvalidExceptionVectorTable => write!(f, "invalid ARM exception vector table"),
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

impl From<nvme::status::Error> for Error {
    fn from(value: nvme::status::Error) -> Self {
        Self::Status(value)
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
    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<Self>) -> bool {
        let drat = identify.additional_supported.is_some_and(|x| x.drat);
        let rzat = identify.additional_supported.is_some_and(|x| x.rzat);
        let sct = identify.sct_supported.sct;
        let valid = drat && rzat && sct;

        if let Some(drive) = drive {
            debug!("[{drive}] Validate identify: {valid} (DRAT: {drat}, RZAT: {rzat}, SCT: {sct})");
        }

        valid
    }

    /// Validate GPL directory and logs matches supported drive.
    fn validate_gpl(self) -> Result<bool, Error> {
        const LOG_SYSTEM_INFO: ata::log::Log = ata::log::Log::VendorSpecific(0xC2);

        // Check directory for vendor-specific system info log
        let directory = self.ata.gpl_directory()?;
        let directory_system_info = directory[LOG_SYSTEM_INFO] as usize
            == system_info::SystemInfo::SIZE.div_ceil(SECTOR_SIZE);
        debug!("[{self}] Validate GPL directory system info: {directory_system_info}");
        if !directory_system_info {
            return Ok(false);
        }

        // Read system info log
        let mut system_info = [0u8; system_info::SystemInfo::SIZE];
        self.ata.gpl_read(&mut system_info, LOG_SYSTEM_INFO, 0, 0)?;

        // Try parse system info
        let system_info_result = system_info::SystemInfo::try_from(&system_info);
        debug!("[{self}] Validate GPL system info: {system_info_result:?}");
        if system_info_result.is_err() {
            return Ok(false);
        }

        Ok(true)
    }

    /// Open drive.
    fn open(ata: &'a ata::Drive) -> Result<Option<Self>, Error> {
        let drive = Self { ata };

        // Run common Phison checks
        if !(&drive as &dyn super::Drive).validate()? {
            return Ok(None);
        }

        // Check identify device fields
        let identify = ata.identify_device()?;
        if !Self::validate_identify(&identify, Some(drive)) {
            return Ok(None);
        }

        // All S12 VUCs involve an ATA 0x31 write command, so system info
        // validated from a vendor-specific GP log instead
        if !Self::validate_gpl(drive)? {
            return Ok(None);
        }

        debug!("[{drive}] Validated");
        Ok(Some(drive))
    }

    /// Execute VUC.
    fn vuc(
        self,
        transfer: Transfer,
        operation: VucOperation,
        parameters: [u32; 4],
    ) -> Result<(), Error> {
        const FEATURE_SEND_SQ: u8 = 0xF1;
        const FEATURE_DATA_WRITE: u8 = 0xF2;
        const FEATURE_DATA_READ: u8 = 0xF3;
        const FEATURE_READ_CQ: u8 = 0xF4;

        let log_info = format!(
            "transfer: {}, parameters: {:#x} {:#x} {:#x} {:#x}",
            transfer, parameters[0], parameters[1], parameters[2], parameters[3],
        );
        debug!("[{self}] Executing VUC: {operation} ({log_info})");

        // Send submission queue
        let sq_entry = phison::nvme::vuc(&transfer, operation, parameters);
        let sq_data = <[u8; _]>::from(&sq_entry);
        (&self as &dyn super::Drive).vuc(Transfer::Write(&sq_data), FEATURE_SEND_SQ, 0)?;

        // No-data VUCs write a single sector
        let transfer = match transfer {
            Transfer::None => Transfer::Write(&[0; SECTOR_SIZE]),
            x => x,
        };
        let data_feature = if matches!(transfer, Transfer::Read(_)) {
            FEATURE_DATA_READ
        } else {
            FEATURE_DATA_WRITE
        };

        // Data transfer
        (&self as &dyn super::Drive).vuc(transfer, data_feature, 0)?;

        // Read completion queue
        let mut cq_data = [0u8; nvme::command::Completion::SIZE];
        (&self as &dyn super::Drive).vuc(Transfer::Read(&mut cq_data), FEATURE_READ_CQ, 0)?;

        let completion = nvme::command::Completion::try_from(&cq_data)?;
        info!(
            "[{self}] Executed VUC: {operation} ({log_info}, status: {}, result: {:#x})",
            completion.status, completion.result
        );

        if completion.status.is_error() {
            return Err(Error::VucStatus(completion.status));
        }

        Ok(())
    }

    /// VUC system info.
    fn vuc_system_info(self) -> Result<system_info::SystemInfo, Error> {
        let mut data = [0u8; system_info::SystemInfo::SIZE];

        self.vuc(
            Transfer::Read(&mut data),
            VucOperation::SystemInfo,
            [0, 0, 0, 0],
        )?;

        let system_info = (&data).try_into()?;
        debug!("[{self}] System info: {system_info:?}");

        Ok(system_info)
    }

    /// VUC read info block.
    fn vuc_read_info_block(self) -> Result<info_block::InfoBlock, Error> {
        let mut data = [0u8; info_block::InfoBlock::SIZE];

        self.vuc(
            Transfer::Read(&mut data),
            VucOperation::ReadInfoBlock,
            [0, 0, 0, 0],
        )?;
        let info_block = (&data).try_into()?;

        debug!("[{self}] Info block: {info_block:?}");

        Ok(info_block)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        self.vuc(
            Transfer::Read(data),
            VucOperation::ReadSram,
            [0, 0, address, 0],
        )
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

impl super::Drive for Drive<'_> {
    fn ata(&self) -> &ata::Drive {
        self.ata
    }
}

impl std::fmt::Display for Drive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{DISPLAY_NAME} {}", self.ata.path().display())
    }
}

/// S12 vendor type.
pub struct VendorType;

impl VendorType {
    /// Check read info block.
    fn check_read_info_block(drive: Drive) -> drive::CheckResult {
        let read_info_block_result = drive.vuc_read_info_block();
        debug!("[{drive}] Read info block: {read_info_block_result:?}");

        let result = match read_info_block_result {
            Ok(x) => Ok(format!("(serial: {}, model: {})", x.serial, x.model)),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult::new("Read info block".into(), result)
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
}

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        DISPLAY_NAME
    }

    /// Run checks on drive.
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

        let system_info = drive.vuc_system_info()?;

        // System Info part of the validation checks (indirect through vendor
        // log), assume it always succeeds
        results.push(drive::CheckResult::new(
            "System info".into(),
            Ok(format!(
                "(firmware: {}-{} ({}), VUC lock: {}, DRAM: {} MB (DDR{} {} MHz), flash: {} {} \
                 {}, CEs: {}, blocks per CE: {}, pages per block: {}, page size: {} KB)",
                system_info.firmware_version,
                system_info.firmware_subversion,
                system_info.firmware_date,
                system_info
                    .vuc_lock_state
                    .as_ref()
                    .map_or("N/A".into(), ToString::to_string),
                system_info.dram_size,
                system_info.dram_type,
                system_info.dram_clock,
                system_info.flash_manufacturer,
                system_info.flash_process,
                match system_info.flash_bits_per_cell {
                    1 => "SLC".to_string(),
                    2 => "MLC".to_string(),
                    3 => "TLC".to_string(),
                    4 => "QLC".to_string(),
                    x => format!("{x}-bit"),
                },
                system_info.ce_count,
                system_info.blocks_per_ce,
                system_info.pages_per_block,
                system_info.page_size,
            )),
        ));

        if matches!(
            system_info.vuc_lock_state,
            None | Some(VucLockState::Unlocked | VucLockState::NoLock)
        ) {
            // Only run other checks if VUCs unlocked
            results.push(Self::check_read_info_block(drive));
            results.push(Self::check_read_memory(drive));
        }

        Ok(Some(results.into()))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn validate_identify() {
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] =
            &[test_data::kingston_dc500r::IDENTIFY];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::kingston_a400::IDENTIFY, // S11
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
