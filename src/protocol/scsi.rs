//! SCSI drive interface.

pub mod command;
pub mod inquiry;
pub mod sense;

use std::path::Path;

use log::{debug, info};

use super::Transfer;
use crate::{os, output};

/// Maximum sense data size.
pub const SENSE_BUFFER_SIZE: usize = 252;
/// Default command timeout in seconds.
const DEFAULT_TIMEOUT: u32 = 30;

/// SCSI error.
#[derive(Debug)]
pub enum Error {
    /// OS error.
    Os(os::Error),
    /// Drive does not support SCSI.
    UnsupportedDrive,
    /// Invalid status value.
    InvalidStatus(u8),
    /// Status value represents error.
    Status(Status),
    /// Invalid sense data.
    InvalidSense(sense::Error),
    /// CHECK CONDITION returned with no sense data.
    NoSense,
    /// Sense represents error.
    Sense(sense::Sense),
    /// Invalid inquiry command response.
    Inquiry(inquiry::Error),
}

impl Error {
    /// Error represents a command failing.
    pub(crate) fn is_command_error(&self) -> bool {
        matches!(self, Self::Status(_) | Self::Sense(_))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Os(x) => Some(x),
            Self::InvalidSense(x) => Some(x),
            Self::Inquiry(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Os(_) => write!(f, "OS error"),
            Self::UnsupportedDrive => write!(f, "unsupported drive"),
            Self::InvalidStatus(x) => write!(f, "invalid status {x:#x}"),
            Self::Status(x) => write!(f, "status {x}"),
            Self::InvalidSense(_) => write!(f, "invalid sense"),
            Self::NoSense => write!(f, "{} no sense", Status::CheckCondition),
            Self::Sense(x) => write!(f, "sense {x}"),
            Self::Inquiry(_) => write!(f, "inquiry error"),
        }
    }
}

impl From<os::Error> for Error {
    fn from(value: os::Error) -> Self {
        Self::Os(value)
    }
}

impl From<sense::Error> for Error {
    fn from(value: sense::Error) -> Self {
        Self::InvalidSense(value)
    }
}

impl From<sense::Sense> for Error {
    fn from(value: sense::Sense) -> Self {
        Self::Sense(value)
    }
}

impl From<inquiry::Error> for Error {
    fn from(value: inquiry::Error) -> Self {
        Self::Inquiry(value)
    }
}

/// Status codes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Status {
    /// Command completed successfully.
    Good = 0x0,
    /// Command failed; sense data is available describing the error.
    CheckCondition = 0x2,
    /// A requested condition was satisfied.
    ConditionMet = 0x4,
    /// Logical unit is busy and cannot accept the command now.
    Busy = 0x8,
    /// Command conflicts with an existing reservation.
    ReservationConflict = 0x18,
    /// The command task set is full.
    TaskSetFull = 0x28,
    /// An auto contingent allegiance condition is active.
    AcaActive = 0x30,
    /// The task was aborted.
    TaskAborted = 0x40,
}

impl TryFrom<u8> for Status {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[Status] = &[
            Status::Good,
            Status::CheckCondition,
            Status::ConditionMet,
            Status::Busy,
            Status::ReservationConflict,
            Status::TaskSetFull,
            Status::AcaActive,
            Status::TaskAborted,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Self::Error::InvalidStatus(value))
    }
}

impl std::fmt::Display for Status {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Good => write!(f, "GOOD"),
            Self::CheckCondition => write!(f, "CHECK CONDITION"),
            Self::ConditionMet => write!(f, "CONDITION MET"),
            Self::Busy => write!(f, "BUSY"),
            Self::ReservationConflict => write!(f, "RESERVATION CONFLICT"),
            Self::TaskSetFull => write!(f, "TASK SET FULL"),
            Self::AcaActive => write!(f, "ACA ACTIVE"),
            Self::TaskAborted => write!(f, "TASK ABORTED"),
        }
    }
}

/// SCSI transport interface.
pub trait Interface: Sized {
    /// Open drive.
    fn open(path: &Path) -> Result<Self, Error>;
    /// Get path of drive.
    fn path(&self) -> &Path;
    /// Execute command.
    fn command(
        &self,
        cdb: &[u8],
        transfer: Transfer,
        timeout: u32,
    ) -> Result<(u8, usize, Box<[u8]>), Error>;
}

/// Drive interface.
#[derive(Debug)]
pub struct Drive {
    /// SCSI transport interface.
    interface: os::scsi::Interface,
}

impl Drive {
    /// Open drive.
    pub(crate) fn open(path: &Path) -> Result<Self, Error> {
        let interface = os::scsi::Interface::open(path)?;
        let drive = Self { interface };

        let inquiry = match drive.inquiry() {
            Ok(x) => x,
            Err(x) if x.is_command_error() => return Err(Error::UnsupportedDrive),
            Err(x) => return Err(x),
        };

        info!("[{drive}] Opened: {inquiry}");

        Ok(drive)
    }

    /// Get path of drive.
    pub(crate) fn path(&self) -> &Path {
        self.interface.path()
    }

    /// Execute command.
    pub(crate) fn command(
        &self,
        cdb: &dyn command::Cdb,
        transfer: Transfer,
        timeout: Option<u32>,
        check_sense: bool,
    ) -> Result<(usize, Option<sense::Sense>), Error> {
        let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);
        let cdb_bytes = Box::<[_]>::from(cdb);

        let log_info = format!("CDB: {cdb}, transfer: {transfer}");
        debug!("[{self}] Executing command: ({log_info})");

        let (status, transfer_size, sense_data) =
            self.interface.command(&cdb_bytes, transfer, timeout)?;

        let status = Status::try_from(status)?;

        let sense = (!sense_data.is_empty())
            .then(|| sense::Sense::try_from(sense_data.as_ref()))
            .transpose()?;

        match status {
            Status::Good | Status::ConditionMet => {},
            Status::CheckCondition => {
                if sense.is_none() {
                    return Err(Error::NoSense);
                }
            },
            x => return Err(Error::Status(x)),
        }

        info!(
            "[{self}] Executed command: ({log_info}, size: {transfer_size}, sense: {})",
            match &sense {
                None => String::from("none"),
                Some(x) => format!("{x} ({})", output::format_bytes_hex(&sense_data)),
            }
        );

        if let Some(sense) = sense {
            // Check sense for error
            if check_sense && sense.sense_key().is_error() {
                return Err(sense.into());
            }

            return Ok((transfer_size, Some(sense)));
        }

        Ok((transfer_size, None))
    }

    /// Execute inquiry command.
    pub(crate) fn inquiry(&self) -> Result<inquiry::Inquiry, Error> {
        let mut data = [0u8; inquiry::Inquiry::MAX_SIZE];

        let allocation_length = size_of_val(&data).try_into().unwrap();
        let cdb = command::Inquiry {
            allocation_length,
            ..Default::default()
        };

        let (data_size, _) = self.command(&cdb, Transfer::Read(&mut data), None, true)?;

        Ok(data[..data_size].try_into()?)
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SCSI {}", self.path().display())
    }
}
