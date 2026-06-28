//! SMART/GPL log functionality.

pub mod phy_event_counters;

use std::ops::{Index, Range};

/// Log page size in bytes.
pub const PAGE_SIZE: usize = super::SECTOR_SIZE;
/// Page count of each host-specific log.
const HOST_SPECIFIC_PAGES: u16 = 16;

/// Log error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid log address value.
    InvalidLog(u8),
    /// Invalid log directory data version.
    InvalidDirectoryVersion(u16),
    /// Invalid host-specific directory entry (address, page-count).
    InvalidHostSpecific(u8, u16),
    /// Invalid log data checksum.
    InvalidChecksum(u8),
    /// SATA PHY Event Counters log error.
    PhyEventCounters(phy_event_counters::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::PhyEventCounters(x) => Some(x),
            _ => None,
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidLog(x) => write!(f, "invalid log {x:#x}"),
            Self::InvalidDirectoryVersion(x) => write!(f, "directory invalid version {x:#x}"),
            Self::InvalidHostSpecific(x, y) => {
                write!(f, "directory invalid host-specific entry {x:#x}-{y:#x}")
            },
            Self::InvalidChecksum(x) => {
                write!(f, "invalid checksum {x:#x}")
            },
            Self::PhyEventCounters(_) => write!(f, "PHY event counters error"),
        }
    }
}

impl From<phy_event_counters::Error> for Error {
    fn from(e: phy_event_counters::Error) -> Self {
        Self::PhyEventCounters(e)
    }
}

/// Log addresses.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Log {
    /// Log directory.
    Directory = 0x0,
    /// Summary SMART Error log.
    SummarySmartError = 0x1,
    /// Comprehensive SMART Error log.
    ComprehensiveSmartError = 0x2,
    /// Extended Comprehensive SMART Error log.
    ExtComprehensiveSmartError = 0x3,
    /// Device Statistics log.
    DeviceStatistics = 0x4,
    /// SMART Self-Test log.
    SmartSelfTest = 0x6,
    /// Extended SMART Self-Test log.
    ExtSmartSelfTest = 0x7,
    /// Power Conditions log.
    PowerConditions = 0x8,
    /// Selective Self-Test log.
    SelectiveSelfTest = 0x9,
    /// Device Statistics Notification log.
    DeviceStatisticsNotification = 0xA,
    /// Pending Defects log.
    PendingDefects = 0xC,
    /// Long Physical Sector misalignment log.
    LpsMisalignment = 0xD,
    /// Sense Data for Successful NCQ Commands log.
    SenseDataSuccessfulNcq = 0xF,
    /// NCQ Command Error log.
    NcqCommandError = 0x10,
    /// SATA PHY Event Counters log.
    PhyEventCounters = 0x11,
    /// SATA NCQ Non-Data log.
    NcqNonData = 0x12,
    /// SATA NCQ Send and Receive log.
    NcqSendReceive = 0x13,
    /// Hybrid Information log.
    HybridInformation = 0x14,
    /// Rebuild Assist log.
    RebuildAssist = 0x15,
    /// LBA Status log.
    LbaStatus = 0x19,
    /// Write Stream Error log.
    WriteStreamError = 0x21,
    /// Read Stream Error log.
    ReadStreamError = 0x22,
    /// Current Device Internal Status Data log.
    CurrentDeviceInternalStatus = 0x24,
    /// Saved Device Internal Status Data log.
    SavedDeviceInternalStatus = 0x25,
    /// Set Sector Configuration log.
    SetSectorConfiguration = 0x2F,
    /// IDENTIFY DEVICE log.
    IdentifyDevice = 0x30,
    /// Host Specific log.
    HostSpecific(u8),
    /// Device Vendor Specific log.
    VendorSpecific(u8),
    /// SCT (SMART Command Transport) Command/Status log.
    SctCommandStatus = 0xE0,
    /// SCT (SMART Command Transport) Data Transfer log.
    SctDataTransfer = 0xE1,
}

impl Log {
    /// Log addresses reserved for host-specific use.
    const HOST_SPECIFIC_RANGE: Range<u8> = 0x80..0xA0;
    /// Log addresses reserved for device-vendor-specific use.
    const VENDOR_SPECIFIC_RANGE: Range<u8> = 0xA0..0xE0;

    /// Is SMART log.
    pub fn smart(self) -> bool {
        matches!(
            self,
            Self::Directory
                | Self::SummarySmartError
                | Self::ComprehensiveSmartError
                | Self::DeviceStatistics
                | Self::SmartSelfTest
                | Self::SelectiveSelfTest
                | Self::LpsMisalignment
                | Self::IdentifyDevice
                | Self::HostSpecific(_)
                | Self::VendorSpecific(_)
                | Self::SctCommandStatus
                | Self::SctDataTransfer
        )
    }

    /// Is GPL log.
    pub fn gpl(self) -> bool {
        matches!(
            self,
            Self::Directory
                | Self::ExtComprehensiveSmartError
                | Self::DeviceStatistics
                | Self::ExtSmartSelfTest
                | Self::PowerConditions
                | Self::DeviceStatisticsNotification
                | Self::PendingDefects
                | Self::LpsMisalignment
                | Self::SenseDataSuccessfulNcq
                | Self::NcqCommandError
                | Self::PhyEventCounters
                | Self::NcqNonData
                | Self::NcqSendReceive
                | Self::HybridInformation
                | Self::RebuildAssist
                | Self::LbaStatus
                | Self::WriteStreamError
                | Self::ReadStreamError
                | Self::CurrentDeviceInternalStatus
                | Self::SavedDeviceInternalStatus
                | Self::SetSectorConfiguration
                | Self::IdentifyDevice
                | Self::HostSpecific(_)
                | Self::VendorSpecific(_)
                | Self::SctCommandStatus
                | Self::SctDataTransfer
        )
    }
}

impl From<Log> for u8 {
    fn from(value: Log) -> Self {
        match value {
            Log::HostSpecific(x) | Log::VendorSpecific(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl TryFrom<u8> for Log {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[Log] = &[
            Log::Directory,
            Log::SummarySmartError,
            Log::ComprehensiveSmartError,
            Log::ExtComprehensiveSmartError,
            Log::DeviceStatistics,
            Log::SmartSelfTest,
            Log::ExtSmartSelfTest,
            Log::PowerConditions,
            Log::SelectiveSelfTest,
            Log::DeviceStatisticsNotification,
            Log::PendingDefects,
            Log::LpsMisalignment,
            Log::SenseDataSuccessfulNcq,
            Log::NcqCommandError,
            Log::PhyEventCounters,
            Log::NcqNonData,
            Log::NcqSendReceive,
            Log::HybridInformation,
            Log::RebuildAssist,
            Log::LbaStatus,
            Log::WriteStreamError,
            Log::ReadStreamError,
            Log::CurrentDeviceInternalStatus,
            Log::SavedDeviceInternalStatus,
            Log::SetSectorConfiguration,
            Log::IdentifyDevice,
            Log::SctCommandStatus,
            Log::SctDataTransfer,
        ];

        if let Some(x) = CONST_VARIANTS
            .iter()
            .find(|&&y| Into::<u8>::into(y) == value)
            .copied()
        {
            return Ok(x);
        }

        if Self::HOST_SPECIFIC_RANGE.contains(&value) {
            return Ok(Self::HostSpecific(value));
        }

        if Self::VENDOR_SPECIFIC_RANGE.contains(&value) {
            return Ok(Self::VendorSpecific(value));
        }

        Err(Error::InvalidLog(value))
    }
}

impl std::fmt::Display for Log {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Directory => write!(f, "Log Directory"),
            Self::SummarySmartError => write!(f, "Summary SMART Error Log"),
            Self::ComprehensiveSmartError => write!(f, "Comprehensive SMART Error Log"),
            Self::ExtComprehensiveSmartError => {
                write!(f, "Extended Comprehensive SMART Error Log")
            },
            Self::DeviceStatistics => write!(f, "Device Statistics"),
            Self::SmartSelfTest => write!(f, "SMART Self-Test Log"),
            Self::ExtSmartSelfTest => write!(f, "Extended SMART Self-Test Log"),
            Self::PowerConditions => write!(f, "Power Conditions"),
            Self::SelectiveSelfTest => write!(f, "Selective Self-Test Log"),
            Self::DeviceStatisticsNotification => write!(f, "Device Statistics Notification"),
            Self::PendingDefects => write!(f, "Pending Defects Log"),
            Self::LpsMisalignment => write!(f, "LPS Mis-alignment Log"),
            Self::SenseDataSuccessfulNcq => {
                write!(f, "Sense Data for Successful NCQ Commands Log")
            },
            Self::NcqCommandError => write!(f, "NCQ Command Error Log"),
            Self::PhyEventCounters => write!(f, "SATA PHY Event Counters Log"),
            Self::NcqNonData => write!(f, "SATA NCQ Non-Data Log"),
            Self::NcqSendReceive => write!(f, "SATA NCQ Send and Receive Log"),
            Self::HybridInformation => write!(f, "Hybrid Information Log"),
            Self::RebuildAssist => write!(f, "Rebuild Assist Log"),
            Self::LbaStatus => write!(f, "LBA Status"),
            Self::WriteStreamError => write!(f, "Write Stream Error Log"),
            Self::ReadStreamError => write!(f, "Read Stream Error Log"),
            Self::CurrentDeviceInternalStatus => {
                write!(f, "Current Device Internal Status Data Log")
            },
            Self::SavedDeviceInternalStatus => write!(f, "Saved Device Internal Status Data Log"),
            Self::SetSectorConfiguration => write!(f, "Set Sector Configuration"),
            Self::IdentifyDevice => write!(f, "IDENTIFY DEVICE data"),
            Self::HostSpecific(x) => write!(f, "Host Specific {x:#x}"),
            Self::VendorSpecific(x) => write!(f, "Vendor Specific {x:#x}"),
            Self::SctCommandStatus => write!(f, "SCT Command/Status"),
            Self::SctDataTransfer => write!(f, "SCT Data Transfer"),
        }
    }
}

/// Log directory.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Directory {
    /// Directory entries, indexed by log address with page count as value.
    entries: [u16; Self::ENTRY_COUNT],
}

impl Directory {
    /// Number of entries in a directory.
    const ENTRY_COUNT: usize = u8::MAX as usize + 1;

    /// Validate directory version.
    fn validate_version(data: &[u8; PAGE_SIZE]) -> Result<(), Error> {
        /// Only directory version defined by the standard (0001h).
        const VERSION: u16 = 1;

        let version = u16::from_le_bytes([data[0], data[1]]);
        if version != VERSION {
            return Err(Error::InvalidDirectoryVersion(version));
        }

        Ok(())
    }

    /// Validate host-specific log entries.
    fn validate_host_specific(entries: &[u16; Self::ENTRY_COUNT]) -> Result<(), Error> {
        for log in Log::HOST_SPECIFIC_RANGE {
            let pages = entries[log as usize];

            if pages != HOST_SPECIFIC_PAGES {
                return Err(Error::InvalidHostSpecific(log, pages));
            }
        }

        Ok(())
    }

    /// Parse SMART log directory.
    pub fn parse_smart(data: &[u8; PAGE_SIZE]) -> Result<Self, Error> {
        const ENTRY_STRIDE: usize = 2;

        Self::validate_version(data)?;

        // First byte defined as 1 (version low-byte), and directory is 1 page,
        // so just use all bytes for entries
        let entries = std::array::from_fn(|i| data[i * ENTRY_STRIDE].into());

        Self::validate_host_specific(&entries)?;

        Ok(Self { entries })
    }

    /// Parse General Purpose Log directory.
    pub fn parse_gpl(data: &[u8; PAGE_SIZE]) -> Result<Self, Error> {
        const ENTRY_SIZE: usize = size_of::<u16>();

        Self::validate_version(data)?;

        // Version defined as 1 and directory is 1 page, so just use all words as
        // entries
        let entries = std::array::from_fn(|i| {
            u16::from_le_bytes([data[i * ENTRY_SIZE], data[i * ENTRY_SIZE + 1]])
        });

        Self::validate_host_specific(&entries)?;

        Ok(Self { entries })
    }
}

impl Index<u8> for Directory {
    type Output = u16;

    fn index(&self, index: u8) -> &Self::Output {
        &self.entries[index as usize]
    }
}

impl Index<Log> for Directory {
    type Output = u16;

    fn index(&self, index: Log) -> &Self::Output {
        let index: u8 = index.into();
        &self[index]
    }
}

/// Validate log page checksum.
fn validate_checksum(data: &[u8; PAGE_SIZE]) -> Result<(), Error> {
    let checksum = data.iter().fold(0u8, |x, &y| x.wrapping_add(y));

    if checksum != 0 {
        return Err(Error::InvalidChecksum(checksum));
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn checksum() {
        const DATA_VALID: &[&[u8; PAGE_SIZE]] = &[
            test_data::westerndigital_scorpioblack::GP_LOG_11H,
            test_data::kingston_a400::GP_LOG_11H,
        ];
        const DATA_INVALID: &[&[u8; PAGE_SIZE]] = &[
            &{
                let mut x = [0; _];
                x[0] = 0x01;
                x
            },
            &{
                let mut x = [0xAA; _];
                x[x.len() - 1] = 0xBB;
                x
            },
        ];

        for &data in DATA_VALID {
            assert!(validate_checksum(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(validate_checksum(data).is_err());
        }
    }

    #[test]
    fn parse_smart_directory() {
        const DATA_VALID: &[&[u8; PAGE_SIZE]] = &[
            test_data::westerndigital_scorpioblack::SMART_LOG_0H,
            test_data::kingston_a400::SMART_LOG_0H,
        ];
        const DATA_INVALID: &[&[u8; PAGE_SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            let directory = Directory::parse_smart(data).unwrap();
            assert!(directory.entries.iter().any(|&x| x != 0));
        }

        for &data in DATA_INVALID {
            assert!(Directory::parse_smart(data).is_err());
        }
    }

    #[test]
    fn parse_gpl_directory() {
        const DATA_VALID: &[&[u8; PAGE_SIZE]] = &[
            test_data::westerndigital_scorpioblack::GP_LOG_0H,
            test_data::kingston_a400::GP_LOG_0H,
        ];
        const DATA_INVALID: &[&[u8; PAGE_SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            let directory = Directory::parse_gpl(data).unwrap();
            assert!(directory.entries.iter().any(|&x| x != 0));
        }

        for &data in DATA_INVALID {
            assert!(Directory::parse_gpl(data).is_err());
        }
    }
}
