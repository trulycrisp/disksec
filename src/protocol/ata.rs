//! ATA drive interface.

pub mod command;
pub mod identify;
pub mod log;
pub mod sat;
pub mod smart;

use std::path::Path;

use ::log::{debug, info};

use crate::{
    os,
    protocol::{
        Transfer,
        ata::{
            command::{CommandRegisters, ResultRegisters},
            log::Directory,
        },
        scsi,
    },
};

/// Standard logical sector size.
pub const SECTOR_SIZE: usize = 512;

/// ATA error.
#[derive(Debug)]
pub enum Error {
    /// OS error.
    Os(os::Error),
    /// SCSI/SAT transport error.
    Scsi(scsi::Error),
    /// Drive does not support ATA.
    UnsupportedDrive,
    /// Invalid data transfer size.
    InvalidTransferSize(usize),
    /// Residual data from transfer.
    Residual(isize),
    /// ATA command failed.
    Command(ResultRegisters),
    /// Identify device error.
    Identify(identify::Error),
    /// Logical sector size is unsupported.
    UnsupportedSectorSize(u64),
    /// SMART error.
    Smart(smart::Error),
    /// SMART/GPL log error.
    Log(log::Error),
}

impl Error {
    /// Error represents a command failing.
    pub(crate) fn is_command_error(&self) -> bool {
        matches!(self, Self::Command(_)) || matches!(self, Self::Scsi(x) if x.is_command_error())
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Os(x) => Some(x),
            Self::Scsi(x) => Some(x),
            Self::Identify(x) => Some(x),
            Self::Smart(x) => Some(x),
            Self::Log(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Os(_) => write!(f, "OS error"),
            Self::Scsi(_) => write!(f, "SCSI error"),
            Self::UnsupportedDrive => write!(f, "unsupported drive"),
            Self::InvalidTransferSize(x) => write!(f, "invalid transfer size {x}"),
            Self::Residual(x) => write!(f, "transfer residual {x}"),
            Self::Command(x) => write!(f, "command error {x}"),
            Self::Identify(_) => write!(f, "{} error", command::Command::IdentifyDevice),
            Self::UnsupportedSectorSize(x) => write!(f, "unsupported sector size {x}"),
            Self::Smart(_) => write!(f, "SMART error"),
            Self::Log(_) => write!(f, "log error"),
        }
    }
}

impl From<os::Error> for Error {
    fn from(value: os::Error) -> Self {
        Self::Os(value)
    }
}

impl From<scsi::Error> for Error {
    fn from(value: scsi::Error) -> Self {
        Self::Scsi(value)
    }
}

impl From<identify::Error> for Error {
    fn from(value: identify::Error) -> Self {
        Self::Identify(value)
    }
}

impl From<smart::Error> for Error {
    fn from(value: smart::Error) -> Self {
        Self::Smart(value)
    }
}

impl From<log::Error> for Error {
    fn from(value: log::Error) -> Self {
        Self::Log(value)
    }
}

/// ATA transport interface.
pub trait Interface: std::fmt::Debug {
    /// Get path of drive.
    fn path(&self) -> &Path;
    /// Execute command.
    fn command(
        &self,
        registers: CommandRegisters,
        transfer: Transfer,
        extend: bool,
        dma: bool,
        timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, Error>;
}

/// Drive interface.
#[derive(Debug)]
pub struct Drive {
    /// ATA transport interface.
    interface: Box<dyn Interface>,
}

impl Drive {
    /// Open drive.
    pub(crate) fn open(path: &Path) -> Result<Self, Error> {
        match scsi::Drive::open(path) {
            Ok(x) => match Self::new(Box::new(sat::Interface::new(x))) {
                Err(Error::UnsupportedDrive) => {},
                x => return x,
            },
            Err(scsi::Error::UnsupportedDrive) => {},
            Err(x) => return Err(x.into()),
        }

        match os::ata::Interface::open(path) {
            Ok(x) => match Self::new(Box::new(x)) {
                Err(Error::UnsupportedDrive) => {},
                x => return x,
            },
            Err(Error::UnsupportedDrive) => {},
            Err(x) => return Err(x),
        }

        Err(Error::UnsupportedDrive)
    }

    /// Open drive over interface.
    fn new(interface: Box<dyn Interface>) -> Result<Self, Error> {
        let drive = Self { interface };

        // Confirm interface reaches an ATA drive with identify device command.
        let identify = match drive.identify_device() {
            Ok(x) => x,
            Err(x) if x.is_command_error() => return Err(Error::UnsupportedDrive),
            Err(x) => return Err(x),
        };

        if identify.logical_sector_size != SECTOR_SIZE as _ {
            return Err(Error::UnsupportedSectorSize(identify.logical_sector_size));
        }

        info!("[{drive}] Opened: {identify}");

        Ok(drive)
    }

    /// File path of drive.
    pub(crate) fn path(&self) -> &Path {
        self.interface.path()
    }

    /// Execute command with sector-aligned transfer size.
    fn command_aligned(
        &self,
        mut registers: CommandRegisters,
        transfer: Transfer,
        extend: bool,
        dma: bool,
        timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, Error> {
        let transfer_size = transfer.size();

        if transfer_size > 0 {
            // For SAT passthrough with data, count register must match size
            let count = u16::try_from(transfer_size / SECTOR_SIZE)
                .or(Err(Error::InvalidTransferSize(transfer_size)))?;

            registers.count = count;
        }

        let log_info = format!("command registers: {registers}, transfer: {transfer}, dma: {dma}");
        debug!("[{self}] Executing command: ({log_info})");

        let out_registers = self
            .interface
            .command(registers, transfer, extend, dma, timeout)?;

        info!(
            "[{self}] Executed command: ({log_info}, result registers: {})",
            match &out_registers {
                None => "none".into(),
                Some(x) => x.to_string(),
            }
        );

        Ok(out_registers)
    }

    /// Execute command.
    pub(crate) fn command(
        &self,
        registers: CommandRegisters,
        transfer: Transfer,
        extend: bool,
        dma: bool,
        timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, Error> {
        let transfer_size = transfer.size();
        let transfer_aligned_size = transfer_size.next_multiple_of(SECTOR_SIZE);

        // Check if transfer already aligned
        if transfer_size == transfer_aligned_size {
            return self.command_aligned(registers, transfer, extend, dma, timeout);
        }

        // Align transfer
        let mut data_aligned = vec![0u8; transfer_aligned_size].into_boxed_slice();

        let transfer_aligned = match transfer {
            Transfer::None => unreachable!(),
            Transfer::Read(_) => Transfer::Read(data_aligned.as_mut()),
            Transfer::Write(data) => {
                data_aligned[..data.len()].copy_from_slice(data);
                Transfer::Write(data_aligned.as_ref())
            },
        };

        let out_registers =
            self.command_aligned(registers, transfer_aligned, extend, dma, timeout)?;

        // Copy read data to original transfer
        if let Transfer::Read(data) = transfer {
            data.copy_from_slice(&data_aligned[..data.len()]);
        }

        Ok(out_registers)
    }

    /// Execute identify device command.
    pub(crate) fn identify_device(&self) -> Result<identify::IdentifyDevice, Error> {
        let registers = CommandRegisters {
            command: command::Command::IdentifyDevice,
            ..Default::default()
        };

        let mut data = [0u8; identify::IdentifyDevice::SIZE];

        self.command(registers, Transfer::Read(&mut data), false, false, None)?;

        let identify = (&data).try_into()?;
        debug!("[{self}] Identify device: {identify:?}");

        Ok(identify)
    }

    /// Execute download microcode command.
    pub(crate) fn download_microcode(
        &self,
        data: &[u8],
        subcommand: command::DlmcSubcommand,
        offset: u16,
    ) -> Result<(), Error> {
        // DLMC spec puts count upper-bits in lower-bits of LBA, incompatible
        // with SAT so this should only be used when count fits a single
        // byte
        let lba = u64::from(offset) << 8;

        let registers = CommandRegisters {
            feature: subcommand as _,
            lba,
            command: command::Command::DownloadMicrocode,
            ..Default::default()
        };

        self.command(registers, Transfer::Write(data), false, false, None)?;

        Ok(())
    }

    /// Execute SMART command.
    pub(crate) fn smart(
        &self,
        subcommand: smart::SmartSubcommand,
        transfer: Transfer,
        lba: u8,
    ) -> Result<Option<ResultRegisters>, Error> {
        let lba = u64::from(lba) | command::SMART_KEY_LBA;

        let registers = CommandRegisters {
            feature: subcommand as _,
            lba,
            command: command::Command::Smart,
            ..Default::default()
        };

        self.command(registers, transfer, false, false, None)
    }

    /// Execute SMART read data subcommand.
    pub(crate) fn smart_read_data(&self) -> Result<smart::Smart, Error> {
        let mut data = [0u8; smart::Smart::SIZE];

        self.smart(
            smart::SmartSubcommand::ReadData,
            Transfer::Read(&mut data),
            0,
        )?;

        Ok((&data).try_into()?)
    }

    /// Execute SMART read log subcommand.
    pub(crate) fn smart_log_read(
        &self,
        data: &mut [u8],
        log: log::Log,
    ) -> Result<Option<ResultRegisters>, Error> {
        self.smart(
            smart::SmartSubcommand::ReadLog,
            Transfer::Read(data),
            log.into(),
        )
    }

    /// Execute SMART write log subcommand.
    pub(crate) fn smart_log_write(
        &self,
        data: &[u8],
        log: log::Log,
    ) -> Result<Option<ResultRegisters>, Error> {
        self.smart(
            smart::SmartSubcommand::WriteLog,
            Transfer::Write(data),
            log.into(),
        )
    }

    /// Get SMART log directory.
    pub(crate) fn smart_log_directory(&self) -> Result<log::Directory, Error> {
        let mut data = [0u8; log::PAGE_SIZE];
        self.smart_log_read(&mut data, log::Log::Directory)?;

        Ok(Directory::parse_smart(&data)?)
    }

    /// Execute READ LOG EXT command.
    pub(crate) fn gpl_read(
        &self,
        data: &mut [u8],
        log: log::Log,
        page: u16,
        feature: u16,
    ) -> Result<Option<ResultRegisters>, Error> {
        let page_low = page & 0xFF;
        let page_high = page >> 8;
        let lba =
            u64::from(u8::from(log)) | (u64::from(page_low) << 8) | (u64::from(page_high) << 32);

        let registers = CommandRegisters {
            feature,
            lba,
            command: command::Command::ReadLogExt,
            ..Default::default()
        };

        self.command(registers, Transfer::Read(data), true, false, None)
    }

    /// Execute WRITE LOG EXT command.
    pub(crate) fn gpl_write(
        &self,
        data: &[u8],
        log: log::Log,
        page: u16,
        feature: u16,
    ) -> Result<(), Error> {
        let page_low = page & 0xFF;
        let page_high = page >> 8;
        let lba =
            u64::from(u8::from(log)) | (u64::from(page_low) << 8) | (u64::from(page_high) << 32);

        let registers = CommandRegisters {
            feature,
            lba,
            command: command::Command::WriteLogExt,
            ..Default::default()
        };

        self.command(registers, Transfer::Write(data), true, false, None)?;

        Ok(())
    }

    /// Get General Purpose Log directory.
    pub(crate) fn gpl_directory(&self) -> Result<log::Directory, Error> {
        let mut data = [0u8; log::PAGE_SIZE];
        self.gpl_read(&mut data, log::Log::Directory, 0, 0)?;

        Ok(Directory::parse_gpl(&data)?)
    }

    /// Get GPL SATA PHY Event Counters log.
    pub(crate) fn gpl_phy_event_counters(
        &self,
    ) -> Result<log::phy_event_counters::PhyEventCounters, Error> {
        let mut data = [0u8; log::PAGE_SIZE];
        self.gpl_read(&mut data, log::Log::PhyEventCounters, 0, 0)?;

        Ok((&data).try_into()?)
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "ATA {}", self.path().display())
    }
}
