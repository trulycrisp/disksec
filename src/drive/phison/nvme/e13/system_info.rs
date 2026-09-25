//! System info parsing.

use std::num::NonZero;

use super::Error;
use crate::{drive::phison::VucLockState, flash_id};

/// Flash interface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FlashInterface {
    /// Async SDR.
    Sdr = 0,
    /// Toggle DDR 1.0.
    Toggle1 = 1,
    /// Toggle DDR 2.0.
    Toggle2 = 2,
    /// NV-DDR.
    NvDdr = 3,
    /// NV-DDR2.
    NvDdr2 = 4,
}

impl FlashInterface {
    /// Parse from raw byte.
    fn parse(value: u8) -> Result<Self, Error> {
        const VARIANTS: &[FlashInterface] = &[
            FlashInterface::Sdr,
            FlashInterface::Toggle1,
            FlashInterface::Toggle2,
            FlashInterface::NvDdr,
            FlashInterface::NvDdr2,
        ];

        VARIANTS
            .iter()
            .find(|&&y| y as u8 == value)
            .copied()
            .ok_or(Error::InvalidSystemInfo)
    }
}

/// VUC system info result.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct SystemInfo {
    /// Flash CE count.
    pub(super) ce_count: u8,
    /// Flash channel count.
    pub(super) channel_count: u8,
    /// Flash CEs per channel.
    pub(super) ces_per_channel: u8,
    /// Flash manufacturer code.
    pub(super) flash_manufacturer: flash_id::Manufacturer,
    /// Flash die capacity in GB.
    pub(super) size_per_die: u8,
    /// Flash bits per cell (e.g. 3 = TLC).
    pub(super) flash_bits_per_cell: u8,
    /// Flash grade qualification tier.
    pub(super) flash_grade: u8,
    /// Flash page size in KB.
    pub(super) page_size: u8,
    /// Flash pages per block.
    pub(super) pages_per_block: u16,
    /// Flash blocks per CE.
    pub(super) blocks_per_ce: u16,
    /// Flash block size in MB.
    pub(super) block_size_mb: u8,
    /// Flash dies per CE.
    pub(super) dies_per_ce: u8,
    /// Flash planes per die.
    pub(super) planes_per_die: u8,
    /// Stride between dies in a flash block address.
    pub(super) die_stride: u16,
    /// Bitmap of active chip enable addresses.
    pub(super) ce_bitmap: u64,
    /// Flash interface type.
    pub(super) flash_interface: FlashInterface,
    /// CPU clock in MHz.
    pub(super) cpu_clock: u16,
    /// Flash clock in MHz.
    pub(super) flash_clock: u16,
    /// Unique controller hardware ID.
    pub(super) controller_id: [u8; 8],
    /// Firmware build date.
    pub(super) firmware_date: String,
    /// VUC lock has a key configured.
    pub(super) vuc_lock_key_set: bool,
    /// VUC lock state.
    pub(super) vuc_lock_state: Option<VucLockState>,
    /// VUC lock key ID.
    pub(super) vuc_lock_key: Option<NonZero<u16>>,
    /// Firmware sub-version.
    pub(super) firmware_subversion: String,
    /// Controller hardware version.
    pub(super) controller_version: String,
    /// Controller hardware revision.
    pub(super) controller_revision: u8,
    /// Mask ROM bootloader revision.
    pub(super) bootloader_revision: u8,
    /// Firmware version.
    pub(super) firmware_version: String,
    /// Firmware minor version.
    pub(super) firmware_version_minor: String,
}

impl SystemInfo {
    /// Size in bytes.
    pub(super) const SIZE: usize = 4_096;

    /// Parse NUL-terminated ASCII string.
    fn parse_str(data: &[u8]) -> Result<&str, Error> {
        let string = std::str::from_utf8(data).map_err(|_| Error::InvalidSystemInfo)?;
        let string = string.split('\0').next().unwrap_or("").trim_end();

        string
            .chars()
            .all(|x| x == ' ' || x.is_ascii_graphic())
            .then_some(string)
            .ok_or(Error::InvalidSystemInfo)
    }
}

impl TryFrom<&[u8; Self::SIZE]> for SystemInfo {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        const MAX_CE_COUNT: u8 = 16;
        const MAX_CHANNEL_COUNT: u8 = 4;
        const CONTROLLER_VERSIONS: &[&str] = &["PS5013", "CS2283"];

        let ce_count = data[0];
        if ce_count > MAX_CE_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let channel_count = data[1];
        if channel_count > MAX_CHANNEL_COUNT {
            return Err(Error::InvalidSystemInfo);
        }

        let ces_per_channel = data[2];
        let flash_manufacturer =
            flash_id::Manufacturer::try_from(data[3]).or(Err(Error::InvalidSystemInfo))?;

        let size_per_die = data[4];

        let flash_bits_per_cell = data[8] & 0xF;
        let flash_grade = data[8] >> 4;
        let page_size = data[9];
        let pages_per_block = u16::from_le_bytes([data[10], data[11]]);
        let blocks_per_ce = u16::from_le_bytes([data[12], data[13]]);
        let block_size_mb = data[14];
        let dies_per_ce = data[15];
        let planes_per_die = data[16];
        let die_stride = u16::from_le_bytes([data[20], data[21]]);

        let ce_bitmap = u64::from_le_bytes([
            data[24], data[25], data[26], data[27], data[28], data[29], data[30], data[31],
        ]);
        if ce_bitmap.count_ones() != ce_count.into() {
            return Err(Error::InvalidSystemInfo);
        }

        let flash_interface = FlashInterface::parse(data[34])?;
        let cpu_clock = u16::from_le_bytes([data[56], data[57]]);
        let flash_clock = u16::from_le_bytes([data[62], data[63]]);
        let controller_id: [_; 8] = data[116..][..8].try_into().unwrap();

        let mut firmware_date: [_; 6] = data[258..][..6].try_into().unwrap();
        firmware_date.reverse();
        if !firmware_date.iter().all(u8::is_ascii_digit) {
            return Err(Error::InvalidSystemInfo);
        }
        let firmware_date = str::from_utf8(&firmware_date).unwrap();
        let firmware_date = format!(
            "20{}-{}-{}",
            &firmware_date[..2],
            &firmware_date[2..4],
            &firmware_date[4..]
        );

        let vuc_lock_key_set = (data[377] & (1 << 1)) != 0;
        let vuc_lock_state = VucLockState::parse(data[378]).or(Err(Error::InvalidSystemInfo))?;
        let vuc_lock_key = NonZero::new(u16::from_be_bytes([data[384], data[385]]));
        let firmware_subversion = Self::parse_str(&data[396..400])?.to_string();

        let controller_version = Self::parse_str(&data[400..406])?.to_string();
        if !CONTROLLER_VERSIONS.contains(&controller_version.as_str()) {
            return Err(Error::InvalidSystemInfo);
        }

        let controller_revision = data[406];
        let bootloader_revision = data[407];

        let firmware_version = Self::parse_str(&data[410..418])?.to_string();
        let firmware_version_minor = Self::parse_str(&data[418..420])?.to_string();

        Ok(Self {
            ce_count,
            channel_count,
            ces_per_channel,
            flash_manufacturer,
            size_per_die,
            flash_bits_per_cell,
            flash_grade,
            page_size,
            pages_per_block,
            blocks_per_ce,
            block_size_mb,
            dies_per_ce,
            planes_per_die,
            die_stride,
            ce_bitmap,
            flash_interface,
            cpu_clock,
            flash_clock,
            controller_id,
            firmware_date,
            vuc_lock_key_set,
            vuc_lock_state,
            vuc_lock_key,
            firmware_subversion,
            controller_version,
            controller_revision,
            bootloader_revision,
            firmware_version,
            firmware_version_minor,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_system_info() {
        const DATA_VALID: &[&[u8; SystemInfo::SIZE]] = &[test_data::patriot_p300::SYSTEM_INFO];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[
            &[0; _],
            &[0xFF; _],
            test_data::kingston_dc500r::SYSTEM_INFO, // S12
        ];

        for &data in DATA_VALID {
            assert!(SystemInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(SystemInfo::try_from(data).is_err());
        }
    }
}
