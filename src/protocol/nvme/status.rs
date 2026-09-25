//! NVMe completion queue entry status field parsing.

use std::ops::{Range, RangeInclusive};

/// I/O command set or fabrics command specific status code values.
const IO_COMMAND_SET_SPECIFIC_RANGE: Range<u8> = 0x80..0xC0;
/// Vendor specific status code values.
const VENDOR_SPECIFIC_RANGE: RangeInclusive<u8> = 0xC0..=0xFF;

/// Status error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid status code type value.
    InvalidStatusCodeType(u8),
    /// Invalid status code value for its status code type.
    InvalidStatusCode(u8, u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidStatusCodeType(x) => write!(f, "invalid status code type {x:#x}"),
            Self::InvalidStatusCode(x, y) => {
                write!(f, "invalid status code type {x:#x} code {y:#x}")
            },
        }
    }
}

/// Status code for the generic command status code type.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum GenericStatus {
    SuccessfulCompletion = 0x0,
    InvalidCommandOpcode = 0x1,
    InvalidFieldInCommand = 0x2,
    CommandIdConflict = 0x3,
    DataTransferError = 0x4,
    CommandsAbortedDueToPowerLossNotification = 0x5,
    InternalError = 0x6,
    CommandAbortRequested = 0x7,
    CommandAbortedDueToSqDeletion = 0x8,
    CommandAbortedDueToFailedFusedCommand = 0x9,
    CommandAbortedDueToMissingFusedCommand = 0xA,
    InvalidNamespaceOrFormat = 0xB,
    CommandSequenceError = 0xC,
    InvalidSglSegmentDescriptor = 0xD,
    InvalidNumberOfSglDescriptors = 0xE,
    DataSglLengthInvalid = 0xF,
    MetadataSglLengthInvalid = 0x10,
    SglDescriptorTypeInvalid = 0x11,
    InvalidUseOfControllerMemoryBuffer = 0x12,
    PrpOffsetInvalid = 0x13,
    AtomicWriteUnitExceeded = 0x14,
    OperationDenied = 0x15,
    SglOffsetInvalid = 0x16,
    HostIdentifierInconsistentFormat = 0x18,
    KeepAliveTimerExpired = 0x19,
    KeepAliveTimeoutInvalid = 0x1A,
    CommandAbortedDueToPreemptAndAbort = 0x1B,
    SanitizeFailed = 0x1C,
    SanitizeInProgress = 0x1D,
    SglDataBlockGranularityInvalid = 0x1E,
    CommandNotSupportedForQueueInCmb = 0x1F,
    NamespaceIsWriteProtected = 0x20,
    CommandInterrupted = 0x21,
    TransientTransportError = 0x22,
    CommandProhibitedByCommandAndFeatureLockdown = 0x23,
    AdminCommandMediaNotReady = 0x24,
    InvalidKeyTag = 0x25,
    HostDispersedNamespaceSupportNotEnabled = 0x26,
    HostIdentifierNotInitialized = 0x27,
    IncorrectKey = 0x28,
    FdpDisabled = 0x29,
    InvalidPlacementHandleList = 0x2A,
    SanitizeNamespaceFailed = 0x2B,
    SanitizeNamespaceInProgress = 0x2C,
    LbaOutOfRange = 0x80,
    CapacityExceeded = 0x81,
    NamespaceNotReady = 0x82,
    ReservationConflict = 0x83,
    FormatInProgress = 0x84,
    InvalidValueSize = 0x85,
    InvalidKeySize = 0x86,
    KvKeyDoesNotExist = 0x87,
    UnrecoveredError = 0x88,
    KeyExists = 0x89,
    /// I/O Command Set specific status code.
    IoCommandSetSpecific(u8),
    /// Vendor specific status code.
    VendorSpecific(u8),
}

impl From<GenericStatus> for u8 {
    fn from(value: GenericStatus) -> Self {
        match value {
            GenericStatus::IoCommandSetSpecific(x) | GenericStatus::VendorSpecific(x) => x,

            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl TryFrom<u8> for GenericStatus {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[GenericStatus] = &[
            GenericStatus::SuccessfulCompletion,
            GenericStatus::InvalidCommandOpcode,
            GenericStatus::InvalidFieldInCommand,
            GenericStatus::CommandIdConflict,
            GenericStatus::DataTransferError,
            GenericStatus::CommandsAbortedDueToPowerLossNotification,
            GenericStatus::InternalError,
            GenericStatus::CommandAbortRequested,
            GenericStatus::CommandAbortedDueToSqDeletion,
            GenericStatus::CommandAbortedDueToFailedFusedCommand,
            GenericStatus::CommandAbortedDueToMissingFusedCommand,
            GenericStatus::InvalidNamespaceOrFormat,
            GenericStatus::CommandSequenceError,
            GenericStatus::InvalidSglSegmentDescriptor,
            GenericStatus::InvalidNumberOfSglDescriptors,
            GenericStatus::DataSglLengthInvalid,
            GenericStatus::MetadataSglLengthInvalid,
            GenericStatus::SglDescriptorTypeInvalid,
            GenericStatus::InvalidUseOfControllerMemoryBuffer,
            GenericStatus::PrpOffsetInvalid,
            GenericStatus::AtomicWriteUnitExceeded,
            GenericStatus::OperationDenied,
            GenericStatus::SglOffsetInvalid,
            GenericStatus::HostIdentifierInconsistentFormat,
            GenericStatus::KeepAliveTimerExpired,
            GenericStatus::KeepAliveTimeoutInvalid,
            GenericStatus::CommandAbortedDueToPreemptAndAbort,
            GenericStatus::SanitizeFailed,
            GenericStatus::SanitizeInProgress,
            GenericStatus::SglDataBlockGranularityInvalid,
            GenericStatus::CommandNotSupportedForQueueInCmb,
            GenericStatus::NamespaceIsWriteProtected,
            GenericStatus::CommandInterrupted,
            GenericStatus::TransientTransportError,
            GenericStatus::CommandProhibitedByCommandAndFeatureLockdown,
            GenericStatus::AdminCommandMediaNotReady,
            GenericStatus::InvalidKeyTag,
            GenericStatus::HostDispersedNamespaceSupportNotEnabled,
            GenericStatus::HostIdentifierNotInitialized,
            GenericStatus::IncorrectKey,
            GenericStatus::FdpDisabled,
            GenericStatus::InvalidPlacementHandleList,
            GenericStatus::SanitizeNamespaceFailed,
            GenericStatus::SanitizeNamespaceInProgress,
            GenericStatus::LbaOutOfRange,
            GenericStatus::CapacityExceeded,
            GenericStatus::NamespaceNotReady,
            GenericStatus::ReservationConflict,
            GenericStatus::FormatInProgress,
            GenericStatus::InvalidValueSize,
            GenericStatus::InvalidKeySize,
            GenericStatus::KvKeyDoesNotExist,
            GenericStatus::UnrecoveredError,
            GenericStatus::KeyExists,
        ];

        CONST_VARIANTS
            .iter()
            .find(|&&x| u8::from(x) == value)
            .copied()
            .ok_or(value)
            .or_else(|x| {
                if IO_COMMAND_SET_SPECIFIC_RANGE.contains(&x) {
                    return Ok(Self::IoCommandSetSpecific(x));
                }

                VENDOR_SPECIFIC_RANGE
                    .contains(&x)
                    .then_some(Self::VendorSpecific(x))
                    .ok_or(x)
            })
    }
}

impl std::fmt::Display for GenericStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::SuccessfulCompletion => write!(f, "Successful Completion"),
            Self::InvalidCommandOpcode => write!(f, "Invalid Command Opcode"),
            Self::InvalidFieldInCommand => write!(f, "Invalid Field in Command"),
            Self::CommandIdConflict => write!(f, "Command ID Conflict"),
            Self::DataTransferError => write!(f, "Data Transfer Error"),
            Self::CommandsAbortedDueToPowerLossNotification => {
                write!(f, "Commands Aborted due to Power Loss Notification")
            },
            Self::InternalError => write!(f, "Internal Error"),
            Self::CommandAbortRequested => write!(f, "Command Abort Requested"),
            Self::CommandAbortedDueToSqDeletion => {
                write!(f, "Command Aborted due to SQ Deletion")
            },
            Self::CommandAbortedDueToFailedFusedCommand => {
                write!(f, "Command Aborted due to Failed Fused Command")
            },
            Self::CommandAbortedDueToMissingFusedCommand => {
                write!(f, "Command Aborted due to Missing Fused Command")
            },
            Self::InvalidNamespaceOrFormat => write!(f, "Invalid Namespace or Format"),
            Self::CommandSequenceError => write!(f, "Command Sequence Error"),
            Self::InvalidSglSegmentDescriptor => write!(f, "Invalid SGL Segment Descriptor"),
            Self::InvalidNumberOfSglDescriptors => {
                write!(f, "Invalid Number of SGL Descriptors")
            },
            Self::DataSglLengthInvalid => write!(f, "Data SGL Length Invalid"),
            Self::MetadataSglLengthInvalid => write!(f, "Metadata SGL Length Invalid"),
            Self::SglDescriptorTypeInvalid => write!(f, "SGL Descriptor Type Invalid"),
            Self::InvalidUseOfControllerMemoryBuffer => {
                write!(f, "Invalid Use of Controller Memory Buffer")
            },
            Self::PrpOffsetInvalid => write!(f, "PRP Offset Invalid"),
            Self::AtomicWriteUnitExceeded => write!(f, "Atomic Write Unit Exceeded"),
            Self::OperationDenied => write!(f, "Operation Denied"),
            Self::SglOffsetInvalid => write!(f, "SGL Offset Invalid"),
            Self::HostIdentifierInconsistentFormat => {
                write!(f, "Host Identifier Inconsistent Format")
            },
            Self::KeepAliveTimerExpired => write!(f, "Keep Alive Timer Expired"),
            Self::KeepAliveTimeoutInvalid => write!(f, "Keep Alive Timeout Invalid"),
            Self::CommandAbortedDueToPreemptAndAbort => {
                write!(f, "Command Aborted due to Preempt and Abort")
            },
            Self::SanitizeFailed => write!(f, "Sanitize Failed"),
            Self::SanitizeInProgress => write!(f, "Sanitize In Progress"),
            Self::SglDataBlockGranularityInvalid => {
                write!(f, "SGL Data Block Granularity Invalid")
            },
            Self::CommandNotSupportedForQueueInCmb => {
                write!(f, "Command Not Supported for Queue in CMB")
            },
            Self::NamespaceIsWriteProtected => write!(f, "Namespace is Write Protected"),
            Self::CommandInterrupted => write!(f, "Command Interrupted"),
            Self::TransientTransportError => write!(f, "Transient Transport Error"),
            Self::CommandProhibitedByCommandAndFeatureLockdown => {
                write!(f, "Command Prohibited by Command and Feature Lockdown")
            },
            Self::AdminCommandMediaNotReady => write!(f, "Admin Command Media Not Ready"),
            Self::InvalidKeyTag => write!(f, "Invalid Key Tag"),
            Self::HostDispersedNamespaceSupportNotEnabled => {
                write!(f, "Host Dispersed Namespace Support Not Enabled")
            },
            Self::HostIdentifierNotInitialized => write!(f, "Host Identifier Not Initialized"),
            Self::IncorrectKey => write!(f, "Incorrect Key"),
            Self::FdpDisabled => write!(f, "FDP Disabled"),
            Self::InvalidPlacementHandleList => write!(f, "Invalid Placement Handle List"),
            Self::SanitizeNamespaceFailed => write!(f, "Sanitize Namespace Failed"),
            Self::SanitizeNamespaceInProgress => write!(f, "Sanitize Namespace In Progress"),
            Self::LbaOutOfRange => write!(f, "LBA Out of Range"),
            Self::CapacityExceeded => write!(f, "Capacity Exceeded"),
            Self::NamespaceNotReady => write!(f, "Namespace Not Ready"),
            Self::ReservationConflict => write!(f, "Reservation Conflict"),
            Self::FormatInProgress => write!(f, "Format In Progress"),
            Self::InvalidValueSize => write!(f, "Invalid Value Size"),
            Self::InvalidKeySize => write!(f, "Invalid Key Size"),
            Self::KvKeyDoesNotExist => write!(f, "KV Key Does Not Exist"),
            Self::UnrecoveredError => write!(f, "Unrecovered Error"),
            Self::KeyExists => write!(f, "Key Exists"),
            Self::IoCommandSetSpecific(x) => write!(f, "I/O command set specific {x:#x}"),
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}

/// Status code for the command specific status code type.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CommandSpecificStatus {
    CompletionQueueInvalid = 0x0,
    InvalidQueueIdentifier = 0x1,
    InvalidQueueSize = 0x2,
    AbortCommandLimitExceeded = 0x3,
    AsynchronousEventRequestLimitExceeded = 0x5,
    InvalidFirmwareSlot = 0x6,
    InvalidFirmwareImage = 0x7,
    InvalidInterruptVector = 0x8,
    InvalidLogPage = 0x9,
    InvalidFormat = 0xA,
    FirmwareActivationRequiresConventionalReset = 0xB,
    InvalidQueueDeletion = 0xC,
    FeatureIdentifierNotSaveable = 0xD,
    FeatureNotChangeable = 0xE,
    FeatureNotNamespaceSpecific = 0xF,
    FirmwareActivationRequiresNvmSubsystemReset = 0x10,
    FirmwareActivationRequiresControllerLevelReset = 0x11,
    FirmwareActivationRequiresMaximumTimeViolation = 0x12,
    FirmwareActivationProhibited = 0x13,
    OverlappingRange = 0x14,
    NamespaceInsufficientCapacity = 0x15,
    NamespaceIdentifierUnavailable = 0x16,
    NamespaceAlreadyAttached = 0x18,
    NamespaceIsPrivate = 0x19,
    NamespaceNotAttached = 0x1A,
    ThinProvisioningNotSupported = 0x1B,
    ControllerListInvalid = 0x1C,
    DeviceSelfTestInProgress = 0x1D,
    BootPartitionWriteProhibited = 0x1E,
    InvalidControllerIdentifier = 0x1F,
    InvalidSecondaryControllerState = 0x20,
    InvalidNumberOfControllerResources = 0x21,
    InvalidResourceIdentifier = 0x22,
    SanitizeProhibitedWhilePersistentMemoryRegionIsEnabled = 0x23,
    AnaGroupIdentifierInvalid = 0x24,
    AnaAttachFailed = 0x25,
    InsufficientCapacity = 0x26,
    NamespaceAttachmentLimitExceeded = 0x27,
    ProhibitionOfCommandExecutionNotSupported = 0x28,
    IoCommandSetNotSupported = 0x29,
    IoCommandSetNotEnabled = 0x2A,
    IoCommandSetCombinationRejected = 0x2B,
    InvalidIoCommandSet = 0x2C,
    IdentifierUnavailable = 0x2D,
    NamespaceIsDispersed = 0x2E,
    InvalidDiscoveryInformation = 0x2F,
    ZoningDataStructureLocked = 0x30,
    ZoningDataStructureNotFound = 0x31,
    InsufficientDiscoveryResources = 0x32,
    RequestedFunctionDisabled = 0x33,
    ZoneGroupOriginatorInvalid = 0x34,
    InvalidHost = 0x35,
    InvalidNvmSubsystem = 0x36,
    InvalidControllerDataQueue = 0x37,
    NotEnoughResources = 0x38,
    ControllerSuspended = 0x39,
    ControllerNotSuspended = 0x3A,
    ControllerDataQueueFull = 0x3B,
    RequestExceedsMaximumNamespaceSanitizeOperationsInProgress = 0x3C,
    ManufacturingDefaultPersonalityRequired = 0x3D,
    InvalidPowerLimit = 0x3E,
    CrossControllerResetInProgress = 0x3F,
    CrossControllerResetLogPageFull = 0x40,
    CrossControllerResetLimitExceeded = 0x41,
    /// Directive specific status code.
    DirectiveSpecific(u8),
    /// I/O Command Set or Fabrics command specific status code.
    IoCommandSetSpecific(u8),
    /// Vendor specific status code.
    VendorSpecific(u8),
}

impl From<CommandSpecificStatus> for u8 {
    fn from(value: CommandSpecificStatus) -> Self {
        match value {
            CommandSpecificStatus::DirectiveSpecific(x)
            | CommandSpecificStatus::IoCommandSetSpecific(x)
            | CommandSpecificStatus::VendorSpecific(x) => x,

            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl TryFrom<u8> for CommandSpecificStatus {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[CommandSpecificStatus] = &[
            CommandSpecificStatus::CompletionQueueInvalid,
            CommandSpecificStatus::InvalidQueueIdentifier,
            CommandSpecificStatus::InvalidQueueSize,
            CommandSpecificStatus::AbortCommandLimitExceeded,
            CommandSpecificStatus::AsynchronousEventRequestLimitExceeded,
            CommandSpecificStatus::InvalidFirmwareSlot,
            CommandSpecificStatus::InvalidFirmwareImage,
            CommandSpecificStatus::InvalidInterruptVector,
            CommandSpecificStatus::InvalidLogPage,
            CommandSpecificStatus::InvalidFormat,
            CommandSpecificStatus::FirmwareActivationRequiresConventionalReset,
            CommandSpecificStatus::InvalidQueueDeletion,
            CommandSpecificStatus::FeatureIdentifierNotSaveable,
            CommandSpecificStatus::FeatureNotChangeable,
            CommandSpecificStatus::FeatureNotNamespaceSpecific,
            CommandSpecificStatus::FirmwareActivationRequiresNvmSubsystemReset,
            CommandSpecificStatus::FirmwareActivationRequiresControllerLevelReset,
            CommandSpecificStatus::FirmwareActivationRequiresMaximumTimeViolation,
            CommandSpecificStatus::FirmwareActivationProhibited,
            CommandSpecificStatus::OverlappingRange,
            CommandSpecificStatus::NamespaceInsufficientCapacity,
            CommandSpecificStatus::NamespaceIdentifierUnavailable,
            CommandSpecificStatus::NamespaceAlreadyAttached,
            CommandSpecificStatus::NamespaceIsPrivate,
            CommandSpecificStatus::NamespaceNotAttached,
            CommandSpecificStatus::ThinProvisioningNotSupported,
            CommandSpecificStatus::ControllerListInvalid,
            CommandSpecificStatus::DeviceSelfTestInProgress,
            CommandSpecificStatus::BootPartitionWriteProhibited,
            CommandSpecificStatus::InvalidControllerIdentifier,
            CommandSpecificStatus::InvalidSecondaryControllerState,
            CommandSpecificStatus::InvalidNumberOfControllerResources,
            CommandSpecificStatus::InvalidResourceIdentifier,
            CommandSpecificStatus::SanitizeProhibitedWhilePersistentMemoryRegionIsEnabled,
            CommandSpecificStatus::AnaGroupIdentifierInvalid,
            CommandSpecificStatus::AnaAttachFailed,
            CommandSpecificStatus::InsufficientCapacity,
            CommandSpecificStatus::NamespaceAttachmentLimitExceeded,
            CommandSpecificStatus::ProhibitionOfCommandExecutionNotSupported,
            CommandSpecificStatus::IoCommandSetNotSupported,
            CommandSpecificStatus::IoCommandSetNotEnabled,
            CommandSpecificStatus::IoCommandSetCombinationRejected,
            CommandSpecificStatus::InvalidIoCommandSet,
            CommandSpecificStatus::IdentifierUnavailable,
            CommandSpecificStatus::NamespaceIsDispersed,
            CommandSpecificStatus::InvalidDiscoveryInformation,
            CommandSpecificStatus::ZoningDataStructureLocked,
            CommandSpecificStatus::ZoningDataStructureNotFound,
            CommandSpecificStatus::InsufficientDiscoveryResources,
            CommandSpecificStatus::RequestedFunctionDisabled,
            CommandSpecificStatus::ZoneGroupOriginatorInvalid,
            CommandSpecificStatus::InvalidHost,
            CommandSpecificStatus::InvalidNvmSubsystem,
            CommandSpecificStatus::InvalidControllerDataQueue,
            CommandSpecificStatus::NotEnoughResources,
            CommandSpecificStatus::ControllerSuspended,
            CommandSpecificStatus::ControllerNotSuspended,
            CommandSpecificStatus::ControllerDataQueueFull,
            CommandSpecificStatus::RequestExceedsMaximumNamespaceSanitizeOperationsInProgress,
            CommandSpecificStatus::ManufacturingDefaultPersonalityRequired,
            CommandSpecificStatus::InvalidPowerLimit,
            CommandSpecificStatus::CrossControllerResetInProgress,
            CommandSpecificStatus::CrossControllerResetLogPageFull,
            CommandSpecificStatus::CrossControllerResetLimitExceeded,
        ];
        const DIRECTIVE_SPECIFIC_RANGE: Range<u8> = 0x70..0x80;

        if DIRECTIVE_SPECIFIC_RANGE.contains(&value) {
            return Ok(Self::DirectiveSpecific(value));
        }

        if IO_COMMAND_SET_SPECIFIC_RANGE.contains(&value) {
            return Ok(Self::IoCommandSetSpecific(value));
        }

        CONST_VARIANTS
            .iter()
            .find(|&&x| u8::from(x) == value)
            .copied()
            .ok_or(value)
            .or_else(|x| {
                VENDOR_SPECIFIC_RANGE
                    .contains(&x)
                    .then_some(Self::VendorSpecific(x))
                    .ok_or(x)
            })
    }
}

impl std::fmt::Display for CommandSpecificStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::CompletionQueueInvalid => write!(f, "Completion Queue Invalid"),
            Self::InvalidQueueIdentifier => write!(f, "Invalid Queue Identifier"),
            Self::InvalidQueueSize => write!(f, "Invalid Queue Size"),
            Self::AbortCommandLimitExceeded => write!(f, "Abort Command Limit Exceeded"),
            Self::AsynchronousEventRequestLimitExceeded => {
                write!(f, "Asynchronous Event Request Limit Exceeded")
            },
            Self::InvalidFirmwareSlot => write!(f, "Invalid Firmware Slot"),
            Self::InvalidFirmwareImage => write!(f, "Invalid Firmware Image"),
            Self::InvalidInterruptVector => write!(f, "Invalid Interrupt Vector"),
            Self::InvalidLogPage => write!(f, "Invalid Log Page"),
            Self::InvalidFormat => write!(f, "Invalid Format"),
            Self::FirmwareActivationRequiresConventionalReset => {
                write!(f, "Firmware Activation Requires Conventional Reset")
            },
            Self::InvalidQueueDeletion => write!(f, "Invalid Queue Deletion"),
            Self::FeatureIdentifierNotSaveable => write!(f, "Feature Identifier Not Saveable"),
            Self::FeatureNotChangeable => write!(f, "Feature Not Changeable"),
            Self::FeatureNotNamespaceSpecific => write!(f, "Feature Not Namespace Specific"),
            Self::FirmwareActivationRequiresNvmSubsystemReset => {
                write!(f, "Firmware Activation Requires NVM Subsystem Reset")
            },
            Self::FirmwareActivationRequiresControllerLevelReset => {
                write!(f, "Firmware Activation Requires Controller Level Reset")
            },
            Self::FirmwareActivationRequiresMaximumTimeViolation => {
                write!(f, "Firmware Activation Requires Maximum Time Violation")
            },
            Self::FirmwareActivationProhibited => write!(f, "Firmware Activation Prohibited"),
            Self::OverlappingRange => write!(f, "Overlapping Range"),
            Self::NamespaceInsufficientCapacity => {
                write!(f, "Namespace Insufficient Capacity")
            },
            Self::NamespaceIdentifierUnavailable => {
                write!(f, "Namespace Identifier Unavailable")
            },
            Self::NamespaceAlreadyAttached => write!(f, "Namespace Already Attached"),
            Self::NamespaceIsPrivate => write!(f, "Namespace Is Private"),
            Self::NamespaceNotAttached => write!(f, "Namespace Not Attached"),
            Self::ThinProvisioningNotSupported => write!(f, "Thin Provisioning Not Supported"),
            Self::ControllerListInvalid => write!(f, "Controller List Invalid"),
            Self::DeviceSelfTestInProgress => write!(f, "Device Self-test In Progress"),
            Self::BootPartitionWriteProhibited => write!(f, "Boot Partition Write Prohibited"),
            Self::InvalidControllerIdentifier => write!(f, "Invalid Controller Identifier"),
            Self::InvalidSecondaryControllerState => {
                write!(f, "Invalid Secondary Controller State")
            },
            Self::InvalidNumberOfControllerResources => {
                write!(f, "Invalid Number of Controller Resources")
            },
            Self::InvalidResourceIdentifier => write!(f, "Invalid Resource Identifier"),
            Self::SanitizeProhibitedWhilePersistentMemoryRegionIsEnabled => {
                write!(
                    f,
                    "Sanitize Prohibited While Persistent Memory Region is Enabled"
                )
            },
            Self::AnaGroupIdentifierInvalid => write!(f, "ANA Group Identifier Invalid"),
            Self::AnaAttachFailed => write!(f, "ANA Attach Failed"),
            Self::InsufficientCapacity => write!(f, "Insufficient Capacity"),
            Self::NamespaceAttachmentLimitExceeded => {
                write!(f, "Namespace Attachment Limit Exceeded")
            },
            Self::ProhibitionOfCommandExecutionNotSupported => {
                write!(f, "Prohibition of Command Execution Not Supported")
            },
            Self::IoCommandSetNotSupported => write!(f, "I/O Command Set Not Supported"),
            Self::IoCommandSetNotEnabled => write!(f, "I/O Command Set Not Enabled"),
            Self::IoCommandSetCombinationRejected => {
                write!(f, "I/O Command Set Combination Rejected")
            },
            Self::InvalidIoCommandSet => write!(f, "Invalid I/O Command Set"),
            Self::IdentifierUnavailable => write!(f, "Identifier Unavailable"),
            Self::NamespaceIsDispersed => write!(f, "Namespace Is Dispersed"),
            Self::InvalidDiscoveryInformation => write!(f, "Invalid Discovery Information"),
            Self::ZoningDataStructureLocked => write!(f, "Zoning Data Structure Locked"),
            Self::ZoningDataStructureNotFound => write!(f, "Zoning Data Structure Not Found"),
            Self::InsufficientDiscoveryResources => {
                write!(f, "Insufficient Discovery Resources")
            },
            Self::RequestedFunctionDisabled => write!(f, "Requested Function Disabled"),
            Self::ZoneGroupOriginatorInvalid => write!(f, "ZoneGroup Originator Invalid"),
            Self::InvalidHost => write!(f, "Invalid Host"),
            Self::InvalidNvmSubsystem => write!(f, "Invalid NVM Subsystem"),
            Self::InvalidControllerDataQueue => write!(f, "Invalid Controller Data Queue"),
            Self::NotEnoughResources => write!(f, "Not Enough Resources"),
            Self::ControllerSuspended => write!(f, "Controller Suspended"),
            Self::ControllerNotSuspended => write!(f, "Controller Not Suspended"),
            Self::ControllerDataQueueFull => write!(f, "Controller Data Queue Full"),
            Self::RequestExceedsMaximumNamespaceSanitizeOperationsInProgress => {
                write!(
                    f,
                    "Request Exceeds Maximum Namespace Sanitize Operations In Progress"
                )
            },
            Self::ManufacturingDefaultPersonalityRequired => {
                write!(f, "Manufacturing Default Personality Required")
            },
            Self::InvalidPowerLimit => write!(f, "Invalid Power Limit"),
            Self::CrossControllerResetInProgress => {
                write!(f, "Cross-Controller Reset in Progress")
            },
            Self::CrossControllerResetLogPageFull => {
                write!(f, "Cross-Controller Reset Log Page Full")
            },
            Self::CrossControllerResetLimitExceeded => {
                write!(f, "Cross-Controller Reset Limit Exceeded")
            },
            Self::DirectiveSpecific(x) => write!(f, "directive specific {x:#x}"),
            Self::IoCommandSetSpecific(x) => write!(f, "I/O command set specific {x:#x}"),
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}

/// Status code for the media and data integrity error status code type.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum MediaDataIntegrityStatus {
    WriteFault = 0x80,
    UnrecoveredReadError = 0x81,
    EndToEndGuardCheckError = 0x82,
    EndToEndApplicationTagCheckError = 0x83,
    EndToEndReferenceTagCheckError = 0x84,
    CompareFailure = 0x85,
    AccessDenied = 0x86,
    DeallocatedOrUnwrittenLogicalBlock = 0x87,
    EndToEndStorageTagCheckError = 0x88,
    /// Vendor specific status code.
    VendorSpecific(u8),
}

impl From<MediaDataIntegrityStatus> for u8 {
    fn from(value: MediaDataIntegrityStatus) -> Self {
        match value {
            MediaDataIntegrityStatus::VendorSpecific(x) => x,

            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl TryFrom<u8> for MediaDataIntegrityStatus {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[MediaDataIntegrityStatus] = &[
            MediaDataIntegrityStatus::WriteFault,
            MediaDataIntegrityStatus::UnrecoveredReadError,
            MediaDataIntegrityStatus::EndToEndGuardCheckError,
            MediaDataIntegrityStatus::EndToEndApplicationTagCheckError,
            MediaDataIntegrityStatus::EndToEndReferenceTagCheckError,
            MediaDataIntegrityStatus::CompareFailure,
            MediaDataIntegrityStatus::AccessDenied,
            MediaDataIntegrityStatus::DeallocatedOrUnwrittenLogicalBlock,
            MediaDataIntegrityStatus::EndToEndStorageTagCheckError,
        ];

        CONST_VARIANTS
            .iter()
            .find(|&&x| u8::from(x) == value)
            .copied()
            .ok_or(value)
            .or_else(|x| {
                VENDOR_SPECIFIC_RANGE
                    .contains(&x)
                    .then_some(Self::VendorSpecific(x))
                    .ok_or(x)
            })
    }
}

impl std::fmt::Display for MediaDataIntegrityStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::WriteFault => write!(f, "Write Fault"),
            Self::UnrecoveredReadError => write!(f, "Unrecovered Read Error"),
            Self::EndToEndGuardCheckError => write!(f, "End-to-end Guard Check Error"),
            Self::EndToEndApplicationTagCheckError => {
                write!(f, "End-to-end Application Tag Check Error")
            },
            Self::EndToEndReferenceTagCheckError => {
                write!(f, "End-to-end Reference Tag Check Error")
            },
            Self::CompareFailure => write!(f, "Compare Failure"),
            Self::AccessDenied => write!(f, "Access Denied"),
            Self::DeallocatedOrUnwrittenLogicalBlock => {
                write!(f, "Deallocated or Unwritten Logical Block")
            },
            Self::EndToEndStorageTagCheckError => {
                write!(f, "End-to-End Storage Tag Check Error")
            },
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}

/// Status code for the path related status code type.
#[allow(clippy::missing_docs_in_private_items)]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PathRelatedStatus {
    InternalPathError = 0x0,
    AsymmetricAccessPersistentLoss = 0x1,
    AsymmetricAccessInaccessible = 0x2,
    AsymmetricAccessTransition = 0x3,
    ControllerPathingError = 0x60,
    HostPathingError = 0x70,
    CommandAbortedByHost = 0x71,
    /// I/O Command Set specific status code.
    IoCommandSetSpecific(u8),
    /// Vendor specific status code.
    VendorSpecific(u8),
}

impl From<PathRelatedStatus> for u8 {
    fn from(value: PathRelatedStatus) -> Self {
        match value {
            PathRelatedStatus::IoCommandSetSpecific(x) | PathRelatedStatus::VendorSpecific(x) => x,

            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl TryFrom<u8> for PathRelatedStatus {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[PathRelatedStatus] = &[
            PathRelatedStatus::InternalPathError,
            PathRelatedStatus::AsymmetricAccessPersistentLoss,
            PathRelatedStatus::AsymmetricAccessInaccessible,
            PathRelatedStatus::AsymmetricAccessTransition,
            PathRelatedStatus::ControllerPathingError,
            PathRelatedStatus::HostPathingError,
            PathRelatedStatus::CommandAbortedByHost,
        ];

        CONST_VARIANTS
            .iter()
            .find(|&&x| u8::from(x) == value)
            .copied()
            .ok_or(value)
            .or_else(|x| {
                if IO_COMMAND_SET_SPECIFIC_RANGE.contains(&x) {
                    return Ok(Self::IoCommandSetSpecific(x));
                }

                VENDOR_SPECIFIC_RANGE
                    .contains(&x)
                    .then_some(Self::VendorSpecific(x))
                    .ok_or(x)
            })
    }
}

impl std::fmt::Display for PathRelatedStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InternalPathError => write!(f, "Internal Path Error"),
            Self::AsymmetricAccessPersistentLoss => {
                write!(f, "Asymmetric Access Persistent Loss")
            },
            Self::AsymmetricAccessInaccessible => write!(f, "Asymmetric Access Inaccessible"),
            Self::AsymmetricAccessTransition => write!(f, "Asymmetric Access Transition"),
            Self::ControllerPathingError => write!(f, "Controller Pathing Error"),
            Self::HostPathingError => write!(f, "Host Pathing Error"),
            Self::CommandAbortedByHost => write!(f, "Command Aborted By Host"),
            Self::IoCommandSetSpecific(x) => write!(f, "I/O command set specific {x:#x}"),
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}

/// Status Code (SC) field, carrying its status code type.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StatusCode {
    /// Status generic across all command types.
    Generic(GenericStatus),
    /// Status specific to the command opcode.
    CommandSpecific(CommandSpecificStatus),
    /// Error associated with the NVM media or data integrity.
    MediaDataIntegrity(MediaDataIntegrityStatus),
    /// Status associated with the path to the controller or namespace.
    PathRelated(PathRelatedStatus),
    /// Status code of a vendor specific status code type.
    VendorSpecific(u8),
}

impl StatusCode {
    /// Parse status code for its status code type.
    pub(crate) fn parse(status_code_type: u8, status_code: u8) -> Result<Self, Error> {
        let invalid = |x| Error::InvalidStatusCode(status_code_type, x);

        Ok(match status_code_type {
            0x0 => Self::Generic(status_code.try_into().map_err(invalid)?),
            0x1 => Self::CommandSpecific(status_code.try_into().map_err(invalid)?),
            0x2 => Self::MediaDataIntegrity(status_code.try_into().map_err(invalid)?),
            0x3 => Self::PathRelated(status_code.try_into().map_err(invalid)?),
            0x7 => Self::VendorSpecific(status_code),
            x => return Err(Error::InvalidStatusCodeType(x)),
        })
    }
}

impl From<StatusCode> for u8 {
    fn from(value: StatusCode) -> Self {
        match value {
            StatusCode::Generic(x) => x.into(),
            StatusCode::CommandSpecific(x) => x.into(),
            StatusCode::MediaDataIntegrity(x) => x.into(),
            StatusCode::PathRelated(x) => x.into(),
            StatusCode::VendorSpecific(x) => x,
        }
    }
}

impl std::fmt::Display for StatusCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Generic(x) => write!(f, "generic command status {x}"),
            Self::CommandSpecific(x) => write!(f, "command specific status {x}"),
            Self::MediaDataIntegrity(x) => write!(f, "media and data integrity error {x}"),
            Self::PathRelated(x) => write!(f, "path related status {x}"),
            Self::VendorSpecific(x) => write!(f, "vendor specific status {x:#x}"),
        }
    }
}

/// Completion queue entry status field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct StatusField {
    /// Status Code (SC) and Status Code Type (SCT) fields.
    pub status_code: StatusCode,
    /// Command Retry Delay field, selecting a controller retry delay time.
    pub command_retry_delay: u8,
    /// More bit set, indicating additional status is available.
    pub more: bool,
    /// Do-not-retry bit set.
    pub do_not_retry: bool,
}

impl StatusField {
    /// Status represents an error.
    pub(crate) fn is_error(self) -> bool {
        !matches!(
            self.status_code,
            StatusCode::Generic(GenericStatus::SuccessfulCompletion)
        )
    }
}

impl TryFrom<u16> for StatusField {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        const COMMAND_RETRY_DELAY_MASK: u16 = 0b11 << 11;
        const DO_NOT_RETRY_MASK: u16 = 1 << 14;
        const MORE_MASK: u16 = 1 << 13;
        const STATUS_CODE_TYPE_MASK: u8 = 0b111;

        let [status_code, upper] = value.to_le_bytes();
        let command_retry_delay =
            (value & COMMAND_RETRY_DELAY_MASK) >> COMMAND_RETRY_DELAY_MASK.trailing_zeros();

        Ok(Self {
            status_code: StatusCode::parse(upper & STATUS_CODE_TYPE_MASK, status_code)?,
            command_retry_delay: u8::try_from(command_retry_delay).unwrap(),
            more: (value & MORE_MASK) != 0,
            do_not_retry: (value & DO_NOT_RETRY_MASK) != 0,
        })
    }
}

impl std::fmt::Display for StatusField {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (DNR: {}, more: {})",
            self.status_code, self.do_not_retry, self.more
        )
    }
}
