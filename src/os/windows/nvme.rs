//! Windows NVMe interface.

use std::{
    fs::{File, OpenOptions},
    os::windows::{fs::OpenOptionsExt, io::AsRawHandle},
    path::{Path, PathBuf},
};

use log::debug;

use super::{
    FILE_SHARE_READ, FILE_SHARE_WRITE, Ioctl, PROPERTY_STANDARD_QUERY, StorageBusType,
    StoragePropertyQuery,
};
use crate::protocol::{
    Transfer,
    nvme::{self, command, command::Completion},
};

/// `STORAGE_PROTOCOL_TYPE.ProtocolTypeNvme`.
const PROTOCOL_TYPE_NVME: u32 = 0x3;

/// Windows NVMe error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid `STORAGE_PROTOCOL_COMMAND.ReturnStatus` value.
    InvalidProtocolStatus(u32),
    /// Invalid `STORAGE_PROTOCOL_COMMAND.ErrorCode` status value.
    InvalidErrorCode(u32),
    /// Invalid `STORAGE_PROTOCOL_DATA_DESCRIPTOR` returned data location.
    InvalidProtocolData(u32, u32),
    /// `STORAGE_PROTOCOL_COMMAND.ReturnStatus` reports error.
    ProtocolStatus(ProtocolStatus, u32),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidProtocolStatus(x) => write!(
                f,
                "invalid {} return_status {x:#x}",
                Ioctl::StorageProtocolCommand
            ),
            Self::InvalidErrorCode(x) => write!(
                f,
                "invalid {} error_code {x:#x}",
                Ioctl::StorageProtocolCommand
            ),
            Self::InvalidProtocolData(x, y) => write!(
                f,
                "invalid {} protocol_data_offset {x:#x} protocol_data_length {y:#x}",
                Ioctl::StorageQueryProperty
            ),
            Self::ProtocolStatus(x, y) => write!(
                f,
                "{} return_status {x} error_code {y:#x}",
                Ioctl::StorageProtocolCommand
            ),
        }
    }
}

impl From<Error> for nvme::Error {
    fn from(value: Error) -> Self {
        super::Error::from(value).into()
    }
}

/// `STORAGE_PROTOCOL_COMMAND.ReturnStatus` values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtocolStatus {
    /// `STORAGE_PROTOCOL_STATUS_PENDING`.
    Pending = 0x0,
    /// `STORAGE_PROTOCOL_STATUS_SUCCESS`.
    Success = 0x1,
    /// `STORAGE_PROTOCOL_STATUS_ERROR`.
    Error = 0x2,
    /// `STORAGE_PROTOCOL_STATUS_INVALID_REQUEST`.
    InvalidRequest = 0x3,
    /// `STORAGE_PROTOCOL_STATUS_NO_DEVICE`.
    NoDevice = 0x4,
    /// `STORAGE_PROTOCOL_STATUS_BUSY`.
    Busy = 0x5,
    /// `STORAGE_PROTOCOL_STATUS_DATA_OVERRUN`.
    DataOverrun = 0x6,
    /// `STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES`.
    InsufficientResources = 0x7,
    /// `STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST`.
    ThrottledRequest = 0x8,
    /// `STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED`.
    NotSupported = 0xFF,
}

impl TryFrom<u32> for ProtocolStatus {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Error> {
        const VARIANTS: &[ProtocolStatus] = &[
            ProtocolStatus::Pending,
            ProtocolStatus::Success,
            ProtocolStatus::Error,
            ProtocolStatus::InvalidRequest,
            ProtocolStatus::NoDevice,
            ProtocolStatus::Busy,
            ProtocolStatus::DataOverrun,
            ProtocolStatus::InsufficientResources,
            ProtocolStatus::ThrottledRequest,
            ProtocolStatus::NotSupported,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u32 == value)
            .copied()
            .ok_or(Error::InvalidProtocolStatus(value))
    }
}

impl std::fmt::Display for ProtocolStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Self::Pending => "STORAGE_PROTOCOL_STATUS_PENDING",
            Self::Success => "STORAGE_PROTOCOL_STATUS_SUCCESS",
            Self::Error => "STORAGE_PROTOCOL_STATUS_ERROR",
            Self::InvalidRequest => "STORAGE_PROTOCOL_STATUS_INVALID_REQUEST",
            Self::NoDevice => "STORAGE_PROTOCOL_STATUS_NO_DEVICE",
            Self::Busy => "STORAGE_PROTOCOL_STATUS_BUSY",
            Self::DataOverrun => "STORAGE_PROTOCOL_STATUS_DATA_OVERRUN",
            Self::InsufficientResources => "STORAGE_PROTOCOL_STATUS_INSUFFICIENT_RESOURCES",
            Self::ThrottledRequest => "STORAGE_PROTOCOL_STATUS_THROTTLED_REQUEST",
            Self::NotSupported => "STORAGE_PROTOCOL_STATUS_NOT_SUPPORTED",
        })
    }
}

/// Structure `STORAGE_PROTOCOL_COMMAND`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct StorageProtocolCommand {
    /// Structure version.
    version: u32,
    /// Size in bytes.
    length: u32,
    /// `STORAGE_PROTOCOL_TYPE` value.
    protocol_type: u32,
    /// Bitmask of request flags.
    flags: u32,
    /// `STORAGE_PROTOCOL_STATUS` value.
    return_status: u32,
    /// Protocol-specific error code.
    error_code: u32,
    /// Command size.
    command_length: u32,
    /// Error information buffer size.
    error_info_length: u32,
    /// Host-to-device data size.
    data_to_device_transfer_length: u32,
    /// Device-to-host data size.
    data_from_device_transfer_length: u32,
    /// Command timeout in seconds.
    time_out_value: u32,
    /// Offset to error information buffer.
    error_info_offset: u32,
    /// Offset to host-to-device data.
    data_to_device_buffer_offset: u32,
    /// Offset to device-to-host data.
    data_from_device_buffer_offset: u32,
    /// Protocol-specific request qualifier.
    command_specific: u32,
    /// Reserved.
    reserved0: u32,
    /// Protocol-specific returned data, completion queue entry dword 0.
    fixed_protocol_return_data: u32,
    /// Protocol-specific returned data, completion queue entry dword 1.
    fixed_protocol_return_data2: u32,
    /// Reserved.
    reserved1: [u32; 2],
    /// Command, followed by the data buffers at their offsets.
    command: [u8; 1],
}

/// Structure `STORAGE_PROTOCOL_SPECIFIC_DATA`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct StorageProtocolSpecificData {
    /// `STORAGE_PROTOCOL_TYPE` value.
    protocol_type: u32,
    /// Protocol-specific data type.
    data_type: u32,
    /// Data request value.
    protocol_data_request_value: u32,
    /// Data sub request value.
    protocol_data_request_sub_value: u32,
    /// Offset to data from this structure.
    protocol_data_offset: u32,
    /// Data size.
    protocol_data_length: u32,
    /// Protocol-specific returned data, completion queue entry dword 0.
    fixed_protocol_return_data: u32,
    /// First additional data sub request value.
    protocol_data_request_sub_value2: u32,
    /// Second additional data sub request value.
    protocol_data_request_sub_value3: u32,
    /// Third additional data sub request value.
    protocol_data_request_sub_value4: u32,
}

/// Structure `STORAGE_PROTOCOL_DATA_DESCRIPTOR`.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
#[repr(C)]
struct StorageProtocolDataDescriptor {
    /// Structure version.
    version: u32,
    /// Size in bytes.
    size: u32,
    /// Protocol-specific data, followed by the data at its offset.
    protocol_specific: StorageProtocolSpecificData,
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
    /// Display path as drive.
    fn format_path(path: &Path) -> String {
        format!("NVMe Windows {}", path.display())
    }

    /// Execute command with `IOCTL_STORAGE_PROTOCOL_COMMAND`.
    fn protocol_command(
        &self,
        command: &command::AdminCommand,
        transfer: Transfer,
        timeout: u32,
    ) -> Result<Completion, nvme::Error> {
        const COMMAND_OFFSET: usize = std::mem::offset_of!(StorageProtocolCommand, command);
        const ERROR_INFO_OFFSET: usize = COMMAND_OFFSET + command::AdminCommand::SIZE;
        const ERROR_INFO_LENGTH: usize = command::AdminCommand::SIZE;
        const DATA_OFFSET: usize = ERROR_INFO_OFFSET + ERROR_INFO_LENGTH;
        const STRUCTURE_VERSION: u32 = 0x1;
        const FLAG_ADAPTER_REQUEST: u32 = 0x8000_0000;
        const SPECIFIC_NVME_ADMIN_COMMAND: u32 = 0x1;

        let transfer_size = transfer.size();

        let length = size_of::<StorageProtocolCommand>().try_into().unwrap();
        let command_length = command::AdminCommand::SIZE.try_into().unwrap();
        let error_info_length = ERROR_INFO_LENGTH.try_into().unwrap();
        let error_info_offset = ERROR_INFO_OFFSET.try_into().unwrap();
        let data_length = transfer_size.try_into().unwrap();
        let data_offset = DATA_OFFSET.try_into().unwrap();

        let (data_to_device_transfer_length, data_to_device_buffer_offset) = match &transfer {
            Transfer::Write(_) => (data_length, data_offset),
            Transfer::None | Transfer::Read(_) => (0, 0),
        };

        let (data_from_device_transfer_length, data_from_device_buffer_offset) = match &transfer {
            Transfer::Read(_) => (data_length, data_offset),
            Transfer::None | Transfer::Write(_) => (0, 0),
        };

        let protocol_command = StorageProtocolCommand {
            version: STRUCTURE_VERSION,
            length,
            protocol_type: PROTOCOL_TYPE_NVME,
            flags: FLAG_ADAPTER_REQUEST,
            command_length,
            error_info_length,
            error_info_offset,
            data_to_device_transfer_length,
            data_from_device_transfer_length,
            time_out_value: timeout,
            data_to_device_buffer_offset,
            data_from_device_buffer_offset,
            command_specific: SPECIFIC_NVME_ADMIN_COMMAND,
            ..Default::default()
        };

        let mut buffer = vec![0; DATA_OFFSET + transfer_size].into_boxed_slice();
        unsafe {
            buffer
                .as_mut_ptr()
                .cast::<StorageProtocolCommand>()
                .write_unaligned(protocol_command);
        }
        buffer[COMMAND_OFFSET..COMMAND_OFFSET + command::AdminCommand::SIZE]
            .copy_from_slice(&<[u8; _]>::from(command));

        if let Transfer::Write(data) = &transfer {
            buffer[DATA_OFFSET..].copy_from_slice(data);
        }

        debug!(
            "[{self}] Executing {}: {protocol_command:?}",
            Ioctl::StorageProtocolCommand
        );

        let result =
            unsafe { Ioctl::StorageProtocolCommand.execute_buffer(&self.file, &mut buffer) };

        let protocol_command = unsafe {
            buffer
                .as_ptr()
                .cast::<StorageProtocolCommand>()
                .read_unaligned()
        };

        debug!(
            "[{self}] Executed {}: {protocol_command:?} ({result:?})",
            Ioctl::StorageProtocolCommand
        );

        match result {
            Err(x) if x.is_ioctl_unsupported() => return Err(nvme::Error::UnsupportedDrive),
            Err(x) => return Err(x.into()),
            Ok(_) => {},
        }

        // Failed command reports the completion queue entry status.
        let status = match ProtocolStatus::try_from(protocol_command.return_status)? {
            ProtocolStatus::Success => nvme::status::StatusField::try_from(0)?,
            ProtocolStatus::NotSupported => return Err(nvme::Error::UnsupportedDrive),
            ProtocolStatus::Error => {
                let error_code = protocol_command.error_code;
                let error =
                    u16::try_from(error_code).map_err(|_| Error::InvalidErrorCode(error_code))?;
                let status = nvme::status::StatusField::try_from(error >> 1)?;

                // Command that failed without status is an interface error.
                if !status.is_error() {
                    return Err(Error::ProtocolStatus(ProtocolStatus::Error, error_code).into());
                }

                status
            },
            x => return Err(Error::ProtocolStatus(x, protocol_command.error_code).into()),
        };

        if let Transfer::Read(data) = transfer {
            data.copy_from_slice(&buffer[DATA_OFFSET..]);
        }

        Ok(Completion {
            status,
            result: protocol_command.fixed_protocol_return_data,
        })
    }

    /// Execute protocol data query with `IOCTL_STORAGE_QUERY_PROPERTY`.
    fn protocol_query(
        &self,
        protocol_specific: StorageProtocolSpecificData,
        data: &mut [u8],
    ) -> Result<Completion, nvme::Error> {
        const PARAMETERS_OFFSET: usize =
            std::mem::offset_of!(StoragePropertyQuery, additional_parameters);
        const SPECIFIC_DATA_OFFSET: usize =
            std::mem::offset_of!(StorageProtocolDataDescriptor, protocol_specific);
        const DESCRIPTOR_SIZE: usize = size_of::<StorageProtocolDataDescriptor>();
        const STORAGE_ADAPTER_PROTOCOL_SPECIFIC_PROPERTY: u32 = 0x31;

        let query = StoragePropertyQuery {
            property_id: STORAGE_ADAPTER_PROTOCOL_SPECIFIC_PROPERTY,
            query_type: PROPERTY_STANDARD_QUERY,
            ..Default::default()
        };

        let protocol_specific = StorageProtocolSpecificData {
            protocol_type: PROTOCOL_TYPE_NVME,
            protocol_data_offset: size_of::<StorageProtocolSpecificData>().try_into().unwrap(),
            protocol_data_length: data.len().try_into().unwrap(),
            ..protocol_specific
        };

        let mut buffer = vec![0; DESCRIPTOR_SIZE + data.len()].into_boxed_slice();
        unsafe {
            buffer
                .as_mut_ptr()
                .cast::<StoragePropertyQuery>()
                .write_unaligned(query);
            buffer[PARAMETERS_OFFSET..]
                .as_mut_ptr()
                .cast::<StorageProtocolSpecificData>()
                .write_unaligned(protocol_specific);
        }

        debug!(
            "[{self}] Executing {}: {query:?}, {protocol_specific:?}",
            Ioctl::StorageQueryProperty
        );

        let result = unsafe { Ioctl::StorageQueryProperty.execute_buffer(&self.file, &mut buffer) };

        let descriptor = unsafe {
            buffer
                .as_ptr()
                .cast::<StorageProtocolDataDescriptor>()
                .read_unaligned()
        };

        debug!(
            "[{self}] Executed {}: {descriptor:?} ({result:?})",
            Ioctl::StorageQueryProperty
        );

        match result {
            Err(x) if x.is_ioctl_unsupported() => return Err(nvme::Error::UnsupportedDrive),
            Err(x) => return Err(x.into()),
            Ok(_) => {},
        }

        let StorageProtocolSpecificData {
            protocol_data_offset,
            protocol_data_length,
            fixed_protocol_return_data,
            ..
        } = descriptor.protocol_specific;

        // Data may be returned at a different offset than requested
        let returned = buffer[SPECIFIC_DATA_OFFSET..]
            .get(protocol_data_offset as usize..)
            .and_then(|x| x.get(..protocol_data_length as usize))
            .filter(|x| x.len() == data.len());

        let Some(returned) = returned else {
            return Err(
                Error::InvalidProtocolData(protocol_data_offset, protocol_data_length).into(),
            );
        };

        data.copy_from_slice(returned);

        // Failed command is reported as an OS error without a status
        Ok(Completion {
            status: nvme::status::StatusField::try_from(0)?,
            result: fixed_protocol_return_data,
        })
    }

    /// Read identify data with `IOCTL_STORAGE_QUERY_PROPERTY`.
    fn identify_query(
        &self,
        command: &command::AdminCommand,
        data: &mut [u8],
    ) -> Result<Completion, nvme::Error> {
        const NVME_DATA_TYPE_IDENTIFY: u32 = 0x1;

        // Identify command fields are passed as query request values
        let cns = command.cdw10 & 0xFF;
        let controller_id = command.cdw10 >> 16;
        let cns_specific_id = command.cdw11 & 0xFFFF;
        let command_set_id = command.cdw11 >> 24;

        self.protocol_query(
            StorageProtocolSpecificData {
                data_type: NVME_DATA_TYPE_IDENTIFY,
                protocol_data_request_value: cns,
                protocol_data_request_sub_value: command.nsid,
                protocol_data_request_sub_value2: cns_specific_id,
                protocol_data_request_sub_value3: controller_id,
                protocol_data_request_sub_value4: command_set_id,
                ..Default::default()
            },
            data,
        )
    }

    /// Read log page data with `IOCTL_STORAGE_QUERY_PROPERTY`.
    fn log_page_query(
        &self,
        command: &command::AdminCommand,
        data: &mut [u8],
    ) -> Result<Completion, nvme::Error> {
        const NVME_DATA_TYPE_LOG_PAGE: u32 = 0x2;

        // Get log page command fields are passed as query request values
        let page = command.cdw10 & 0xFF;
        let log_specific_field = (command.cdw10 >> 8) & 0xF;
        let retain_async_event = (command.cdw10 >> 15) & 0x1;
        let log_specific_id = command.cdw11 >> 16;
        let uuid_index = command.cdw14 & 0x7F;

        // Retain event, log specific field and UUID index share one sub value
        let log_page_sub_value = retain_async_event | (log_specific_field << 1) | (uuid_index << 8);

        self.protocol_query(
            StorageProtocolSpecificData {
                data_type: NVME_DATA_TYPE_LOG_PAGE,
                protocol_data_request_value: page,
                protocol_data_request_sub_value: command.cdw12,
                protocol_data_request_sub_value2: command.cdw13,
                protocol_data_request_sub_value3: log_specific_id,
                protocol_data_request_sub_value4: log_page_sub_value,
                ..Default::default()
            },
            data,
        )
    }
}

impl Interface {
    /// Open drive.
    pub(crate) fn open(path: &Path) -> Result<Self, nvme::Error> {
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
            Ok(StorageBusType::Nvme) => {},
            Ok(_) => return Err(nvme::Error::UnsupportedDrive),
            Err(x) if x.is_ioctl_unsupported() => return Err(nvme::Error::UnsupportedDrive),
            Err(x) => return Err(x.into()),
        }

        Ok(Self {
            path: path.to_owned(),
            file,
        })
    }
}

impl nvme::Interface for Interface {
    fn path(&self) -> &Path {
        &self.path
    }

    fn admin_command(
        &self,
        command: &command::AdminCommand,
        transfer: Transfer,
        timeout: u32,
    ) -> Result<Completion, nvme::Error> {
        // IOCTL_STORAGE_PROTOCOL_COMMAND doesn't allow identify or get log
        // page, so use IOCTL_STORAGE_QUERY_PROPERTY.
        match (command.opcode, transfer) {
            (command::AdminOpcode::Identify, Transfer::Read(data)) => {
                self.identify_query(command, data)
            },
            (command::AdminOpcode::GetLogPage, Transfer::Read(data)) => {
                self.log_page_query(command, data)
            },
            (_, transfer) => self.protocol_command(command, transfer, timeout),
        }
    }
}

impl std::fmt::Display for Interface {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", Self::format_path(&self.path))
    }
}
