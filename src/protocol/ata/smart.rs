//! SMART functionality.

/// SMART error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid checksum.
    InvalidChecksum(u8),
    /// Invalid offline data collection status value.
    InvalidOfflineDataCollectionStatus(u8),
    /// Invalid self-test execution status value.
    InvalidSelfTestExecutionStatus(u8),
    /// Invalid self-test percent-remaining value.
    InvalidSelfTestPercentageRemaining(u8),
    /// SMART ENABLE subcommand failed.
    EnableFailed,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidChecksum(x) => write!(f, "invalid checksum {x:#x}"),
            Self::InvalidOfflineDataCollectionStatus(x) => {
                write!(f, "invalid offline collection status {x:#x}")
            },
            Self::InvalidSelfTestExecutionStatus(x) => {
                write!(f, "invalid self-test execution status {x:#x}")
            },
            Self::InvalidSelfTestPercentageRemaining(x) => {
                write!(f, "invalid self-test percentage remaining {x:#x}")
            },
            Self::EnableFailed => {
                write!(f, "subcommand ENABLE failed")
            },
        }
    }
}

/// SMART subcommands.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SmartSubcommand {
    /// SMART READ DATA.
    ReadData = 0xD0,
    /// SMART READ ATTRIBUTE THRESHOLDS.
    ReadThresholds = 0xD1,
    /// SMART ENABLE/DISABLE ATTRIBUTE AUTOSAVE.
    Autosave = 0xD2,
    /// SMART SAVE ATTRIBUTE VALUES.
    Save = 0xD3,
    /// SMART EXECUTE OFF-LINE IMMEDIATE.
    ExecuteOffline = 0xD4,
    /// SMART READ LOG.
    ReadLog = 0xD5,
    /// SMART WRITE LOG.
    WriteLog = 0xD6,
    /// SMART WRITE ATTRIBUTE THRESHOLDS.
    WriteThresholds = 0xD7,
    /// SMART ENABLE OPERATIONS.
    Enable = 0xD8,
    /// SMART DISABLE OPERATIONS.
    Disable = 0xD9,
    /// SMART RETURN STATUS.
    ReturnStatus = 0xDA,
}

impl std::fmt::Display for SmartSubcommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ReadData => write!(f, "READ DATA"),
            Self::ReadThresholds => write!(f, "READ ATTRIBUTE THRESHOLDS"),
            Self::Autosave => write!(f, "ENABLE/DISABLE ATTRIBUTE AUTOSAVE"),
            Self::Save => write!(f, "SAVE ATTRIBUTE VALUES"),
            Self::ExecuteOffline => write!(f, "EXECUTE OFF-LINE IMMEDIATE"),
            Self::ReadLog => write!(f, "READ LOG"),
            Self::WriteLog => write!(f, "WRITE LOG"),
            Self::WriteThresholds => write!(f, "WRITE ATTRIBUTE THRESHOLDS"),
            Self::Enable => write!(f, "ENABLE OPERATIONS"),
            Self::Disable => write!(f, "DISABLE OPERATIONS"),
            Self::ReturnStatus => write!(f, "RETURN STATUS"),
        }
    }
}

/// Off-line data collection status field of SMART data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OfflineDataCollectionStatus {
    /// Never started since the last power-on.
    NeverStarted,
    /// The last offline collection completed without error.
    CompletedWithoutError,
    /// Offline collection is currently running.
    InProgress,
    /// Suspended by an interrupting host command.
    SuspendedByHost,
    /// Aborted by an interrupting host command.
    AbortedByHost,
    /// Aborted because the device hit a fatal/read error.
    AbortedWithFatalError,
    /// Vendor-specific status.
    VendorSpecific(u8),
}

impl TryFrom<u8> for OfflineDataCollectionStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0x00 | 0x80 => Ok(Self::NeverStarted),
            0x02 | 0x82 => Ok(Self::CompletedWithoutError),
            0x03 => Ok(Self::InProgress),
            0x04 | 0x84 => Ok(Self::SuspendedByHost),
            0x05 | 0x85 => Ok(Self::AbortedByHost),
            0x06 | 0x86 => Ok(Self::AbortedWithFatalError),
            0x40..=0x7F | 0xC0..=0xFF => Ok(Self::VendorSpecific(value)),
            _ => Err(Error::InvalidOfflineDataCollectionStatus(value)),
        }
    }
}

/// Self-test execution status field of SMART data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SelfTestExecutionStatus {
    /// Completed without error, or no self-test has been run.
    CompletedOrNoStatus = 0x0,
    /// Aborted by a host command.
    AbortedByHost = 0x1,
    /// Interrupted by a host reset.
    InterruptedByReset = 0x2,
    /// Could not complete due to a fatal/unknown error.
    FatalError = 0x3,
    /// Failed, but the failing test element could not be determined.
    FailedUnknownElement = 0x4,
    /// Failed the electrical element of the self-test.
    FailedElectrical = 0x5,
    /// Failed the servo (positioning) element of the self-test.
    FailedServo = 0x6,
    /// Failed the read element of the self-test.
    FailedRead = 0x7,
    /// Failed and the device may have suffered handling damage.
    FailedHandlingDamage = 0x8,
    /// A self-test is currently in progress.
    InProgress = 0xF,
}

impl TryFrom<u8> for SelfTestExecutionStatus {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[SelfTestExecutionStatus] = &[
            SelfTestExecutionStatus::CompletedOrNoStatus,
            SelfTestExecutionStatus::AbortedByHost,
            SelfTestExecutionStatus::InterruptedByReset,
            SelfTestExecutionStatus::FatalError,
            SelfTestExecutionStatus::FailedUnknownElement,
            SelfTestExecutionStatus::FailedElectrical,
            SelfTestExecutionStatus::FailedServo,
            SelfTestExecutionStatus::FailedRead,
            SelfTestExecutionStatus::FailedHandlingDamage,
            SelfTestExecutionStatus::InProgress,
        ];

        VARIANTS
            .iter()
            .find(|&&y| y as u8 == value)
            .copied()
            .ok_or(Error::InvalidSelfTestExecutionStatus(value))
    }
}

/// Off-line data collection capability field of SMART data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct OfflineDataCollectionCapability {
    /// Selective self-test routine is supported.
    pub selective_self_test: bool,
    /// Conveyance self-test routine is supported.
    pub conveyance_self_test: bool,
    /// Short and extended self-test routines are supported.
    pub self_test: bool,
    /// Offline read scanning is supported.
    pub offline_read_scanning: bool,
    /// Offline data collection is aborted by an interrupting command.
    pub aborts_on_interrupting_command: bool,
    /// Vendor-specific.
    pub vendor_specific_bit1: bool,
    /// SMART EXECUTE OFF-LINE IMMEDIATE subcommand is supported.
    pub execute_offline_immediate: bool,
}

impl From<u8> for OfflineDataCollectionCapability {
    fn from(value: u8) -> Self {
        Self {
            selective_self_test: value & (1 << 6) != 0,
            conveyance_self_test: value & (1 << 5) != 0,
            self_test: value & (1 << 4) != 0,
            offline_read_scanning: value & (1 << 3) != 0,
            aborts_on_interrupting_command: value & (1 << 2) != 0,
            vendor_specific_bit1: value & (1 << 1) != 0,
            execute_offline_immediate: value & (1 << 0) != 0,
        }
    }
}

/// SMART capability field of SMART data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SmartCapability {
    /// Saves SMART data before entering a power-saving mode.
    pub saves_smart_data_before_power_saving: bool,
    /// Attribute autosave is supported.
    pub attribute_autosave_supported: bool,
}

impl From<u16> for SmartCapability {
    fn from(value: u16) -> Self {
        let saves_smart_data_before_power_saving = value & (1 << 0) != 0;
        let attribute_autosave_supported = value & (1 << 1) != 0;

        Self {
            saves_smart_data_before_power_saving,
            attribute_autosave_supported,
        }
    }
}

/// SMART READ DATA subcommand response.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Smart {
    /// Vendor-specific bytes 0-361.
    pub vendor_specific_1: [u8; 362],
    /// Automatic offline data collection status.
    pub offline_data_collection_status: OfflineDataCollectionStatus,
    /// Result of the most recent self-test.
    pub self_test_execution_status: SelfTestExecutionStatus,
    /// Self-test percentage still remaining.
    pub self_test_percent_remaining: u8,
    /// Vendor-specific bytes 364-365.
    pub vendor_specific_2: [u8; 2],
    /// Vendor-specific byte 366.
    pub vendor_specific_3: u8,
    /// Off-line data collection capability fields.
    pub offline_data_collection_capability: OfflineDataCollectionCapability,
    /// SMART capability fields.
    #[allow(clippy::struct_field_names)]
    pub smart_capability: SmartCapability,
    /// Supports SMART error logging.
    pub error_logging_supported: bool,
    /// Vendor-specific byte 371.
    pub vendor_specific_4: u8,
    /// Short self-test polling time in minutes.
    pub short_self_test_poll_time: u8,
    /// Extended self-test polling time in minutes.
    pub extended_self_test_poll_time: u16,
    /// Conveyance self-test polling time in minutes.
    pub conveyance_self_test_poll_time: u8,
    /// Vendor-specific bytes 386-510.
    pub vendor_specific_5: [u8; 125],
}

impl Smart {
    /// Size in bytes.
    pub const SIZE: usize = super::SECTOR_SIZE;
}

impl TryFrom<&[u8; Smart::SIZE]> for Smart {
    type Error = Error;

    fn try_from(data: &[u8; Smart::SIZE]) -> Result<Self, Self::Error> {
        let checksum = data.iter().fold(0u8, |x, &y| x.wrapping_add(y));
        if checksum != 0 {
            return Err(Error::InvalidChecksum(checksum));
        }

        let vendor_specific_1 = std::array::from_fn(|i| data[i]);
        let offline_data_collection_status = data[362].try_into()?;
        let self_test_execution_raw = data[363];
        let self_test_execution_status = (self_test_execution_raw >> 4).try_into()?;
        let self_test_percent_remaining = match self_test_execution_raw & 0xF {
            x if x < 10 => Ok(x * 10),
            x => Err(Error::InvalidSelfTestPercentageRemaining(x)),
        }?;

        let vendor_specific_2 = [data[364], data[365]];
        let vendor_specific_3 = data[366];
        let offline_data_collection_capability = data[367].into();
        let smart_capability = u16::from_le_bytes([data[368], data[369]]).into();
        let error_logging_supported = data[370] & 0b1 != 0;
        let vendor_specific_4 = data[371];
        let short_self_test_poll_time = data[372];

        let extended_self_test_poll_time = match data[373] {
            0xFF => u16::from_le_bytes([data[375], data[376]]),
            x => x.into(),
        };

        let conveyance_self_test_poll_time = data[374];
        let vendor_specific_5 = std::array::from_fn(|i| data[386 + i]);

        Ok(Self {
            vendor_specific_1,
            offline_data_collection_status,
            self_test_execution_status,
            self_test_percent_remaining,
            vendor_specific_2,
            vendor_specific_3,
            offline_data_collection_capability,
            smart_capability,
            error_logging_supported,
            vendor_specific_4,
            short_self_test_poll_time,
            extended_self_test_poll_time,
            conveyance_self_test_poll_time,
            vendor_specific_5,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_smart_data() {
        const DATA_VALID: &[&[u8; Smart::SIZE]] = &[
            test_data::westerndigital_scorpioblack::SMART_DATA,
            test_data::kingston_a400::SMART_DATA,
        ];
        const DATA_INVALID: &[&[u8; Smart::SIZE]] = &[
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
            assert!(Smart::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(Smart::try_from(data).is_err());
        }
    }
}
