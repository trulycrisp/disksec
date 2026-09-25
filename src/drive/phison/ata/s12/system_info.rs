//! System info parsing.

use std::num::NonZero;

use super::Error;
use crate::drive::phison::VucLockState;
use crate::flash_id;
use crate::protocol::ata::identify;

/// Flash interface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum FlashInterface {
    /// Async SDR.
    Sdr = 0,
    /// Toggle DDR 1.0.
    Toggle1 = 1,
    ///  Toggle DDR 2.0.
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
    /// Flash die process.
    pub(super) flash_process: String,
    /// Flash bits per cell (e.g. 3 = TLC)
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
    /// FTL virtual block count
    pub(super) virtual_block_count: u16,
    /// CPU clock in MHz.
    pub(super) cpu_clock: u16,
    /// DRAM clock in MHz.
    pub(super) dram_clock: u16,
    /// DRAM DDR type (DDR<x>).
    pub(super) dram_type: u8,
    /// Drive form factor.
    pub(super) form_factor: Option<identify::FormFactor>,
    /// DRAM size in MB.
    pub(super) dram_size: u32,
    /// Unique controller hardware ID.
    pub(super) controller_id: [u8; Self::CONTROLLER_ID_SIZE],
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
    /// Flash maximum rated program/erase cycles.
    pub(super) flash_max_pe_cycles: u16,
    /// Flash read clock in MT/s.
    pub(super) flash_read_clock: u16,
    /// Flash write clock in MT/s.
    pub(super) flash_write_clock: u16,
}

impl SystemInfo {
    /// Controller ID field size in bytes.
    const CONTROLLER_ID_SIZE: usize = 8;
    /// Size in bytes.
    pub(super) const SIZE: usize = 4_096;

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
        const MAX_CE_COUNT: u8 = 32;
        const MAX_CHANNEL_COUNT: u8 = 8;
        const CONTROLLER_VERSION: &str = "PS3112";
        const FIRMWARE_VERSION_PREFIX: &str = "SC";

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

        let flash_process = format!(
            "{}{}{}{}",
            Self::parse_str(&data[7..8])?,
            data[6],
            Self::parse_str(&data[5..6])?,
            Self::parse_str(&data[4..5])?,
        )
        .trim_end()
        .to_string();

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
        let virtual_block_count = u16::from_le_bytes([data[38], data[39]]);
        let cpu_clock = u16::from_le_bytes([data[56], data[57]]);
        let dram_clock = u16::from_le_bytes([data[64], data[65]]);
        let dram_type = data[69];

        let form_factor =
            identify::FormFactor::parse(data[71]).or(Err(Error::InvalidSystemInfo))?;

        let dram_size = u32::from_le_bytes([data[72], data[73], data[74], data[75]]);
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
        if controller_version != CONTROLLER_VERSION {
            return Err(Error::InvalidSystemInfo);
        }

        let controller_revision = data[406];
        let bootloader_revision = data[407];

        let firmware_version = Self::parse_str(&data[410..418])?.to_string();
        if !firmware_version.starts_with(FIRMWARE_VERSION_PREFIX) {
            return Err(Error::InvalidSystemInfo);
        }

        let firmware_version_minor = Self::parse_str(&data[418..420])?.to_string();
        let flash_max_pe_cycles = u16::from_le_bytes([data[652], data[653]]);
        let flash_read_clock = u16::from_le_bytes([data[2944], data[2945]]);
        let flash_write_clock = u16::from_le_bytes([data[2946], data[2947]]);

        Ok(Self {
            ce_count,
            channel_count,
            ces_per_channel,
            flash_manufacturer,
            flash_process,
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
            virtual_block_count,
            cpu_clock,
            dram_clock,
            dram_type,
            form_factor,
            dram_size,
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
            flash_max_pe_cycles,
            flash_read_clock,
            flash_write_clock,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_system_info() {
        const DATA_VALID: &[&[u8; SystemInfo::SIZE]] = &[test_data::kingston_dc500r::SYSTEM_INFO];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[
            &[0; _],
            &[0xFF; _],
            test_data::patriot_p300::SYSTEM_INFO, // E13
        ];

        for &data in DATA_VALID {
            assert!(SystemInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(SystemInfo::try_from(data).is_err());
        }
    }
}
