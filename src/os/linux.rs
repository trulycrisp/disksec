//! Linux OS interface.

pub mod ata;
pub mod nvme;
pub mod scsi;

use std::{
    ffi::c_int,
    fs::File,
    io,
    os::unix::{
        fs::{FileTypeExt, MetadataExt},
        io::AsRawFd,
    },
    path::{Path, PathBuf},
};

use log::debug;

/// Linux error.
#[derive(Debug)]
pub enum Error {
    /// IO error.
    Io(io::Error),
    /// SCSI error.
    Scsi(scsi::Error),
    /// NVMe error.
    Nvme(nvme::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(x) => Some(x),
            Self::Scsi(x) => Some(x),
            Self::Nvme(x) => Some(x),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(_) => write!(f, "IO error"),
            Self::Scsi(_) => write!(f, "SCSI error"),
            Self::Nvme(_) => write!(f, "NVMe error"),
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

impl From<nvme::Error> for Error {
    fn from(value: nvme::Error) -> Self {
        Self::Nvme(value)
    }
}

/// Ioctl requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
enum Ioctl {
    /// `SG_GET_VERSION_NUM`.
    SgGetVersionNum = 0x2282,
    /// `SG_IO`.
    SgIo = 0x2285,
    /// `NVME_IOCTL_ADMIN_CMD`.
    NvmeAdminCmd = 0xC048_4E41,
}

impl Ioctl {
    /// Execute ioctl request.
    unsafe fn execute<T>(self, file: &File, arg: &mut T) -> Result<c_int, Error> {
        let fd = file.as_raw_fd();
        let result = unsafe { libc::ioctl(fd, self as _, std::ptr::from_mut(arg)) };
        if result < 0 {
            return Err(io::Error::last_os_error().into());
        }

        Ok(result)
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
        const LINK_SELF: &str = "subsystem";
        const LINK_PARENT: &str = "device/subsystem";
        const SUBSYSTEM_SCSI_GENERIC: &str = "scsi_generic";
        const SUBSYSTEM_SCSI: &str = "scsi";
        const SUBSYSTEM_NVME: &str = "nvme";

        let metadata = file.metadata()?;

        // A character node's own link names its class; a block node is always
        // in class `block`, so its parent's link is needed instead
        let (class, link) = match metadata.file_type() {
            x if x.is_block_device() => (CLASS_BLOCK, LINK_PARENT),
            x if x.is_char_device() => (CLASS_CHAR, LINK_SELF),
            _ => return Ok(None),
        };

        let major = libc::major(metadata.rdev());
        let minor = libc::minor(metadata.rdev());
        let link_path = format!("/sys/dev/{class}/{major}:{minor}/{link}");
        let subsystem_path = match std::fs::canonicalize(link_path) {
            Ok(x) => x,
            Err(x) if x.kind() == io::ErrorKind::NotFound => return Ok(None),
            Err(x) => return Err(x.into()),
        };

        let Some(subsystem_name) = subsystem_path.file_name().and_then(|x| x.to_str()) else {
            return Ok(None);
        };

        let subsystem = match subsystem_name {
            SUBSYSTEM_SCSI_GENERIC | SUBSYSTEM_SCSI => Some(Self::Scsi),
            SUBSYSTEM_NVME => Some(Self::Nvme),
            _ => None,
        };

        debug!(
            "[FD {}] Subsystem: {}",
            file.as_raw_fd(),
            subsystem.map_or("N/A".to_string(), |x| x.to_string())
        );

        Ok(subsystem)
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

/// Get all drive paths of a given class.
pub fn drive_paths_class(class: &str) -> Result<Box<[PathBuf]>, Error> {
    const DEV_PATH: &str = "/dev";
    const CLASS_PATH: &str = "/sys/class";

    let dev_base = Path::new(DEV_PATH);
    let class_base = Path::new(CLASS_PATH).join(class);

    let entries = match class_base.read_dir() {
        Ok(x) => x,
        Err(x) if x.kind() == io::ErrorKind::NotFound => return Ok(Box::default()),
        Err(x) => return Err(x.into()),
    };

    let mut paths = Vec::new();
    for entry_result in entries {
        let path = dev_base.join(entry_result?.file_name());

        // A class entry without a device node cannot be opened
        if path.exists() {
            paths.push(path);
        }
    }

    Ok(paths.into())
}

/// Enumerate drives.
pub fn drive_paths() -> Result<Box<[PathBuf]>, Error> {
    let mut paths = Vec::new();
    paths.extend(scsi::drive_paths()?);
    paths.extend(nvme::drive_paths()?);

    Ok(paths.into())
}
