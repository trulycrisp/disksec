//! Linux OS interface.

pub mod scsi;

use std::{
    fs::File,
    io,
    os::unix::{
        fs::{FileTypeExt, MetadataExt},
        io::AsRawFd,
    },
    path::PathBuf,
};

/// Linux error.
#[derive(Debug)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// SCSI error.
    Scsi(scsi::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(x) => Some(x),
            Self::Scsi(x) => Some(x),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(_) => write!(f, "IO error"),
            Self::Scsi(_) => write!(f, "SCSI error"),
        }
    }
}

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<scsi::Error> for Error {
    fn from(value: scsi::Error) -> Self {
        Self::Scsi(value)
    }
}

/// Ioctl requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ioctl {
    /// `SG_GET_VERSION_NUM`.
    SgGetVersionNum = 0x2282,
    /// `SG_IO`.
    SgIo = 0x2285,
}

impl Ioctl {
    /// Execute ioctl request.
    unsafe fn execute<T>(self, file: &File, arg: &mut T) -> Result<(), Error> {
        let fd = file.as_raw_fd();
        let result = unsafe { libc::ioctl(fd, self as _, std::ptr::from_mut(arg)) };
        if result < 0 {
            return Err(io::Error::last_os_error().into());
        }

        Ok(())
    }
}

/// Drive kernel subsystem.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Subsystem {
    /// SCSI.
    Scsi,
    /// `NVMe`.
    Nvme,
}

impl Subsystem {
    /// Get subsystem of drive file.
    fn get(file: &File) -> Result<Option<Self>, Error> {
        const CLASS_BLOCK: &str = "block";
        const CLASS_CHAR: &str = "char";
        const SUBSYSTEM_SCSI: &str = "scsi";
        const SUBSYSTEM_NVME: &str = "nvme";

        let metadata = file.metadata()?;

        let class = match metadata.file_type() {
            x if x.is_block_device() => CLASS_BLOCK,
            x if x.is_char_device() => CLASS_CHAR,
            _ => return Ok(None),
        };

        let major = libc::major(metadata.rdev());
        let minor = libc::minor(metadata.rdev());
        let link_path = format!("/sys/dev/{class}/{major}:{minor}/device/subsystem");
        let subsystem_path = match std::fs::canonicalize(link_path) {
            Ok(x) => x,
            Err(x) if x.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(x) => return Err(x.into()),
        };

        let Some(subsystem_name) = subsystem_path.file_name().and_then(|x| x.to_str()) else {
            return Ok(None);
        };

        Ok(match subsystem_name {
            SUBSYSTEM_SCSI => Some(Self::Scsi),
            SUBSYSTEM_NVME => Some(Self::Nvme),
            _ => None,
        })
    }
}

impl std::fmt::Display for Subsystem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Scsi => "SCSI",
            Self::Nvme => "NVMe",
        })
    }
}

/// Enumerate drives.
pub fn list_drives() -> Result<Box<[PathBuf]>, Error> {
    scsi::list_drives()
}
