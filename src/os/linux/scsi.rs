//! Linux SCSI interface.

use std::{
    ffi::{c_int, c_uchar, c_uint, c_ushort, c_void},
    fs::{File, OpenOptions},
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
    path::{Path, PathBuf},
};

use log::debug;

use super::Subsystem;
use crate::protocol::{Transfer, scsi};

/// Linux SCSI error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid SG host status value.
    InvalidSgHostStatus(c_ushort),
    /// SG host status reports error.
    SgHostStatus(SgHostStatus),
    /// `SG_IO` transfer residual data.
    InvalidSgIoResidual(i32),
    /// Invalid sense size returned by `SG_IO`.
    InvalidSgIoSenseSize(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSgHostStatus(x) => write!(f, "invalid SG host status {x}"),
            Self::SgHostStatus(x) => write!(f, "SG host status {x}"),
            Self::InvalidSgIoResidual(x) => write!(f, "invalid SG_IO resid {x}"),
            Self::InvalidSgIoSenseSize(x) => write!(f, "invalid SG_IO sb_len_wr {x}"),
        }
    }
}

impl From<Error> for scsi::Error {
    fn from(value: Error) -> Self {
        super::Error::from(value).into()
    }
}

/// SG `dxfer_direction` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum SgDxfer {
    /// No transfer (`SG_DXFER_NONE`).
    None = -1,
    /// To device (`SG_DXFER_TO_DEV`).
    ToDev = -2,
    /// From device (`SG_DXFER_FROM_DEV`).
    FromDev = -3,
}

impl std::fmt::Display for SgDxfer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "SG_DXFER_NONE"),
            Self::ToDev => write!(f, "SG_DXFER_TO_DEV"),
            Self::FromDev => write!(f, "SG_DXFER_FROM_DEV"),
        }
    }
}

/// SG host status values.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SgHostStatus {
    /// `DID_OK`.
    Ok = 0x0,
    /// `DID_NO_CONNECT`.
    NoConnect = 0x1,
    /// `DID_BUS_BUSY`.
    BusBusy = 0x2,
    /// `DID_TIME_OUT`.
    TimeOut = 0x3,
    /// `DID_BAD_TARGET`.
    BadTarget = 0x4,
    /// `DID_ABORT`.
    Abort = 0x5,
    /// `DID_PARITY`.
    Parity = 0x6,
    /// `DID_ERROR`.
    Error = 0x7,
    /// `DID_RESET`.
    Reset = 0x8,
    /// `DID_BAD_INTR`.
    BadIntr = 0x9,
    /// `DID_PASSTHROUGH`.
    Passthrough = 0xA,
    /// `DID_SOFT_ERROR`.
    SoftError = 0xB,
    /// `DID_IMM_RETRY`.
    ImmRetry = 0xC,
    /// `DID_REQUEUE`.
    Requeue = 0xD,
    /// `DID_TRANSPORT_DISRUPTED`.
    TransportDisrupted = 0xE,
    /// `DID_TRANSPORT_FAILFAST`.
    TransportFailfast = 0xF,
    /// `DID_TARGET_FAILURE`.
    TargetFailure = 0x10,
    /// `DID_NEXUS_FAILURE`.
    NexusFailure = 0x11,
    /// `DID_ALLOC_FAILURE`.
    AllocFailure = 0x12,
    /// `DID_MEDIUM_ERROR`.
    MediumError = 0x13,
}

impl TryFrom<c_ushort> for SgHostStatus {
    type Error = Error;

    fn try_from(value: c_ushort) -> Result<Self, Error> {
        const VARIANTS: &[SgHostStatus] = &[
            SgHostStatus::Ok,
            SgHostStatus::NoConnect,
            SgHostStatus::BusBusy,
            SgHostStatus::TimeOut,
            SgHostStatus::BadTarget,
            SgHostStatus::Abort,
            SgHostStatus::Parity,
            SgHostStatus::Error,
            SgHostStatus::Reset,
            SgHostStatus::BadIntr,
            SgHostStatus::Passthrough,
            SgHostStatus::SoftError,
            SgHostStatus::ImmRetry,
            SgHostStatus::Requeue,
            SgHostStatus::TransportDisrupted,
            SgHostStatus::TransportFailfast,
            SgHostStatus::TargetFailure,
            SgHostStatus::NexusFailure,
            SgHostStatus::AllocFailure,
            SgHostStatus::MediumError,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as c_ushort == value)
            .copied()
            .ok_or(Error::InvalidSgHostStatus(value))
    }
}

impl std::fmt::Display for SgHostStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Ok => "DID_OK",
            Self::NoConnect => "DID_NO_CONNECT",
            Self::BusBusy => "DID_BUS_BUSY",
            Self::TimeOut => "DID_TIME_OUT",
            Self::BadTarget => "DID_BAD_TARGET",
            Self::Abort => "DID_ABORT",
            Self::Parity => "DID_PARITY",
            Self::Error => "DID_ERROR",
            Self::Reset => "DID_RESET",
            Self::BadIntr => "DID_BAD_INTR",
            Self::Passthrough => "DID_PASSTHROUGH",
            Self::SoftError => "DID_SOFT_ERROR",
            Self::ImmRetry => "DID_IMM_RETRY",
            Self::Requeue => "DID_REQUEUE",
            Self::TransportDisrupted => "DID_TRANSPORT_DISRUPTED",
            Self::TransportFailfast => "DID_TRANSPORT_FAILFAST",
            Self::TargetFailure => "DID_TARGET_FAILURE",
            Self::NexusFailure => "DID_NEXUS_FAILURE",
            Self::AllocFailure => "DID_ALLOC_FAILURE",
            Self::MediumError => "DID_MEDIUM_ERROR",
        })
    }
}

/// Structure `sg_io_hdr_t` for `SG_IO`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct SgIoHdr {
    /// Interface (must be 'S').
    interface_id: c_int,
    /// Transfer direction.
    dxfer_direction: c_int,
    /// CDB size.
    cmd_len: c_uchar,
    /// Sense buffer size.
    mx_sb_len: c_uchar,
    /// Scatter-gather element count.
    iovec_count: c_ushort,
    /// Transfer size.
    dxfer_len: c_uint,
    /// Transfer data.
    dxferp: *mut c_void,
    /// CDB.
    cmdp: *mut c_uchar,
    /// Sense buffer.
    sbp: *mut c_uchar,
    /// Command timeout in milliseconds.
    timeout: c_uint,
    /// Flags.
    flags: c_uint,
    /// Unused.
    pack_id: c_int,
    /// Unused.
    usr_ptr: *mut c_void,
    /// SCSI status.
    status: c_uchar,
    /// Deprecated.
    masked_status: c_uchar,
    /// Deprecated.
    msg_status: c_uchar,
    /// Sense size.
    sb_len_wr: c_uchar,
    /// SG host status.
    host_status: c_ushort,
    /// Driver status, mostly deprecated.
    driver_status: c_ushort,
    /// Transfer residual data size.
    resid: c_int,
    /// Command duration in milliseconds.
    duration: c_uint,
    /// Bitmask of result info flags.
    info: c_uint,
}

/// Enumerate SCSI drives.
pub fn drive_paths() -> Result<Box<[PathBuf]>, super::Error> {
    const CLASS: &str = "scsi_generic";

    super::drive_paths_class(CLASS)
}

/// SCSI drive interface.
#[derive(Debug)]
pub struct Interface {
    /// Drive path.
    path: PathBuf,
    /// Drive file.
    file: File,
}

impl scsi::Interface for Interface {
    fn open(path: &Path) -> Result<Self, scsi::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)
            .map_err(super::Error::from)?;

        if Subsystem::get(&file)? != Some(Subsystem::Scsi) {
            return Err(scsi::Error::UnsupportedDrive);
        }

        let interface = Self {
            path: path.to_owned(),
            file,
        };

        debug!("[{interface}] Opened FD: {}", interface.file.as_raw_fd());

        let mut sg_version: c_int = 0;
        match unsafe { super::Ioctl::SgGetVersionNum.execute(&interface.file, &mut sg_version) } {
            Ok(_) => {},
            Err(super::Error::Io(x)) if x.raw_os_error() == Some(libc::ENOTTY) => {
                debug!("[{interface}] SG unsupported");
                return Err(scsi::Error::UnsupportedDrive);
            },
            Err(x) => return Err(x.into()),
        }
        debug!("[{interface}] SG version: {sg_version}");

        Ok(interface)
    }

    fn path(&self) -> &Path {
        &self.path
    }

    fn command(
        &self,
        cdb: &[u8],
        transfer: Transfer,
        timeout: u32,
    ) -> Result<(u8, usize, Box<[u8]>), scsi::Error> {
        const MS_PER_SEC: u32 = 1000;
        const SG_IO_INTERFACE_ID: c_int = b'S' as _;

        let transfer_size = transfer.size();
        let mut sense_buffer = [0; scsi::SENSE_BUFFER_SIZE];

        let dxfer_direction = match transfer {
            Transfer::None => SgDxfer::None,
            Transfer::Read(_) => SgDxfer::FromDev,
            Transfer::Write(_) => SgDxfer::ToDev,
        };

        let cmd_len = cdb.len().try_into().unwrap();
        let mx_sb_len = sense_buffer.len().try_into().unwrap();
        let dxfer_len = transfer_size.try_into().unwrap();

        let dxferp = (match transfer {
            Transfer::None => std::ptr::null_mut(),
            Transfer::Read(x) => x.as_mut_ptr(),
            Transfer::Write(x) => x.as_ptr().cast_mut(),
        })
        .cast();

        let cmdp = cdb.as_ptr().cast_mut();
        let sbp = sense_buffer.as_mut_ptr();
        let timeout_ms = timeout.saturating_mul(MS_PER_SEC);

        let mut sg_io_header = SgIoHdr {
            interface_id: SG_IO_INTERFACE_ID,
            dxfer_direction: dxfer_direction as _,
            cmd_len,
            mx_sb_len,
            dxfer_len,
            dxferp,
            cmdp,
            sbp,
            timeout: timeout_ms,
            ..Default::default()
        };

        debug!("[{self}] Executing SG_IO: {sg_io_header:?}");

        unsafe { super::Ioctl::SgIo.execute(&self.file, &mut sg_io_header) }.map_err(
            |x| match x {
                super::Error::Io(y) if y.raw_os_error() == Some(libc::ENOTTY) => {
                    scsi::Error::UnsupportedDrive
                },
                _ => x.into(),
            },
        )?;

        debug!("[{self}] Executed SG_IO: {sg_io_header:?}");

        let status = sg_io_header.status;

        let residual_size = match usize::try_from(sg_io_header.resid) {
            Ok(size) if size <= transfer_size => size,
            _ => return Err(Error::InvalidSgIoResidual(sg_io_header.resid).into()),
        };

        let sense_size = match sg_io_header.sb_len_wr {
            x if x as usize <= sense_buffer.len() => x as usize,
            x => return Err(Error::InvalidSgIoSenseSize(x).into()),
        };

        let host_status = SgHostStatus::try_from(sg_io_header.host_status)?;

        // If sense returned or non-zero status always return to SCSI layer to check
        // for SCSI-level errors, only otherwise handle as an OS-level error
        if host_status != SgHostStatus::Ok && status == 0 && sense_size == 0 {
            return Err(Error::SgHostStatus(host_status).into());
        }

        let transferred_size = transfer_size - residual_size;
        let sense = (&sense_buffer[..sense_size]).into();

        Ok((status, transferred_size, sense))
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "SCSI Linux {}", self.path.display())
    }
}
