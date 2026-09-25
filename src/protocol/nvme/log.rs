//! Get log page functionality.

pub mod command_effects;

/// Log error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Commands supported and effects log error.
    CommandEffects(command_effects::Error),
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Self::CommandEffects(x) => Some(x),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CommandEffects(_) => write!(f, "commands supported and effects log error"),
        }
    }
}

impl From<command_effects::Error> for Error {
    fn from(value: command_effects::Error) -> Self {
        Self::CommandEffects(value)
    }
}

/// Log page identifiers.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum LogPage {
    /// Log pages supported by the controller.
    SupportedLogPages = 0x0,
    /// Error information for recent failures.
    ErrorInformation = 0x1,
    /// SMART health information of the controller or a namespace.
    HealthInformation = 0x2,
    /// Firmware revision of each slot.
    FirmwareSlotInformation = 0x3,
    /// Namespaces with changed attributes.
    ChangedNamespaceList = 0x4,
    /// Commands supported by the controller and their effects.
    CommandEffects = 0x5,
    /// Results of the most recent device self-test operations.
    DeviceSelfTest = 0x6,
    /// Vendor specific internal state captured on host request.
    TelemetryHostInitiated = 0x7,
    /// Vendor specific internal state captured by the controller.
    TelemetryControllerInitiated = 0x8,
    /// Endurance group information.
    EnduranceGroupInformation = 0x9,
    /// Predictable latency of an NVM set.
    PredictableLatencyNvmSet = 0xA,
    /// NVM sets with a predictable latency event.
    PredictableLatencyEventAggregate = 0xB,
    /// Asymmetric namespace access group states.
    AsymmetricNamespaceAccess = 0xC,
    /// Events persisted across power cycles.
    PersistentEvent = 0xD,
    /// LBA ranges with a potential data loss.
    LbaStatusInformation = 0xE,
    /// Endurance groups with an aggregate event.
    EnduranceGroupEventAggregate = 0xF,
    /// Media unit status of an endurance group.
    MediaUnitStatus = 0x10,
    /// Capacity configurations supported by the NVM subsystem.
    SupportedCapacityConfigurationList = 0x11,
    /// Features supported by the controller and their effects.
    FeatureIdentifiersEffects = 0x12,
    /// NVMe-MI commands supported by the controller and their effects.
    NvmeMiCommandsEffects = 0x13,
    /// Commands and features prohibited by lockdown.
    CommandFeatureLockdown = 0x14,
    /// Boot partition contents.
    BootPartition = 0x15,
    /// Rotational media information of an endurance group.
    RotationalMediaInformation = 0x16,
    /// Discovery information of a Discovery controller.
    Discovery = 0x70,
    /// Reservation notifications.
    ReservationNotification = 0x80,
    /// Progress of the most recent sanitize operation.
    SanitizeStatus = 0x81,
    /// Zones with a changed state.
    ChangedZoneList = 0xBF,
    /// Vendor specific.
    VendorSpecific(u8),
}

impl From<LogPage> for u8 {
    fn from(value: LogPage) -> Self {
        match value {
            LogPage::VendorSpecific(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl std::fmt::Display for LogPage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SupportedLogPages => write!(f, "Supported Log Pages"),
            Self::ErrorInformation => write!(f, "Error Information"),
            Self::HealthInformation => write!(f, "SMART / Health Information"),
            Self::FirmwareSlotInformation => write!(f, "Firmware Slot Information"),
            Self::ChangedNamespaceList => write!(f, "Changed Namespace List"),
            Self::CommandEffects => write!(f, "Commands Supported and Effects"),
            Self::DeviceSelfTest => write!(f, "Device Self-test"),
            Self::TelemetryHostInitiated => write!(f, "Telemetry Host-Initiated"),
            Self::TelemetryControllerInitiated => write!(f, "Telemetry Controller-Initiated"),
            Self::EnduranceGroupInformation => write!(f, "Endurance Group Information"),
            Self::PredictableLatencyNvmSet => write!(f, "Predictable Latency Per NVM Set"),
            Self::PredictableLatencyEventAggregate => {
                write!(f, "Predictable Latency Event Aggregate")
            },
            Self::AsymmetricNamespaceAccess => write!(f, "Asymmetric Namespace Access"),
            Self::PersistentEvent => write!(f, "Persistent Event Log"),
            Self::LbaStatusInformation => write!(f, "LBA Status Information"),
            Self::EnduranceGroupEventAggregate => write!(f, "Endurance Group Event Aggregate"),
            Self::MediaUnitStatus => write!(f, "Media Unit Status"),
            Self::SupportedCapacityConfigurationList => {
                write!(f, "Supported Capacity Configuration List")
            },
            Self::FeatureIdentifiersEffects => {
                write!(f, "Feature Identifiers Supported and Effects")
            },
            Self::NvmeMiCommandsEffects => write!(f, "NVMe-MI Commands Supported and Effects"),
            Self::CommandFeatureLockdown => write!(f, "Command and Feature Lockdown"),
            Self::BootPartition => write!(f, "Boot Partition"),
            Self::RotationalMediaInformation => write!(f, "Rotational Media Information"),
            Self::Discovery => write!(f, "Discovery"),
            Self::ReservationNotification => write!(f, "Reservation Notification"),
            Self::SanitizeStatus => write!(f, "Sanitize Status"),
            Self::ChangedZoneList => write!(f, "Changed Zone List"),
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}
