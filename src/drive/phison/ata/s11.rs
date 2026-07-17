//! S11 controller.

mod algorithm;
mod vuc;

use std::num::NonZero;

use log::{debug, info};

use super::VucLockState;
use crate::{
    cpu::VECTOR_TABLE_SIZE,
    drive,
    protocol::{
        Transfer,
        ata::{self, SECTOR_SIZE, identify},
    },
};

/// Display name of drive type.
const DISPLAY_NAME: &str = "Phison S11";
/// Memory address of Xtensa exception vector table.
const EXCEPTION_VECTOR_TABLE_ADDRESS: u32 = 0x5C0E_0000;

/// S11 error.
#[derive(Debug)]
enum Error {
    /// ATA error.
    Ata(ata::Error),
    /// Invalid system info data.
    InvalidSystemInfo,
    /// Invalid VUC unlock key.
    InvalidVucKey(u16),
    /// VUC unlock handshake failed.
    VucUnlockFailed(Option<VucLockState>),
    /// Re-locking VUC access failed.
    VucLockFailed(Option<VucLockState>),
    /// Invalid info block data.
    InvalidInfoBlock,
    /// Invalid firmware flash header data.
    InvalidFirmwareFlashHeader,
    /// Invalid VUC read/write register size.
    InvalidRegisterSize(usize),
    /// Invalid Xtensa exception vector table in memory.
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
        let display_vuc_lock =
            |x: Option<VucLockState>| x.as_ref().map_or("N/A".into(), ToString::to_string);

        match self {
            Self::Ata(_) => write!(f, "ATA error"),
            Self::InvalidSystemInfo => write!(f, "invalid system info"),
            Self::InvalidVucKey(x) => write!(f, "invalid VUC key {x}"),
            Self::VucUnlockFailed(x) => write!(f, "VUC unlock failed ({})", display_vuc_lock(*x)),
            Self::VucLockFailed(x) => write!(f, "VUC lock failed ({})", display_vuc_lock(*x)),
            Self::InvalidInfoBlock => write!(f, "invalid info block"),
            Self::InvalidFirmwareFlashHeader => write!(f, "invalid firmware flash header"),
            Self::InvalidRegisterSize(x) => write!(f, "invalid register size {x}"),
            Self::InvalidExceptionVectorTable => write!(f, "invalid Xtensa exception vector table"),
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
    /// Read installed firmware (intended for verifying firmware flashing).
    VerifyFlash = 0x31,
    /// Read memory value (intended for hardware registers).
    ReadRegister = 0x61,
    /// Start VUC unlock handshake.
    VucUnlockStart = 0xC4,
    /// Read VUC unlock challenge.
    VucUnlockRead = 0xC5,
    /// Write VUC unlock challenge-response.
    VucUnlockWrite = 0xC6,
    /// Re-lock VUC access.
    VucLock = 0xC7,
}

impl std::fmt::Display for VucOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SystemInfo => write!(f, "system info"),
            Self::SetParameter => write!(f, "set parameter"),
            Self::ReadInfoBlock => write!(f, "read info block"),
            Self::VerifyFlash => write!(f, "verify flash"),
            Self::ReadRegister => write!(f, "read register"),
            Self::VucUnlockStart => write!(f, "VUC unlock start"),
            Self::VucUnlockRead => write!(f, "VUC unlock read"),
            Self::VucUnlockWrite => write!(f, "VUC unlock write"),
            Self::VucLock => write!(f, "VUC lock"),
        }
    }
}

/// Flash interface type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FlashInterface {
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
    /// Die-level interleaving enabled, requires multi-die flash.
    die_interleave: bool,
    /// Number of die groups a CE is split into for interleaving.
    die_interleave_factor: u8,
    /// Flash blocks per die.
    blocks_per_die: u32,
    /// Flash blocks per CE.
    blocks_per_ce: u32,
    /// Sectors per flash page.
    sectors_per_page: u32,
    /// Number of blocks per channel in a superblock if any.
    channel_interleave_factor: u8,
    /// Flash planes per die.
    planes_per_die: u8,
    /// Drive form factor.
    form_factor: Option<identify::FormFactor>,
    /// Flash blocks per superblock.
    blocks_per_superblock: u8,
    /// Firmware sub-version.
    firmware_subversion: String,
    /// Stride between dies in a flash block address.
    die_stride: Option<NonZero<u16>>,
    ///  Total flash size divided by superblock size.
    superblock_index_count: u32,
    /// Number of sectors per superblock.
    sectors_per_superblock: u32,
    /// Number of usable superblocks.
    superblock_count: u32,
    /// Bitmask of chip enable addresses with errors.
    ce_error_bitmap: u64,
    /// Sectors per superblock page.
    sectors_per_superblock_page: u32,
    /// Flash interface type.
    flash_interface: Option<FlashInterface>,
    /// Protected mode active.
    protected_mode: bool,
    /// Total firmware update count.
    firmware_update_count: u16,
    /// Total system unit (drive config data) update count.
    system_unit_update_count: u16,
    /// Bitmap of active chip enable addresses.
    ce_bitmap: u64,
    /// Firmware version.
    firmware_version: String,
    /// Firmware build date.
    firmware_date: String,
    /// VUC PRAM icode program is complete.
    pram_icode_programmed: bool,
    /// Page size including metadata if any.
    physical_page_size: Option<NonZero<u16>>,
    /// VUC lock has a key configured.
    vuc_lock_key_set: bool,
    /// VUC lock key ID.
    vuc_lock_key: Option<NonZero<u16>>,
    /// VUC lock state.
    vuc_lock_state: Option<VucLockState>,
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

    #[allow(clippy::too_many_lines)]
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

/// Firmware header.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FirmwareHeader {
    /// Change VUC mode to engineering.
    engineering_mode: bool,
}

impl FirmwareHeader {
    /// Pack a boolean value.
    fn pack_bool(value: bool) -> u8 {
        const FALSE: u8 = 0;
        const TRUE: u8 = 0x33;

        if value { TRUE } else { FALSE }
    }
}

impl From<FirmwareHeader> for [u8; SECTOR_SIZE] {
    fn from(value: FirmwareHeader) -> Self {
        const MAGIC: &[u8] = b"PS\x0C\x27"; // 0xC27 = 3111
        const ENGINEERING_MODE_OFFSET: usize = 53;

        let mut data = [0; _];

        data[..MAGIC.len()].copy_from_slice(MAGIC);
        data[ENGINEERING_MODE_OFFSET] = FirmwareHeader::pack_bool(value.engineering_mode);

        data
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

        let serial = identify::parse_string(&data[18..38]).or(Err(Error::InvalidInfoBlock))?;
        let model = identify::parse_string(&data[38..78]).or(Err(Error::InvalidInfoBlock))?;

        Ok(Self { serial, model })
    }
}

/// Firmware header stored on flash.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct FirmwareFlashHeader {
    /// Address and size of each section.
    sections: [(u32, u32); Self::SECTION_COUNT],
}

impl FirmwareFlashHeader {
    /// Number of sections.
    const SECTION_COUNT: usize = 8;
    /// Size in bytes.
    const SIZE: usize = 4096;
}

impl TryFrom<&[u8; Self::SIZE]> for FirmwareFlashHeader {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        const MAGIC: &[u8] = b"ID";

        if !data.starts_with(MAGIC) {
            return Err(Error::InvalidFirmwareFlashHeader);
        }

        let section_values: [_; Self::SECTION_COUNT * 2] = std::array::from_fn(|x| {
            let offset = 288 + x * size_of::<u32>();
            u32::from_le_bytes([
                data[offset],
                data[offset + 1],
                data[offset + 2],
                data[offset + 3],
            ])
        });
        let sections =
            std::array::from_fn(|i| (section_values[i], section_values[Self::SECTION_COUNT + i]));

        Ok(Self { sections })
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
        let no_drat = identify.additional_supported.is_some_and(|x| !x.drat);
        let no_rzat = identify.additional_supported.is_some_and(|x| !x.rzat);
        let no_sct = !identify.sct_supported.sct;
        let valid = no_drat && no_rzat && no_sct;

        if let Some(drive) = drive {
            debug!(
                "[{drive}] Validate identify: {valid} (DRAT: {no_drat}, RZAT: {no_rzat}, SCT: \
                 {no_sct})"
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

    /// Derive CRC-16 based cipher seed from VUC unlock key.
    fn vuc_lock_key_crc16_cipher_seed(key: &[u8]) -> u32 {
        const BUFFER_SIZE: usize = 32;

        let mut buffer = [0u8; BUFFER_SIZE];

        for chunk in key.chunks(BUFFER_SIZE) {
            for (x, &y) in buffer.iter_mut().zip(chunk) {
                *x ^= y;
            }
        }

        u32::from_le_bytes([buffer[0], buffer[1], buffer[2], buffer[3]])
    }

    /// Unlock VUC mode to engineering.
    fn vuc_unlock_engineering(self) -> Result<(), Error> {
        // Return if engineering mode not needed
        if matches!(
            self.vuc_system_info()?.vuc_lock_state,
            None | Some(VucLockState::Engineering)
        ) {
            return Ok(());
        }

        let dlmc_data: [_; _] = FirmwareHeader {
            engineering_mode: true,
        }
        .into();
        self.ata
            .download_microcode(&dlmc_data, ata::command::DlmcSubcommand::Full, 0)?;

        // Verify engineering mode
        let vuc_lock_state = self.vuc_system_info()?.vuc_lock_state;
        if vuc_lock_state != Some(VucLockState::Engineering) {
            return Err(Error::VucUnlockFailed(vuc_lock_state));
        }

        Ok(())
    }

    /// Unlock VUC mode to unlocked.
    fn vuc_unlock(self) -> Result<(), Error> {
        let system_info = self.vuc_system_info()?;

        // Return if unlock not needed
        if matches!(
            system_info.vuc_lock_state,
            None | Some(VucLockState::Unlocked | VucLockState::NoLock)
        ) {
            return Ok(());
        }

        // Get key for id
        let key_id = system_info.vuc_lock_key.map_or(0, NonZero::get);
        let key = vuc::KEYS
            .get((key_id as usize).wrapping_sub(1))
            .ok_or(Error::InvalidVucKey(key_id))?;

        // Reset state if needed
        self.vuc_lock()?;

        // Change to engineering mode if needed
        self.vuc_unlock_engineering()?;

        // Start unlock handshake
        self.vuc(Transfer::None, VucOperation::VucUnlockStart, 0)?;

        // Read challenge
        let mut read_data = [0u8; SECTOR_SIZE];
        self.vuc(
            Transfer::Read(&mut read_data),
            VucOperation::VucUnlockRead,
            0,
        )?;

        // Encrypt challenge, two rounds
        let mut seed = Self::vuc_lock_key_crc16_cipher_seed(key);
        let mut write_data = algorithm::cipher_crc16(&read_data, seed, 0);
        seed ^= Self::vuc_lock_key_crc16_cipher_seed(&write_data);
        write_data = algorithm::cipher_crc16(&write_data, seed, 0);

        // Write challenge
        self.vuc(
            Transfer::Write(&write_data),
            VucOperation::VucUnlockWrite,
            0,
        )?;

        // Verify unlocked
        let vuc_lock_state = self.vuc_system_info()?.vuc_lock_state;
        if vuc_lock_state != Some(VucLockState::Unlocked) {
            self.vuc_lock()?; // Reset state
            return Err(Error::VucUnlockFailed(vuc_lock_state));
        }

        debug!("[{self}] VUC unlocked");

        Ok(())
    }

    /// Re-lock the VUC access.
    fn vuc_lock(self) -> Result<(), Error> {
        // Return if lock not needed
        if matches!(
            self.vuc_system_info()?.vuc_lock_state,
            None | Some(VucLockState::Locked | VucLockState::NoLock)
        ) {
            return Ok(());
        }

        self.vuc(Transfer::None, VucOperation::VucLock, 0)?;

        // Verify locked to default mode. Sometimes (e.g. burner firmware or protected
        // mode) default mode is engineering
        let system_info = self.vuc_system_info()?;
        if !matches!(
            system_info.vuc_lock_state,
            Some(VucLockState::Locked | VucLockState::Engineering)
        ) {
            return Err(Error::VucLockFailed(system_info.vuc_lock_state));
        }

        debug!("[{self}] VUC locked");

        Ok(())
    }

    /// VUC read info block.
    fn vuc_read_info_block(self) -> Result<InfoBlock, Error> {
        let mut data = [0u8; InfoBlock::SIZE];

        self.vuc(Transfer::Read(&mut data), VucOperation::ReadInfoBlock, 0)?;
        let info_block = (&data).try_into()?;

        debug!("[{self}] Info block: {info_block:?}");

        Ok(info_block)
    }

    /// VUC verify flash.
    fn vuc_verify_flash(self, data: &mut [u8], code: bool) -> Result<(), Error> {
        let lba = u32::from(code) << 16;

        self.vuc(Transfer::Read(data), VucOperation::VerifyFlash, lba)?;

        Ok(())
    }

    /// Read firmware header from flash.
    fn read_firmware_flash_header(self) -> Result<FirmwareFlashHeader, Error> {
        let mut data = [0u8; FirmwareFlashHeader::SIZE];
        self.vuc_verify_flash(&mut data, false)?;

        let flash_header = (&data).try_into()?;
        debug!("[{self}] Firmware flash header: {flash_header:?}");

        Ok(flash_header)
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

    /// Check data is an Xtensa exception vector table.
    fn is_exception_vector_table(data: &[u8; VECTOR_TABLE_SIZE]) -> bool {
        const PREFIX: &[u8] = &[
            0x00, 0xC5, 0x49, // s32e a0,a5,-0x10
            0x10, 0xD5, 0x49, // s32e a1,a5,-0xc
            0x20, 0xE5, 0x49, // s32e a2,a5,-0x8
            0x30, 0xF5, 0x49, // s32e a3,a5,-0x4
        ];

        data.starts_with(PREFIX)
    }

    /// Read Xtensa exception vector table from controller memory.
    fn read_exception_vector_table(self) -> Result<[u8; VECTOR_TABLE_SIZE], Error> {
        let mut data = [0; _];
        self.read_memory(EXCEPTION_VECTOR_TABLE_ADDRESS, &mut data)?;

        if !Self::is_exception_vector_table(&data) {
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

/// S11 vendor type.
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

    /// Check read firmware.
    fn check_read_firmware(drive: Drive) -> drive::CheckResult {
        let read_firmware_result = drive.read_firmware_flash_header();
        debug!("[{drive}] Read firmware flash header: {read_firmware_result:?}");

        let result = match read_firmware_result {
            Ok(x) => Ok(format!(
                "Sections ({})",
                x.sections
                    .iter()
                    .filter(|&(_, x)| *x != 0)
                    .map(|(x, y)| format!("{x:#x}: <{y} bytes>"))
                    .collect::<Vec<_>>()
                    .join(", ")
            )),
            Err(x) => Err(x.into()),
        };

        drive::CheckResult {
            name: "Read firmware".into(),
            result,
        }
    }

    /// Check read controller memory.
    fn check_read_memory(drive: Drive) -> drive::CheckResult {
        let result = match drive.read_exception_vector_table() {
            Ok(_) => Ok(format!(
                "Xtensa exception vector table at {EXCEPTION_VECTOR_TABLE_ADDRESS:#010x}"
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

        // VUC System Info part of the validation checks, assume it always succeeds
        let system_info = drive.vuc_system_info()?;
        results.push(drive::CheckResult::new(
            "System info".into(),
            Ok(format!(
                "(firmware: {}-{} ({}), CEs: {}, blocks per CE: {}, pages per block: {}, sectors \
                 per page: {})",
                system_info.firmware_version,
                system_info.firmware_subversion,
                system_info.firmware_date,
                system_info.ce_count,
                system_info.blocks_per_ce,
                system_info.pages_per_block,
                system_info.sectors_per_page
            )),
        ));

        let vuc_unlock_result = drive.vuc_unlock();
        let vuc_unlocked = vuc_unlock_result.is_ok();
        let vuc_unlock_check_result = match vuc_unlock_result {
            Ok(()) => Ok(match system_info.vuc_lock_state {
                None | Some(VucLockState::NoLock) => "N/A (no lock)".into(),
                Some(VucLockState::Unlocked) => "N/A (unlocked)".into(),
                _ => format!("key {}", system_info.vuc_lock_key.map_or(0, NonZero::get)),
            }),
            Err(x) => Err(x.into()),
        };
        results.push(drive::CheckResult::new(
            "VUC unlock".into(),
            vuc_unlock_check_result,
        ));

        if vuc_unlocked {
            // Only proceed with other checks if VUC unlock succeeded
            results.push(Self::check_read_info_block(drive));
            results.push(Self::check_read_firmware(drive));
            results.push(Self::check_read_memory(drive));

            // Re-lock VUCs
            drive.vuc_lock()?;
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
        const DATA_VALID: &[&[u8; identify::IdentifyDevice::SIZE]] = &[
            test_data::kingston_a400::IDENTIFY,
            test_data::inland_professional::IDENTIFY,
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

    #[test]
    fn parse_info_block() {
        const DATA_VALID: &[&[u8; InfoBlock::SIZE]] = &[
            test_data::kingston_a400::INFO_BLOCK,
            test_data::inland_professional::INFO_BLOCK,
        ];
        const DATA_INVALID: &[&[u8; InfoBlock::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(InfoBlock::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(InfoBlock::try_from(data).is_err());
        }
    }

    #[test]
    fn parse_firmware_flash_header() {
        const DATA_VALID: &[&[u8; FirmwareFlashHeader::SIZE]] = &[
            test_data::kingston_a400::FIRMWARE_FLASH_HEADER,
            test_data::inland_professional::FIRMWARE_FLASH_HEADER,
        ];
        const DATA_INVALID: &[&[u8; FirmwareFlashHeader::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(FirmwareFlashHeader::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(FirmwareFlashHeader::try_from(data).is_err());
        }
    }

    #[test]
    fn is_exception_vector_table() {
        const DATA_VALID: &[&[u8; VECTOR_TABLE_SIZE]] = &[
            test_data::kingston_a400::EXCEPTION_VECTOR_TABLE,
            test_data::inland_professional::EXCEPTION_VECTOR_TABLE,
        ];
        const DATA_INVALID: &[&[u8; VECTOR_TABLE_SIZE]] = &[
            &[0; _],
            &[0xFF; _],
            test_data::phison_s5::VECTOR_TABLE, // ARCompact
            test_data::kingston_dc500r::EXCEPTION_VECTOR_TABLE, // ARM32
        ];

        for &data in DATA_VALID {
            assert!(Drive::is_exception_vector_table(data));
        }

        for &data in DATA_INVALID {
            assert!(!Drive::is_exception_vector_table(data));
        }
    }
}
