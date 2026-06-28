//! S9 controller.

use log::debug;

use crate::{
    drive,
    protocol::{
        Transfer,
        ata::{self, SECTOR_SIZE, identify},
    },
};

/// S9 error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// Invalid VUC system info data.
    InvalidSystemInfo,
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Ata(x) => Some(x),
            Self::InvalidSystemInfo => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::InvalidSystemInfo => write!(f, "invalid system info"),
        }
    }
}

impl drive::VendorError for Error {}

impl From<ata::Error> for Error {
    fn from(value: ata::Error) -> Self {
        Self::Ata(value)
    }
}

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Ata(x) => Self::Ata(x),
            x @ Error::InvalidSystemInfo => Self::Vendor(Box::new(x)),
        }
    }
}

/// VUC operation in feature register.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VucFeature {
    /// Read system information.
    SystemInfo = 0x13,
}

/// VUC system info result.
#[derive(Clone, Debug, PartialEq, Eq)]
struct SystemInfo {
    /// Number of flash CEs.
    ce_count: u8,
    /// Number of flash channels.
    channel_count: u8,
    /// SRAM size in MB.
    sram_size: u8,
    /// Flash pages per block.
    pages_per_block: u16,
    /// Flash dies per CE.
    dies_per_ce: u8,
    /// Flash blocks per die.
    blocks_per_die: u32,
    /// Flash blocks per CE.
    blocks_per_ce: u32,
    /// Sectors per flash page.
    sectors_per_page: u32,
    /// Firmware sub-version revision.
    firmware_revision: String,
    /// Total flash size divided by superblock size.
    superblock_index_count: u32,
    /// Number of sectors per superblock.
    sectors_per_superblock: u32,
    /// Number of usable superblocks.
    superblock_count: u32,
    /// Firmware build date.
    firmware_date: Option<String>,
    /// Firmware version.
    firmware_version: Option<String>,
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
        const MAX_CE_COUNT: u8 = 16;
        const MAX_CHANNEL_COUNT: u8 = 4;
        const SRAM_SIZES: &[u8] = &[8, 32];
        const FIRMWARE_DATE_PREFIX: &str = "20";
        const FIRMWARE_VERSION_PREFIX: &str = "S9";

        let ce_count = data[0];
        if ce_count > MAX_CE_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let channel_count = data[2];
        if channel_count > MAX_CHANNEL_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let sram_size = 1u8
            .checked_shl(data[3].into())
            .ok_or(Error::InvalidSystemInfo)?;
        if !SRAM_SIZES.contains(&sram_size) {
            return Err(Error::InvalidSystemInfo);
        }

        let pages_per_block = u16::from_le_bytes([data[4], data[5]]);
        let dies_per_ce = data[6];
        let blocks_per_die = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let blocks_per_ce = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let sectors_per_page = u32::from_le_bytes([data[40], data[41], data[42], data[43]]);
        let firmware_revision = Self::parse_str(&data[201..203])?.into();

        let superblock_index_count =
            u32::from_le_bytes([data[208], data[209], data[210], data[211]]);
        let sectors_per_superblock =
            u32::from_le_bytes([data[212], data[213], data[214], data[215]]);
        let superblock_count = u32::from_le_bytes([data[216], data[217], data[218], data[219]]);

        let firmware_date = match &data[332..341] {
            x if x.iter().all(|&y| y == 0) => None,
            x => {
                let parsed = Self::parse_str(x)?;

                if !parsed.starts_with(FIRMWARE_DATE_PREFIX) {
                    return Err(Error::InvalidSystemInfo);
                }

                Some(format!(
                    "{} {} {}",
                    &parsed[..4],
                    &parsed[4..7],
                    parsed[7..].trim()
                ))
            },
        };

        let firmware_version = match &data[344..352] {
            x if x.iter().all(|&y| y == 0) => None,
            x => Some(identify::parse_string(x).or(Err(Error::InvalidSystemInfo))?),
        };
        if firmware_version
            .as_ref()
            .is_some_and(|x| !x.starts_with(FIRMWARE_VERSION_PREFIX))
        {
            return Err(Error::InvalidSystemInfo);
        }

        Ok(Self {
            ce_count,
            channel_count,
            sram_size,
            pages_per_block,
            dies_per_ce,
            blocks_per_die,
            blocks_per_ce,
            sectors_per_page,
            firmware_revision,
            superblock_index_count,
            sectors_per_superblock,
            superblock_count,
            firmware_date,
            firmware_version,
        })
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
            acs_3: true,
            acs_2: true,
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
    fn vuc(self, transfer: Transfer, feature: VucFeature, lba: u32) -> Result<(), ata::Error> {
        (&self as &dyn super::Drive).vuc(transfer, feature as _, lba)
    }

    /// VUC system info.
    fn vuc_system_info(self) -> Result<SystemInfo, Error> {
        let mut data = [0u8; SystemInfo::SIZE];

        self.vuc(Transfer::Read(&mut data), VucFeature::SystemInfo, 0)?;

        let system_info = (&data).try_into()?;
        debug!("[{self}] System info: {system_info:?}");

        Ok(system_info)
    }
}

impl super::Drive for Drive<'_> {
    fn ata(&self) -> &ata::Drive {
        self.ata
    }
}

impl std::fmt::Display for Drive<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Phison S9 {}", self.ata.path().display())
    }
}

/// S9 vendor type.
pub struct VendorType;

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        const NAME: &str = "Phison S9";

        NAME
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
        results.push(drive::CheckResult {
            name: "System info".into(),
            result: format!(
                "(firmware: {}-{} ({}), SRAM: {} MB, channels: {}, CEs: {}, blocks per CE: {}, \
                 pages per block: {}, sectors per page: {})",
                system_info.firmware_version.unwrap_or("N/A".into()),
                system_info.firmware_revision,
                system_info.firmware_date.unwrap_or("N/A".into()),
                system_info.sram_size,
                system_info.channel_count,
                system_info.ce_count,
                system_info.blocks_per_ce,
                system_info.pages_per_block,
                system_info.sectors_per_page
            ),
        });

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
            test_data::patriot_blaze::IDENTIFY,
            test_data::phison_s9::IDENTIFY,
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
            test_data::patriot_blaze::SYSTEM_INFO,
            test_data::phison_s9::SYSTEM_INFO,
        ];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[
            &[0; _],
            test_data::corsair_nova2::SYSTEM_INFO,      // S5
            test_data::kingston_ssdnow100::SYSTEM_INFO, // S8
            test_data::ocz_trion150::SYSTEM_INFO,       // S10
            test_data::kingston_a400::SYSTEM_INFO,      // S11
        ];

        for &data in DATA_VALID {
            assert!(SystemInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(SystemInfo::try_from(data).is_err());
        }
    }
}
