//! S12 controller.

use std::num::NonZero;

use log::{debug, info};

use super::VucLockState;
use crate::{
    cpu::{VECTOR_TABLE_SIZE, arm32},
    drive, flash_id,
    protocol::{
        Transfer,
        ata::{self, SECTOR_SIZE, identify},
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
    /// VUC status code.
    VucStatus(u16),
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
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::VucStatus(x) => write!(f, "VUC status {x:#x}"),
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

impl From<Error> for drive::Error {
    fn from(value: Error) -> Self {
        match value {
            Error::Ata(x) => Self::Ata(x),
            x => Self::Vendor(Box::new(x)),
        }
    }
}

/// Parse string.
fn parse_str(data: &[u8]) -> Option<&str> {
    data.iter()
        .all(|x| x.is_ascii_graphic() || x.is_ascii_whitespace())
        .then(|| std::str::from_utf8(data).unwrap())
}

/// VUC operation code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VucOperation {
    /// Read system information.
    SystemInfo = 0x80,
    /// Read data from memory address (intended for SRAM).
    ReadSram = 0xA0,
    /// Read drive info block (configuration data).
    ReadInfoBlock = 0xB1,
}

impl std::fmt::Display for VucOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemInfo => write!(f, "system info"),
            Self::ReadSram => write!(f, "read SRAM"),
            Self::ReadInfoBlock => write!(f, "read info block"),
        }
    }
}

/// Vendor Unique Command descriptor.
/// S12 is based on the E12 `NVMe` controller, so VUCs use structure of NVM
/// commands.
#[derive(Debug, PartialEq, Eq)]
struct Vuc<'a> {
    /// Data transfer.
    transfer: Transfer<'a>,
    /// Operation code.
    operation: VucOperation,
    /// CDW11 parameter.
    cdw11: u32,
    /// CDW12 (high 24 bits) parameter.
    cdw12: u32,
    /// CDW13 parameter.
    cdw13: u32,
    /// CDW14 parameter.
    cdw14: u32,
}

impl<'a> Vuc<'a> {
    /// Size in bytes.
    const SIZE: usize = 64;

    /// Construct VUC.
    fn new(
        transfer: Transfer<'a>,
        operation: VucOperation,
        cdw11: u32,
        cdw12: u32,
        cdw13: u32,
        cdw14: u32,
    ) -> Self {
        Self {
            transfer,
            operation,
            cdw11,
            cdw12,
            cdw13,
            cdw14,
        }
    }
}

impl From<&Vuc<'_>> for [u8; Vuc::SIZE] {
    fn from(value: &Vuc) -> Self {
        const CDW_SIZE: usize = size_of::<u32>();
        const FORMAT_PLAIN: u8 = 0xD;
        const TRANSFER_NONE: u8 = 0;
        const TRANSFER_WRITE: u8 = 1;
        const TRANSFER_READ: u8 = 2;

        let transfer_size = value.transfer.size();

        let transfer_type = match value.transfer {
            Transfer::None => TRANSFER_NONE,
            _ if transfer_size == 0 => TRANSFER_NONE,
            Transfer::Write(_) => TRANSFER_WRITE,
            Transfer::Read(_) => TRANSFER_READ,
        };

        let cdw0 = u32::from((FORMAT_PLAIN << 4) | transfer_type);

        let dword_count = transfer_size.next_multiple_of(SECTOR_SIZE) / size_of::<u32>();
        let cdw10 = u32::try_from(dword_count).unwrap();

        let cdw12 = (value.operation as u32) | (value.cdw12 << 8);

        let mut data = [0; _];
        data[..CDW_SIZE].copy_from_slice(&cdw0.to_le_bytes());
        data[10 * CDW_SIZE..][..CDW_SIZE].copy_from_slice(&cdw10.to_le_bytes());
        data[11 * CDW_SIZE..][..CDW_SIZE].copy_from_slice(&value.cdw11.to_le_bytes());
        data[12 * CDW_SIZE..][..CDW_SIZE].copy_from_slice(&cdw12.to_le_bytes());
        data[13 * CDW_SIZE..][..CDW_SIZE].copy_from_slice(&value.cdw13.to_le_bytes());
        data[14 * CDW_SIZE..][..CDW_SIZE].copy_from_slice(&value.cdw14.to_le_bytes());

        data
    }
}

/// Flash interface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlashInterface {
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
struct SystemInfo {
    /// Flash CE count.
    ce_count: u8,
    /// Flash channel count.
    channel_count: u8,
    /// Flash CEs per channel.
    ces_per_channel: u8,
    /// Flash manufacturer code.
    flash_manufacturer: flash_id::Manufacturer,
    /// Flash die process.
    flash_process: String,
    /// Flash bits per cell (e.g. 3 = TLC)
    flash_bits_per_cell: u8,
    /// Flash grade qualification tier.
    flash_grade: u8,
    /// Flash page size in KB.
    page_size: u8,
    /// Flash pages per block.
    pages_per_block: u16,
    /// Flash blocks per CE.
    blocks_per_ce: u16,
    /// Flash block size in MB.
    block_size_mb: u8,
    /// Flash dies per CE.
    dies_per_ce: u8,
    /// Flash planes per die.
    planes_per_die: u8,
    /// Stride between dies in a flash block address.
    die_stride: u16,
    /// Bitmap of active chip enable addresses.
    ce_bitmap: u64,
    /// Flash interface type.
    flash_interface: FlashInterface,
    /// FTL virtual block count
    virtual_block_count: u16,
    /// CPU clock in MHz.
    cpu_clock: u16,
    /// DRAM clock in MHz.
    dram_clock: u16,
    /// DRAM DDR type (DDR<x>).
    dram_type: u8,
    /// Drive form factor.
    form_factor: Option<identify::FormFactor>,
    /// DRAM size in MB.
    dram_size: u32,
    /// Unique controller hardware ID.
    controller_id: [u8; Self::CONTROLLER_ID_SIZE],
    /// Firmware build date.
    firmware_date: String,
    /// VUC lock has a key configured.
    vuc_lock_key_set: bool,
    /// VUC lock state.
    vuc_lock_state: Option<VucLockState>,
    /// VUC lock key ID.
    vuc_lock_key: Option<NonZero<u16>>,
    /// Firmware sub-version.
    firmware_subversion: String,
    /// Controller hardware version.
    controller_version: String,
    /// Controller hardware revision.
    controller_revision: u8,
    /// Mask ROM bootloader revision.
    bootloader_revision: u8,
    /// Firmware version.
    firmware_version: String,
    /// Firmware minor version.
    firmware_version_minor: String,
    /// Flash maximum rated program/erase cycles.
    flash_max_pe_cycles: u16,
    /// Flash read clock in MT/s.
    flash_read_clock: u16,
    /// Flash write clock in MT/s.
    flash_write_clock: u16,
}

impl SystemInfo {
    /// Controller ID field size in bytes.
    const CONTROLLER_ID_SIZE: usize = 8;
    /// Size in bytes.
    const SIZE: usize = 4_096;

    /// Parse string.
    fn parse_str(data: &[u8]) -> Result<&str, Error> {
        parse_str(data).ok_or(Error::InvalidSystemInfo)
    }
}

impl TryFrom<&[u8; Self::SIZE]> for SystemInfo {
    type Error = Error;

    #[allow(clippy::too_many_lines)]
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
    const SIZE: usize = 4_096;

    /// Parse string.
    fn parse_string(data: &[u8]) -> Result<String, Error> {
        parse_str(data)
            .map(|x| x.trim_end().to_string())
            .ok_or(Error::InvalidInfoBlock)
    }
}

impl TryFrom<&[u8; Self::SIZE]> for InfoBlock {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        if !data.starts_with(super::INFO_BLOCK_MAGIC) {
            return Err(Error::InvalidInfoBlock);
        }

        let serial = Self::parse_string(&data[6..26])?;
        let model = Self::parse_string(&data[34..74])?;

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
        let directory_system_info =
            directory[LOG_SYSTEM_INFO] as usize == SystemInfo::SIZE.div_ceil(SECTOR_SIZE);
        debug!("[{self}] Validate GPL directory system info: {directory_system_info}");
        if !directory_system_info {
            return Ok(false);
        }

        // Read system info log
        let mut system_info = [0u8; SystemInfo::SIZE];
        self.ata.gpl_read(&mut system_info, LOG_SYSTEM_INFO, 0, 0)?;

        // Try parse system info
        let system_info_result = SystemInfo::try_from(&system_info);
        debug!("[{self}] Validate GPL system info: {system_info_result:?}");
        if system_info_result.is_err() {
            return Ok(false);
        }

        Ok(true)
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

        // All S12 VUCs involve an ATA 0x31 write command, so system info validated from
        // a vendor-specific GP log instead
        if !Self::validate_gpl(self)? {
            return Ok(false);
        }

        debug!("[{self}] Validated");
        Ok(true)
    }

    /// Execute VUC.
    fn vuc(self, command: Vuc) -> Result<(), Error> {
        const FEATURE_SEND_SQ: u8 = 0xF1;
        const FEATURE_DATA_WRITE: u8 = 0xF2;
        const FEATURE_DATA_READ: u8 = 0xF3;
        const FEATURE_READ_CQ: u8 = 0xF4;

        let log_info = format!(
            "transfer: {}, CDW11: {:#x}, CDW12: {:#x}, CDW13: {:#x}, CDW14: {:#x}",
            command.transfer, command.cdw11, command.cdw12, command.cdw13, command.cdw14
        );
        debug!("[{self}] Executing VUC: {} ({log_info})", command.operation);

        // Send submission queue
        let sq_data = <[u8; _]>::from(&command);
        (&self as &dyn super::Drive).vuc(Transfer::Write(&sq_data), FEATURE_SEND_SQ, 0)?;

        // No-data VUCs write a single sector
        let transfer = match command.transfer {
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
        let mut cq_data = [0u8; SECTOR_SIZE];
        (&self as &dyn super::Drive).vuc(Transfer::Read(&mut cq_data), FEATURE_READ_CQ, 0)?;
        let status = u16::from_le_bytes([cq_data[14], cq_data[15]]);

        info!(
            "[{self}] Executed VUC: {} ({log_info}, status: {status:#x})",
            command.operation
        );

        if status != 0 {
            return Err(Error::VucStatus(status));
        }

        Ok(())
    }

    /// VUC system info.
    fn vuc_system_info(self) -> Result<SystemInfo, Error> {
        let mut data = [0u8; SystemInfo::SIZE];

        self.vuc(Vuc::new(
            Transfer::Read(&mut data),
            VucOperation::SystemInfo,
            0,
            0,
            0,
            0,
        ))?;

        let system_info = (&data).try_into()?;
        debug!("[{self}] System info: {system_info:?}");

        Ok(system_info)
    }

    /// VUC read info block.
    fn vuc_read_info_block(self) -> Result<InfoBlock, Error> {
        let mut data = [0u8; InfoBlock::SIZE];

        self.vuc(Vuc::new(
            Transfer::Read(&mut data),
            VucOperation::ReadInfoBlock,
            0,
            0,
            0,
            0,
        ))?;
        let info_block = (&data).try_into()?;

        debug!("[{self}] Info block: {info_block:?}");

        Ok(info_block)
    }

    /// Read controller memory.
    fn read_memory(self, address: u32, data: &mut [u8]) -> Result<(), Error> {
        self.vuc(Vuc::new(
            Transfer::Read(data),
            VucOperation::ReadSram,
            0,
            0,
            address,
            0,
        ))
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
        let drive = match drive {
            drive::Drive::Ata(ata) => Drive { ata },
            drive::Drive::Scsi(_) => return Ok(None),
        };

        if !drive.validate()? {
            return Ok(None);
        }

        let mut results = Vec::new();

        let system_info = drive.vuc_system_info()?;

        // System Info part of the validation checks (indirect through vendor log),
        // assume it always succeeds
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

    #[test]
    fn parse_system_info() {
        const DATA_VALID: &[&[u8; SystemInfo::SIZE]] = &[test_data::kingston_dc500r::SYSTEM_INFO];
        const DATA_INVALID: &[&[u8; SystemInfo::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(SystemInfo::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(SystemInfo::try_from(data).is_err());
        }
    }

    #[test]
    fn parse_info_block() {
        const DATA_VALID: &[&[u8; InfoBlock::SIZE]] = &[test_data::kingston_dc500r::INFO_BLOCK];
        const DATA_INVALID: &[&[u8; InfoBlock::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(InfoBlock::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(InfoBlock::try_from(data).is_err());
        }
    }
}
