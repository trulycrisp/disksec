//! System info parsing.

use super::Error;
use crate::protocol::ata::{SECTOR_SIZE, identify};

/// VUC system info result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SystemInfo {
    /// Number of flash CEs.
    pub(super) ce_count: u8,
    /// Number of flash channels.
    pub(super) channel_count: u8,
    /// SRAM size in MB.
    pub(super) sram_size: u8,
    /// Flash pages per block.
    pub(super) pages_per_block: u16,
    /// Flash dies per CE.
    pub(super) dies_per_ce: u8,
    /// Flash blocks per die.
    pub(super) blocks_per_die: u32,
    /// Flash blocks per CE.
    pub(super) blocks_per_ce: u32,
    /// Sectors per flash page.
    pub(super) sectors_per_page: u32,
    /// Firmware sub-version.
    pub(super) firmware_subversion: String,
    /// Total flash size divided by superblock size.
    pub(super) superblock_index_count: u32,
    /// Number of sectors per superblock.
    pub(super) sectors_per_superblock: u32,
    /// Number of usable superblocks.
    pub(super) superblock_count: u32,
    /// Firmware build date.
    pub(super) firmware_date: Option<String>,
    /// Firmware version.
    pub(super) firmware_version: Option<String>,
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
        let firmware_subversion = Self::parse_str(&data[201..203])?.into();

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
            firmware_subversion,
            superblock_index_count,
            sectors_per_superblock,
            superblock_count,
            firmware_date,
            firmware_version,
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
