//! NVMe drive interface.

pub mod command;
pub mod identify;
pub mod log;
pub mod status;

use std::path::Path;

use ::log::{debug, info};

use super::Transfer;
use crate::os;

/// Standard page size in bytes.
pub const PAGE_SIZE: usize = 4_096;
/// Default command timeout in seconds.
const DEFAULT_TIMEOUT: u32 = 30;

/// NVMe error.
#[derive(Debug)]
pub enum Error {
    /// OS error.
    Os(os::Error),
    /// Drive does not support NVMe.
    UnsupportedDrive,
    /// Status represents error.
    Status(status::StatusField),
    /// Invalid completion queue entry status field.
    InvalidStatus(status::Error),
    /// Namespace reports logical blocks without a usable LBA format.
    LbaFormatUnavailable(u8),
    /// Namespace list did not terminate within the supported number of pages.
    NamespaceListUnterminated,
    /// Invalid identify command response.
    Identify(identify::Error),
    /// Invalid get log page command response.
    Log(log::Error),
}

impl Error {
    /// Error represents a command failing.
    pub(crate) fn is_command_error(&self) -> bool {
        matches!(self, Self::Status(_))
    }
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::Os(x) => Some(x),
            Self::InvalidStatus(x) => Some(x),
            Self::Identify(x) => Some(x),
            Self::Log(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Os(_) => write!(f, "OS error"),
            Self::UnsupportedDrive => write!(f, "unsupported drive"),
            Self::Status(x) => write!(f, "status {x}"),
            Self::InvalidStatus(_) => write!(f, "invalid status"),
            Self::LbaFormatUnavailable(x) => write!(f, "unavailable LBA format {x}"),
            Self::NamespaceListUnterminated => write!(f, "unterminated namespace list"),
            Self::Identify(_) => write!(f, "{} error", command::AdminOpcode::Identify),
            Self::Log(_) => write!(f, "{} error", command::AdminOpcode::GetLogPage),
        }
    }
}

impl From<os::Error> for Error {
    fn from(value: os::Error) -> Self {
        Self::Os(value)
    }
}

impl From<identify::Error> for Error {
    fn from(value: identify::Error) -> Self {
        Self::Identify(value)
    }
}

impl From<log::Error> for Error {
    fn from(value: log::Error) -> Self {
        Self::Log(value)
    }
}

impl From<status::Error> for Error {
    fn from(value: status::Error) -> Self {
        Self::InvalidStatus(value)
    }
}

impl From<status::StatusField> for Error {
    fn from(value: status::StatusField) -> Self {
        Self::Status(value)
    }
}

/// NVMe transport interface.
pub trait Interface: std::fmt::Debug {
    /// Get path of drive.
    fn path(&self) -> &Path;
    /// Execute admin command.
    fn admin_command(
        &self,
        command: &command::AdminCommand,
        transfer: Transfer,
        timeout: u32,
    ) -> Result<command::Completion, Error>;
}

/// Drive interface.
#[derive(Debug)]
pub struct Drive {
    /// NVMe transport interface.
    interface: Box<dyn Interface>,
}

impl Drive {
    /// Open drive.
    pub(crate) fn open(path: &Path) -> Result<Self, Error> {
        let interface = Box::new(os::nvme::Interface::open(path)?);
        let drive = Self { interface };

        let identify = match drive.identify_controller() {
            Ok(x) => x,
            Err(x) if x.is_command_error() => return Err(Error::UnsupportedDrive),
            Err(x) => return Err(x),
        };

        info!("[{drive}] Opened: {identify}");

        Ok(drive)
    }

    /// Get path of drive.
    pub(crate) fn path(&self) -> &Path {
        self.interface.path()
    }

    /// Execute admin command.
    pub(crate) fn admin_command(
        &self,
        command: command::AdminCommand,
        transfer: Transfer,
        timeout: Option<u32>,
    ) -> Result<command::Completion, Error> {
        let timeout = timeout.unwrap_or(DEFAULT_TIMEOUT);

        let log_info = format!("command: {command}, transfer: {transfer}");
        debug!("[{self}] Executing admin command: ({log_info})");

        let completion = self.interface.admin_command(&command, transfer, timeout)?;

        info!(
            "[{self}] Executed admin command: ({log_info}, status: {}, result: {:#x})",
            completion.status, completion.result
        );

        if completion.status.is_error() {
            return Err(completion.status.into());
        }

        Ok(completion)
    }

    /// Execute identify command.
    pub(crate) fn identify(
        &self,
        data: &mut [u8],
        cns: u8,
        controller_id: u16,
        cns_specific_id: u16,
        nsid: u32,
    ) -> Result<command::Completion, Error> {
        let cdw10 = u32::from(cns) | (u32::from(controller_id) << 16);
        let cdw11 = u32::from(cns_specific_id);

        let command = command::AdminCommand {
            opcode: command::AdminOpcode::Identify,
            nsid,
            cdw10,
            cdw11,
            ..Default::default()
        };

        self.admin_command(command, Transfer::Read(data), None)
    }

    /// Execute identify controller command.
    pub(crate) fn identify_controller(&self) -> Result<identify::controller::Identify, Error> {
        const CNS_CONTROLLER: u8 = 0x1;

        let mut data = [0u8; identify::controller::Identify::SIZE];

        self.identify(&mut data, CNS_CONTROLLER, 0, 0, 0)?;

        let identify = (&data).try_into().map_err(identify::Error::from)?;
        debug!("[{self}] Identify controller: {identify:?}");

        Ok(identify)
    }

    /// Execute identify namespace command.
    pub(crate) fn identify_namespace(
        &self,
        nsid: u32,
    ) -> Result<identify::namespace::Identify, Error> {
        const CNS_NAMESPACE: u8 = 0x0;

        let mut data = [0u8; identify::namespace::Identify::SIZE];
        self.identify(&mut data, CNS_NAMESPACE, 0, 0, nsid)?;

        let identify: identify::namespace::Identify =
            (&data).try_into().map_err(identify::Error::from)?;
        debug!("[{self}] Identify namespace {nsid}: {identify:?}");

        Ok(identify)
    }

    /// Execute identify LBA format command, listing supported LBA formats.
    pub(crate) fn identify_lba_formats(
        &self,
        format_index: u8,
    ) -> Result<Box<[identify::namespace::LbaFormat]>, Error> {
        const CNS_LBA_FORMAT: u8 = 0x9;

        let mut data = [0u8; identify::namespace::Identify::SIZE];
        self.identify(&mut data, CNS_LBA_FORMAT, 0, format_index.into(), 0)?;

        let formats = identify::namespace::LbaFormat::parse_list(&data);
        debug!("[{self}] Identify LBA format {format_index}: {formats:?}");

        Ok(formats)
    }

    /// Execute identify namespace list command for namespaces after `nsid`.
    pub(crate) fn identify_namespace_list(
        &self,
        nsid: u32,
    ) -> Result<identify::namespace_list::NamespaceList, Error> {
        const CNS_ACTIVE_NAMESPACE_LIST: u8 = 0x2;

        let mut data = [0u8; identify::namespace_list::NamespaceList::SIZE];

        self.identify(&mut data, CNS_ACTIVE_NAMESPACE_LIST, 0, 0, nsid)?;

        let list = identify::namespace_list::NamespaceList::from(&data);
        debug!("[{self}] Identify active namespace list after {nsid}: {list:?}");

        Ok(list)
    }

    /// Execute get log page command.
    pub(crate) fn get_log_page(
        &self,
        data: &mut [u8],
        page: log::LogPage,
        log_specific_id: u16,
        offset: u64,
        nsid: u32,
    ) -> Result<command::Completion, Error> {
        // Transfer size is a zero-based count of dwords split across two dwords
        let dwords = u32::try_from(data.len() / size_of::<u32>())
            .unwrap()
            .saturating_sub(1);

        let cdw10 = u32::from(u8::from(page)) | ((dwords & 0xFFFF) << 16);
        let cdw11 = (dwords >> 16) | (u32::from(log_specific_id) << 16);
        let cdw12 = u32::try_from(offset & u64::from(u32::MAX)).unwrap();
        let cdw13 = u32::try_from(offset >> u32::BITS).unwrap();

        let command = command::AdminCommand {
            opcode: command::AdminOpcode::GetLogPage,
            nsid,
            cdw10,
            cdw11,
            cdw12,
            cdw13,
            ..Default::default()
        };

        self.admin_command(command, Transfer::Read(data), None)
    }

    /// Get commands supported and effects log.
    pub(crate) fn command_effects(&self) -> Result<log::command_effects::CommandEffects, Error> {
        let mut data = [0u8; log::command_effects::CommandEffects::SIZE];

        self.get_log_page(&mut data, log::LogPage::CommandEffects, 0, 0, 0)?;

        let effects: log::command_effects::CommandEffects = (&data).try_into()?;
        debug!("[{self}] Command effects: {effects:?}");

        Ok(effects)
    }

    /// Get total size of a namespace in bytes.
    pub(crate) fn namespace_size(&self, nsid: u32) -> Result<u64, Error> {
        let identify = self.identify_namespace(nsid)?;

        if identify.size == 0 {
            return Ok(0);
        }

        let format_index = identify.lba_format_index;

        // Formats with unique attributes are not included in identify
        // namespace, use identify LBA format instead
        let data_size = match identify.lba_format().and_then(|x| x.data_size) {
            Some(x) => Some(x),
            None => self
                .identify_lba_formats(format_index)?
                .get(usize::from(format_index))
                .and_then(|x| x.data_size),
        };

        let Some(data_size) = data_size else {
            return Err(Error::LbaFormatUnavailable(format_index));
        };

        Ok(identify.size.saturating_mul(data_size.into()))
    }

    /// Get total size of the active namespaces in bytes.
    pub(crate) fn total_namespace_size(&self) -> Result<u64, Error> {
        const MAX_PAGE_COUNT: usize = 64;

        let mut size = 0u64;
        let mut list = self.identify_namespace_list(0)?;

        for &nsid in &list.ids {
            size = size.saturating_add(self.namespace_size(nsid)?);
        }

        for _ in 1..MAX_PAGE_COUNT {
            if list.is_last() {
                return Ok(size);
            }

            let Some(&last) = list.ids.last() else {
                return Ok(size);
            };

            list = self.identify_namespace_list(last)?;

            // End iteration if list not advancing
            if list.ids.first().is_none_or(|&x| x <= last) {
                return Ok(size);
            }

            for &nsid in &list.ids {
                size = size.saturating_add(self.namespace_size(nsid)?);
            }
        }

        Err(Error::NamespaceListUnterminated)
    }
}

impl std::fmt::Display for Drive {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "NVMe {}", self.path().display())
    }
}
