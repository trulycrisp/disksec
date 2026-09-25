//! System info parsing.

use super::Error;
use crate::protocol::ata::SECTOR_SIZE;

/// VUC system info result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SystemInfo {
    /// Number of flash CEs.
    pub(super) ce_count: u8,
    /// Number of flash channels.
    pub(super) channel_count: u8,
    /// DRAM size in MB.
    pub(super) dram_size: u16,
    /// Flash pages per block.
    pub(super) pages_per_block: u16,
    /// Flash dies per CE.
    pub(super) dies_per_ce: u8,
    /// Stride between dies in a flash block address.
    pub(super) die_stride: u32,
    /// Flash blocks per die.
    pub(super) blocks_per_die: u32,
    /// Sectors per flash page.
    pub(super) sectors_per_page: u8,
    /// Flash blocks per CE.
    pub(super) blocks_per_ce: u32,
    /// Firmware build date.
    pub(super) firmware_date: String,
    /// Firmware version.
    pub(super) firmware_version: String,
    /// Firmware sub-version.
    pub(super) firmware_subversion: String,
}

impl SystemInfo {
    /// Size in bytes.
    pub(super) const SIZE: usize = SECTOR_SIZE;

    /// Parse string.
    fn parse_str(data: &[u8]) -> Result<&str, Error> {
        data.iter()
            .all(|&x| x == b' ' || x.is_ascii_graphic())
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

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
}
