//! ATA commands.

/// SMART signature required in the LBA register.
pub const SMART_KEY_LBA: u64 = 0xC24F << 8;

/// ATA command opcodes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Command {
    /// No-op.
    Nop = 0x0,
    /// TRIM/deallocate LBA ranges.
    DataSetManagement = 0x6,
    /// 64-bit-entry variant of DATA SET MANAGEMENT.
    DataSetManagementXl = 0x7,
    /// Return saved sense data for the most recent NCQ error (48-bit).
    RequestSenseDataExt = 0xB,
    /// Report status of physical storage elements (e.g. depopulated heads).
    GetPhysicalElementStatus = 0x12,
    /// 28-bit PIO read of one or more sectors.
    ReadSectors = 0x20,
    /// 48-bit PIO read of one or more sectors.
    ReadSectorsExt = 0x24,
    /// 48-bit DMA read.
    ReadDmaExt = 0x25,
    /// 48-bit streaming DMA read (bounded-latency media streaming).
    ReadStreamDmaExt = 0x2A,
    /// 48-bit streaming PIO read.
    ReadStreamExt = 0x2B,
    /// 48-bit PIO read of a SMART/GPL log page.
    ReadLogExt = 0x2F,
    /// 28-bit PIO write of one or more sectors.
    WriteSectors = 0x30,
    /// 48-bit PIO write of one or more sectors.
    WriteSectorsExt = 0x34,
    /// 48-bit DMA write.
    WriteDmaExt = 0x35,
    /// 48-bit streaming DMA write.
    WriteStreamDmaExt = 0x3A,
    /// 48-bit streaming PIO write.
    WriteStreamExt = 0x3B,
    /// 48-bit DMA write with Forced Unit Access (bypass write cache).
    WriteDmaFuaExt = 0x3D,
    /// 48-bit PIO write of a SMART/GPL log page.
    WriteLogExt = 0x3F,
    /// 28-bit read-verify (read without data transfer to host).
    ReadVerifySectors = 0x40,
    /// 48-bit read-verify.
    ReadVerifySectorsExt = 0x42,
    /// 48-bit fill of an LBA range with zeros.
    ZeroExt = 0x44,
    /// 48-bit mark LBAs as containing an uncorrectable (pseudo-)error.
    WriteUncorrectableExt = 0x45,
    /// 48-bit DMA read of a SMART/GPL log page.
    ReadLogDmaExt = 0x47,
    /// Zoned-device (ZAC) management input subcommands.
    ZacManagementIn = 0x4A,
    /// Configure a media stream for bounded-latency streaming.
    ConfigureStream = 0x51,
    /// 48-bit DMA write of a SMART/GPL log page.
    WriteLogDmaExt = 0x57,
    /// Trusted Computing non-data command (security protocol).
    TrustedNonData = 0x5B,
    /// Trusted Computing PIO receive (e.g. TCG/Opal).
    TrustedReceive = 0x5C,
    /// Trusted Computing DMA receive.
    TrustedReceiveDma = 0x5D,
    /// Trusted Computing PIO send.
    TrustedSend = 0x5E,
    /// Trusted Computing DMA send.
    TrustedSendDma = 0x5F,
    /// Queued (NCQ) read; FPDMA = First-Party DMA.
    ReadFpdmaQueued = 0x60,
    /// Queued (NCQ) write.
    WriteFpdmaQueued = 0x61,
    /// NCQ non-data command (carries a subcommand, no data transfer).
    NcqNonData = 0x63,
    /// NCQ send (e.g. queued DATA SET MANAGEMENT / TRIM).
    SendFpdmaQueued = 0x64,
    /// NCQ receive (e.g. queued REQUEST SENSE DATA / READ LOG).
    ReceiveFpdmaQueued = 0x65,
    /// 48-bit set the device's internal date/time.
    SetDateTimeExt = 0x77,
    /// Accessible Max Address Configuration.
    AccessibleMaxAddressConfiguration = 0x78,
    /// Depopulate a physical element and shrink reported capacity.
    RemoveElementAndTruncate = 0x7C,
    /// Run the device's built-in self-diagnostic.
    ExecuteDeviceDiagnostic = 0x90,
    /// Update firmware with PIO transfer.
    DownloadMicrocode = 0x92,
    /// Update firmware with DMA transfer.
    DownloadMicrocodeDma = 0x93,
    /// Zoned-device (ZAC) management output subcommands.
    ZacManagementOut = 0x9F,
    /// SMART operations.
    Smart = 0xB0,
    /// Select a logical/physical sector size configuration.
    SetSectorConfigurationExt = 0xB2,
    /// Securely erase device.
    SanitizeDevice = 0xB4,
    /// 28-bit DMA read.
    ReadDma = 0xC8,
    /// 28-bit DMA write.
    WriteDma = 0xCA,
    /// Enter standby immediately.
    StandbyImmediate = 0xE0,
    /// Enter idle power state immediately.
    IdleImmediate = 0xE1,
    /// Enter standby (with standby timer).
    Standby = 0xE2,
    /// Enter idle (with standby timer).
    Idle = 0xE3,
    /// PIO read from buffer.
    ReadBuffer = 0xE4,
    /// Report the current power mode.
    CheckPowerMode = 0xE5,
    /// Enter sleep (lowest power; requires reset to wake).
    Sleep = 0xE6,
    /// Flush write cache (28-bit).
    FlushCache = 0xE7,
    /// PIO write to buffer.
    WriteBuffer = 0xE8,
    /// DMA read from buffer.
    ReadBufferDma = 0xE9,
    /// Flush write cache (48-bit).
    FlushCacheExt = 0xEA,
    /// DMA write to buffer.
    WriteBufferDma = 0xEB,
    /// Get device information.
    IdentifyDevice = 0xEC,
    /// Enable/disable features.
    SetFeatures = 0xEF,
    /// Set security password.
    SecuritySetPassword = 0xF1,
    /// Unlock with the security password.
    SecurityUnlock = 0xF2,
    /// Prepare for secure erase.
    SecurityErasePrepare = 0xF3,
    /// Secure-erase the entire user data area.
    SecurityEraseUnit = 0xF4,
    /// Freeze the security state until the next power cycle.
    SecurityFreezeLock = 0xF5,
    /// Disable the ATA security password.
    SecurityDisablePassword = 0xF6,
    /// Vendor-specific.
    VendorSpecific(u8),
}

impl From<Command> for u8 {
    fn from(value: Command) -> Self {
        match value {
            Command::VendorSpecific(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

impl From<u8> for Command {
    fn from(value: u8) -> Self {
        const CONST_VARIANTS: &[Command] = &[
            Command::Nop,
            Command::DataSetManagement,
            Command::DataSetManagementXl,
            Command::RequestSenseDataExt,
            Command::GetPhysicalElementStatus,
            Command::ReadSectors,
            Command::ReadSectorsExt,
            Command::ReadDmaExt,
            Command::ReadStreamDmaExt,
            Command::ReadStreamExt,
            Command::ReadLogExt,
            Command::WriteSectors,
            Command::WriteSectorsExt,
            Command::WriteDmaExt,
            Command::WriteStreamDmaExt,
            Command::WriteStreamExt,
            Command::WriteDmaFuaExt,
            Command::WriteLogExt,
            Command::ReadVerifySectors,
            Command::ReadVerifySectorsExt,
            Command::ZeroExt,
            Command::WriteUncorrectableExt,
            Command::ReadLogDmaExt,
            Command::ZacManagementIn,
            Command::ConfigureStream,
            Command::WriteLogDmaExt,
            Command::TrustedNonData,
            Command::TrustedReceive,
            Command::TrustedReceiveDma,
            Command::TrustedSend,
            Command::TrustedSendDma,
            Command::ReadFpdmaQueued,
            Command::WriteFpdmaQueued,
            Command::NcqNonData,
            Command::SendFpdmaQueued,
            Command::ReceiveFpdmaQueued,
            Command::SetDateTimeExt,
            Command::AccessibleMaxAddressConfiguration,
            Command::RemoveElementAndTruncate,
            Command::ExecuteDeviceDiagnostic,
            Command::DownloadMicrocode,
            Command::DownloadMicrocodeDma,
            Command::ZacManagementOut,
            Command::Smart,
            Command::SetSectorConfigurationExt,
            Command::SanitizeDevice,
            Command::ReadDma,
            Command::WriteDma,
            Command::StandbyImmediate,
            Command::IdleImmediate,
            Command::Standby,
            Command::Idle,
            Command::ReadBuffer,
            Command::CheckPowerMode,
            Command::Sleep,
            Command::FlushCache,
            Command::WriteBuffer,
            Command::ReadBufferDma,
            Command::FlushCacheExt,
            Command::WriteBufferDma,
            Command::IdentifyDevice,
            Command::SetFeatures,
            Command::SecuritySetPassword,
            Command::SecurityUnlock,
            Command::SecurityErasePrepare,
            Command::SecurityEraseUnit,
            Command::SecurityFreezeLock,
            Command::SecurityDisablePassword,
        ];

        if let Some(x) = CONST_VARIANTS
            .iter()
            .find(|&&y| u8::from(y) == value)
            .copied()
        {
            return x;
        }

        Self::VendorSpecific(value)
    }
}

impl std::fmt::Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Nop => write!(f, "NOP"),
            Self::DataSetManagement => write!(f, "DATA SET MANAGEMENT"),
            Self::DataSetManagementXl => write!(f, "DATA SET MANAGEMENT XL"),
            Self::RequestSenseDataExt => write!(f, "REQUEST SENSE DATA EXT"),
            Self::GetPhysicalElementStatus => write!(f, "GET PHYSICAL ELEMENT STATUS"),
            Self::ReadSectors => write!(f, "READ SECTOR(S)"),
            Self::ReadSectorsExt => write!(f, "READ SECTOR(S) EXT"),
            Self::ReadDmaExt => write!(f, "READ DMA EXT"),
            Self::ReadStreamDmaExt => write!(f, "READ STREAM DMA EXT"),
            Self::ReadStreamExt => write!(f, "READ STREAM EXT"),
            Self::ReadLogExt => write!(f, "READ LOG EXT"),
            Self::WriteSectors => write!(f, "WRITE SECTOR(S)"),
            Self::WriteSectorsExt => write!(f, "WRITE SECTOR(S) EXT"),
            Self::WriteDmaExt => write!(f, "WRITE DMA EXT"),
            Self::WriteStreamDmaExt => write!(f, "WRITE STREAM DMA EXT"),
            Self::WriteStreamExt => write!(f, "WRITE STREAM EXT"),
            Self::WriteDmaFuaExt => write!(f, "WRITE DMA FUA EXT"),
            Self::WriteLogExt => write!(f, "WRITE LOG EXT"),
            Self::ReadVerifySectors => write!(f, "READ VERIFY SECTOR(S)"),
            Self::ReadVerifySectorsExt => write!(f, "READ VERIFY SECTOR(S) EXT"),
            Self::ZeroExt => write!(f, "ZERO EXT"),
            Self::WriteUncorrectableExt => write!(f, "WRITE UNCORRECTABLE EXT"),
            Self::ReadLogDmaExt => write!(f, "READ LOG DMA EXT"),
            Self::ZacManagementIn => write!(f, "ZAC Management In"),
            Self::ConfigureStream => write!(f, "CONFIGURE STREAM"),
            Self::WriteLogDmaExt => write!(f, "WRITE LOG DMA EXT"),
            Self::TrustedNonData => write!(f, "TRUSTED NON-DATA"),
            Self::TrustedReceive => write!(f, "TRUSTED RECEIVE"),
            Self::TrustedReceiveDma => write!(f, "TRUSTED RECEIVE DMA"),
            Self::TrustedSend => write!(f, "TRUSTED SEND"),
            Self::TrustedSendDma => write!(f, "TRUSTED SEND DMA"),
            Self::ReadFpdmaQueued => write!(f, "READ FPDMA QUEUED"),
            Self::WriteFpdmaQueued => write!(f, "WRITE FPDMA QUEUED"),
            Self::NcqNonData => write!(f, "NCQ NON-DATA"),
            Self::SendFpdmaQueued => write!(f, "SEND FPDMA QUEUED"),
            Self::ReceiveFpdmaQueued => write!(f, "RECEIVE FPDMA QUEUED"),
            Self::SetDateTimeExt => write!(f, "SET DATE & TIME EXT"),
            Self::AccessibleMaxAddressConfiguration => {
                write!(f, "ACCESSIBLE MAX ADDRESS CONFIGURATION")
            },
            Self::RemoveElementAndTruncate => write!(f, "REMOVE ELEMENT AND TRUNCATE"),
            Self::ExecuteDeviceDiagnostic => write!(f, "EXECUTE DEVICE DIAGNOSTIC"),
            Self::DownloadMicrocode => write!(f, "DOWNLOAD MICROCODE"),
            Self::DownloadMicrocodeDma => write!(f, "DOWNLOAD MICROCODE DMA"),
            Self::ZacManagementOut => write!(f, "ZAC Management Out"),
            Self::Smart => write!(f, "SMART"),
            Self::SetSectorConfigurationExt => write!(f, "SET SECTOR CONFIGURATION EXT"),
            Self::SanitizeDevice => write!(f, "Sanitize Device"),
            Self::ReadDma => write!(f, "READ DMA"),
            Self::WriteDma => write!(f, "WRITE DMA"),
            Self::StandbyImmediate => write!(f, "STANDBY IMMEDIATE"),
            Self::IdleImmediate => write!(f, "IDLE IMMEDIATE"),
            Self::Standby => write!(f, "STANDBY"),
            Self::Idle => write!(f, "IDLE"),
            Self::ReadBuffer => write!(f, "READ BUFFER"),
            Self::CheckPowerMode => write!(f, "CHECK POWER MODE"),
            Self::Sleep => write!(f, "SLEEP"),
            Self::FlushCache => write!(f, "FLUSH CACHE"),
            Self::WriteBuffer => write!(f, "WRITE BUFFER"),
            Self::ReadBufferDma => write!(f, "READ BUFFER DMA"),
            Self::FlushCacheExt => write!(f, "FLUSH CACHE EXT"),
            Self::WriteBufferDma => write!(f, "WRITE BUFFER DMA"),
            Self::IdentifyDevice => write!(f, "IDENTIFY DEVICE"),
            Self::SetFeatures => write!(f, "SET FEATURES"),
            Self::SecuritySetPassword => write!(f, "SECURITY SET PASSWORD"),
            Self::SecurityUnlock => write!(f, "SECURITY UNLOCK"),
            Self::SecurityErasePrepare => write!(f, "SECURITY ERASE PREPARE"),
            Self::SecurityEraseUnit => write!(f, "SECURITY ERASE UNIT"),
            Self::SecurityFreezeLock => write!(f, "SECURITY FREEZE LOCK"),
            Self::SecurityDisablePassword => write!(f, "SECURITY DISABLE PASSWORD"),
            Self::VendorSpecific(x) => write!(f, "vendor-specific {x:#x}"),
        }
    }
}

/// Command register set.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CommandRegisters {
    /// Feature.
    pub feature: u16,
    /// Count.
    pub count: u16,
    /// Logical Block Address.
    pub lba: u64,
    /// Device.
    pub device: u8,
    /// Command opcode.
    pub command: Command,
}

impl Default for CommandRegisters {
    fn default() -> Self {
        Self {
            feature: 0,
            count: 0,
            lba: 0,
            device: 0,
            command: Command::VendorSpecific(0),
        }
    }
}

impl std::fmt::Display for CommandRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} (feature: {:#x}, count: {:#x}, LBA: {:#x}, device: {:#x})",
            self.command, self.feature, self.count, self.lba, self.device
        )
    }
}

/// Status register in command result register set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct StatusRegister(u8);

impl StatusRegister {
    /// STATUS DEVICE FAULT bit.
    pub(crate) const DEVICE_FAULT_MASK: u8 = 1 << 5;
    /// STATUS ALIGNMENT ERROR bit.
    pub(crate) const ALIGNMENT_ERROR_MASK: u8 = 1 << 2;
    /// STATUS SENSE DATA AVAILABLE bit.
    pub(crate) const SENSE_DATA_AVAILABLE_MASK: u8 = 1 << 1;
    /// STATUS ERROR bit.
    pub(crate) const ERROR_MASK: u8 = 1 << 0;

    /// Device-fault bit set.
    pub(crate) fn device_fault(self) -> bool {
        (self.0 & Self::DEVICE_FAULT_MASK) != 0
    }

    /// Alignment-error bit set.
    pub(crate) fn alignment_error(self) -> bool {
        (self.0 & Self::ALIGNMENT_ERROR_MASK) != 0
    }

    /// Sense data available bit set.
    pub(crate) fn sense_data_available(self) -> bool {
        (self.0 & Self::SENSE_DATA_AVAILABLE_MASK) != 0
    }

    /// Error bit set.
    pub(crate) fn error(self) -> bool {
        (self.0 & Self::ERROR_MASK) != 0
    }
}

impl From<u8> for StatusRegister {
    fn from(value: u8) -> Self {
        Self(value)
    }
}

impl From<StatusRegister> for u8 {
    fn from(value: StatusRegister) -> Self {
        value.0
    }
}

/// Command result register set.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ResultRegisters {
    /// Error.
    pub error: u8,
    /// Count.
    pub count: u16,
    /// Logical Block Address.
    pub lba: u64,
    /// Device.
    pub device: u8,
    /// Status.
    pub status: StatusRegister,
}

impl std::fmt::Display for ResultRegisters {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "(error: {:#x}, count: {:#x}, LBA: {:#x}, device: {:#x}, status: {:#x})",
            self.error, self.count, self.lba, self.device, self.status.0
        )
    }
}

/// DOWNLOAD MICROCODE subcommand.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DlmcSubcommand {
    /// Download with offsets and save microcode for immediate and future use.
    Segmented = 3,
    /// Download and save for immediate and future use.
    Full = 7,
}
