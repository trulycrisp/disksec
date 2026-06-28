//! Windows OS interface.

#![cfg(windows)]

pub mod scsi;

use std::{ffi::c_void, fs::File, os::windows::io::AsRawHandle, path::PathBuf};

use log::debug;

/// File share flag `FILE_SHARE_READ`.
const FILE_SHARE_READ: u32 = 1;
/// File share flag `FILE_SHARE_WRITE`.
const FILE_SHARE_WRITE: u32 = 2;
/// Error code `ERROR_INVALID_FUNCTION`.
const ERROR_INVALID_FUNCTION: i32 = 1;
/// Error code `ERROR_NOT_SUPPORTED`.
const ERROR_NOT_SUPPORTED: i32 = 50;
/// `STORAGE_PROPERTY_ID::StorageDeviceProperty`.
const STORAGE_DEVICE_PROPERTY: u32 = 0;
/// `STORAGE_PROPERTY_ID::StorageAdapterProperty`.
const STORAGE_ADAPTER_PROPERTY: u32 = 1;
/// `STORAGE_QUERY_TYPE::PropertyStandardQuery`.
const PROPERTY_STANDARD_QUERY: u32 = 0;

/// Windows error.
#[derive(Debug)]
pub enum Error {
    /// IO error.
    Io(std::io::Error),
    /// Invalid value for `STORAGE_DEVICE_DESCRIPTOR::BusType`.
    InvalidStorageBusType(u32),
    /// SCSI interface error.
    Scsi(scsi::Error),
}

impl Error {
    /// Get last OS error.
    fn last_os_error() -> Self {
        std::io::Error::last_os_error().into()
    }

    /// If error represents an IOCTL being unsupported by the handle.
    fn is_ioctl_unsupported(&self) -> bool {
        matches!(self, Error::Io(x) if matches!(x.raw_os_error(), Some(ERROR_INVALID_FUNCTION | ERROR_NOT_SUPPORTED)))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Io(x) => Some(x),
            Self::Scsi(x) => Some(x),
            Self::InvalidStorageBusType(_) => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io(_) => write!(f, "IO error"),
            Self::InvalidStorageBusType(x) => write!(f, "invalid storage bus type {x:#x}"),
            Self::Scsi(_) => write!(f, "SCSI error"),
        }
    }
}

impl From<std::io::Error> for Error {
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<scsi::Error> for Error {
    fn from(value: scsi::Error) -> Self {
        Self::Scsi(value)
    }
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn QueryDosDeviceA(lpDeviceName: *const u8, lpTargetPath: *mut u8, ucchMax: u32) -> u32;
    fn DeviceIoControl(
        hDevice: std::os::windows::raw::HANDLE,
        dwIoControlCode: u32,
        lpInBuffer: *mut c_void,
        nInBufferSize: u32,
        lpOutBuffer: *mut c_void,
        nOutBufferSize: u32,
        lpBytesReturned: *mut u32,
        lpOverlapped: *mut c_void,
    ) -> i32;
}

/// IOCTL requests.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Ioctl {
    /// `IOCTL_STORAGE_QUERY_PROPERTY`.
    StorageQueryProperty = 0x2D_1400,
    /// `IOCTL_SCSI_PASS_THROUGH_DIRECT`.
    ScsiPassThroughDirect = 0x4D014,
}

impl Ioctl {
    /// Execute ioctl.
    unsafe fn execute<I, O>(
        self,
        file: &File,
        input: *mut I,
        output: *mut O,
    ) -> Result<u32, Error> {
        let mut returned = 0;
        let result = unsafe {
            DeviceIoControl(
                file.as_raw_handle(),
                self as _,
                input.cast(),
                size_of::<I>().try_into().unwrap(),
                output.cast(),
                size_of::<O>().try_into().unwrap(),
                &raw mut returned,
                std::ptr::null_mut(),
            )
        };
        if result == 0 {
            return Err(Error::last_os_error());
        }

        Ok(returned)
    }
}

impl std::fmt::Display for Ioctl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::StorageQueryProperty => "IOCTL_STORAGE_QUERY_PROPERTY",
            Self::ScsiPassThroughDirect => "IOCTL_SCSI_PASS_THROUGH_DIRECT",
        })
    }
}

/// Enumerates drives.
pub fn list_drives() -> Result<Box<[PathBuf]>, Error> {
    const BUFFER_SIZE: usize = 131_072;
    const PATH_PREFIX: &str = r"\\.\";
    const NAME_PREFIX: &str = "PhysicalDrive";

    let mut buffer = vec![0; BUFFER_SIZE].into_boxed_slice();

    let size = unsafe {
        QueryDosDeviceA(
            std::ptr::null(),
            buffer.as_mut_ptr(),
            buffer.len().try_into().unwrap(),
        )
    };
    if size == 0 {
        return Err(Error::last_os_error());
    }

    let paths = buffer[..size as _]
        .split(|&x| x == 0)
        .filter_map(|x| std::str::from_utf8(x).ok())
        .filter(|x| x.starts_with(NAME_PREFIX))
        .map(|x| format!("{PATH_PREFIX}{x}").into())
        .collect();

    Ok(paths)
}

/// Structure `STORAGE_PROPERTY_QUERY`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct StoragePropertyQuery {
    /// `STORAGE_PROPERTY_ID` value.
    property_id: u32,
    /// `STORAGE_QUERY_TYPE` value.
    query_type: u32,
    /// Property-specific parameters.
    additional_parameters: [u8; 1],
}

/// Structure `STORAGE_DEVICE_DESCRIPTOR`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct StorageDeviceDescriptor {
    /// Structure version.
    version: u32,
    /// Size in bytes.
    size: u32,
    /// Device type.
    device_type: u8,
    /// Device-type modifier qualifier.
    device_type_modifier: u8,
    /// Media is removable.
    removable_media: u8,
    /// Supports command queueing.
    command_queueing: u8,
    /// Offset of vendor ID.
    vendor_id_offset: u32,
    /// Offset of product ID.
    product_id_offset: u32,
    /// Offset of product revision.
    product_revision_offset: u32,
    /// Offset of serial number.
    serial_number_offset: u32,
    /// `STORAGE_BUS_TYPE` value.
    bus_type: u32,
    /// Size in bytes of `raw_device_properties`.
    raw_properties_length: u32,
    /// Raw properties data referenced by offset fields.
    raw_device_properties: [u8; 1],
}

/// Structure `STORAGE_ADAPTER_DESCRIPTOR`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct StorageAdapterDescriptor {
    /// Structure version.
    version: u32,
    /// Size in bytes.
    size: u32,
    /// Maximum supported transfer size.
    maximum_transfer_length: u32,
    /// Maximum number of physical pages per transfer.
    maximum_physical_pages: u32,
    /// Address alignment mask.
    alignment_mask: u32,
    /// Adapter uses programmed I/O rather than DMA.
    adapter_uses_pio: u8,
    /// Adapter scans down (descending addresses).
    adapter_scans_down: u8,
    /// Supports command queueing.
    command_queueing: u8,
    /// Supports accelerated transfer.
    accelerated_transfer: u8,
    /// `STORAGE_BUS_TYPE` value.
    bus_type: u8,
    /// Bus interface major version.
    bus_major_version: u16,
    /// Bus interface minor version.
    bus_minor_version: u16,
    /// SCSI request block type.
    srb_type: u8,
    /// Address type.
    address_type: u8,
}

/// Execute `IOCTL_STORAGE_QUERY_PROPERTY`,
unsafe fn storage_query_property<T>(
    file: &File,
    property_id: u32,
    output: &mut T,
) -> Result<(), Error> {
    let mut query = StoragePropertyQuery {
        property_id,
        query_type: PROPERTY_STANDARD_QUERY,
        ..Default::default()
    };

    debug!(
        "[Handle {:#x}] Executing {}: {query:?}",
        file.as_raw_handle() as usize,
        Ioctl::StorageQueryProperty
    );

    unsafe { Ioctl::StorageQueryProperty.execute(file, &raw mut query, output) }?;

    Ok(())
}

/// Enum `STORAGE_BUS_TYPE`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StorageBusType {
    /// Bus type could not be determined.
    Unknown = 0x0,
    /// Parallel SCSI.
    Scsi = 0x1,
    /// ATAPI (packet) device.
    Atapi = 0x2,
    /// Parallel ATA.
    Ata = 0x3,
    /// IEEE 1394 (`FireWire`).
    Ieee1394 = 0x4,
    /// Serial Storage Architecture.
    Ssa = 0x5,
    /// Fibre Channel.
    Fibre = 0x6,
    /// USB mass storage.
    Usb = 0x7,
    /// RAID volume.
    Raid = 0x8,
    /// iSCSI.
    IScsi = 0x9,
    /// Serial Attached SCSI.
    Sas = 0xA,
    /// Serial ATA.
    Sata = 0xB,
    /// SD card.
    Sd = 0xC,
    /// MMC card.
    Mmc = 0xD,
    /// Virtual disk.
    Virtual = 0xE,
    /// File-backed virtual disk.
    FileBackedVirtual = 0xF,
    /// Storage Spaces virtual disk.
    Spaces = 0x10,
    /// NVM Express.
    Nvme = 0x11,
    /// Storage Class Memory (persistent memory).
    Scm = 0x12,
    /// Universal Flash Storage.
    Ufs = 0x13,
    /// `NVMe` over Fabrics.
    NvmeOf = 0x14,
}

impl StorageBusType {
    /// Get bus type for device.
    fn get_device(file: &File) -> Result<Self, Error> {
        let mut descriptor = StorageDeviceDescriptor::default();

        unsafe { storage_query_property(file, STORAGE_DEVICE_PROPERTY, &mut descriptor) }?;

        let bus_type = descriptor.bus_type.try_into()?;
        debug!(
            "[Handle {:#x}] Device bus type: {bus_type}",
            file.as_raw_handle() as usize
        );

        Ok(bus_type)
    }
}

impl std::fmt::Display for StorageBusType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Unknown => "Unknown",
            Self::Scsi => "SCSI",
            Self::Atapi => "ATAPI",
            Self::Ata => "ATA",
            Self::Ieee1394 => "IEEE 1394",
            Self::Ssa => "SSA",
            Self::Fibre => "Fibre Channel",
            Self::Usb => "USB",
            Self::Raid => "RAID",
            Self::IScsi => "iSCSI",
            Self::Sas => "SAS",
            Self::Sata => "SATA",
            Self::Sd => "SD",
            Self::Mmc => "MMC",
            Self::Virtual => "Virtual",
            Self::FileBackedVirtual => "Virtual (file-backed)",
            Self::Spaces => "Storage Spaces",
            Self::Nvme => "NVMe",
            Self::Scm => "SCM",
            Self::Ufs => "UFS",
            Self::NvmeOf => "NVMe-oF",
        })
    }
}

impl TryFrom<u32> for StorageBusType {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Error> {
        const VARIANTS: &[StorageBusType] = &[
            StorageBusType::Unknown,
            StorageBusType::Scsi,
            StorageBusType::Atapi,
            StorageBusType::Ata,
            StorageBusType::Ieee1394,
            StorageBusType::Ssa,
            StorageBusType::Fibre,
            StorageBusType::Usb,
            StorageBusType::Raid,
            StorageBusType::IScsi,
            StorageBusType::Sas,
            StorageBusType::Sata,
            StorageBusType::Sd,
            StorageBusType::Mmc,
            StorageBusType::Virtual,
            StorageBusType::FileBackedVirtual,
            StorageBusType::Spaces,
            StorageBusType::Nvme,
            StorageBusType::Scm,
            StorageBusType::Ufs,
            StorageBusType::NvmeOf,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u32 == value)
            .copied()
            .ok_or(Error::InvalidStorageBusType(value))
    }
}

/// Get required alignment size for drive file.
fn get_alignment(file: &File) -> Result<usize, Error> {
    let mut descriptor = StorageAdapterDescriptor::default();

    unsafe { storage_query_property(file, STORAGE_ADAPTER_PROPERTY, &mut descriptor) }?;

    let alignment = (descriptor.alignment_mask as usize).saturating_add(1);

    Ok(alignment)
}
