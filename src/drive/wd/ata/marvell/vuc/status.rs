//! VUC status definitions.

use super::{DebugStopCode, ErrorCode};
use crate::protocol::ata::{
    SECTOR_SIZE,
    command::{ResultRegisters, StatusRegister},
};

/// VUC status error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid version.
    Version(u16),
    /// Invalid command type.
    CommandType(u8),
    /// Invalid drive protect state.
    ProtectState(u8),
    /// Invalid offline pending status.
    OfflinePending(u8),
    /// Invalid Data Lifeguard 2 state.
    Dlg2State(u8),
    /// Invalid drive state.
    State(u8),
    /// Invalid test type.
    TestType(u8),
    /// Invalid power management state.
    PowerState(u8),
    /// Invalid host connection speed.
    ConnectionSpeed(u8),
    /// Invalid thermal monitor state.
    SdhmState(u8),
    /// Invalid thermal monitor message type.
    SdhmMessageType(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Version(x) => write!(f, "invalid version {x:#x}"),
            Self::CommandType(x) => write!(f, "invalid command type {x:#x}"),
            Self::ProtectState(x) => write!(f, "invalid drive protect state {x:#x}"),
            Self::OfflinePending(x) => write!(f, "invalid offline pending status {x:#x}"),
            Self::Dlg2State(x) => write!(f, "invalid Data Lifeguard 2 state {x:#x}"),
            Self::State(x) => write!(f, "invalid drive state {x:#x}"),
            Self::TestType(x) => write!(f, "invalid test type {x:#x}"),
            Self::PowerState(x) => write!(f, "invalid power management state {x:#x}"),
            Self::ConnectionSpeed(x) => write!(f, "invalid host connection speed {x:#x}"),
            Self::SdhmState(x) => write!(f, "invalid thermal monitor state {x:#x}"),
            Self::SdhmMessageType(x) => {
                write!(f, "invalid thermal monitor message type {x:#x}")
            },
        }
    }
}

/// Last command type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CommandType {
    /// 28-bit ATA.
    Ata28 = 0,
    /// 48-bit ATA.
    Ata48 = 1,
    /// VUC key sector.
    VucKey = 2,
    /// VUC data transfer.
    VucData = 3,
    /// SCT key sector.
    SctKey = 4,
    /// SCT data transfer.
    SctData = 5,
    /// SCSI.
    Scsi = 6,
}

impl std::fmt::Display for CommandType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Ata28 => write!(f, "28-bit ATA"),
            Self::Ata48 => write!(f, "48-bit ATA"),
            Self::VucKey => write!(f, "VUC key sector"),
            Self::VucData => write!(f, "VUC data transfer"),
            Self::SctKey => write!(f, "SCT key sector"),
            Self::SctData => write!(f, "SCT data transfer"),
            Self::Scsi => write!(f, "SCSI"),
        }
    }
}

impl TryFrom<u8> for CommandType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[CommandType] = &[
            CommandType::Ata28,
            CommandType::Ata48,
            CommandType::VucKey,
            CommandType::VucData,
            CommandType::SctKey,
            CommandType::SctData,
            CommandType::Scsi,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::CommandType(value))
    }
}

/// Drive protect state field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtectState {
    /// Unlocked.
    Unlocked = 0,
    /// Lock count down.
    LockCountdown = 1,
    /// Locked.
    Locked = 2,
}

impl std::fmt::Display for ProtectState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unlocked => write!(f, "unlocked"),
            Self::LockCountdown => write!(f, "lock count down"),
            Self::Locked => write!(f, "locked"),
        }
    }
}

impl TryFrom<u8> for ProtectState {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[ProtectState] = &[
            ProtectState::Unlocked,
            ProtectState::LockCountdown,
            ProtectState::Locked,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::ProtectState(value))
    }
}

/// Offline pending status field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum OfflinePending {
    /// None pending.
    None = 0,
    /// Pending in normal mode.
    Normal = 1,
    /// Pending in aggressive mode.
    Aggressive = 2,
}

impl std::fmt::Display for OfflinePending {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Normal => write!(f, "normal mode"),
            Self::Aggressive => write!(f, "aggressive mode"),
        }
    }
}

impl TryFrom<u8> for OfflinePending {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[OfflinePending] = &[
            OfflinePending::None,
            OfflinePending::Normal,
            OfflinePending::Aggressive,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::OfflinePending(value))
    }
}

/// Data Lifeguard 2 state field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Dlg2State {
    /// Disabled due to failure or otherwise.
    Disabled = 0,
    /// Idle.
    Idle = 1,
    /// Refresh in progress.
    Pending = 2,
    /// Unknown.
    Unknown = 3,
}

impl std::fmt::Display for Dlg2State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Disabled => write!(f, "disabled"),
            Self::Idle => write!(f, "idle"),
            Self::Pending => write!(f, "refresh in progress"),
            Self::Unknown => write!(f, "unknown"),
        }
    }
}

impl TryFrom<u8> for Dlg2State {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[Dlg2State] = &[
            Dlg2State::Disabled,
            Dlg2State::Idle,
            Dlg2State::Pending,
            Dlg2State::Unknown,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::Dlg2State(value))
    }
}

/// Current drive state field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum State {
    /// Idle.
    Idle = 0,
    /// Standby.
    Standby = 1,
    /// Sleep.
    Sleep = 2,
    /// Background self-test in progress.
    SelfTest = 3,
    /// Background SMART off-line activity in progress.
    Offline = 4,
    /// Background VUC or SCT command in progress.
    BackgroundCommand = 5,
    /// Process Self-Test in progress.
    ProcessTest = 6,
    /// Background Data Lifeguard 2 in progress.
    Dlg2 = 7,
    /// Unknown.
    Unknown(u8),
}

impl std::fmt::Display for State {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Idle => write!(f, "idle"),
            Self::Standby => write!(f, "standby"),
            Self::Sleep => write!(f, "sleep"),
            Self::SelfTest => write!(f, "background self-test"),
            Self::Offline => write!(f, "background off-line activity"),
            Self::BackgroundCommand => write!(f, "background VUC or SCT command"),
            Self::ProcessTest => write!(f, "Process Self-Test"),
            Self::Dlg2 => write!(f, "background Data Lifeguard 2"),
            Self::Unknown(x) => write!(f, "unknown {x:#x}"),
        }
    }
}

impl From<State> for u8 {
    fn from(value: State) -> Self {
        match value {
            State::Unknown(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl TryFrom<u8> for State {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const CONST_VARIANTS: &[State] = &[
            State::Idle,
            State::Standby,
            State::Sleep,
            State::SelfTest,
            State::Offline,
            State::BackgroundCommand,
            State::ProcessTest,
            State::Dlg2,
        ];

        if let Some(x) = CONST_VARIANTS
            .iter()
            .find(|&&y| u8::from(y) == value)
            .copied()
        {
            return Ok(x);
        }

        Ok(Self::Unknown(value))
    }
}

/// Background test type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum TestType {
    /// Short self-test.
    Short = 1,
    /// Extended self-test.
    Extended = 2,
    /// Conveyance self-test.
    Conveyance = 3,
    /// Selective self-test.
    Selective = 4,
    /// Individual self-test subtest.
    Subtest = 6,
    /// SMART off-line immediate scan.
    OfflineScan = 7,
    /// SMART off-line immediate TA margining.
    OfflineTaMargining = 8,
    /// SMART auto off-line scan.
    AutoOfflineScan = 9,
    /// SMART auto off-line TA margining.
    AutoOfflineTaMargining = 10,
    /// SMART off-line selective scan.
    OfflineSelective = 11,
    /// Data Lifeguard 2 refresh.
    Dlg2Refresh = 12,
}

impl TestType {
    /// Parse from raw value.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[TestType] = &[
            TestType::Short,
            TestType::Extended,
            TestType::Conveyance,
            TestType::Selective,
            TestType::Subtest,
            TestType::OfflineScan,
            TestType::OfflineTaMargining,
            TestType::AutoOfflineScan,
            TestType::AutoOfflineTaMargining,
            TestType::OfflineSelective,
            TestType::Dlg2Refresh,
        ];

        if value == 0 {
            return Ok(None);
        }

        if let Some(x) = VARIANTS.iter().find(|&&x| x as u8 == value).copied() {
            return Ok(Some(x));
        }

        Err(Error::TestType(value))
    }
}

impl std::fmt::Display for TestType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Short => write!(f, "short self-test"),
            Self::Extended => write!(f, "extended self-test"),
            Self::Conveyance => write!(f, "conveyance self-test"),
            Self::Selective => write!(f, "selective self-test"),
            Self::Subtest => write!(f, "self-test subtest"),
            Self::OfflineScan => write!(f, "off-line scan"),
            Self::OfflineTaMargining => write!(f, "off-line TA margining"),
            Self::AutoOfflineScan => write!(f, "auto off-line scan"),
            Self::AutoOfflineTaMargining => write!(f, "auto off-line TA margining"),
            Self::OfflineSelective => write!(f, "off-line selective scan"),
            Self::Dlg2Refresh => write!(f, "Data Lifeguard 2 refresh"),
        }
    }
}

/// Power management state field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum PowerState {
    /// Active.
    Active = 0,
    /// Idle 1.
    Idle1 = 1,
    /// Idle 2.
    Idle2 = 2,
    /// Idle 3.
    Idle3 = 3,
    /// Spindle stopped.
    SpindleStopped = 4,
    /// Standby.
    Standby = 5,
}

impl std::fmt::Display for PowerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Active => write!(f, "active"),
            Self::Idle1 => write!(f, "idle 1"),
            Self::Idle2 => write!(f, "idle 2"),
            Self::Idle3 => write!(f, "idle 3"),
            Self::SpindleStopped => write!(f, "spindle stopped"),
            Self::Standby => write!(f, "standby"),
        }
    }
}

impl TryFrom<u8> for PowerState {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[PowerState] = &[
            PowerState::Active,
            PowerState::Idle1,
            PowerState::Idle2,
            PowerState::Idle3,
            PowerState::SpindleStopped,
            PowerState::Standby,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::PowerState(value))
    }
}

/// Thermal monitor state field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdhmState {
    /// Normal operational temperature.
    Normal = 0,
    /// Functionality or reliability compromised.
    ReliabilityCompromised = 1,
    /// Data integrity compromised.
    IntegrityCompromised = 2,
    /// Thermal emergency.
    Emergency = 3,
}

impl std::fmt::Display for SdhmState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Normal => write!(f, "normal operational temperature"),
            Self::ReliabilityCompromised => write!(f, "reliability compromised"),
            Self::IntegrityCompromised => write!(f, "data integrity compromised"),
            Self::Emergency => write!(f, "thermal emergency"),
        }
    }
}

impl TryFrom<u8> for SdhmState {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[SdhmState] = &[
            SdhmState::Normal,
            SdhmState::ReliabilityCompromised,
            SdhmState::IntegrityCompromised,
            SdhmState::Emergency,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::SdhmState(value))
    }
}

/// Host connection speed field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ConnectionSpeed {
    /// Not supported.
    Unsupported = 0,
    /// Parallel ATA.
    Pata = 1,
    /// SATA generation 1, 1.5 Gbps.
    SataGen1 = 2,
    /// SATA generation 2, 3.0 Gbps.
    SataGen2 = 3,
    /// SATA generation 3, 6.0 Gbps.
    SataGen3 = 4,
}

impl std::fmt::Display for ConnectionSpeed {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unsupported => write!(f, "not supported"),
            Self::Pata => write!(f, "PATA"),
            Self::SataGen1 => write!(f, "SATA gen1 (1.5 Gbps)"),
            Self::SataGen2 => write!(f, "SATA gen2 (3.0 Gbps)"),
            Self::SataGen3 => write!(f, "SATA gen3 (6.0 Gbps)"),
        }
    }
}

impl TryFrom<u8> for ConnectionSpeed {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[ConnectionSpeed] = &[
            ConnectionSpeed::Unsupported,
            ConnectionSpeed::Pata,
            ConnectionSpeed::SataGen1,
            ConnectionSpeed::SataGen2,
            ConnectionSpeed::SataGen3,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::ConnectionSpeed(value))
    }
}

/// Thermal monitor message type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SdhmMessageType {
    /// One byte message.
    OneByte = 0,
}

impl std::fmt::Display for SdhmMessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::OneByte => write!(f, "one byte message"),
        }
    }
}

impl TryFrom<u8> for SdhmMessageType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[SdhmMessageType] = &[SdhmMessageType::OneByte];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::SdhmMessageType(value))
    }
}

/// VUC status.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Status {
    /// VSC implementation version, minor.
    pub minor: u8,
    /// VSC implementation version, major.
    pub major: u8,
    /// Highest action code supported.
    pub max_action_code: u16,
    /// Type of the last command.
    pub last_command_type: CommandType,
    /// Command register of the last command.
    pub last_command: u8,
    /// Action code of the last command.
    pub last_action_code: u16,
    /// Feature register of the last command.
    pub last_feature: u16,
    /// ATA registers of the last command, error and status are output and the
    /// rest match input.
    pub registers: ResultRegisters,
    /// Device control register of the last command.
    pub device_control: u8,
    /// Last error code.
    pub error: Option<ErrorCode>,
    /// Pending transfer size in 512-byte sectors.
    pub sector_count: u64,
    /// Data returned in the ATA taskfile of the last VUC, bits 7:0 are count
    /// bits 7:0, bits 15:8 are LBA bits 7:0.
    pub task_file_data: u16,
    /// VUC command set enabled.
    pub vuc_enabled: bool,
    /// SMART enabled.
    pub smart_enabled: bool,
    /// Legacy native mode supported.
    pub legacy_native_mode: bool,
    /// Unstable, a firmware hard reset is recommended.
    pub unstable: bool,
    /// VUC transfer to the host pending.
    pub transfer_pending: bool,
    /// VUC command executing in the background.
    pub background_command: bool,
    /// Process Self-Test buffers allocated.
    pub pst_buffers_allocated: bool,
    /// PSV mode.
    pub psv_mode: bool,
    /// Debug Stop event since the last DRM clear.
    pub debug_stop_event: bool,
    /// Reset while busy event since the last DRM clear.
    pub reset_while_busy_event: bool,
    /// Powered up in standby and not yet spun up.
    pub power_up_in_standby: bool,
    /// Queue abort mode.
    pub queue_abort_mode: bool,
    /// Power-on reset by VUC occurred, cleared by the next status read.
    pub vuc_reset: bool,
    /// Power-on reset occurred, cleared by the next status read.
    pub power_on_reset: bool,
    /// Hardware reset occurred, cleared by the next status read.
    pub hardware_reset: bool,
    /// Software reset occurred, cleared by the next status read.
    pub software_reset: bool,
    /// Debug Stop occurred, cleared by the next status read.
    pub debug_stop: bool,
    /// Power Large Scale Integration reported an internal reset.
    pub plsi_internal_reset: bool,
    /// Power Large Scale Integration reported an external reset.
    pub plsi_external_reset: bool,
    /// SCT command executing in the background.
    pub background_sct_command: bool,
    /// Fatal write fault occurred.
    pub write_fault: bool,
    /// Drive protect state.
    pub protect_state: ProtectState,
    /// Encryption key loaded.
    pub encryption_key_loaded: bool,
    /// Sagem drive protect security erase pending.
    pub security_erase_pending: bool,
    /// Offline pending status.
    pub offline_pending: OfflinePending,
    /// Data Lifeguard 2 state.
    pub dlg2_state: Dlg2State,
    /// Low vibration mode detected.
    pub low_vibration_detected: bool,
    /// High vibration mode detected.
    pub high_vibration_detected: bool,
    /// Low vibration mode forced.
    pub low_vibration_forced: bool,
    /// High vibration mode forced.
    pub high_vibration_forced: bool,
    /// Current self-test or offline activity checkpoint position value.
    pub checkpoint: u8,
    /// Advanced Power Management level from identify device word 91 bits 7:0.
    pub apm_level: u8,
    /// Secondary error code.
    pub secondary_error: Option<ErrorCode>,
    /// Initiator of the current error code, unknown specific meaning.
    pub initiator_code: u32,
    /// Current drive state.
    pub state: State,
    /// Background self-test or off-line activity is active.
    pub test_active: bool,
    /// Background test type.
    pub test_type: Option<TestType>,
    /// Power management state.
    pub power_state: PowerState,
    /// Background scan current LBA.
    pub lba: u64,
    /// Current track offset.
    pub track_offset: i16,
    /// Virtual cylinder of the active track.
    pub virtual_cylinder: i32,
    /// Virtual head of the active track.
    pub virtual_head: u8,
    /// Current data zone.
    pub zone: u8,
    /// Counts from servo to the start of the wedge.
    pub wedge_start: u16,
    /// Wedge size in bytes.
    pub wedge_size: u16,
    /// Drive average temperature in celsius.
    pub drive_temperature: i16,
    /// SoC die junction temperature in celsius.
    pub junction_temperature: i16,
    /// Coil temperature, unknown specific meaning.
    pub coil_temperature: u16,
    /// Servo status, unknown specific meaning.
    pub servo_status: u16,
    /// Servo error code, unknown specific meaning.
    pub servo_error: u16,
    /// ECC seed cylinder of the last read.
    pub seed_cylinder: u32,
    /// ECC seed head of the last read.
    pub seed_head: u8,
    /// ECC seed sector of the last read.
    pub seed_sector: u16,
    /// Sector skew of the current track.
    pub sector_skew: u16,
    /// Wedge skew of the current track.
    pub wedge_skew: u16,
    /// Debug Stop code of the last firmware halt.
    pub debug_stop_code: Option<DebugStopCode>,
    /// Debug Stop parameter.
    pub debug_stop_parameter: u32,
    /// Progress of current background operation, in units of 0.1%.
    pub background_progress: u16,
    /// Head-stack flex/preamp temperature in celsius.
    pub flex_temperature: i16,
    /// LBA of the write fault.
    pub write_fault_lba: u64,
    /// Current track offset, in servo units.
    pub track_offset_servo: i16,
    /// Servo cylinder of the active servo track.
    pub servo_cylinder: i32,
    /// Data-to-servo conversion offset of the active servo track.
    pub data_to_servo_offset: u16,
    /// Host connection speed.
    pub connection_speed: ConnectionSpeed,
    /// SDHM (unknown meaning) state, for thermal monitoring.
    pub sdhm_state: SdhmState,
    /// Thermal monitor offset past the state threshold.
    pub sdhm_offset: u8,
    /// Thermal monitor message type.
    pub sdhm_message_type: SdhmMessageType,
    /// UCCM (unknown meaning) config command feature ID.
    pub uccm_feature_id: u16,
    /// UCCM config command parameter ID.
    pub uccm_parameter_id: u16,
    /// UCCM config command error type.
    pub uccm_error_type: u8,
    /// Pressure sensor reference value, unknown specific meaning.
    pub pressure_reference: u16,
    /// Delta pressure, unknown specific meaning.
    pub pressure_delta: u16,
    /// File ID of the Process Test Module executing.
    pub ptm_id: u16,
    /// Test ID of the Process Test Module executing.
    pub test_id: u16,
    /// Action code of the VSC command executing.
    pub action_code: u16,
    /// Extended error of the last VSC command issued by Process Self-Test.
    pub pst_error: Option<ErrorCode>,
    /// Extended error of the last VSC command.
    pub last_vsc_error: Option<ErrorCode>,
    /// Result ID of the failing low-level Process Test Module command.
    pub ptm_command_id: u16,
    /// Extended error of the failing low-level Process Test Module command.
    pub ptm_error: Option<ErrorCode>,
    /// Process Test Module request for host action, unknown specific meaning.
    pub ptm_to_host: u16,
    /// ECM stack host GEC.
    pub ecm_host_gec: Option<ErrorCode>,
    /// ECM stack command ID.
    pub ecm_command_id: u32,
    /// ECM stack drive principal GEC.
    pub ecm_drive_principal_gec: Option<ErrorCode>,
    /// ECM stack drive supplemental GEC.
    pub ecm_drive_supplemental_gec: Option<ErrorCode>,
    /// Last float cylinder, unknown meaning.
    pub last_float_cylinder: u32,
    /// DTPL checkpoint.
    pub dtpl_checkpoint: u64,
    /// Data Lifeguard 1 checkpoint.
    pub dlg1_checkpoint: u64,
    /// DTPL state.
    pub dtpl_state: u8,
    /// Data Lifeguard 1 state.
    pub dlg1_state: u8,
    /// DTPL scan head.
    pub dtpl_scan_head: u8,
    /// DTPL scan zip index.
    pub dtpl_scan_zip_index: u32,
}

impl Status {
    /// Size in bytes.
    pub(crate) const SIZE: usize = SECTOR_SIZE;
}

impl TryFrom<&[u8; Self::SIZE]> for Status {
    type Error = Error;

    fn try_from(value: &[u8; Self::SIZE]) -> Result<Self, Error> {
        const VERSION: u16 = 0x1;

        let version = u16::from_le_bytes([value[0], value[1]]);
        if version != VERSION {
            return Err(Error::Version(version));
        }

        let minor = value[2];
        let major = value[3];
        let max_action_code = u16::from_le_bytes([value[4], value[5]]);
        let last_command_type = value[6].try_into()?;
        let last_command = value[7];
        let last_action_code = u16::from_le_bytes([value[8], value[9]]);
        let last_feature = u16::from_le_bytes([value[10], value[11]]);

        let registers = ResultRegisters {
            error: value[22],
            count: u16::from_le_bytes([value[12], value[13]]),
            lba: u64::from_le_bytes([
                value[14], value[16], value[17], value[15], value[18], value[19], 0, 0,
            ]),
            device: value[21],
            status: StatusRegister::from(value[23]),
        };

        let device_control = value[20];
        let error = ErrorCode::parse(u16::from_le_bytes([value[24], value[25]]).into());
        let sector_count = u64::from_le_bytes([
            value[26], value[27], value[28], value[29], value[30], value[31], 0, 0,
        ]);
        let task_file_data = u16::from_le_bytes([value[32], value[33]]);

        let status_flags_1 = u32::from_le_bytes([value[34], value[35], value[36], value[37]]);
        let vuc_enabled = status_flags_1 & 1 != 0;
        let smart_enabled = status_flags_1 & (1 << 1) != 0;
        let legacy_native_mode = status_flags_1 & (1 << 2) != 0;
        let unstable = status_flags_1 & (1 << 3) != 0;
        let transfer_pending = status_flags_1 & (1 << 4) != 0;
        let background_command = status_flags_1 & (1 << 5) != 0;
        let pst_buffers_allocated = status_flags_1 & (1 << 6) != 0;
        let psv_mode = status_flags_1 & (1 << 8) != 0;
        let debug_stop_event = status_flags_1 & (1 << 9) != 0;
        let reset_while_busy_event = status_flags_1 & (1 << 10) != 0;
        let power_up_in_standby = status_flags_1 & (1 << 11) != 0;
        let unsigned_temperature = status_flags_1 & (1 << 12) != 0;
        let queue_abort_mode = status_flags_1 & (1 << 13) != 0;
        let vuc_reset = status_flags_1 & (1 << 16) != 0;
        let power_on_reset = status_flags_1 & (1 << 17) != 0;
        let hardware_reset = status_flags_1 & (1 << 18) != 0;
        let software_reset = status_flags_1 & (1 << 19) != 0;
        let debug_stop = status_flags_1 & (1 << 20) != 0;
        let plsi_internal_reset = status_flags_1 & (1 << 21) != 0;
        let plsi_external_reset = status_flags_1 & (1 << 22) != 0;
        let background_sct_command = status_flags_1 & (1 << 24) != 0;
        let write_fault = status_flags_1 & (1 << 25) != 0;
        let protect_state = (((status_flags_1 >> 26) & 0b11) as u8).try_into()?;
        let encryption_key_loaded = status_flags_1 & (1 << 28) != 0;
        let security_erase_pending = status_flags_1 & (1 << 29) != 0;
        let offline_pending = (((status_flags_1 >> 30) & 0b11) as u8).try_into()?;

        let status_flags_2 = u32::from_le_bytes([value[38], value[39], value[40], value[41]]);
        let dlg2_state = ((status_flags_2 & 0b11) as u8).try_into()?;
        let low_vibration_detected = status_flags_2 & (1 << 4) != 0;
        let high_vibration_detected = status_flags_2 & (1 << 5) != 0;
        let low_vibration_forced = status_flags_2 & (1 << 6) != 0;
        let high_vibration_forced = status_flags_2 & (1 << 7) != 0;

        let checkpoint = value[46];
        let apm_level = value[47];
        let secondary_error = ErrorCode::parse(u16::from_le_bytes([value[48], value[49]]).into());
        let initiator_code = u32::from_le_bytes([value[50], value[51], value[52], value[53]]);
        let state = value[54].try_into()?;

        let state_2 = value[55];
        let test_active = state_2 & 1 != 0;
        let test_type = TestType::parse((state_2 >> 1) & 0b1111)?;
        let power_state = ((state_2 >> 5) & 0b111).try_into()?;

        let lba = u64::from_le_bytes([
            value[56], value[57], value[58], value[59], value[60], value[61], 0, 0,
        ]);
        let track_offset = i16::from_le_bytes([value[62], value[63]]);
        let virtual_cylinder = i32::from_le_bytes([value[64], value[65], value[66], value[67]]);
        let virtual_head = value[68];
        let zone = value[69];
        let wedge_start = u16::from_le_bytes([value[70], value[71]]);
        let wedge_size = u16::from_le_bytes([value[72], value[73]]);

        let parse_temperature = |x: u8| {
            if unsigned_temperature {
                i16::from(x)
            } else {
                i16::from(x.cast_signed())
            }
        };

        let drive_temperature = parse_temperature(value[74]);
        let junction_temperature = parse_temperature(value[75]);
        let coil_temperature = u16::from_le_bytes([value[76], value[77]]);
        let servo_status = u16::from_le_bytes([value[78], value[79]]);
        let servo_error = u16::from_le_bytes([value[80], value[81]]);
        let seed_cylinder = u32::from_le_bytes([value[84], value[85], value[86], 0]);
        let seed_head = value[87];
        let seed_sector = u16::from_le_bytes([value[88], value[89]]);
        let sector_skew = u16::from_le_bytes([value[90], value[91]]);
        let wedge_skew = u16::from_le_bytes([value[92], value[93]]);
        let debug_stop_code = DebugStopCode::parse(u32::from_le_bytes([
            value[96], value[97], value[98], value[99],
        ]));
        let debug_stop_parameter =
            u32::from_le_bytes([value[100], value[101], value[102], value[103]]);
        let background_progress = u16::from_le_bytes([value[104], value[105]]);
        let flex_temperature = parse_temperature(value[106]);
        let write_fault_lba = u64::from_le_bytes([
            value[108], value[109], value[110], value[111], value[112], value[113], 0, 0,
        ]);
        let track_offset_servo = i16::from_le_bytes([value[114], value[115]]);
        let servo_cylinder = i32::from_le_bytes([value[116], value[117], value[118], value[119]]);
        let data_to_servo_offset = u16::from_le_bytes([value[120], value[121]]);
        let connection_speed = value[122].try_into()?;
        let sdhm = value[123];
        let sdhm_state = (sdhm & 0b11).try_into()?;
        let sdhm_offset = (sdhm >> 2) & 0b111;
        let sdhm_message_type = ((sdhm >> 5) & 0b11).try_into()?;
        let uccm_feature_id = u16::from_le_bytes([value[124], value[125]]);
        let uccm_parameter_id = u16::from_le_bytes([value[126], value[127]]);
        let uccm_error_type = value[128];
        let pressure_reference = u16::from_le_bytes([value[134], value[135]]);
        let pressure_delta = u16::from_le_bytes([value[136], value[137]]);
        let ptm_id = u16::from_le_bytes([value[400], value[401]]);
        let test_id = u16::from_le_bytes([value[402], value[403]]);
        let action_code = u16::from_le_bytes([value[404], value[405]]);
        let pst_error = ErrorCode::parse(u16::from_le_bytes([value[406], value[407]]).into());
        let last_vsc_error = ErrorCode::parse(u16::from_le_bytes([value[428], value[429]]).into());
        let ptm_command_id = u16::from_le_bytes([value[432], value[433]]);
        let ptm_error = ErrorCode::parse(u16::from_le_bytes([value[434], value[435]]).into());
        let ptm_to_host = u16::from_le_bytes([value[440], value[441]]);
        let ecm_host_gec = ErrorCode::parse(u32::from_le_bytes([
            value[464], value[465], value[466], value[467],
        ]));
        let ecm_command_id = u32::from_le_bytes([value[468], value[469], value[470], value[471]]);
        let ecm_drive_principal_gec = ErrorCode::parse(u32::from_le_bytes([
            value[472], value[473], value[474], value[475],
        ]));
        let ecm_drive_supplemental_gec = ErrorCode::parse(u32::from_le_bytes([
            value[476], value[477], value[478], value[479],
        ]));
        let last_float_cylinder =
            u32::from_le_bytes([value[480], value[481], value[482], value[483]]);
        let dtpl_checkpoint = u64::from_le_bytes([
            value[484], value[485], value[486], value[487], value[488], value[489], 0, 0,
        ]);
        let dlg1_checkpoint = u64::from_le_bytes([
            value[490], value[491], value[492], value[493], value[494], value[495], 0, 0,
        ]);
        let dtpl_state = value[496];
        let dlg1_state = value[497];
        let dtpl_scan_head = value[498];
        let dtpl_scan_zip_index =
            u32::from_le_bytes([value[499], value[500], value[501], value[502]]);

        Ok(Self {
            minor,
            major,
            max_action_code,
            last_command_type,
            last_command,
            last_action_code,
            last_feature,
            registers,
            device_control,
            error,
            sector_count,
            task_file_data,
            vuc_enabled,
            smart_enabled,
            legacy_native_mode,
            unstable,
            transfer_pending,
            background_command,
            pst_buffers_allocated,
            psv_mode,
            debug_stop_event,
            reset_while_busy_event,
            power_up_in_standby,

            queue_abort_mode,
            vuc_reset,
            power_on_reset,
            hardware_reset,
            software_reset,
            debug_stop,
            plsi_internal_reset,
            plsi_external_reset,
            background_sct_command,
            write_fault,
            protect_state,
            encryption_key_loaded,
            security_erase_pending,
            offline_pending,
            dlg2_state,
            low_vibration_detected,
            high_vibration_detected,
            low_vibration_forced,
            high_vibration_forced,
            checkpoint,
            apm_level,
            secondary_error,
            initiator_code,
            state,
            test_active,
            test_type,
            power_state,
            lba,
            track_offset,
            virtual_cylinder,
            virtual_head,
            zone,
            wedge_start,
            wedge_size,
            drive_temperature,
            junction_temperature,
            coil_temperature,
            servo_status,
            servo_error,
            seed_cylinder,
            seed_head,
            seed_sector,
            sector_skew,
            wedge_skew,
            debug_stop_code,
            debug_stop_parameter,
            background_progress,
            flex_temperature,
            write_fault_lba,
            track_offset_servo,
            servo_cylinder,
            data_to_servo_offset,
            connection_speed,
            sdhm_state,
            sdhm_offset,
            sdhm_message_type,
            uccm_feature_id,
            uccm_parameter_id,
            uccm_error_type,
            pressure_reference,
            pressure_delta,
            ptm_id,
            test_id,
            action_code,
            pst_error,
            last_vsc_error,
            ptm_command_id,
            ptm_error,
            ptm_to_host,
            ecm_host_gec,
            ecm_command_id,
            ecm_drive_principal_gec,
            ecm_drive_supplemental_gec,
            last_float_cylinder,
            dtpl_checkpoint,
            dlg1_checkpoint,
            dtpl_state,
            dlg1_state,
            dtpl_scan_head,
            dtpl_scan_zip_index,
        })
    }
}
