//! System info parsing.

use std::num::NonZero;

use super::Error;
use crate::drive::phison::VucLockState;
use crate::protocol::ata::SECTOR_SIZE;

/// VUC system info result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SystemInfo {
    /// Number of flash CEs.
    pub(super) ce_count: u8,
    /// Number of flash channels.
    pub(super) channel_count: u8,
    /// Sectors per flash page.
    pub(super) sectors_per_page: u8,
    /// Flash pages per block.
    pub(super) pages_per_block: u16,
    /// Flash blocks per CE.
    pub(super) blocks_per_ce: u16,
    /// Flash dies per CE.
    pub(super) dies_per_ce: u8,
    /// Stride between dies in a flash block address.
    pub(super) die_stride: u16,
    /// Bitmap of active chip enable addresses.
    pub(super) ce_bitmap: u64,
    /// DRAM size in MB.
    pub(super) dram_size: u16,
    /// VUC lock state.
    pub(super) vuc_lock_state: Option<VucLockState>,
    /// VUC lock key ID.
    pub(super) vuc_lock_key: Option<NonZero<u16>>,
    /// Firmware build date.
    pub(super) firmware_date: String,
    /// Firmware sub-version.
    pub(super) firmware_subversion: String,
    /// VUC lock has a key configured.
    pub(super) vuc_lock_key_set: bool,
}

impl SystemInfo {
    /// Size in bytes.
    pub(super) const SIZE: usize = SECTOR_SIZE;
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

        let vuc_lock_state = VucLockState::parse(data[248]).or(Err(Error::InvalidSystemInfo))?;
        let vuc_lock_key = NonZero::new(u16::from_be_bytes([data[250], data[251]]));

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

        let firmware_subversion = &firmware_info[6..];
        if !firmware_subversion.iter().all(u8::is_ascii_alphanumeric) {
            return Err(Error::InvalidSystemInfo);
        }
        let firmware_subversion = str::from_utf8(firmware_subversion).unwrap().into();

        let vuc_lock_key_set = (data[367] & (1 << 2)) != 0;

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
            vuc_lock_state,
            vuc_lock_key,
            firmware_date,
            firmware_subversion,
            vuc_lock_key_set,
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
