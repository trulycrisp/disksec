//! Windows ATA interface.

use std::{
    ffi::c_void,
    fs::{File, OpenOptions},
    os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
    path::{Path, PathBuf},
    ptr::null_mut,
};

use log::debug;

use super::{FILE_SHARE_READ, FILE_SHARE_WRITE, Ioctl, StorageBusType};
use crate::protocol::{
    Transfer, ata,
    ata::command::{CommandRegisters, ResultRegisters},
};

/// Default command timeout in seconds.
const DEFAULT_TIMEOUT: u32 = 30;
/// Flag `ATA_FLAGS_DATA_IN`.
const ATA_FLAGS_DATA_IN: u16 = 1 << 1;
/// Flag `ATA_FLAGS_DATA_OUT`.
const ATA_FLAGS_DATA_OUT: u16 = 1 << 2;
/// Flag `ATA_FLAGS_48BIT_COMMAND`.
const ATA_FLAGS_48BIT_COMMAND: u16 = 1 << 3;
/// Flag `ATA_FLAGS_USE_DMA`.
const ATA_FLAGS_USE_DMA: u16 = 1 << 4;
/// Flag `ATA_FLAGS_NO_MULTIPLE`.
const ATA_FLAGS_NO_MULTIPLE: u16 = 1 << 5;

/// Structure `ATA_PASS_THROUGH_DIRECT`.
#[derive(Clone, Copy, Debug, Default)]
#[repr(C)]
struct AtaPassThroughDirect {
    /// Size.
    length: u16,
    /// Transfer flags.
    ata_flags: u16,
    /// SCSI port/bus number.
    path_id: u8,
    /// Target device id on bus.
    target_id: u8,
    /// Logical unit number.
    lun: u8,
    /// Reserved.
    reserved_as_uchar: u8,
    /// Data size.
    data_transfer_length: u32,
    /// Command timeout in seconds.
    time_out_value: u32,
    /// Reserved.
    reserved_as_ulong: u32,
    /// Data.
    data_buffer: *mut c_void,
    /// Upper bytes of 48-bit register set.
    previous_task_file: [u8; 8],
    /// Register set.
    current_task_file: [u8; 8],
}

impl AtaPassThroughDirect {
    /// Get result register set.
    fn result_registers(&self, extend: bool) -> ResultRegisters {
        let current = self.current_task_file;
        let previous = if extend {
            self.previous_task_file
        } else {
            [0; _]
        };

        ResultRegisters {
            error: current[0],
            count: u16::from_le_bytes([current[1], previous[1]]),
            lba: u64::from_le_bytes([
                current[2],
                current[3],
                current[4],
                previous[2],
                previous[3],
                previous[4],
                0,
                0,
            ]),
            device: current[5],
            status: current[6].into(),
        }
    }
}

/// ATA drive interface.
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
    /// Open drive.
    pub(crate) fn open(path: &Path) -> Result<Self, ata::Error> {
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
            Ok(StorageBusType::Ata | StorageBusType::Sata) => {},
            Ok(_) => return Err(ata::Error::UnsupportedDrive),
            Err(x) if x.is_ioctl_unsupported() => return Err(ata::Error::UnsupportedDrive),
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

    /// Get offset in data that matches alignment.
    fn get_aligned_offset(&self, data: &[u8]) -> usize {
        let address = data.as_ptr() as usize;
        address.next_multiple_of(self.alignment) - address
    }

    /// Execute command with aligned data.
    fn command_aligned(
        &self,
        registers: CommandRegisters,
        transfer: Transfer,
        extend: bool,
        dma: bool,
        timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, ata::Error> {
        let transfer_size = transfer.size();
        let length = size_of::<AtaPassThroughDirect>().try_into().unwrap();
        let data_transfer_length = transfer_size.try_into().unwrap();

        let (mut ata_flags, data_buffer) = match transfer {
            Transfer::None => (0, null_mut()),
            Transfer::Read(x) => (ATA_FLAGS_DATA_IN, x.as_mut_ptr().cast::<c_void>()),
            Transfer::Write(x) => (ATA_FLAGS_DATA_OUT, x.as_ptr() as *mut c_void),
        };

        if extend {
            ata_flags |= ATA_FLAGS_48BIT_COMMAND;
        }

        if dma {
            ata_flags |= ATA_FLAGS_USE_DMA;
        } else if data_transfer_length > 0 {
            ata_flags |= ATA_FLAGS_NO_MULTIPLE;
        }

        let feature = registers.feature.to_le_bytes();
        let count = registers.count.to_le_bytes();
        let lba = registers.lba.to_le_bytes();

        let previous_task_file = if extend {
            [feature[1], count[1], lba[3], lba[4], lba[5], 0, 0, 0]
        } else {
            [0; _]
        };

        let mut aptd = AtaPassThroughDirect {
            length,
            ata_flags,
            data_transfer_length,
            time_out_value: timeout.unwrap_or(DEFAULT_TIMEOUT),
            data_buffer,
            previous_task_file,
            current_task_file: [
                feature[0],
                count[0],
                lba[0],
                lba[1],
                lba[2],
                registers.device,
                registers.command.into(),
                0,
            ],
            ..Default::default()
        };

        debug!(
            "[{self}] Executing {}: {aptd:?}",
            Ioctl::AtaPassThroughDirect
        );

        let result = unsafe {
            Ioctl::AtaPassThroughDirect.execute(&self.file, &raw mut aptd, &raw mut aptd)
        };

        debug!(
            "[{self}] Executed {}: {aptd:?} ({result:?})",
            Ioctl::AtaPassThroughDirect
        );

        match result {
            Err(x) if x.is_ioctl_unsupported() => return Err(ata::Error::UnsupportedDrive),
            Err(x) => return Err(x.into()),
            Ok(_) => {},
        }

        let out_registers = aptd.result_registers(extend);

        if out_registers.status.error() || out_registers.status.device_fault() {
            return Err(ata::Error::Command(out_registers));
        }

        let residual = transfer_size
            .checked_signed_diff(aptd.data_transfer_length as usize)
            .unwrap_or(isize::MAX);
        if residual != 0 {
            return Err(ata::Error::Residual(residual));
        }

        Ok(Some(out_registers))
    }

    /// Display path as drive.
    fn format_path(path: &Path) -> String {
        format!("ATA Windows {}", path.display())
    }
}

impl ata::Interface for Interface {
    fn path(&self) -> &Path {
        &self.path
    }

    fn command(
        &self,
        registers: CommandRegisters,
        transfer: Transfer,
        extend: bool,
        dma: bool,
        timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, ata::Error> {
        // IOCTL_ATA_PASS_THROUGH_DIRECT requires an adapter-aligned data address
        let already_aligned = match &transfer {
            Transfer::None => true,
            Transfer::Read(x) => self.get_aligned_offset(x) == 0,
            Transfer::Write(x) => self.get_aligned_offset(x) == 0,
        };
        if already_aligned {
            return self.command_aligned(registers, transfer, extend, dma, timeout);
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

        let out_registers =
            self.command_aligned(registers, transfer_aligned, extend, dma, timeout)?;

        // Copy read data to original transfer
        if let Transfer::Read(data) = transfer {
            data.copy_from_slice(data_aligned);
        }

        Ok(out_registers)
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::format_path(&self.path))
    }
}
