//! S8 controller.

use log::{debug, info};

use crate::{
    cpu::{VECTOR_TABLE_SIZE, arcompact},
    drive,
    protocol::{
        Transfer,
        ata::{self, SECTOR_SIZE, identify},
    },
};

/// Display name of drive type.
const DISPLAY_NAME: &str = "Phison S8";
/// Memory address of `ARCompact` vector table.
const VECTOR_TABLE_ADDRESS: u32 = 0;

/// S8 error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// Invalid system info data.
    InvalidSystemInfo,
    /// Invalid info block data.
    InvalidInfoBlock,
    /// Invalid VUC read/write register size.
    InvalidRegisterSize(usize),
    /// Invalid `ARCompact` vector table in memory.
    InvalidVectorTable,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::InvalidSystemInfo => write!(f, "invalid system info"),
            Self::InvalidInfoBlock => write!(f, "invalid info block"),
            Self::InvalidRegisterSize(x) => write!(f, "invalid register size {x}"),
            Self::InvalidVectorTable => write!(f, "invalid ARCompact vector table"),
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

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Ata(x) => Self::Ata(x),
            x => Self::Vendor(Box::new(x)),
        }
    }
}

/// VUC operation in feature register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VucOperation {
    /// Read system information.
    SystemInfo = 0x13,
    /// Set parameter data for following VUC.
    SetParameter = 0x24,
    /// Read drive info block (configuration data).
    ReadInfoBlock = 0x28,
    /// Read memory value (intended for hardware registers).
    ReadRegister = 0x61,
}

impl std::fmt::Display for VucOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemInfo => write!(f, "system info"),
            Self::SetParameter => write!(f, "set parameter"),
            Self::ReadInfoBlock => write!(f, "read info block"),
            Self::ReadRegister => write!(f, "read register"),
        }
    }
}

/// VUC system info result.
#[derive(Clone, Debug, PartialEq, Eq)]
struct SystemInfo {
    /// Number of flash CEs.
    ce_count: u8,
    /// Number of flash channels.
    channel_count: u8,
    /// DRAM size in MB.
    dram_size: u16,
    /// Flash pages per block.
    pages_per_block: u16,
    /// Flash dies per CE.
    dies_per_ce: u8,
    /// Stride between dies in a flash block address.
    die_stride: u32,
    /// Flash blocks per die.
    blocks_per_die: u32,
    /// Sectors per flash page.
    sectors_per_page: u8,
    /// Flash blocks per CE.
    blocks_per_ce: u32,
    /// Firmware build date.
    firmware_date: String,
    /// Firmware version.
    firmware_version: String,
    /// Firmware sub-version.
    firmware_subversion: String,
}

impl SystemInfo {
    /// Size in bytes.
    const SIZE: usize = SECTOR_SIZE;

    /// Parse string.
    fn parse_str(data: &[u8]) -> Result<&str, Error> {
        data.iter()
            .all(|x| x.is_ascii_graphic() || x.is_ascii_whitespace())
            .then(|| std::str::from_utf8(data).unwrap())
            .ok_or(Error::InvalidSystemInfo)
    }
}

impl TryFrom<&[u8; Self::SIZE]> for SystemInfo {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        const MAX_CE_COUNT: u8 = 64;
        const MAX_CHANNEL_COUNT: u8 = 8;
        const FIRMWARE_DATE_PREFIX: &str = "20";
        const FIRMWARE_VERSION_PREFIX: &str = "S8";

        let ce_count = data[0];
        if ce_count > MAX_CE_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let channel_count = data[2];
        if channel_count > MAX_CHANNEL_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let dram_size = 1u16
            .checked_shl(data[3].into())
            .ok_or(Error::InvalidSystemInfo)?;

        let pages_per_block = u16::from_le_bytes([data[4], data[5]]);
        let dies_per_ce = data[6];
        let die_stride = u32::from_le_bytes([data[12], data[13], data[14], data[15]]);
        let blocks_per_die = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let sectors_per_page = data[24].checked_mul(4).ok_or(Error::InvalidSystemInfo)?;
        let blocks_per_ce = u32::from_le_bytes([data[36], data[37], data[38], data[39]]);

        let firmware_date = format!(
            "{}{} {} {} {}",
            Self::parse_str(&data[464..465])?,
            Self::parse_str(&data[467..470])?,
            Self::parse_str(&data[470..473])?,
            Self::parse_str(&data[474..476])?.trim(),
            Self::parse_str(&data[480..488])?,
        );
        if !firmware_date.starts_with(FIRMWARE_DATE_PREFIX) {
            return Err(Error::InvalidSystemInfo);
        }

        let firmware_version = Self::parse_str(&data[496..504])?.to_string();
        if !firmware_version.starts_with(FIRMWARE_VERSION_PREFIX) {
            return Err(Error::InvalidSystemInfo);
        }

        let firmware_subversion = Self::parse_str(&data[504..506])?.to_string();

        Ok(Self {
            ce_count,
            channel_count,
            dram_size,
            pages_per_block,
            dies_per_ce,
            die_stride,
            blocks_per_die,
            sectors_per_page,
            blocks_per_ce,
            firmware_date,
            firmware_version,
            firmware_subversion,
        })
    }
}

/// Info block (drive configuration).
#[derive(Clone, Debug, PartialEq, Eq)]
struct InfoBlock {
    /// Serial number.
    serial: String,
    /// Model.
    model: String,
}

impl InfoBlock {
    /// Size in bytes.
    const SIZE: usize = SECTOR_SIZE;
}

impl TryFrom<&[u8; Self::SIZE]> for InfoBlock {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        if !data.starts_with(super::INFO_BLOCK_MAGIC) {
            return Err(Error::InvalidInfoBlock);
        }

        let serial = identify::parse_string(&data[16..36]).or(Err(Error::InvalidInfoBlock))?;
        let model = identify::parse_string(&data[36..76]).or(Err(Error::InvalidInfoBlock))?;

        Ok(Self { serial, model })
    }
}

/// Drive interface.
#[derive(Clone, Copy, Debug)]
struct Drive<'a> {
    /// ATA drive.
    ata: &'a ata::Drive,
}

impl Drive<'_> {
    /// Validate identify device result matches supported drive.
    fn validate_identify(identify: &identify::IdentifyDevice, drive: Option<Self>) -> bool {
        const MAJOR_VERSION: Option<identify::MajorVersion> = Some(identify::MajorVersion {
            acs_5: false,
            acs_4: false,
            acs_3: false,
            acs_2: false,
            ata8_acs: true,
            ata_atapi_7: true,
            ata_atapi_6: true,
            ata_atapi_5: true,
            ata_atapi_4: true,
            ata_3: true,
            ata_2: false,
            ata_1: false,
        });

        let no_drat = identify.additional_supported.is_some_and(|x| !x.drat);
        let no_rzat = identify.additional_supported.is_some_and(|x| !x.rzat);
        let major_version_match = identify.major_version == MAJOR_VERSION;
        let no_sct = !identify.sct_supported.sct;
        let valid = no_drat && no_rzat && major_version_match && no_sct;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (DRAT: {no_drat}, RZAT: {no_rzat}, major \
                 version: {major_version_match}, SCT: {no_sct})"
            );
        }

        valid
    }

    /// Validate supported drive.
    fn validate(self) -> Result<bool, Error> {
        // Run common Phison checks
        if !(&self as &dyn super::Drive).validate()? {
            return Ok(false);
        }

        // Check identify device fields
        let identify = self.ata.identify_device()?;
        if !Self::validate_identify(&identify, Some(self)) {
            return Ok(false);
        }

        // Check VUC system info
        let system_info_result = self.vuc_system_info();
        debug!("[{self}] Validate VUC system info: {system_info_result:?}");
        let system_info = match system_info_result {
            Ok(_) => true,
            Err(Error::Ata(x)) if x.is_command_error() => false,
            Err(Error::InvalidSystemInfo) => false,
            Err(x) => return Err(x),
        };
        if !system_info {
            return Ok(false);
        }

        debug!("[{self}] Validated");
        Ok(true)
    }

    /// Execute VUC.
    fn vuc(self, transfer: Transfer, operation: VucOperation, lba: u32) -> Result<(), ata::Error> {
        let log_info = format!("{operation} (transfer: {transfer}, lba: {lba:#x})");
        debug!("[{self}] Executing VUC: {log_info}");

        (&self as &dyn super::Drive).vuc(transfer, operation as _, lba)?;

        info!("[{self}] Executed VUC: {log_info}");

        Ok(())
    }

    /// VUC system info.
    fn vuc_system_info(self) -> Result<SystemInfo, Error> {
        let mut data = [0u8; SystemInfo::SIZE];

        self.vuc(Transfer::Read(&mut data), VucOperation::SystemInfo, 0)?;

        let system_info = (&data).try_into()?;
        debug!("[{self}] System info: {system_info:?}");

        Ok(system_info)
    }

    /// VUC read info block.
    fn vuc_read_info_block(self) -> Result<InfoBlock, Error> {
        let mut data = [0u8; InfoBlock::SIZE];

        self.vuc(Transfer::Read(&mut data), VucOperation::ReadInfoBlock, 0)?;
        let info_block = (&data).try_into()?;

        debug!("[{self}] Info block: {info_block:?}");

        Ok(info_block)
    }

    /// VUC read register.
    fn vuc_read_register(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const SIZES: &[usize] = &[1, 2, 4];
        const RESULT_OFFSET: usize = size_of::<u32>();

        let size = data.len();
        if !SIZES.contains(&size) {
            return Err(Error::InvalidRegisterSize(size));
        }

        let parameter = address.to_le_bytes();
        self.vuc(Transfer::Write(&parameter), VucOperation::SetParameter, 0)?;

        let mut buffer = [0u8; SECTOR_SIZE];
        let lba = u32::try_from(size << 8).unwrap();
        self.vuc(Transfer::Read(&mut buffer), VucOperation::ReadRegister, lba)?;

        data.copy_from_slice(&buffer[RESULT_OFFSET..RESULT_OFFSET + size]);

        Ok(())
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        const REGISTER_SIZE: usize = 4;

        for (index, chunk) in data.chunks_mut(REGISTER_SIZE).enumerate() {
            let register_address = address + u32::try_from(index * REGISTER_SIZE).unwrap();
            let mut buffer = [0u8; REGISTER_SIZE];

            self.vuc_read_register(register_address, &mut buffer)?;

            chunk.copy_from_slice(&buffer[..chunk.len()]);
        }

        Ok(())
    }

    /// Read `ARCompact` vector table from controller memory.
    fn read_vector_table(self) -> Result<[u8; VECTOR_TABLE_SIZE], Error> {
        let mut data = [0; _];
        self.read_memory(VECTOR_TABLE_ADDRESS, &mut data)?;

        if !arcompact::is_vector_table(&data) {
            return Err(Error::InvalidVectorTable);
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

/// S8 vendor type.
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
        let result = match drive.read_vector_table() {
            Ok(_) => Ok(format!(
                "ARCompact vector table at {VECTOR_TABLE_ADDRESS:#010x}"
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
        let drive = match drive {
            drive::Drive::Ata(ata) => Drive { ata },
            drive::Drive::Scsi(_) => return Ok(None),
        };

        if !drive.validate()? {
            return Ok(None);
        }

        let mut results = Vec::new();

        let system_info = drive.vuc_system_info()?;

        // VUC System Info part of the validation checks, assume it always succeeds
        results.push(drive::CheckResult::new(
            "System info".into(),
            Ok(format!(
                "(firmware: {}-{} ({}), DRAM: {} MB, channels: {}, CEs: {}, blocks per CE: {}, \
                 pages per block: {}, sectors per page: {})",
                system_info.firmware_version,
                system_info.firmware_subversion,
                system_info.firmware_date,
                system_info.dram_size,
                system_info.channel_count,
                system_info.ce_count,
                system_info.blocks_per_ce,
                system_info.pages_per_block,
                system_info.sectors_per_page
            )),
        ));

        results.push(Self::check_read_info_block(drive));
        results.push(Self::check_read_memory(drive));

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
            test_data::kingston_ssdnow100::IDENTIFY,
            test_data::apacer_sfd25hm::IDENTIFY,
        ];
        const DATA_INVALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::kingston_dc500r::IDENTIFY, // S12
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

    #[test]
    fn parse_system_info() {
        const DATA_VALID: &[&[u8; SystemInfo::SIZE]] = &[
            test_data::kingston_ssdnow100::SYSTEM_INFO,
            test_data::apacer_sfd25hm::SYSTEM_INFO,
        ];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[
            &[0; _],
            test_data::corsair_nova2::SYSTEM_INFO, // S5
            test_data::patriot_blaze::SYSTEM_INFO, // S9
            test_data::ocz_trion150::SYSTEM_INFO,  // S10
            test_data::kingston_a400::SYSTEM_INFO, // S11
        ];

        for &data in DATA_VALID {
            assert!(SystemInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(SystemInfo::try_from(data).is_err());
        }
    }

    #[test]
    fn parse_info_block() {
        const DATA_VALID: &[&[u8; InfoBlock::SIZE]] = &[
            test_data::kingston_ssdnow100::INFO_BLOCK,
            test_data::apacer_sfd25hm::INFO_BLOCK,
        ];
        const DATA_INVALID: &[&[u8; InfoBlock::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(InfoBlock::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(InfoBlock::try_from(data).is_err());
        }
    }
}
