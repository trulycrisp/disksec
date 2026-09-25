//! NVMe commands.

use std::ops::RangeInclusive;

use super::status;

/// Admin command opcodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum AdminOpcode {
    /// Delete an I/O submission queue.
    DeleteIoSubmissionQueue = 0x0,
    /// Create an I/O submission queue.
    CreateIoSubmissionQueue = 0x1,
    /// Read a controller log page.
    GetLogPage = 0x2,
    /// Delete an I/O completion queue.
    DeleteIoCompletionQueue = 0x4,
    /// Create an I/O completion queue.
    CreateIoCompletionQueue = 0x5,
    /// Get controller or namespace information.
    Identify = 0x6,
    /// Abort a previously submitted command.
    Abort = 0x8,
    /// Configure a feature.
    SetFeatures = 0x9,
    /// Report the current configuration of a feature.
    GetFeatures = 0xA,
    /// Queue a request for an asynchronous event notification.
    AsynchronousEventRequest = 0xC,
    /// Create or delete a namespace.
    NamespaceManagement = 0xD,
    /// Activate a downloaded firmware image.
    FirmwareCommit = 0x10,
    /// Transfer a firmware image to the controller.
    FirmwareImageDownload = 0x11,
    /// Run the controller built-in self-test.
    DeviceSelfTest = 0x14,
    /// Attach or detach a namespace from controllers.
    NamespaceAttachment = 0x15,
    /// Maintain the keep alive timer.
    KeepAlive = 0x18,
    /// Send a directive to the controller.
    DirectiveSend = 0x19,
    /// Retrieve a directive from the controller.
    DirectiveReceive = 0x1A,
    /// Manage SR-IOV virtual functions and their resources.
    VirtualizationManagement = 0x1C,
    /// Send an NVMe Management Interface command.
    NvmeMiSend = 0x1D,
    /// Receive an NVMe Management Interface response.
    NvmeMiReceive = 0x1E,
    /// Configure endurance group capacity.
    CapacityManagement = 0x20,
    /// Manage discovery information on a Discovery controller.
    DiscoveryInformationManagement = 0x21,
    /// Retrieve fabric zoning information.
    FabricZoningReceive = 0x22,
    /// Prohibit commands and features from being used.
    Lockdown = 0x24,
    /// Look up a fabric zoning entry.
    FabricZoningLookup = 0x25,
    /// Clear an exported NVM resource configuration.
    ClearExportedNvmResourceConfiguration = 0x28,
    /// Send fabric zoning information.
    FabricZoningSend = 0x29,
    /// Create an exported NVM subsystem.
    CreateExportedNvmSubsystem = 0x2A,
    /// Manage an exported NVM subsystem.
    ManageExportedNvmSubsystem = 0x2D,
    /// Manage an exported namespace.
    ManageExportedNamespace = 0x31,
    /// Manage an exported port.
    ManageExportedPort = 0x35,
    /// Reset another controller in the NVM subsystem.
    CrossControllerReset = 0x38,
    /// Send a discovery log page to a Discovery controller.
    SendDiscoveryLogPage = 0x39,
    /// Send user data tracking information.
    TrackSend = 0x3D,
    /// Receive user data tracking information.
    TrackReceive = 0x3E,
    /// Send controller state for live migration.
    MigrationSend = 0x41,
    /// Receive controller state for live migration.
    MigrationReceive = 0x42,
    /// Manage a controller data queue.
    ControllerDataQueue = 0x45,
    /// Configure the shadow doorbell buffer.
    DoorbellBufferConfig = 0x7C,
    /// NVMe over Fabrics command, carrying a fabrics command type.
    Fabrics = 0x7F,
    /// Format the NVM media of a namespace.
    FormatNvm = 0x80,
    /// Send a security protocol payload (e.g. TCG/Opal).
    SecuritySend = 0x81,
    /// Receive a security protocol payload.
    SecurityReceive = 0x82,
    /// Securely erase the NVM subsystem.
    Sanitize = 0x84,
    /// Load a computational program.
    LoadProgram = 0x85,
    /// Report the allocation status of LBA ranges.
    GetLbaStatus = 0x86,
    /// Activate or deactivate a computational program.
    ProgramActivationManagement = 0x88,
    /// Manage a memory range set.
    MemoryRangeSetManagement = 0x89,
    /// Securely erase a namespace.
    SanitizeNamespace = 0x8C,
    /// Vendor specific.
    VendorSpecific(u8),
}

impl From<AdminOpcode> for u8 {
    fn from(value: AdminOpcode) -> Self {
        match value {
            AdminOpcode::VendorSpecific(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl AdminOpcode {
    /// Vendor specific opcode range.
    const VENDOR_SPECIFIC_RANGE: RangeInclusive<u8> = 0xC0..=0xFF;
}

impl TryFrom<u8> for AdminOpcode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[AdminOpcode] = &[
            AdminOpcode::DeleteIoSubmissionQueue,
            AdminOpcode::CreateIoSubmissionQueue,
            AdminOpcode::GetLogPage,
            AdminOpcode::DeleteIoCompletionQueue,
            AdminOpcode::CreateIoCompletionQueue,
            AdminOpcode::Identify,
            AdminOpcode::Abort,
            AdminOpcode::SetFeatures,
            AdminOpcode::GetFeatures,
            AdminOpcode::AsynchronousEventRequest,
            AdminOpcode::NamespaceManagement,
            AdminOpcode::FirmwareCommit,
            AdminOpcode::FirmwareImageDownload,
            AdminOpcode::DeviceSelfTest,
            AdminOpcode::NamespaceAttachment,
            AdminOpcode::KeepAlive,
            AdminOpcode::DirectiveSend,
            AdminOpcode::DirectiveReceive,
            AdminOpcode::VirtualizationManagement,
            AdminOpcode::NvmeMiSend,
            AdminOpcode::NvmeMiReceive,
            AdminOpcode::CapacityManagement,
            AdminOpcode::DiscoveryInformationManagement,
            AdminOpcode::FabricZoningReceive,
            AdminOpcode::Lockdown,
            AdminOpcode::FabricZoningLookup,
            AdminOpcode::ClearExportedNvmResourceConfiguration,
            AdminOpcode::FabricZoningSend,
            AdminOpcode::CreateExportedNvmSubsystem,
            AdminOpcode::ManageExportedNvmSubsystem,
            AdminOpcode::ManageExportedNamespace,
            AdminOpcode::ManageExportedPort,
            AdminOpcode::CrossControllerReset,
            AdminOpcode::SendDiscoveryLogPage,
            AdminOpcode::TrackSend,
            AdminOpcode::TrackReceive,
            AdminOpcode::MigrationSend,
            AdminOpcode::MigrationReceive,
            AdminOpcode::ControllerDataQueue,
            AdminOpcode::DoorbellBufferConfig,
            AdminOpcode::Fabrics,
            AdminOpcode::FormatNvm,
            AdminOpcode::SecuritySend,
            AdminOpcode::SecurityReceive,
            AdminOpcode::Sanitize,
            AdminOpcode::LoadProgram,
            AdminOpcode::GetLbaStatus,
            AdminOpcode::ProgramActivationManagement,
            AdminOpcode::MemoryRangeSetManagement,
            AdminOpcode::SanitizeNamespace,
        ];

        CONST_VARIANTS
            .iter()
            .find(|&&x| u8::from(x) == value)
            .copied()
            .ok_or(value)
            .or_else(|x| {
                Self::VENDOR_SPECIFIC_RANGE
                    .contains(&x)
                    .then_some(Self::VendorSpecific(x))
                    .ok_or(x)
            })
    }
}

impl std::fmt::Display for AdminOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DeleteIoSubmissionQueue => write!(f, "Delete I/O Submission Queue"),
            Self::CreateIoSubmissionQueue => write!(f, "Create I/O Submission Queue"),
            Self::GetLogPage => write!(f, "Get Log Page"),
            Self::DeleteIoCompletionQueue => write!(f, "Delete I/O Completion Queue"),
            Self::CreateIoCompletionQueue => write!(f, "Create I/O Completion Queue"),
            Self::Identify => write!(f, "Identify"),
            Self::Abort => write!(f, "Abort"),
            Self::SetFeatures => write!(f, "Set Features"),
            Self::GetFeatures => write!(f, "Get Features"),
            Self::AsynchronousEventRequest => write!(f, "Asynchronous Event Request"),
            Self::NamespaceManagement => write!(f, "Namespace Management"),
            Self::FirmwareCommit => write!(f, "Firmware Commit"),
            Self::FirmwareImageDownload => write!(f, "Firmware Image Download"),
            Self::DeviceSelfTest => write!(f, "Device Self-test"),
            Self::NamespaceAttachment => write!(f, "Namespace Attachment"),
            Self::KeepAlive => write!(f, "Keep Alive"),
            Self::DirectiveSend => write!(f, "Directive Send"),
            Self::DirectiveReceive => write!(f, "Directive Receive"),
            Self::VirtualizationManagement => write!(f, "Virtualization Management"),
            Self::NvmeMiSend => write!(f, "NVMe-MI Send"),
            Self::NvmeMiReceive => write!(f, "NVMe-MI Receive"),
            Self::CapacityManagement => write!(f, "Capacity Management"),
            Self::DiscoveryInformationManagement => write!(f, "Discovery Information Management"),
            Self::FabricZoningReceive => write!(f, "Fabric Zoning Receive"),
            Self::Lockdown => write!(f, "Lockdown"),
            Self::FabricZoningLookup => write!(f, "Fabric Zoning Lookup"),
            Self::ClearExportedNvmResourceConfiguration => {
                write!(f, "Clear Exported NVM Resource Configuration")
            },
            Self::FabricZoningSend => write!(f, "Fabric Zoning Send"),
            Self::CreateExportedNvmSubsystem => write!(f, "Create Exported NVM Subsystem"),
            Self::ManageExportedNvmSubsystem => write!(f, "Manage Exported NVM Subsystem"),
            Self::ManageExportedNamespace => write!(f, "Manage Exported Namespace"),
            Self::ManageExportedPort => write!(f, "Manage Exported Port"),
            Self::CrossControllerReset => write!(f, "Cross-Controller Reset"),
            Self::SendDiscoveryLogPage => write!(f, "Send Discovery Log Page"),
            Self::TrackSend => write!(f, "Track Send"),
            Self::TrackReceive => write!(f, "Track Receive"),
            Self::MigrationSend => write!(f, "Migration Send"),
            Self::MigrationReceive => write!(f, "Migration Receive"),
            Self::ControllerDataQueue => write!(f, "Controller Data Queue"),
            Self::DoorbellBufferConfig => write!(f, "Doorbell Buffer Config"),
            Self::Fabrics => write!(f, "Fabrics Commands"),
            Self::FormatNvm => write!(f, "Format NVM"),
            Self::SecuritySend => write!(f, "Security Send"),
            Self::SecurityReceive => write!(f, "Security Receive"),
            Self::Sanitize => write!(f, "Sanitize"),
            Self::LoadProgram => write!(f, "Load Program"),
            Self::GetLbaStatus => write!(f, "Get LBA Status"),
            Self::ProgramActivationManagement => write!(f, "Program Activation Management"),
            Self::MemoryRangeSetManagement => write!(f, "Memory Range Set Management"),
            Self::SanitizeNamespace => write!(f, "Sanitize Namespace"),
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}

/// NVM command set I/O opcodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum IoOpcode {
    /// Commit written data to non-volatile media.
    Flush = 0x0,
    /// Write logical blocks.
    Write = 0x1,
    /// Read logical blocks.
    Read = 0x2,
    /// Mark logical blocks as invalid.
    WriteUncorrectable = 0x4,
    /// Compare logical blocks against transferred data.
    Compare = 0x5,
    /// Write zeroes to logical blocks.
    WriteZeroes = 0x8,
    /// Provide attributes for ranges of logical blocks.
    DatasetManagement = 0x9,
    /// Verify the integrity of logical blocks.
    Verify = 0xC,
    /// Register, unregister or replace a reservation key.
    ReservationRegister = 0xD,
    /// Report the registered reservations.
    ReservationReport = 0xE,
    /// Acquire a reservation.
    ReservationAcquire = 0x11,
    /// Receive I/O management information.
    IoManagementReceive = 0x12,
    /// Release or clear a reservation.
    ReservationRelease = 0x15,
    /// Cancel a previously submitted command.
    Cancel = 0x18,
    /// Copy logical blocks between ranges.
    Copy = 0x19,
    /// Send I/O management information.
    IoManagementSend = 0x1D,
    /// Manage the zones of a namespace.
    ZoneManagementSend = 0x79,
    /// Report the zones of a namespace.
    ZoneManagementReceive = 0x7A,
    /// Append logical blocks to a zone.
    ZoneAppend = 0x7D,
    /// NVMe over Fabrics command, carrying a fabrics command type.
    Fabrics = 0x7F,
    /// Vendor specific.
    VendorSpecific(u8),
}

impl From<IoOpcode> for u8 {
    fn from(value: IoOpcode) -> Self {
        match value {
            IoOpcode::VendorSpecific(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl IoOpcode {
    /// Vendor specific opcode range.
    const VENDOR_SPECIFIC_RANGE: RangeInclusive<u8> = 0x80..=0xFF;
}

impl TryFrom<u8> for IoOpcode {
    type Error = u8;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[IoOpcode] = &[
            IoOpcode::Flush,
            IoOpcode::Write,
            IoOpcode::Read,
            IoOpcode::WriteUncorrectable,
            IoOpcode::Compare,
            IoOpcode::WriteZeroes,
            IoOpcode::DatasetManagement,
            IoOpcode::Verify,
            IoOpcode::ReservationRegister,
            IoOpcode::ReservationReport,
            IoOpcode::ReservationAcquire,
            IoOpcode::IoManagementReceive,
            IoOpcode::ReservationRelease,
            IoOpcode::Cancel,
            IoOpcode::Copy,
            IoOpcode::IoManagementSend,
            IoOpcode::ZoneManagementSend,
            IoOpcode::ZoneManagementReceive,
            IoOpcode::ZoneAppend,
            IoOpcode::Fabrics,
        ];

        CONST_VARIANTS
            .iter()
            .find(|&&x| u8::from(x) == value)
            .copied()
            .ok_or(value)
            .or_else(|x| {
                Self::VENDOR_SPECIFIC_RANGE
                    .contains(&x)
                    .then_some(Self::VendorSpecific(x))
                    .ok_or(x)
            })
    }
}

impl std::fmt::Display for IoOpcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Flush => write!(f, "Flush"),
            Self::Write => write!(f, "Write"),
            Self::Read => write!(f, "Read"),
            Self::WriteUncorrectable => write!(f, "Write Uncorrectable"),
            Self::Compare => write!(f, "Compare"),
            Self::WriteZeroes => write!(f, "Write Zeroes"),
            Self::DatasetManagement => write!(f, "Dataset Management"),
            Self::Verify => write!(f, "Verify"),
            Self::ReservationRegister => write!(f, "Reservation Register"),
            Self::ReservationReport => write!(f, "Reservation Report"),
            Self::ReservationAcquire => write!(f, "Reservation Acquire"),
            Self::IoManagementReceive => write!(f, "I/O Management Receive"),
            Self::ReservationRelease => write!(f, "Reservation Release"),
            Self::Cancel => write!(f, "Cancel"),
            Self::Copy => write!(f, "Copy"),
            Self::IoManagementSend => write!(f, "I/O Management Send"),
            Self::ZoneManagementSend => write!(f, "Zone Management Send"),
            Self::ZoneManagementReceive => write!(f, "Zone Management Receive"),
            Self::ZoneAppend => write!(f, "Zone Append"),
            Self::Fabrics => write!(f, "Fabrics Commands"),
            Self::VendorSpecific(x) => write!(f, "vendor specific {x:#x}"),
        }
    }
}

/// Admin command submission queue entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdminCommand {
    /// Opcode (CDW0.OPC).
    pub opcode: AdminOpcode,
    /// Namespace identifier.
    pub nsid: u32,
    /// Command dword 2.
    pub cdw2: u32,
    /// Command dword 3.
    pub cdw3: u32,
    /// Command dword 10.
    pub cdw10: u32,
    /// Command dword 11.
    pub cdw11: u32,
    /// Command dword 12.
    pub cdw12: u32,
    /// Command dword 13.
    pub cdw13: u32,
    /// Command dword 14.
    pub cdw14: u32,
    /// Command dword 15.
    pub cdw15: u32,
}

impl AdminCommand {
    /// Size in bytes.
    pub(crate) const SIZE: usize = 0x40;
}

impl From<&AdminCommand> for [u8; AdminCommand::SIZE] {
    fn from(value: &AdminCommand) -> Self {
        let dwords = [
            u8::from(value.opcode).into(),
            value.nsid,
            value.cdw2,
            value.cdw3,
            // Metadata pointer and data pointer dwords
            0,
            0,
            0,
            0,
            0,
            0,
            value.cdw10,
            value.cdw11,
            value.cdw12,
            value.cdw13,
            value.cdw14,
            value.cdw15,
        ];

        let mut bytes = [0; _];

        for (chunk, dword) in bytes
            .as_chunks_mut::<{ size_of::<u32>() }>()
            .0
            .iter_mut()
            .zip(dwords)
        {
            *chunk = u32::to_le_bytes(dword);
        }

        bytes
    }
}

impl Default for AdminCommand {
    fn default() -> Self {
        Self {
            opcode: AdminOpcode::VendorSpecific(0),
            nsid: 0,
            cdw2: 0,
            cdw3: 0,
            cdw10: 0,
            cdw11: 0,
            cdw12: 0,
            cdw13: 0,
            cdw14: 0,
            cdw15: 0,
        }
    }
}

impl std::fmt::Display for AdminCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (NSID: {:#x}, CDW2: {:#x}, CDW3: {:#x}, CDW10: {:#x}, CDW11: {:#x}, CDW12: {:#x}, \
             CDW13: {:#x}, CDW14: {:#x}, CDW15: {:#x})",
            self.opcode,
            self.nsid,
            self.cdw2,
            self.cdw3,
            self.cdw10,
            self.cdw11,
            self.cdw12,
            self.cdw13,
            self.cdw14,
            self.cdw15
        )
    }
}

/// Admin command completion queue entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Completion {
    /// Status field.
    pub status: status::StatusField,
    /// Command specific result from completion queue entry dword 0.
    pub result: u32,
}

impl Completion {
    /// Size in bytes.
    pub const SIZE: usize = 16;
}

impl TryFrom<&[u8; Self::SIZE]> for Completion {
    type Error = status::Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        let status = status::StatusField::try_from(u16::from_le_bytes([data[14], data[15]]) >> 1)?;
        let result = u32::from_le_bytes([data[0], data[1], data[2], data[3]]);

        Ok(Self { status, result })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn admin_command_to_bytes() {
        const OPCODE: AdminOpcode = AdminOpcode::Sanitize;
        const OPCODE_BYTE: u8 = 0x84;
        const NSID: u32 = 0x1234_5678;
        const CDW2: u32 = 0x9ABC_DEF0;
        const CDW3: u32 = 0x0FED_CBA9;
        const CDW10: u32 = 0x1122_3344;
        const CDW11: u32 = 0x5566_7788;
        const CDW12: u32 = 0x99AA_BBCC;
        const CDW13: u32 = 0xDDEE_FF01;
        const CDW14: u32 = 0x2345_6789;
        const CDW15: u32 = 0xABCD_EF13;
        const DATA: [u8; AdminCommand::SIZE] = [
            OPCODE_BYTE,
            0x00,
            0x00,
            0x00,
            (NSID & 0xFF) as _,
            ((NSID >> 8) & 0xFF) as _,
            ((NSID >> 16) & 0xFF) as _,
            (NSID >> 24) as _,
            (CDW2 & 0xFF) as _,
            ((CDW2 >> 8) & 0xFF) as _,
            ((CDW2 >> 16) & 0xFF) as _,
            (CDW2 >> 24) as _,
            (CDW3 & 0xFF) as _,
            ((CDW3 >> 8) & 0xFF) as _,
            ((CDW3 >> 16) & 0xFF) as _,
            (CDW3 >> 24) as _,
            // Metadata pointer and data pointer dwords
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            0x00,
            (CDW10 & 0xFF) as _,
            ((CDW10 >> 8) & 0xFF) as _,
            ((CDW10 >> 16) & 0xFF) as _,
            (CDW10 >> 24) as _,
            (CDW11 & 0xFF) as _,
            ((CDW11 >> 8) & 0xFF) as _,
            ((CDW11 >> 16) & 0xFF) as _,
            (CDW11 >> 24) as _,
            (CDW12 & 0xFF) as _,
            ((CDW12 >> 8) & 0xFF) as _,
            ((CDW12 >> 16) & 0xFF) as _,
            (CDW12 >> 24) as _,
            (CDW13 & 0xFF) as _,
            ((CDW13 >> 8) & 0xFF) as _,
            ((CDW13 >> 16) & 0xFF) as _,
            (CDW13 >> 24) as _,
            (CDW14 & 0xFF) as _,
            ((CDW14 >> 8) & 0xFF) as _,
            ((CDW14 >> 16) & 0xFF) as _,
            (CDW14 >> 24) as _,
            (CDW15 & 0xFF) as _,
            ((CDW15 >> 8) & 0xFF) as _,
            ((CDW15 >> 16) & 0xFF) as _,
            (CDW15 >> 24) as _,
        ];

        let command = AdminCommand {
            opcode: OPCODE,
            nsid: NSID,
            cdw2: CDW2,
            cdw3: CDW3,
            cdw10: CDW10,
            cdw11: CDW11,
            cdw12: CDW12,
            cdw13: CDW13,
            cdw14: CDW14,
            cdw15: CDW15,
        };

        assert_eq!(<[u8; AdminCommand::SIZE]>::from(&command), DATA);
    }

    #[test]
    fn admin_command_vendor_opcode_to_bytes() {
        const OPCODE_BYTE: u8 = 0xD0;

        let mut expected = [0; AdminCommand::SIZE];
        expected[0] = OPCODE_BYTE;

        let command = AdminCommand {
            opcode: AdminOpcode::VendorSpecific(OPCODE_BYTE),
            ..Default::default()
        };

        assert_eq!(<[u8; AdminCommand::SIZE]>::from(&command), expected);
    }

    #[test]
    fn completion_parse_status_fields() {
        // Dword 3 0x5017_0000, phase tag set, dword 1 and 2 unused
        const GENERIC: &[u8; Completion::SIZE] = &[
            0x78, 0x56, 0x34, 0x12, 0xF0, 0xDE, 0xBC, 0x9A, 0xDE, 0xC0, 0xAD, 0x0B, 0x00, 0x00,
            0x17, 0x50,
        ];
        // Dword 3 0xA6E2_0000, phase tag clear
        const PATH_RELATED: &[u8; Completion::SIZE] = &[
            0xEF, 0xBE, 0xAD, 0xDE, 0xFF, 0xFF, 0xFF, 0xFF, 0x11, 0x11, 0x11, 0x11, 0x00, 0x00,
            0xE2, 0xA6,
        ];

        let completion = Completion::try_from(GENERIC).unwrap();

        assert_eq!(completion.result, 0x1234_5678);
        assert_eq!(
            completion.status.status_code,
            status::StatusCode::Generic(status::GenericStatus::InvalidNamespaceOrFormat)
        );
        assert_eq!(completion.status.command_retry_delay, 1);
        assert!(completion.status.more);
        assert!(!completion.status.do_not_retry);
        assert!(completion.status.is_error());

        let completion = Completion::try_from(PATH_RELATED).unwrap();

        assert_eq!(completion.result, 0xDEAD_BEEF);
        assert_eq!(
            completion.status.status_code,
            status::StatusCode::PathRelated(status::PathRelatedStatus::CommandAbortedByHost)
        );
        assert_eq!(completion.status.command_retry_delay, 2);
        assert!(!completion.status.more);
        assert!(completion.status.do_not_retry);
        assert!(completion.status.is_error());
    }

    #[test]
    fn completion_parse_ignores_phase_tag() {
        const PHASE_CLEAR: &[u8; Completion::SIZE] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00,
        ];
        const PHASE_SET: &[u8; Completion::SIZE] = &[
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x01, 0x00,
        ];

        let clear = Completion::try_from(PHASE_CLEAR).unwrap();
        let set = Completion::try_from(PHASE_SET).unwrap();

        assert_eq!(clear, set);
        assert_eq!(
            clear.status.status_code,
            status::StatusCode::Generic(status::GenericStatus::SuccessfulCompletion)
        );
        assert!(!clear.status.is_error());
    }
}
