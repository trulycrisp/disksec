//! Linux NVMe interface.

use std::{
    ffi::{c_uchar, c_uint, c_ulonglong, c_ushort},
    fs::{File, OpenOptions},
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
    path::{Path, PathBuf},
};

use log::debug;

use super::Subsystem;
use crate::protocol::{
    Transfer,
    nvme::{self, command::Completion},
};

/// Linux NVMe error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid status returned by `NVME_IOCTL_ADMIN_CMD`.
    InvalidAdminCommandStatus(i32),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidAdminCommandStatus(x) => {
                write!(f, "invalid NVME_IOCTL_ADMIN_CMD status {x}")
            },
        }
    }
}

impl From<Error> for nvme::Error {
    fn from(value: Error) -> Self {
        super::Error::from(value).into()
    }
}

/// Structure `nvme_passthru_cmd` for `NVME_IOCTL_ADMIN_CMD`.
#[repr(C)]
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct NvmePassthruCmd {
    /// Opcode.
    opcode: c_uchar,
    /// Command flags, fused operation and data transfer selection.
    flags: c_uchar,
    /// Reserved.
    rsvd1: c_ushort,
    /// Namespace identifier.
    nsid: c_uint,
    /// Command dword 2.
    cdw2: c_uint,
    /// Command dword 3.
    cdw3: c_uint,
    /// Metadata address.
    metadata: c_ulonglong,
    /// Data address.
    addr: c_ulonglong,
    /// Metadata size.
    metadata_len: c_uint,
    /// Data size.
    data_len: c_uint,
    /// Command dword 10.
    cdw10: c_uint,
    /// Command dword 11.
    cdw11: c_uint,
    /// Command dword 12.
    cdw12: c_uint,
    /// Command dword 13.
    cdw13: c_uint,
    /// Command dword 14.
    cdw14: c_uint,
    /// Command dword 15.
    cdw15: c_uint,
    /// Command timeout in milliseconds.
    timeout_ms: c_uint,
    /// Completion queue entry dword 0.
    result: c_uint,
}

/// Enumerate NVME drives.
pub fn drive_paths() -> Result<Box<[PathBuf]>, super::Error> {
    const CLASS: &str = "nvme";

    super::drive_paths_class(CLASS)
}

/// NVMe drive interface.
#[derive(Debug)]
pub struct Interface {
    /// Drive path.
    path: PathBuf,
    /// Drive file.
    file: File,
}

impl Interface {
    /// Open drive.
    pub(crate) fn open(path: &Path) -> Result<Self, nvme::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .custom_flags(libc::O_NONBLOCK)
            .open(path)
            .map_err(super::Error::from)?;

        if Subsystem::get(&file)? != Some(Subsystem::Nvme) {
            return Err(nvme::Error::UnsupportedDrive);
        }

        let interface = Self {
            path: path.to_owned(),
            file,
        };

        debug!("[{interface}] Opened FD: {}", interface.file.as_raw_fd());

        Ok(interface)
    }
}

impl nvme::Interface for Interface {
    fn path(&self) -> &Path {
        &self.path
    }

    fn admin_command(
        &self,
        command: &nvme::command::AdminCommand,
        transfer: Transfer,
        timeout: u32,
    ) -> Result<Completion, nvme::Error> {
        const MS_PER_SEC: u32 = 1000;

        let transfer_size = transfer.size();

        let addr = (match transfer {
            Transfer::None => std::ptr::null_mut(),
            Transfer::Read(x) => x.as_mut_ptr(),
            Transfer::Write(x) => x.as_ptr().cast_mut(),
        }) as _;

        let data_len = transfer_size.try_into().unwrap();
        let timeout_ms = timeout.saturating_mul(MS_PER_SEC);

        let mut passthru_command = NvmePassthruCmd {
            opcode: command.opcode.into(),
            nsid: command.nsid,
            cdw2: command.cdw2,
            cdw3: command.cdw3,
            addr,
            data_len,
            cdw10: command.cdw10,
            cdw11: command.cdw11,
            cdw12: command.cdw12,
            cdw13: command.cdw13,
            cdw14: command.cdw14,
            cdw15: command.cdw15,
            timeout_ms,
            ..Default::default()
        };

        debug!("[{self}] Executing NVME_IOCTL_ADMIN_CMD: {passthru_command:?}");

        // The ioctl returns the completion status field.
        let status =
            unsafe { super::Ioctl::NvmeAdminCmd.execute(&self.file, &mut passthru_command) }
                .map_err(|x| match x {
                    super::Error::Io(y) if y.raw_os_error() == Some(libc::ENOTTY) => {
                        nvme::Error::UnsupportedDrive
                    },
                    _ => x.into(),
                })?;

        debug!("[{self}] Executed NVME_IOCTL_ADMIN_CMD: {passthru_command:?} ({status})");

        let status = u16::try_from(status)
            .map_err(|_| Error::InvalidAdminCommandStatus(status))?
            .try_into()?;

        Ok(Completion {
            status,
            result: passthru_command.result,
        })
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NVMe Linux {}", self.path.display())
    }
}
