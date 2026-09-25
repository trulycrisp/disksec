//! Identify controller result parsing.

use crate::{output, protocol::nvme::PAGE_SIZE};

/// Identify controller error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid string.
    String(Box<[u8]>),
    /// Invalid controller type value.
    ControllerType(u8),
    /// Invalid no-deallocate modifies media value.
    NoDeallocateModifies(u8),
    /// Invalid volatile write cache flush behavior value.
    FlushBehavior(u8),
    /// Invalid SGL alignment value.
    SglAlignment(u8),
    /// Invalid idle I/O exit latency limit scope value.
    IdleExitLatencyScope(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(x) => write!(f, "invalid string {}", output::format_bytes_hex(x)),
            Self::ControllerType(x) => write!(f, "invalid controller type {x:#x}"),
            Self::NoDeallocateModifies(x) => {
                write!(f, "invalid no-deallocate modifies media {x:#x}")
            },
            Self::FlushBehavior(x) => write!(f, "invalid flush behavior {x:#x}"),
            Self::SglAlignment(x) => write!(f, "invalid SGL alignment {x:#x}"),
            Self::IdleExitLatencyScope(x) => {
                write!(f, "invalid idle exit latency scope {x:#x}")
            },
        }
    }
}

/// Parse a string field.
pub fn parse_string(data: &[u8]) -> Result<String, Error> {
    // Strings are null-terminated
    let data = match data.iter().position(|&x| x == 0) {
        Some(x) => &data[..x],
        None => data,
    };

    let Ok(string) = str::from_utf8(data) else {
        return Err(Error::String(data.into()));
    };

    let string = string.trim_end();

    if !string
        .chars()
        .all(|x| x == ' ' || x.is_ascii_graphic())
    {
        return Err(Error::String(data.into()));
    }

    Ok(string.into())
}

/// Controller type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ControllerType {
    /// I/O controller.
    Io = 0x1,
    /// Discovery controller.
    Discovery = 0x2,
    /// Administrative controller.
    Administrative = 0x3,
}

impl ControllerType {
    /// Parse controller type field.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[ControllerType] = &[
            ControllerType::Io,
            ControllerType::Discovery,
            ControllerType::Administrative,
        ];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::ControllerType(value))
            .map(Some)
    }
}

impl std::fmt::Display for ControllerType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Io => write!(f, "I/O"),
            Self::Discovery => write!(f, "discovery"),
            Self::Administrative => write!(f, "administrative"),
        }
    }
}

/// NVM Express specification version field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Version {
    /// Major version.
    pub major: u16,
    /// Minor version.
    pub minor: u8,
    /// Tertiary version.
    pub tertiary: u8,
}

impl From<u32> for Version {
    fn from(value: u32) -> Self {
        let [tertiary, minor, major @ ..] = value.to_le_bytes();

        Self {
            major: u16::from_le_bytes(major),
            minor,
            tertiary,
        }
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}.{}.{}", self.major, self.minor, self.tertiary)
    }
}

/// Multi-path I/O and namespace sharing capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MultiPathCapabilities {
    /// NVM subsystem may contain more than one port.
    pub multiple_ports: bool,
    /// NVM subsystem may contain more than one controller.
    pub multiple_controllers: bool,
    /// Controller is associated with an SR-IOV virtual function.
    pub virtual_function: bool,
    /// Asymmetric Namespace Access Reporting is supported.
    pub ana_reporting: bool,
}

impl From<u8> for MultiPathCapabilities {
    fn from(value: u8) -> Self {
        Self {
            multiple_ports: (value & (1 << 0)) != 0,
            multiple_controllers: (value & (1 << 1)) != 0,
            virtual_function: (value & (1 << 2)) != 0,
            ana_reporting: (value & (1 << 3)) != 0,
        }
    }
}

/// Optional asynchronous events supported field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AsyncEvents {
    /// Namespace Attribute Changed event.
    pub namespace_attribute_changed: bool,
    /// Firmware Activation Notice event.
    pub firmware_activation: bool,
    /// Asymmetric Namespace Access Change Notice event.
    pub asymmetric_access_changed: bool,
    /// Predictable Latency Event Aggregate Log Change Notice event.
    pub predictable_latency_changed: bool,
    /// LBA Status Information Notice event.
    pub lba_status_changed: bool,
    /// Endurance Group Event Aggregate Log Change Notice event.
    pub endurance_group_changed: bool,
    /// Normal NVM Subsystem Shutdown event.
    pub subsystem_shutdown: bool,
    /// Zone Descriptor Changed Notice event.
    pub zone_information_changed: bool,
    /// Discovery Log Page Change Notice event.
    pub discovery_log_changed: bool,
}

impl From<u32> for AsyncEvents {
    fn from(value: u32) -> Self {
        Self {
            namespace_attribute_changed: (value & (1 << 8)) != 0,
            firmware_activation: (value & (1 << 9)) != 0,
            asymmetric_access_changed: (value & (1 << 11)) != 0,
            predictable_latency_changed: (value & (1 << 12)) != 0,
            lba_status_changed: (value & (1 << 13)) != 0,
            endurance_group_changed: (value & (1 << 14)) != 0,
            subsystem_shutdown: (value & (1 << 15)) != 0,
            zone_information_changed: (value & (1 << 27)) != 0,
            discovery_log_changed: (value & (1 << 31)) != 0,
        }
    }
}

/// Scope of the idle I/O exit latency limit.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdleExitLatencyScope {
    /// Limit applies to each operational power state.
    PowerState = 0x1,
    /// Limit applies to all operational power states.
    Global = 0x2,
}

impl IdleExitLatencyScope {
    /// Parse idle exit latency scope field, absent when unsupported.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[IdleExitLatencyScope] = &[
            IdleExitLatencyScope::PowerState,
            IdleExitLatencyScope::Global,
        ];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::IdleExitLatencyScope(value))
            .map(Some)
    }
}

impl std::fmt::Display for IdleExitLatencyScope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::PowerState => write!(f, "power state"),
            Self::Global => write!(f, "global"),
        }
    }
}

/// Controller attributes field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ControllerAttributes {
    /// 128-bit Host Identifier is supported.
    pub host_id_128bit: bool,
    /// Non-Operational Power State Permissive Mode is supported.
    pub non_operational_permissive: bool,
    /// NVM Sets are supported.
    pub nvm_sets: bool,
    /// Read Recovery Levels are supported.
    pub read_recovery_levels: bool,
    /// Endurance Groups are supported.
    pub endurance_groups: bool,
    /// Predictable Latency Mode is supported.
    pub predictable_latency: bool,
    /// Traffic Based Keep Alive is supported.
    pub traffic_keep_alive: bool,
    /// Namespace Granularity reporting is supported.
    pub namespace_granularity: bool,
    /// SQ Associations are supported.
    pub sq_associations: bool,
    /// UUID List reporting is supported.
    pub uuid_list: bool,
    /// NVM subsystem contains more than one domain.
    pub multiple_domains: bool,
    /// Fixed Capacity Management is supported.
    pub fixed_capacity_management: bool,
    /// Variable Capacity Management is supported.
    pub variable_capacity_management: bool,
    /// Endurance Groups can be deleted.
    pub delete_endurance_group: bool,
    /// NVM Sets can be deleted.
    pub delete_nvm_set: bool,
    /// Extended LBA formats are supported.
    pub extended_lba_formats: bool,
    /// Maximum transfer size and size limits exclude metadata.
    pub size_limits_exclude_metadata: bool,
    /// Host memory buffer access is restricted in non-operational power states.
    pub host_memory_buffer_restricted: bool,
    /// Reservations and Host Identifier interaction is supported.
    pub reservation_host_id_interaction: bool,
    /// Flexible Data Placement is supported.
    pub flexible_data_placement: bool,
    /// Power limit configuration is supported.
    pub power_limit: bool,
    /// Power measurement is supported.
    pub power_measurement: bool,
    /// Voltage measurement is supported.
    pub voltage_measurement: bool,
    /// Scope of the idle I/O exit latency limit.
    pub idle_exit_latency_scope: Option<IdleExitLatencyScope>,
}

impl TryFrom<u32> for ControllerAttributes {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            host_id_128bit: (value & (1 << 0)) != 0,
            non_operational_permissive: (value & (1 << 1)) != 0,
            nvm_sets: (value & (1 << 2)) != 0,
            read_recovery_levels: (value & (1 << 3)) != 0,
            endurance_groups: (value & (1 << 4)) != 0,
            predictable_latency: (value & (1 << 5)) != 0,
            traffic_keep_alive: (value & (1 << 6)) != 0,
            namespace_granularity: (value & (1 << 7)) != 0,
            sq_associations: (value & (1 << 8)) != 0,
            uuid_list: (value & (1 << 9)) != 0,
            multiple_domains: (value & (1 << 10)) != 0,
            fixed_capacity_management: (value & (1 << 11)) != 0,
            variable_capacity_management: (value & (1 << 12)) != 0,
            delete_endurance_group: (value & (1 << 13)) != 0,
            delete_nvm_set: (value & (1 << 14)) != 0,
            extended_lba_formats: (value & (1 << 15)) != 0,
            size_limits_exclude_metadata: (value & (1 << 16)) != 0,
            host_memory_buffer_restricted: (value & (1 << 17)) != 0,
            reservation_host_id_interaction: (value & (1 << 18)) != 0,
            flexible_data_placement: (value & (1 << 19)) != 0,
            power_limit: (value & (1 << 20)) != 0,
            power_measurement: (value & (1 << 21)) != 0,
            voltage_measurement: (value & (1 << 22)) != 0,
            idle_exit_latency_scope: IdleExitLatencyScope::parse(
                u8::try_from((value >> 23) & 0b11).unwrap(),
            )?,
        })
    }
}

/// NVM subsystem report field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SubsystemReport {
    /// NVM subsystem is part of an NVMe storage device.
    pub storage_device: bool,
    /// NVM subsystem is part of an NVMe enclosure.
    pub enclosure: bool,
}

impl From<u8> for SubsystemReport {
    fn from(value: u8) -> Self {
        Self {
            storage_device: (value & (1 << 0)) != 0,
            enclosure: (value & (1 << 1)) != 0,
        }
    }
}

/// Management endpoint capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ManagementEndpoint {
    /// Management endpoint is present on an SMBus/I2C port.
    pub smbus_port: bool,
    /// Management endpoint is present on a PCIe port.
    pub pcie_port: bool,
}

impl From<u8> for ManagementEndpoint {
    fn from(value: u8) -> Self {
        Self {
            smbus_port: (value & (1 << 0)) != 0,
            pcie_port: (value & (1 << 1)) != 0,
        }
    }
}

/// Optional admin commands supported field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptionalAdminCommands {
    /// SECURITY SEND and SECURITY RECEIVE commands.
    pub security_send_receive: bool,
    /// FORMAT NVM command.
    pub format_nvm: bool,
    /// FIRMWARE COMMIT and FIRMWARE IMAGE DOWNLOAD commands.
    pub firmware_download: bool,
    /// Namespace management capability.
    pub namespace_management: bool,
    /// DEVICE SELF-TEST command.
    pub device_self_test: bool,
    /// Directives, with the DIRECTIVE SEND and DIRECTIVE RECEIVE commands.
    pub directives: bool,
    /// NVMe-MI SEND and NVMe-MI RECEIVE commands.
    pub nvme_mi_send_receive: bool,
    /// VIRTUALIZATION MANAGEMENT command.
    pub virtualization_management: bool,
    /// DOORBELL BUFFER CONFIG command.
    pub doorbell_buffer_config: bool,
    /// GET LBA STATUS capability.
    pub get_lba_status: bool,
    /// Command and Feature Lockdown capability.
    pub command_feature_lockdown: bool,
    /// Host Managed Live Migration capability.
    pub host_managed_live_migration: bool,
}

impl From<u16> for OptionalAdminCommands {
    fn from(value: u16) -> Self {
        Self {
            security_send_receive: (value & (1 << 0)) != 0,
            format_nvm: (value & (1 << 1)) != 0,
            firmware_download: (value & (1 << 2)) != 0,
            namespace_management: (value & (1 << 3)) != 0,
            device_self_test: (value & (1 << 4)) != 0,
            directives: (value & (1 << 5)) != 0,
            nvme_mi_send_receive: (value & (1 << 6)) != 0,
            virtualization_management: (value & (1 << 7)) != 0,
            doorbell_buffer_config: (value & (1 << 8)) != 0,
            get_lba_status: (value & (1 << 9)) != 0,
            command_feature_lockdown: (value & (1 << 10)) != 0,
            host_managed_live_migration: (value & (1 << 11)) != 0,
        }
    }
}

/// Firmware update capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FirmwareUpdates {
    /// First firmware slot is read only.
    pub first_slot_read_only: bool,
    /// Number of firmware slots supported.
    pub slots: u8,
    /// Firmware can be activated without a reset.
    pub activation_without_reset: bool,
    /// Detects overlapping firmware or boot partition update sequences.
    pub multiple_update_detection: bool,
}

impl From<u8> for FirmwareUpdates {
    fn from(value: u8) -> Self {
        Self {
            first_slot_read_only: (value & (1 << 0)) != 0,
            slots: (value >> 1) & 0b111,
            activation_without_reset: (value & (1 << 4)) != 0,
            multiple_update_detection: (value & (1 << 5)) != 0,
        }
    }
}

/// Log page attributes field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LogPageAttributes {
    /// SMART/Health log page may be requested per namespace.
    pub smart_per_namespace: bool,
    /// Commands Supported and Effects log page is supported.
    pub command_effects: bool,
    /// GET LOG PAGE supports an offset and lengths beyond a page.
    pub extended_data: bool,
    /// Telemetry log pages and the Telemetry Log Notice event are supported.
    pub telemetry: bool,
    /// Persistent Event log page is supported.
    pub persistent_event: bool,
    /// Supported Log Pages log page is supported.
    pub supported_log_pages: bool,
    /// Telemetry log pages contain a fourth data area.
    pub telemetry_data_area4: bool,
}

impl From<u8> for LogPageAttributes {
    fn from(value: u8) -> Self {
        Self {
            smart_per_namespace: (value & (1 << 0)) != 0,
            command_effects: (value & (1 << 1)) != 0,
            extended_data: (value & (1 << 2)) != 0,
            telemetry: (value & (1 << 3)) != 0,
            persistent_event: (value & (1 << 4)) != 0,
            supported_log_pages: (value & (1 << 5)) != 0,
            telemetry_data_area4: (value & (1 << 6)) != 0,
        }
    }
}

/// Replay protected memory block support field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RpmbSupport {
    /// Number of RPMB units.
    pub units: u8,
    /// Authentication method, zero is HMAC SHA-256.
    pub authentication: u8,
    /// Total size of each unit in bytes.
    pub total_size: u32,
    /// Maximum size accessed by a single command in bytes.
    pub access_size: u32,
}

impl RpmbSupport {
    /// Parse field from raw value, absent when unsupported.
    fn parse(value: u32) -> Option<Self> {
        const TOTAL_SIZE_UNIT: u32 = 128 * 1024;
        const ACCESS_SIZE_UNIT: u32 = 512;

        let units = u8::try_from(value & 0b111).unwrap();
        if units == 0 {
            return None;
        }

        Some(Self {
            units,
            authentication: u8::try_from((value >> 3) & 0b111).unwrap(),
            total_size: (((value >> 16) & 0xFF) + 1) * TOTAL_SIZE_UNIT,
            access_size: (((value >> 24) & 0xFF) + 1) * ACCESS_SIZE_UNIT,
        })
    }
}

/// Firmware update granularity field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FirmwareUpdateGranularity {
    /// Size in bytes.
    Size(u32),
    /// Unlimited.
    Unlimited,
}

impl FirmwareUpdateGranularity {
    /// Parse field from raw value.
    pub(crate) fn parse(value: u8) -> Option<Self> {
        match value {
            0 => None,
            u8::MAX => Some(Self::Unlimited),
            x => Some(Self::Size(u32::from(x) * u32::try_from(PAGE_SIZE).unwrap())),
        }
    }
}

/// Media state after a sanitize operation that deallocates.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NoDeallocateModifies {
    /// Media is not additionally modified.
    Unmodified = 0x1,
    /// Media is additionally modified.
    Modified = 0x2,
}

impl NoDeallocateModifies {
    /// Parse field, absent when not defined.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[NoDeallocateModifies] = &[
            NoDeallocateModifies::Unmodified,
            NoDeallocateModifies::Modified,
        ];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::NoDeallocateModifies(value))
            .map(Some)
    }
}

impl std::fmt::Display for NoDeallocateModifies {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unmodified => write!(f, "unmodified"),
            Self::Modified => write!(f, "modified"),
        }
    }
}

/// Sanitize capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SanitizeCapabilities {
    /// Crypto erase sanitize operation is supported.
    pub crypto_erase: bool,
    /// Block erase sanitize operation is supported.
    pub block_erase: bool,
    /// Overwrite sanitize operation is supported.
    pub overwrite: bool,
    /// Sanitize verification is supported.
    pub verification: bool,
    /// Namespace sanitize verification is supported.
    pub namespace_verification: bool,
    /// Sanitize purge request and reporting is supported.
    pub purge_reporting: bool,
    /// No-Deallocate After Sanitize is prohibited.
    pub no_deallocate_inhibited: bool,
    /// Media state after a sanitize operation that deallocates.
    pub no_deallocate_modifies: Option<NoDeallocateModifies>,
}

impl TryFrom<u32> for SanitizeCapabilities {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            crypto_erase: (value & (1 << 0)) != 0,
            block_erase: (value & (1 << 1)) != 0,
            overwrite: (value & (1 << 2)) != 0,
            verification: (value & (1 << 3)) != 0,
            namespace_verification: (value & (1 << 4)) != 0,
            purge_reporting: (value & (1 << 5)) != 0,
            no_deallocate_inhibited: (value & (1 << 29)) != 0,
            no_deallocate_modifies: NoDeallocateModifies::parse(
                u8::try_from((value >> 30) & 0b11).unwrap(),
            )?,
        })
    }
}

/// Asymmetric Namespace Access capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AnaCapabilities {
    /// ANA Optimized state is reported.
    pub optimized: bool,
    /// ANA Non-Optimized state is reported.
    pub non_optimized: bool,
    /// ANA Inaccessible state is reported.
    pub inaccessible: bool,
    /// ANA Persistent Loss state is reported.
    pub persistent_loss: bool,
    /// ANA Change state is reported.
    pub change: bool,
    /// Namespace group identifiers do not change while attached.
    pub static_group_id: bool,
    /// Namespace management supports a non-zero group identifier.
    pub non_zero_group_id: bool,
}

impl From<u8> for AnaCapabilities {
    fn from(value: u8) -> Self {
        Self {
            optimized: (value & (1 << 0)) != 0,
            non_optimized: (value & (1 << 1)) != 0,
            inaccessible: (value & (1 << 2)) != 0,
            persistent_loss: (value & (1 << 3)) != 0,
            change: (value & (1 << 4)) != 0,
            static_group_id: (value & (1 << 6)) != 0,
            non_zero_group_id: (value & (1 << 7)) != 0,
        }
    }
}

/// Queue entry size field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct QueueEntrySize {
    /// Required entry size in bytes.
    pub required: u32,
    /// Maximum entry size in bytes.
    pub maximum: u32,
}

impl From<u8> for QueueEntrySize {
    fn from(value: u8) -> Self {
        Self {
            required: 2u32.saturating_pow((value & 0b1111).into()),
            maximum: 2u32.saturating_pow(((value >> 4) & 0b1111).into()),
        }
    }
}

/// Optional NVM commands supported field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct OptionalNvmCommands {
    /// COMPARE command.
    pub compare: bool,
    /// WRITE UNCORRECTABLE command.
    pub write_uncorrectable: bool,
    /// DATASET MANAGEMENT command.
    pub dataset_management: bool,
    /// WRITE ZEROES command.
    pub write_zeroes: bool,
    /// Save field in SET FEATURES and select field in GET FEATURES.
    pub feature_field: bool,
    /// Reservations.
    pub reservations: bool,
    /// Timestamp feature.
    pub timestamp: bool,
    /// VERIFY command.
    pub verify: bool,
}

impl From<u16> for OptionalNvmCommands {
    fn from(value: u16) -> Self {
        Self {
            compare: (value & (1 << 0)) != 0,
            write_uncorrectable: (value & (1 << 1)) != 0,
            dataset_management: (value & (1 << 2)) != 0,
            write_zeroes: (value & (1 << 3)) != 0,
            feature_field: (value & (1 << 4)) != 0,
            reservations: (value & (1 << 5)) != 0,
            timestamp: (value & (1 << 6)) != 0,
            verify: (value & (1 << 7)) != 0,
        }
    }
}

/// Format NVM attributes field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormatAttributes {
    /// FORMAT NVM applies to all namespaces.
    pub all_namespaces: bool,
    /// Secure erase applies to all namespaces.
    pub secure_erase_all_namespaces: bool,
    /// Cryptographic erase is supported.
    pub crypto_erase: bool,
    /// FORMAT NVM supports a namespace identifier of `FFFFFFFFh`.
    pub broadcast_namespace: bool,
}

impl From<u8> for FormatAttributes {
    fn from(value: u8) -> Self {
        Self {
            all_namespaces: (value & (1 << 0)) != 0,
            secure_erase_all_namespaces: (value & (1 << 1)) != 0,
            crypto_erase: (value & (1 << 2)) != 0,
            broadcast_namespace: (value & (1 << 3)) == 0,
        }
    }
}

/// Behavior of FLUSH applied to all namespaces.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FlushBehavior {
    /// Not supported for all namespaces.
    Unsupported = 0x2,
    /// Supported for all namespaces.
    Supported = 0x3,
}

impl FlushBehavior {
    /// Parse flush behavior field, absent when not indicated.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[FlushBehavior] = &[FlushBehavior::Unsupported, FlushBehavior::Supported];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::FlushBehavior(value))
            .map(Some)
    }
}

impl std::fmt::Display for FlushBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported => write!(f, "unsupported"),
            Self::Supported => write!(f, "supported"),
        }
    }
}

/// Volatile write cache field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WriteCache {
    /// Volatile write cache is present.
    pub present: bool,
    /// Behavior of FLUSH applied to all namespaces.
    pub flush_behavior: Option<FlushBehavior>,
}

impl TryFrom<u8> for WriteCache {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            present: (value & (1 << 0)) != 0,
            flush_behavior: FlushBehavior::parse((value >> 1) & 0b11)?,
        })
    }
}

/// Namespace write protection capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct WriteProtectCapabilities {
    /// No Write Protect and Write Protect states are supported.
    pub write_protect: bool,
    /// Write Protect Until Power Cycle state is supported.
    pub until_power_cycle: bool,
    /// Permanent Write Protect state is supported.
    pub permanent: bool,
}

impl From<u8> for WriteProtectCapabilities {
    fn from(value: u8) -> Self {
        Self {
            write_protect: (value & (1 << 0)) != 0,
            until_power_cycle: (value & (1 << 1)) != 0,
            permanent: (value & (1 << 2)) != 0,
        }
    }
}

/// Scatter gather list alignment requirement.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SglAlignment {
    /// No alignment requirement.
    Unaligned = 0x1,
    /// Dword aligned lengths and addresses.
    Dword = 0x2,
}

impl SglAlignment {
    /// Parse SGL support field, absent when unsupported.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[SglAlignment] = &[SglAlignment::Unaligned, SglAlignment::Dword];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::SglAlignment(value))
            .map(Some)
    }
}

impl std::fmt::Display for SglAlignment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unaligned => write!(f, "unaligned"),
            Self::Dword => write!(f, "dword"),
        }
    }
}

/// Scatter gather list support field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SglSupport {
    /// Alignment required of SGL data blocks.
    pub alignment: Option<SglAlignment>,
    /// Keyed SGL Data Block descriptor is supported.
    pub keyed_data_block: bool,
    /// SGL Bit Bucket descriptor is supported.
    pub bit_bucket: bool,
    /// Byte aligned contiguous physical buffers are supported.
    pub byte_aligned_buffer: bool,
    /// SGL length may be longer than the data amount.
    pub longer_than_data: bool,
    /// Metadata pointer may contain an SGL descriptor.
    pub metadata_pointer: bool,
    /// SGL Data Block descriptor address field is supported.
    pub address_field: bool,
    /// Transport SGL Data Block descriptor is supported.
    pub transport_data_block: bool,
}

impl TryFrom<u32> for SglSupport {
    type Error = Error;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        Ok(Self {
            alignment: SglAlignment::parse(u8::try_from(value & 0b11).unwrap())?,
            keyed_data_block: (value & (1 << 2)) != 0,
            bit_bucket: (value & (1 << 16)) != 0,
            byte_aligned_buffer: (value & (1 << 17)) != 0,
            longer_than_data: (value & (1 << 18)) != 0,
            metadata_pointer: (value & (1 << 19)) != 0,
            address_field: (value & (1 << 20)) != 0,
            transport_data_block: (value & (1 << 21)) != 0,
        })
    }
}

/// Power state descriptor.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct PowerState {
    /// Maximum power consumed in microwatts.
    pub max_power: Option<u32>,
    /// State does not process I/O commands.
    pub non_operational: bool,
    /// Entry latency in microseconds.
    pub entry_latency: Option<u32>,
    /// Exit latency in microseconds.
    pub exit_latency: Option<u32>,
    /// Relative read throughput, lower is higher throughput.
    pub relative_read_throughput: u8,
    /// Relative read latency, lower is lower latency.
    pub relative_read_latency: u8,
    /// Relative write throughput, lower is higher throughput.
    pub relative_write_throughput: u8,
    /// Relative write latency, lower is lower latency.
    pub relative_write_latency: u8,
    /// Idle power consumed in microwatts.
    pub idle_power: Option<u32>,
    /// Active power consumed in microwatts.
    pub active_power: Option<u32>,
    /// Workload the active power was measured with.
    pub active_power_workload: u8,
}

impl PowerState {
    /// Size in bytes.
    pub(crate) const SIZE: usize = 32;

    /// Convert a power value and its scale to microwatts.
    fn power(value: u16, scale: u8) -> Option<u32> {
        const SCALE_HUNDRED_MICROWATT: u8 = 0x1;
        const SCALE_CENTIWATT: u8 = 0x2;

        let unit = match scale {
            SCALE_HUNDRED_MICROWATT => 100,
            SCALE_CENTIWATT => 10_000,
            _ => return None,
        };

        (value != 0).then(|| u32::from(value) * unit)
    }

    /// Parse power state list.
    pub(crate) fn parse_list(data: &[u8; Identify::SIZE]) -> Box<[Self]> {
        // Number of supported states, the count is zero-based
        let count = usize::from(data[263]) + 1;

        data[2048..3072]
            .as_chunks::<{ Self::SIZE }>()
            .0
            .iter()
            .take(count)
            .map(Self::from)
            .collect()
    }
}

impl From<&[u8; PowerState::SIZE]> for PowerState {
    fn from(value: &[u8; PowerState::SIZE]) -> Self {
        // Maximum power uses a single scale bit selecting the same units as the
        // idle and active power scales
        let max_power_scale = if (value[3] & (1 << 0)) != 0 { 0x1 } else { 0x2 };

        Self {
            max_power: Self::power(u16::from_le_bytes([value[0], value[1]]), max_power_scale),
            non_operational: (value[3] & (1 << 1)) != 0,
            entry_latency: Some(u32::from_le_bytes(*value[4..8].first_chunk().unwrap()))
                .filter(|&x| x != 0),
            exit_latency: Some(u32::from_le_bytes(*value[8..12].first_chunk().unwrap()))
                .filter(|&x| x != 0),
            relative_read_throughput: value[12] & 0b1_1111,
            relative_read_latency: value[13] & 0b1_1111,
            relative_write_throughput: value[14] & 0b1_1111,
            relative_write_latency: value[15] & 0b1_1111,
            idle_power: Self::power(
                u16::from_le_bytes([value[16], value[17]]),
                (value[18] >> 6) & 0b11,
            ),
            active_power: Self::power(
                u16::from_le_bytes([value[20], value[21]]),
                (value[22] >> 6) & 0b11,
            ),
            active_power_workload: value[22] & 0b111,
        }
    }
}

/// Identify controller data structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identify {
    /// PCI Vendor ID.
    pub vendor_id: u16,
    /// PCI Subsystem Vendor ID.
    pub subsystem_vendor_id: u16,
    /// Serial number.
    pub serial: String,
    /// Model name.
    pub model: String,
    /// Active firmware version.
    pub firmware: String,
    /// Recommended arbitration burst size in commands.
    pub arbitration_burst: u32,
    /// IEEE OUI Identifier of the vendor.
    pub ieee_oui: u32,
    /// Multi-path I/O and namespace sharing capabilities.
    pub multi_path: MultiPathCapabilities,
    /// Maximum Data Transfer Size as a power of two in units of the minimum
    /// memory page size, None means unlimited.
    pub max_transfer_size: Option<u8>,
    /// Controller ID unique within the NVM subsystem.
    pub controller_id: u16,
    /// NVM Express specification version.
    pub version: Version,
    /// Resume latency from the D3 power state in microseconds.
    pub rtd3_resume_latency: Option<u32>,
    /// Entry latency to the D3 power state in microseconds.
    pub rtd3_entry_latency: Option<u32>,
    /// Optional asynchronous events supported.
    pub async_events: AsyncEvents,
    /// Controller attributes.
    pub controller_attributes: ControllerAttributes,
    /// Supported Read Recovery Levels.
    pub read_recovery_levels: Box<[u8]>,
    /// Controller type.
    pub controller_type: Option<ControllerType>,
    /// FRU Globally Unique Identifier.
    pub fru_guid: [u8; 16],
    /// Command retry delay times in milliseconds.
    pub command_retry_delays: [Option<u32>; 3],
    /// NVM subsystem report.
    pub subsystem_report: SubsystemReport,
    /// Remaining VPD write cycles.
    pub vpd_write_cycles: Option<u8>,
    /// Management endpoint capabilities.
    pub management_endpoint: ManagementEndpoint,
    /// Optional admin command support.
    pub optional_admin_commands: OptionalAdminCommands,
    /// Maximum number of concurrent ABORT commands.
    pub abort_command_limit: u16,
    /// Maximum number of active ASYNCHRONOUS EVENT REQUEST commands.
    pub async_event_request_limit: u16,
    /// Firmware update capabilities.
    pub firmware_updates: FirmwareUpdates,
    /// Log page attributes.
    pub log_page_attributes: LogPageAttributes,
    /// Maximum number of error log page entries supported.
    pub error_log_page_entries: u16,
    /// All vendor specific admin commands use the standard format.
    pub admin_vsc_format: bool,
    /// Autonomous power state transitions are supported.
    pub autonomous_power_states: bool,
    /// Composite temperature warning threshold in Kelvin.
    pub warning_temperature: Option<u16>,
    /// Composite temperature critical threshold in Kelvin.
    pub critical_temperature: Option<u16>,
    /// Maximum time for firmware activation in milliseconds.
    pub max_firmware_activation: Option<u32>,
    /// Preferred host memory buffer size in bytes.
    pub host_memory_preferred: Option<u64>,
    /// Minimum host memory buffer size in bytes.
    pub host_memory_minimum: Option<u64>,
    /// Total NVM capacity accessible by the controller in bytes.
    pub total_capacity: Option<u128>,
    /// Unallocated NVM capacity accessible by the controller in bytes.
    pub unallocated_capacity: Option<u128>,
    /// Replay protected memory block support.
    pub rpmb: Option<RpmbSupport>,
    /// Extended device self-test time in minutes.
    pub extended_self_test_time: Option<u16>,
    /// Device self-test applies to the whole NVM subsystem.
    pub self_test_subsystem_scope: bool,
    /// Firmware update granularity size.
    pub fw_update_granularity: Option<FirmwareUpdateGranularity>,
    /// Keep alive timer granularity in milliseconds.
    pub keep_alive_granularity: Option<u32>,
    /// Host controlled thermal management is supported.
    pub thermal_management: bool,
    /// Minimum thermal management temperature in Kelvin.
    pub min_thermal_temperature: Option<u16>,
    /// Maximum thermal management temperature in Kelvin.
    pub max_thermal_temperature: Option<u16>,
    /// Sanitize capabilities.
    pub sanitize: SanitizeCapabilities,
    /// Minimum host memory buffer descriptor size in bytes, `None` if no
    /// minimum.
    pub host_memory_descriptor_min: Option<u64>,
    /// Maximum number of host memory buffer descriptors, `None` if unlimited.
    pub host_memory_descriptor_max: Option<u16>,
    /// Maximum valid NVM Set Identifier.
    pub nvm_set_id_max: u16,
    /// Maximum valid Endurance Group Identifier.
    pub endurance_group_id_max: u16,
    /// Maximum time for an Asymmetric Namespace Access transition in seconds.
    pub ana_transition_time: u8,
    /// Asymmetric Namespace Access capabilities.
    pub ana_capabilities: AnaCapabilities,
    /// Maximum valid Asymmetric Namespace Access Group Identifier.
    pub ana_group_id_max: u32,
    /// Number of Asymmetric Namespace Access Group Identifiers supported.
    pub ana_group_ids: u32,
    /// Persistent event log size in bytes.
    pub persistent_event_log_size: Option<u64>,
    /// Submission queue entry size.
    pub submission_queue_entry: QueueEntrySize,
    /// Completion queue entry size.
    pub completion_queue_entry: QueueEntrySize,
    /// Maximum number of outstanding commands.
    pub max_outstanding_commands: Option<u16>,
    /// Maximum Namespace ID (NSID) value.
    pub namespace_count: u32,
    /// Optional NVM command support.
    pub optional_nvm_commands: OptionalNvmCommands,
    /// Fused COMPARE and WRITE operation is supported.
    pub fused_compare_write: bool,
    /// Format NVM attributes.
    pub format_attributes: FormatAttributes,
    /// Volatile write cache.
    pub write_cache: WriteCache,
    /// Atomic write size in logical blocks.
    pub atomic_write: u32,
    /// Atomic write size on power failure in logical blocks.
    pub atomic_write_power_fail: u32,
    /// All vendor specific NVM commands use the standard format.
    pub nvm_vsc_format: bool,
    /// Namespace write protection capabilities.
    pub write_protect: WriteProtectCapabilities,
    /// Atomic compare and write size in logical blocks.
    pub atomic_compare_write: u32,
    /// Scatter gather list support.
    pub sgl: SglSupport,
    /// Maximum namespaces supported, if `None` the maximum is
    /// `namespace_count`.
    pub max_namespaces: Option<u32>,
    /// NVM subsystem NVMe qualified name.
    pub subsystem_nqn: String,
    /// Supported power states.
    pub power_state_descriptors: Box<[PowerState]>,
    /// Vendor specific data.
    pub vendor_specific: [u8; 1024],
}

impl Identify {
    /// Size in bytes.
    pub(crate) const SIZE: usize = PAGE_SIZE;

    /// Get total NVM capacity accessible by the controller in bytes.
    pub(crate) fn capacity(&self) -> Option<u64> {
        self.total_capacity
            .map(|x| u64::try_from(x).unwrap_or(u64::MAX))
    }

    /// Display with a total capacity.
    pub(crate) fn display_capacity(&self, total_capacity: u64) -> String {
        format!(
            "(model: {}, serial: {}, firmware: {}, size: {})",
            self.model.trim(),
            self.serial.trim(),
            self.firmware.trim(),
            output::format_byte_size_decimal(total_capacity),
        )
    }
}

impl TryFrom<&[u8; Identify::SIZE]> for Identify {
    type Error = Error;

    fn try_from(data: &[u8; Identify::SIZE]) -> Result<Self, Self::Error> {
        const COMMAND_RETRY_DELAY_OFFSET: usize = 128;
        const DECISECOND: u32 = 100;
        const PERSISTENT_EVENT_LOG_UNIT: u64 = 64 * 1024;

        let vendor_id = u16::from_le_bytes([data[0], data[1]]);
        let subsystem_vendor_id = u16::from_le_bytes([data[2], data[3]]);
        let serial = parse_string(&data[4..24])?;
        let model = parse_string(&data[24..64])?;
        let firmware = parse_string(&data[64..72])?;
        let arbitration_burst = 2u32.saturating_pow(data[72].into());
        let ieee_oui = u32::from_le_bytes([data[73], data[74], data[75], 0]);
        let multi_path = data[76].into();
        let max_transfer_size = (data[77] != 0).then_some(data[77]);
        let controller_id = u16::from_le_bytes([data[78], data[79]]);
        let version = u32::from_le_bytes([data[80], data[81], data[82], data[83]]).into();
        let rtd3_resume_latency =
            Some(u32::from_le_bytes(*data[84..88].first_chunk().unwrap())).filter(|&x| x != 0);
        let rtd3_entry_latency =
            Some(u32::from_le_bytes(*data[88..92].first_chunk().unwrap())).filter(|&x| x != 0);
        let async_events = u32::from_le_bytes(*data[92..96].first_chunk().unwrap()).into();
        let controller_attributes =
            u32::from_le_bytes(*data[96..100].first_chunk().unwrap()).try_into()?;
        // Each bit position is the Read Recovery Level it reports support for
        let levels = u16::from_le_bytes([data[100], data[101]]);
        let read_recovery_levels = (0..u16::BITS)
            .filter(|x| (levels & (1 << x)) != 0)
            .map(|x| u8::try_from(x).unwrap())
            .collect();
        let controller_type = ControllerType::parse(data[111])?;
        let fru_guid = *data[112..128].first_chunk().unwrap();

        // Retry delay times are in units of 100 milliseconds
        let command_retry_delays = std::array::from_fn(|i| {
            let offset = COMMAND_RETRY_DELAY_OFFSET + i * size_of::<u16>();
            let value = u16::from_le_bytes([data[offset], data[offset + 1]]);
            (value != 0).then(|| u32::from(value) * DECISECOND)
        });

        let subsystem_report = data[253].into();

        // Remaining cycles are only valid when the field is reported
        let vpd_write_cycles = ((data[254] & (1 << 7)) != 0).then(|| data[254] & 0b111_1111);

        let management_endpoint = data[255].into();
        let optional_admin_commands =
            OptionalAdminCommands::from(u16::from_le_bytes([data[256], data[257]]));
        let abort_command_limit = u16::from(data[258]) + 1;
        let async_event_request_limit = u16::from(data[259]) + 1;
        let firmware_updates = data[260].into();
        let log_page_attributes = LogPageAttributes::from(data[261]);
        let error_log_page_entries = u16::from(data[262]) + 1;
        let admin_vsc_format = (data[264] & (1 << 0)) != 0;
        let autonomous_power_states = (data[265] & (1 << 0)) != 0;
        let warning_temperature =
            Some(u16::from_le_bytes([data[266], data[267]])).filter(|&x| x != 0);
        let critical_temperature =
            Some(u16::from_le_bytes([data[268], data[269]])).filter(|&x| x != 0);
        let max_firmware_activation = Some(u16::from_le_bytes([data[270], data[271]]))
            .filter(|&x| x != 0)
            .map(|x| u32::from(x) * DECISECOND);

        // Host memory buffer sizes are in units of the standard page size
        let page_size = u64::try_from(PAGE_SIZE).unwrap();
        let host_memory_preferred =
            Some(u32::from_le_bytes(*data[272..276].first_chunk().unwrap()))
                .filter(|&x| x != 0)
                .map(|x| u64::from(x) * page_size);
        let host_memory_minimum = Some(u32::from_le_bytes(*data[276..280].first_chunk().unwrap()))
            .filter(|&x| x != 0)
            .map(|x| u64::from(x) * page_size);

        let total_capacity =
            Some(u128::from_le_bytes(*data[280..296].first_chunk().unwrap())).filter(|&x| x != 0);
        let unallocated_capacity =
            Some(u128::from_le_bytes(*data[296..312].first_chunk().unwrap())).filter(|&x| x != 0);
        let rpmb = RpmbSupport::parse(u32::from_le_bytes(*data[312..316].first_chunk().unwrap()));

        // Only defined when the device self-test command is supported
        let extended_self_test_time = optional_admin_commands
            .device_self_test
            .then(|| u16::from_le_bytes([data[316], data[317]]));
        let self_test_subsystem_scope = (data[318] & (1 << 0)) != 0;
        let fw_update_granularity = FirmwareUpdateGranularity::parse(data[319]);
        let keep_alive_granularity = Some(u16::from_le_bytes([data[320], data[321]]))
            .filter(|&x| x != 0)
            .map(|x| u32::from(x) * DECISECOND);
        let thermal_management = (data[322] & (1 << 0)) != 0;
        let min_thermal_temperature =
            Some(u16::from_le_bytes([data[324], data[325]])).filter(|&x| x != 0);
        let max_thermal_temperature =
            Some(u16::from_le_bytes([data[326], data[327]])).filter(|&x| x != 0);
        let sanitize = u32::from_le_bytes(*data[328..332].first_chunk().unwrap()).try_into()?;
        let host_memory_descriptor_min =
            Some(u32::from_le_bytes(*data[332..336].first_chunk().unwrap()))
                .filter(|&x| x != 0)
                .map(|x| u64::from(x) * page_size);
        let host_memory_descriptor_max =
            Some(u16::from_le_bytes([data[336], data[337]])).filter(|&x| x != 0);
        let nvm_set_id_max = u16::from_le_bytes([data[338], data[339]]);
        let endurance_group_id_max = u16::from_le_bytes([data[340], data[341]]);
        let ana_transition_time = data[342];
        let ana_capabilities = data[343].into();
        let ana_group_id_max = u32::from_le_bytes(*data[344..348].first_chunk().unwrap());
        let ana_group_ids = u32::from_le_bytes(*data[348..352].first_chunk().unwrap());

        // Reserved unless the persistent event log is supported
        let persistent_event_log_size = log_page_attributes.persistent_event.then(|| {
            u64::from(u32::from_le_bytes(*data[352..356].first_chunk().unwrap()))
                * PERSISTENT_EVENT_LOG_UNIT
        });
        let submission_queue_entry = data[512].into();
        let completion_queue_entry = data[513].into();
        let max_outstanding_commands =
            Some(u16::from_le_bytes([data[514], data[515]])).filter(|&x| x != 0);
        let namespace_count = u32::from_le_bytes([data[516], data[517], data[518], data[519]]);
        let optional_nvm_commands = u16::from_le_bytes([data[520], data[521]]).into();
        let fused_compare_write = (data[522] & (1 << 0)) != 0;
        let format_attributes = data[524].into();
        let write_cache = data[525].try_into()?;
        let atomic_write = u32::from(u16::from_le_bytes([data[526], data[527]])) + 1;
        let atomic_write_power_fail = u32::from(u16::from_le_bytes([data[528], data[529]])) + 1;
        let nvm_vsc_format = (data[530] & (1 << 0)) != 0;
        let write_protect = data[531].into();
        let atomic_compare_write = u32::from(u16::from_le_bytes([data[532], data[533]])) + 1;
        let sgl = u32::from_le_bytes(*data[536..540].first_chunk().unwrap()).try_into()?;
        let max_namespaces =
            Some(u32::from_le_bytes(*data[540..544].first_chunk().unwrap())).filter(|&x| x != 0);
        let subsystem_nqn = parse_string(&data[768..1024])?;
        let power_state_descriptors = PowerState::parse_list(data);
        let vendor_specific = std::array::from_fn(|i| data[3072 + i]);

        Ok(Self {
            vendor_id,
            subsystem_vendor_id,
            serial,
            model,
            firmware,
            arbitration_burst,
            ieee_oui,
            multi_path,
            max_transfer_size,
            controller_id,
            version,
            rtd3_resume_latency,
            rtd3_entry_latency,
            async_events,
            controller_attributes,
            read_recovery_levels,
            controller_type,
            fru_guid,
            command_retry_delays,
            subsystem_report,
            vpd_write_cycles,
            management_endpoint,
            optional_admin_commands,
            abort_command_limit,
            async_event_request_limit,
            firmware_updates,
            log_page_attributes,
            error_log_page_entries,
            admin_vsc_format,
            autonomous_power_states,
            warning_temperature,
            critical_temperature,
            max_firmware_activation,
            host_memory_preferred,
            host_memory_minimum,
            total_capacity,
            unallocated_capacity,
            rpmb,
            extended_self_test_time,
            self_test_subsystem_scope,
            fw_update_granularity,
            keep_alive_granularity,
            thermal_management,
            min_thermal_temperature,
            max_thermal_temperature,
            sanitize,
            host_memory_descriptor_min,
            host_memory_descriptor_max,
            nvm_set_id_max,
            endurance_group_id_max,
            ana_transition_time,
            ana_capabilities,
            ana_group_id_max,
            ana_group_ids,
            persistent_event_log_size,
            submission_queue_entry,
            completion_queue_entry,
            max_outstanding_commands,
            namespace_count,
            optional_nvm_commands,
            fused_compare_write,
            format_attributes,
            write_cache,
            atomic_write,
            atomic_write_power_fail,
            nvm_vsc_format,
            write_protect,
            atomic_compare_write,
            sgl,
            max_namespaces,
            subsystem_nqn,
            power_state_descriptors,
            vendor_specific,
        })
    }
}

impl std::fmt::Display for Identify {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(model: {}, serial: {}, firmware: {})",
            self.model.trim(),
            self.serial.trim(),
            self.firmware.trim(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn identify_parse() {
        const DATA_VALID: &[&[u8; Identify::SIZE]] =
            &[test_data::patriot_p300::IDENTIFY_CONTROLLER];
        const DATA_INVALID: &[&[u8; Identify::SIZE]] = &[&[0x01; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(Identify::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(Identify::try_from(data).is_err());
        }
    }
}
