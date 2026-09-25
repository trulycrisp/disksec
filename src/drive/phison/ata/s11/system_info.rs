//! System info parsing.

use std::num::NonZero;

use super::Error;
use crate::drive::phison::VucLockState;
use crate::protocol::ata::{SECTOR_SIZE, identify};

/// Flash interface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FlashInterface {
    /// Async SDR.
    Sdr = 1,
    /// Toggle DDR 1.0.
    Toggle1 = 2,
    ///  Toggle DDR 2.0.
    Toggle2 = 3,
    /// NV-DDR.
    NvDdr = 4,
    /// NV-DDR2.
    NvDdr2 = 5,
    /// NV-DDR3.
    NvDdr3 = 8,
}

impl FlashInterface {
    /// Parse from raw byte.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[FlashInterface] = &[
            FlashInterface::Sdr,
            FlashInterface::Toggle1,
            FlashInterface::Toggle2,
            FlashInterface::NvDdr,
            FlashInterface::NvDdr2,
            FlashInterface::NvDdr3,
        ];

        Ok(match value {
            0 | 0xFF => None,
            x => Some(
                VARIANTS
                    .iter()
                    .find(|&&y| y as u8 == x)
                    .copied()
                    .ok_or(Error::InvalidSystemInfo)?,
            ),
        })
    }
}

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
    /// Die-level interleaving enabled, requires multi-die flash.
    pub(super) die_interleave: bool,
    /// Number of die groups a CE is split into for interleaving.
    pub(super) die_interleave_factor: u8,
    /// Flash blocks per die.
    pub(super) blocks_per_die: u32,
    /// Flash blocks per CE.
    pub(super) blocks_per_ce: u32,
    /// Sectors per flash page.
    pub(super) sectors_per_page: u32,
    /// Number of blocks per channel in a superblock if any.
    pub(super) channel_interleave_factor: u8,
    /// Flash planes per die.
    pub(super) planes_per_die: u8,
    /// Drive form factor.
    pub(super) form_factor: Option<identify::FormFactor>,
    /// Flash blocks per superblock.
    pub(super) blocks_per_superblock: u8,
    /// Firmware sub-version.
    pub(super) firmware_subversion: String,
    /// Stride between dies in a flash block address.
    pub(super) die_stride: Option<NonZero<u16>>,
    ///  Total flash size divided by superblock size.
    pub(super) superblock_index_count: u32,
    /// Number of sectors per superblock.
    pub(super) sectors_per_superblock: u32,
    /// Number of usable superblocks.
    pub(super) superblock_count: u32,
    /// Bitmask of chip enable addresses with errors.
    pub(super) ce_error_bitmap: u64,
    /// Sectors per superblock page.
    pub(super) sectors_per_superblock_page: u32,
    /// Flash interface type.
    pub(super) flash_interface: Option<FlashInterface>,
    /// Protected mode active.
    pub(super) protected_mode: bool,
    /// Total firmware update count.
    pub(super) firmware_update_count: u16,
    /// Total system unit (drive config data) update count.
    pub(super) system_unit_update_count: u16,
    /// Bitmap of active chip enable addresses.
    pub(super) ce_bitmap: u64,
    /// Firmware version.
    pub(super) firmware_version: String,
    /// Firmware build date.
    pub(super) firmware_date: String,
    /// VUC PRAM icode program is complete.
    pub(super) pram_icode_programmed: bool,
    /// Page size including metadata if any.
    pub(super) physical_page_size: Option<NonZero<u16>>,
    /// VUC lock has a key configured.
    pub(super) vuc_lock_key_set: bool,
    /// VUC lock key ID.
    pub(super) vuc_lock_key: Option<NonZero<u16>>,
    /// VUC lock state.
    pub(super) vuc_lock_state: Option<VucLockState>,
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
        const MAX_CHANNEL_COUNT: u8 = 2;
        const SRAM_SIZE: u8 = 32;
        const FIRMWARE_VERSION_PREFIX: &str = "SB";
        const FIRMWARE_DATE_PREFIX: &str = "20";

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
        if sram_size != SRAM_SIZE {
            return Err(Error::InvalidSystemInfo);
        }

        let pages_per_block = u16::from_le_bytes([data[4], data[5]]);
        let dies_per_ce = data[6];
        let die_interleave = data[10] != 0;
        let die_interleave_factor = data[11];
        let blocks_per_die = u32::from_le_bytes([data[16], data[17], data[18], data[19]]);
        let blocks_per_ce = u32::from_le_bytes([data[28], data[29], data[30], data[31]]);
        let sectors_per_page = u32::from_le_bytes([data[40], data[41], data[42], data[43]]);
        let channel_interleave_factor = data[72];
        let planes_per_die = data[73];

        let form_factor =
            identify::FormFactor::parse(data[75]).or(Err(Error::InvalidSystemInfo))?;

        let blocks_per_superblock = data[200];
        let firmware_subversion = Self::parse_str(&data[201..203])?.into();

        let die_stride = NonZero::new(u16::from_le_bytes([data[204], data[205]]));
        let superblock_index_count =
            u32::from_le_bytes([data[208], data[209], data[210], data[211]]);

        let sectors_per_superblock =
            u32::from_le_bytes([data[212], data[213], data[214], data[215]]);

        let superblock_count = u32::from_le_bytes([data[216], data[217], data[218], data[219]]);

        let ce_error_bitmap = u64::from_le_bytes([
            data[236], data[237], data[238], data[239], data[462], data[463], data[464], data[465],
        ]);

        let sectors_per_superblock_page =
            u32::from_le_bytes([data[240], data[241], data[242], data[243]]);

        let flash_interface = FlashInterface::parse(data[244])?;
        let protected_mode = data[247] != 0;
        let firmware_update_count = u16::from_le_bytes([data[248], data[249]]);
        let system_unit_update_count = u16::from_le_bytes([data[250], data[251]]);

        let ce_bitmap = u64::from_le_bytes([
            data[328], data[329], data[330], data[331], data[332], data[333], data[334], data[335],
        ]);
        if ce_bitmap.count_ones() != ce_count.into() {
            return Err(Error::InvalidSystemInfo);
        }

        let firmware_version =
            identify::parse_string(&data[344..352]).or(Err(Error::InvalidSystemInfo))?;
        if !firmware_version.starts_with(FIRMWARE_VERSION_PREFIX) {
            return Err(Error::InvalidSystemInfo);
        }

        let firmware_date = format!(
            "{} {} {}",
            Self::parse_str(&data[352..356])?,
            Self::parse_str(&data[356..359])?,
            Self::parse_str(&data[359..361])?.trim(),
        );
        if !firmware_date.starts_with(FIRMWARE_DATE_PREFIX) {
            return Err(Error::InvalidSystemInfo);
        }

        let pram_icode_programmed = data[363] != 0;
        let physical_page_size = NonZero::new(u16::from_le_bytes([data[418], data[419]]));

        let vuc_lock_key_set = (data[423] & (1 << 3)) != 0;
        let vuc_lock_key = NonZero::new(u16::from_be_bytes([data[424], data[425]]));
        let vuc_lock_state = VucLockState::parse(data[426]).or(Err(Error::InvalidSystemInfo))?;

        Ok(Self {
            ce_count,
            channel_count,
            sram_size,
            pages_per_block,
            dies_per_ce,
            die_interleave,
            die_interleave_factor,
            blocks_per_die,
            blocks_per_ce,
            sectors_per_page,
            channel_interleave_factor,
            planes_per_die,
            form_factor,
            blocks_per_superblock,
            firmware_subversion,
            die_stride,
            superblock_index_count,
            sectors_per_superblock,
            superblock_count,
            ce_error_bitmap,
            sectors_per_superblock_page,
            flash_interface,
            protected_mode,
            firmware_update_count,
            system_unit_update_count,
            ce_bitmap,
            firmware_version,
            firmware_date,
            pram_icode_programmed,
            physical_page_size,
            vuc_lock_key_set,
            vuc_lock_key,
            vuc_lock_state,
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
            test_data::kingston_a400::SYSTEM_INFO,
            test_data::inland_professional::SYSTEM_INFO,
        ];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[
            &[0; _],
            &[0xFF; _],
            test_data::corsair_nova2::SYSTEM_INFO,      // S5
            test_data::kingston_ssdnow100::SYSTEM_INFO, // S8
            test_data::patriot_blaze::SYSTEM_INFO,      // S9
            test_data::ocz_trion150::SYSTEM_INFO,       // S10
        ];

        for &data in DATA_VALID {
            assert!(SystemInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(SystemInfo::try_from(data).is_err());
        }
    }
}
