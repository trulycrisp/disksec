//! S10 controller.

use log::debug;

use crate::{
    drive,
    protocol::{
        Transfer,
        ata::{self, SECTOR_SIZE, identify},
    },
};

/// S10 error.
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
    /// Sectors per flash page.
    sectors_per_page: u8,
    /// Flash pages per block.
    pages_per_block: u16,
    /// Flash blocks per CE.
    blocks_per_ce: u16,
    /// Flash dies per CE.
    dies_per_ce: u8,
    /// Stride between dies in a flash block address.
    die_stride: u16,
    /// Bitmap of active chip enable addresses.
    ce_bitmap: u64,
    /// DRAM size in MB.
    dram_size: u16,
    /// Firmware build date.
    firmware_date: String,
    /// Firmware sub-version revision.
    firmware_revision: String,
}

impl SystemInfo {
    /// Size in bytes.
    const SIZE: usize = SECTOR_SIZE;
}

impl TryFrom<&[u8; Self::SIZE]> for SystemInfo {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        const MAX_CE_COUNT: u8 = 32;
        const MAX_CHANNEL_COUNT: u8 = 8;

        let ce_count = data[0];
        if ce_count > MAX_CE_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let channel_count = data[1];
        if channel_count > MAX_CHANNEL_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let sectors_per_page = data[9].checked_mul(2).ok_or(Error::InvalidSystemInfo)?;
        let pages_per_block = u16::from_le_bytes([data[10], data[11]]);
        let blocks_per_ce = u16::from_le_bytes([data[12], data[13]]);
        let dies_per_ce = data[15];
        let die_stride = u16::from_le_bytes([data[20], data[21]]);

        let ce_bitmap = u64::from_le_bytes([
            data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31],
        ]);
        if ce_bitmap.count_ones() != ce_count.into() {
            return Err(Error::InvalidSystemInfo);
        }

        let dram_size = u16::from_le_bytes([data[70], data[71]]) >> 4;

        let mut firmware_info: [_; 8] = std::array::from_fn(|x| data[256 + x]);
        firmware_info.reverse();

        let firmware_date = &firmware_info[..6];
        if !firmware_date.iter().all(u8::is_ascii_digit) {
            return Err(Error::InvalidSystemInfo);
        }
        let firmware_date = str::from_utf8(firmware_date).unwrap();
        let firmware_date = format!(
            "20{}-{}-{}",
            &firmware_date[..2],
            &firmware_date[2..4],
            &firmware_date[4..]
        );

        let firmware_revision = &firmware_info[6..];
        if !firmware_revision.iter().all(u8::is_ascii_alphanumeric) {
            return Err(Error::InvalidSystemInfo);
        }
        let firmware_revision = str::from_utf8(firmware_revision).unwrap().into();

        Ok(Self {
            ce_count,
            channel_count,
            sectors_per_page,
            pages_per_block,
            blocks_per_ce,
            dies_per_ce,
            die_stride,
            ce_bitmap,
            dram_size,
            firmware_date,
            firmware_revision,
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
            acs_3: false,
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

        let drat = identify.additional_supported.is_some_and(|x| x.drat);
        let rzat = identify.additional_supported.is_some_and(|x| x.rzat);
        let major_version_match = identify.major_version == MAJOR_VERSION;
        let no_sct = !identify.sct_supported.sct;
        let valid = drat && rzat && major_version_match && no_sct;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (DRAT: {drat}, RZAT: {rzat}, major version: \
                 {major_version_match}, SCT: {no_sct})"
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
        write!(f, "Phison S10 {}", self.ata.path().display())
    }
}

/// S10 vendor type.
pub struct VendorType;

impl drive::VendorType for VendorType {
    fn name(&self) -> &str {
        const NAME: &str = "Phison S10";

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
                "(firmware: {} ({}), DRAM: {} MB, channels: {}. CEs: {}, blocks per CE: {}, pages \
                 per block: {}, sectors per page: {})",
                system_info.firmware_revision,
                system_info.firmware_date,
                system_info.dram_size,
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
            test_data::patriot_blast::IDENTIFY,
            test_data::ocz_trion150::IDENTIFY,
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
            test_data::patriot_blast::SYSTEM_INFO,
            test_data::ocz_trion150::SYSTEM_INFO,
        ];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[
            &[0; _],
            test_data::corsair_nova2::SYSTEM_INFO,      // S5
            test_data::kingston_ssdnow100::SYSTEM_INFO, // S8
            test_data::patriot_blaze::SYSTEM_INFO,      // S9
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
