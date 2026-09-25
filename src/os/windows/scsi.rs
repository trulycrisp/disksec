//! Windows SCSI interface.

use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
    path::{Path, PathBuf},
    ptr::null_mut,
};

use log::debug;

use super::{FILE_SHARE_READ, FILE_SHARE_WRITE, Ioctl, StorageBusType};
use crate::protocol::{Transfer, scsi};

/// Data out transfer.
const SCSI_IOCTL_DATA_OUT: u8 = 0;
/// Data in transfer.
const SCSI_IOCTL_DATA_IN: u8 = 1;
/// No transfer.
const SCSI_IOCTL_DATA_UNSPECIFIED: u8 = 2;

/// Windows SCSI error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// `SCSI_PASS_THROUGH_DIRECT` returned invalid `data_transfer_length`.
    InvalidSptdTransferLength(u32),
    /// `SCSI_PASS_THROUGH_DIRECT` returned Invalid sense size.
    InvalidSptdSenseSize(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidSptdTransferLength(x) => write!(
                f,
                "invalid {} data_transfer_length {x}",
                Ioctl::ScsiPassThroughDirect
            ),
            Self::InvalidSptdSenseSize(x) => write!(
                f,
                "invalid {} sense_info_length {x}",
                Ioctl::ScsiPassThroughDirect
            ),
        }
    }
}

impl From<Error> for scsi::Error {
    fn from(value: Error) -> Self {
        super::Error::from(value).into()
    }
}

/// Structure `SCSI_PASS_THROUGH_DIRECT`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct ScsiPassThroughDirect {
    /// Size.
    length: u16,
    /// SCSI status.
    scsi_status: u8,
    /// SCSI port/bus number.
    path_id: u8,
    /// Target device id on bus.
    target_id: u8,
    /// Logical unit number.
    lun: u8,
    /// CDB size.
    cdb_length: u8,
    /// Sense buffer size.
    sense_info_length: u8,
    /// Transfer direction.
    data_in: u8,
    /// Data size.
    data_transfer_length: u32,
    /// Command timeout in seconds.
    time_out_value: u32,
    /// Data.
    data_buffer: *mut c_void,
    /// Offset to sense buffer.
    sense_info_offset: u32,
    /// SCSI CDB.
    cdb: [u8; 16],
}

/// Structure `SCSI_PASS_THROUGH_DIRECT` with appended sense buffer.
#[derive(Clone, Copy, Debug)]
#[repr(C)]
struct ScsiPassThroughDirectSense {
    /// Struct `SCSI_PASS_THROUGH_DIRECT`.
    sptd: ScsiPassThroughDirect,
    /// Sense buffer.
    sense: [u8; scsi::SENSE_BUFFER_SIZE],
}

impl Default for ScsiPassThroughDirectSense {
    fn default() -> Self {
        Self {
            sptd: ScsiPassThroughDirect::default(),
            sense: [0; _],
        }
    }
}

/// SCSI drive interface.
#[derive(Debug)]
pub struct Interface {
    /// Drive path.
    path: PathBuf,
    /// Drive file.
    file: File,
    /// Adapter required alignment size.
    alignment: usize,
}

impl Interface {
    /// Get offset in data that matches alignment.
    fn get_aligned_offset(&self, data: &[u8]) -> usize {
        let address = data.as_ptr() as usize;
        address.next_multiple_of(self.alignment) - address
    }

    /// Execute SCSI command with aligned data.
    fn command_aligned(
        &self,
        cdb: &[u8],
        transfer: Transfer,
        timeout: u32,
    ) -> Result<(u8, usize, Box<[u8]>), scsi::Error> {
        let transfer_size = transfer.size();

        let length = size_of::<ScsiPassThroughDirect>().try_into().unwrap();
        let cdb_length = cdb.len().try_into().unwrap();
        let sense_info_length = scsi::SENSE_BUFFER_SIZE.try_into().unwrap();

        let (data_in, data_buffer) = match transfer {
            Transfer::None => (SCSI_IOCTL_DATA_UNSPECIFIED, null_mut()),
            Transfer::Read(x) => (SCSI_IOCTL_DATA_IN, x.as_mut_ptr().cast::<c_void>()),
            Transfer::Write(x) => (SCSI_IOCTL_DATA_OUT, x.as_ptr() as *mut c_void),
        };

        let data_transfer_length = transfer_size.try_into().unwrap();
        let sense_info_offset = std::mem::offset_of!(ScsiPassThroughDirectSense, sense)
            .try_into()
            .unwrap();

        let mut sptds = ScsiPassThroughDirectSense {
            sptd: ScsiPassThroughDirect {
                length,
                cdb_length,
                sense_info_length,
                data_in,
                data_transfer_length,
                time_out_value: timeout,
                data_buffer,
                sense_info_offset,
                ..Default::default()
            },
            ..Default::default()
        };
        sptds.sptd.cdb[..cdb.len()].copy_from_slice(cdb);

        debug!(
            "[{self}] Executing {}: {:?}",
            Ioctl::ScsiPassThroughDirect,
            sptds.sptd
        );

        let result = unsafe {
            Ioctl::ScsiPassThroughDirect.execute(&self.file, &raw mut sptds, &raw mut sptds)
        };

        debug!(
            "[{self}] Executed {}: {:?} ({result:?})",
            Ioctl::ScsiPassThroughDirect,
            sptds.sptd
        );

        match result {
            Err(x) if x.is_ioctl_unsupported() => return Err(scsi::Error::UnsupportedDrive),
            Err(x) => return Err(x.into()),
            Ok(_) => {},
        }

        let status = sptds.sptd.scsi_status;

        let sense_size = sptds.sptd.sense_info_length as _;
        if sense_size > sptds.sense.len() {
            return Err(Error::InvalidSptdSenseSize(sptds.sptd.sense_info_length).into());
        }

        let transferred_size = match sptds.sptd.data_transfer_length {
            x if x as usize <= transfer_size => x as _,
            x => return Err(Error::InvalidSptdTransferLength(x).into()),
        };

        let sense = (&sptds.sense[..sense_size]).into();

        Ok((status, transferred_size, sense))
    }

    /// Display path as drive.
    fn format_path(path: &Path) -> String {
        format!("SCSI Windows {}", path.display())
    }
}

impl scsi::Interface for Interface {
    fn open(path: &Path) -> Result<Self, scsi::Error> {
        let file = OpenOptions::new()
            .read(true)
            .write(true)
            .share_mode(FILE_SHARE_READ | FILE_SHARE_WRITE)
            .open(path)
            .map_err(super::Error::from)?;

        debug!(
            "[{}] Opened handle: {:#x}",
            Self::format_path(path),
            file.as_raw_handle() as usize
        );

        match StorageBusType::get_device(&file) {
            Ok(
                StorageBusType::Scsi
                | StorageBusType::Atapi
                | StorageBusType::Ata
                | StorageBusType::Usb
                | StorageBusType::Sas
                | StorageBusType::Sata
                | StorageBusType::Ufs,
            ) => {},
            Ok(_) => return Err(scsi::Error::UnsupportedDrive),
            Err(x) if x.is_ioctl_unsupported() => return Err(scsi::Error::UnsupportedDrive),
            Err(x) => return Err(x.into()),
        }

        let alignment = super::get_alignment(&file)?;
        debug!("[{}] Alignment: {alignment}", Self::format_path(path));

        Ok(Self {
            path: path.to_owned(),
            file,
            alignment,
        })
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
        // IOCTL_SCSI_PASS_THROUGH_DIRECT requires an adapter-aligned data address
        let already_aligned = match &transfer {
            Transfer::None => true,
            Transfer::Read(x) => self.get_aligned_offset(x) == 0,
            Transfer::Write(x) => self.get_aligned_offset(x) == 0,
        };
        if already_aligned {
            return self.command_aligned(cdb, transfer, timeout);
        }

        let transfer_size = transfer.size();
        let mut buffer = vec![0; transfer_size + self.alignment - 1].into_boxed_slice();
        let data_aligned_offset = self.get_aligned_offset(&buffer);
        let data_aligned = &mut buffer[data_aligned_offset..data_aligned_offset + transfer_size];

        let transfer_aligned = match transfer {
            Transfer::None => unreachable!(),
            Transfer::Read(_) => Transfer::Read(data_aligned),
            Transfer::Write(data) => {
                data_aligned.copy_from_slice(data);
                Transfer::Write(data_aligned)
            },
        };

        let (status, transferred_size, sense) =
            self.command_aligned(cdb, transfer_aligned, timeout)?;

        // Copy read data to original transfer
        if let Transfer::Read(data) = transfer {
            data[..transferred_size].copy_from_slice(&data_aligned[..transferred_size]);
        }

        Ok((status, transferred_size, sense))
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::format_path(&self.path))
    }
}
