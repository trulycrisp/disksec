//! Identify device result parsing.

use std::ops::RangeInclusive;

use crate::{output, protocol::ata::SECTOR_SIZE};

/// Identify device error.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid string.
    String(Box<[u8]>),
    /// Invalid ZONED capabilities field.
    ZonedCapabilities(u8),
    /// Invalid logical sector size field.
    LogicalSectorSize(u64),
    /// Invalid World Wide Name Network Address Authority value.
    WwnNaa(u8),
    /// Invalid nominal form factor value.
    FormFactor(u8),
    /// Invalid nominal media rotation rate value.
    RotationRate(u16),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::String(x) => write!(f, "invalid string {}", output::format_bytes_hex(x)),
            Self::ZonedCapabilities(x) => {
                write!(f, "invalid zoned capabilities {x:#x}")
            },
            Self::LogicalSectorSize(x) => {
                write!(f, "invalid logical sector size {x}")
            },
            Self::WwnNaa(x) => {
                write!(f, "invalid WWN NAA {x:#x}")
            },
            Self::FormFactor(x) => write!(f, "invalid form factor {x:#x}"),
            Self::RotationRate(x) => {
                write!(f, "invalid rotation rate {x:#x}")
            },
        }
    }
}

/// Swap the bytes of every 16-bit word.
fn swap_word_bytes(data: &[u8]) -> Box<[u8]> {
    const WORD_SIZE: usize = size_of::<u16>();

    data.as_chunks::<WORD_SIZE>()
        .0
        .iter()
        .flat_map(|c| [c[1], c[0]])
        .collect()
}

/// Parse a string field.
pub fn parse_string(data: &[u8]) -> Result<String, Error> {
    let error = || Error::String(data.into());

    // Identify strings must have a word-aligned size
    if !data.len().is_multiple_of(size_of::<u16>()) {
        return Err(error());
    }

    let swapped_data = swap_word_bytes(data);
    let Ok(mut string) = str::from_utf8(&swapped_data) else {
        return Err(error());
    };

    // Trim null terminator and trailing spaces. Null isn't allowed in ACS, but
    // some old drives use it
    string = string.split('\0').next().unwrap_or("").trim_end();

    if !string.chars().all(|x| x == ' ' || x.is_ascii_graphic()) {
        return Err(error());
    }

    Ok(string.into())
}

/// ZONED capabilities field reported in word 69 bits 1:0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ZonedCapabilities {
    /// Supports the Host Aware Zones.
    HostAware = 0b1,
    /// Supports device-managed zoned device capabilities.
    DeviceManaged = 0b10,
}

impl ZonedCapabilities {
    /// Parse ZONED capabilities from word 69.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        match value {
            0 => Ok(None),
            x if x == Self::HostAware as _ => Ok(Some(Self::HostAware)),
            x if x == Self::DeviceManaged as _ => Ok(Some(Self::DeviceManaged)),
            x => Err(Error::ZonedCapabilities(x)),
        }
    }
}

/// Additional supported from word 69.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AdditionalSupported {
    /// `CFast` feature set.
    pub cfast: bool,
    /// Deterministic Read After Trim.
    pub drat: bool,
    /// Long Physical Sector alignment error reporting control.
    pub lps_align_error_report: bool,
    /// READ BUFFER DMA command.
    pub read_buffer_dma: bool,
    /// WRITE BUFFER DMA command.
    pub write_buffer_dma: bool,
    /// DOWNLOAD MICROCODE DMA command.
    pub dlmc_dma: bool,
    /// Optional ATA device 28-bit commands.
    pub optional_28bit: bool,
    /// Deterministic Read Zero After TRIM
    pub rzat: bool,
    /// Device encrypts all user data.
    pub encrypted: bool,
    /// Extended Number of User Addressable Sectors (words 230-233).
    pub sector_count_ext: bool,
    /// All write cache is non-volatile.
    pub write_cache_nv: bool,
    /// ZONED capabilities.
    pub zoned: Option<ZonedCapabilities>,
}

impl TryFrom<u16> for AdditionalSupported {
    type Error = Error;

    fn try_from(word69: u16) -> Result<Self, Self::Error> {
        let cfast = (word69 & (1 << 15)) != 0;
        let drat = (word69 & (1 << 14)) != 0;
        let lps_align_error_report = (word69 & (1 << 13)) != 0;
        let read_buffer_dma = (word69 & (1 << 11)) != 0;
        let write_buffer_dma = (word69 & (1 << 10)) != 0;
        let dlmc_dma = (word69 & (1 << 8)) != 0;
        // 28-BIT SUPPORTED is set when the commands are not supported
        let optional_28bit = (word69 & (1 << 6)) == 0;
        let rzat = (word69 & (1 << 5)) != 0;
        let encrypted = (word69 & (1 << 4)) != 0;
        let sector_count_ext = (word69 & (1 << 3)) != 0;
        let write_cache_nv = (word69 & (1 << 2)) != 0;
        let zoned = ZonedCapabilities::parse((word69 & 0b11) as _)?;

        Ok(Self {
            cfast,
            drat,
            lps_align_error_report,
            read_buffer_dma,
            write_buffer_dma,
            dlmc_dma,
            optional_28bit,
            rzat,
            encrypted,
            sector_count_ext,
            write_cache_nv,
            zoned,
        })
    }
}

/// SATA capabilities from word 76.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SataCapabilities {
    /// READ LOG DMA EXT command.
    pub read_log_dma_ext: bool,
    /// Device-initiated automatic partial-to-slumber transitions.
    pub device_apst: bool,
    /// Host-initiated automatic partial-to-slumber transitions.
    pub host_apst: bool,
    /// NCQ priority information.
    pub ncq_priority: bool,
    /// Unload while NCQ commands are outstanding.
    pub ncq_unload: bool,
    /// SATA PHY event counters log.
    pub phy_event_counters: bool,
    /// Receipt of host-initiated power management requests.
    pub hipm_receipt: bool,
    /// NCQ feature set.
    pub ncq: bool,
    /// SATA Gen3 (6.0 Gb/s) signaling speed.
    pub gen3: bool,
    /// SATA Gen2 (3.0 Gb/s) signaling speed.
    pub gen2: bool,
    /// SATA Gen1 (1.5 Gb/s) signaling speed.
    pub gen1: bool,
}

impl SataCapabilities {
    /// Parse SATA capabilities from word 76.
    fn parse(word76: u16) -> Option<Self> {
        const UNSUPPORTED: &[u16] = &[0x0, 0xFFFF];

        if UNSUPPORTED.contains(&word76) {
            return None;
        }

        Some(Self {
            read_log_dma_ext: word76 & (1 << 15) != 0,
            device_apst: word76 & (1 << 14) != 0,
            host_apst: word76 & (1 << 13) != 0,
            ncq_priority: word76 & (1 << 12) != 0,
            ncq_unload: word76 & (1 << 11) != 0,
            phy_event_counters: word76 & (1 << 10) != 0,
            hipm_receipt: word76 & (1 << 9) != 0,
            ncq: word76 & (1 << 8) != 0,
            gen3: word76 & (1 << 3) != 0,
            gen2: word76 & (1 << 2) != 0,
            gen1: word76 & (1 << 1) != 0,
        })
    }
}

/// Major version (standards-conformance) from word 80.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct MajorVersion {
    /// ACS-5.
    pub acs_5: bool,
    /// ACS-4.
    pub acs_4: bool,
    /// ACS-3.
    pub acs_3: bool,
    /// ACS-2.
    pub acs_2: bool,
    /// ATA8-ACS.
    pub ata8_acs: bool,
    /// ATA/ATAPI-7.
    pub ata_atapi_7: bool,
    /// ATA/ATAPI-6.
    pub ata_atapi_6: bool,
    /// ATA/ATAPI-5.
    pub ata_atapi_5: bool,
    /// ATA/ATAPI-4.
    pub ata_atapi_4: bool,
    /// ATA-3.
    pub ata_3: bool,
    /// ATA-2.
    pub ata_2: bool,
    /// ATA-1.
    pub ata_1: bool,
}

impl MajorVersion {
    /// Parse major version from word 80.
    pub(crate) fn parse(word80: u16) -> Option<Self> {
        if matches!(word80, 0 | 0xFFFF) {
            return None;
        }

        Some(Self {
            acs_5: word80 & (1 << 12) != 0,
            acs_4: word80 & (1 << 11) != 0,
            acs_3: word80 & (1 << 10) != 0,
            acs_2: word80 & (1 << 9) != 0,
            ata8_acs: word80 & (1 << 8) != 0,
            ata_atapi_7: word80 & (1 << 7) != 0,
            ata_atapi_6: word80 & (1 << 6) != 0,
            ata_atapi_5: word80 & (1 << 5) != 0,
            ata_atapi_4: word80 & (1 << 4) != 0,
            ata_3: word80 & (1 << 3) != 0,
            ata_2: word80 & (1 << 2) != 0,
            ata_1: word80 & (1 << 1) != 0,
        })
    }
}

/// Feature flags shared by the supported (words 82-84 & 119) and enabled (words
/// 85-87 & 120) groups.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Features {
    /// NOP command.
    pub nop: Option<bool>,
    /// READ BUFFER command.
    pub read_buffer: Option<bool>,
    /// WRITE BUFFER command.
    pub write_buffer: Option<bool>,
    /// DEVICE RESET command.
    pub device_reset: Option<bool>,
    /// Read look-ahead.
    pub read_lookahead: Option<bool>,
    /// Volatile write cache.
    pub volatile_write_cache: Option<bool>,
    /// PACKET feature set.
    pub packet: Option<bool>,
    /// Power Management feature set.
    pub power_management: Option<bool>,
    /// Security feature set.
    pub security: Option<bool>,
    /// SMART feature set.
    pub smart: Option<bool>,
    /// FLUSH CACHE EXT command.
    pub flush_cache_ext: Option<bool>,
    /// FLUSH CACHE command.
    pub flush_cache: Option<bool>,
    /// 48-bit Address feature set.
    pub address_48bit: Option<bool>,
    /// SET FEATURES subcommand required after power-up.
    pub set_features_required: Option<bool>,
    /// Power Up In Standby feature set.
    pub puis: Option<bool>,
    /// Advanced Power Management feature set.
    pub apm: Option<bool>,
    /// `CompactFlash` Association feature set.
    pub cfa: Option<bool>,
    /// DOWNLOAD MICROCODE command.
    pub dlmc: Option<bool>,
    /// IDLE IMMEDIATE command UNLOAD feature.
    pub idle_immediate_unload: Option<bool>,
    /// World Wide Name.
    pub wwn: Option<bool>,
    /// WRITE DMA FUA EXT command.
    pub write_dma_fua_ext: Option<bool>,
    /// General Purpose Logging feature set.
    pub gpl: Option<bool>,
    /// SMART self-test.
    pub smart_self_test: Option<bool>,
    /// SMART error logging.
    pub smart_error_log: Option<bool>,
    /// Device Statistics Notification feature set.
    pub dsn: Option<bool>,
    /// Extended Power Conditions feature set.
    pub epc: Option<bool>,
    /// Sense Data Reporting feature set.
    pub sense_data_reporting: Option<bool>,
    /// Free-fall Control feature set.
    pub freefall_control: Option<bool>,
    /// DOWNLOAD MICROCODE command mode 3.
    pub dlmc_mode3: Option<bool>,
    /// READ LOG DMA EXT and WRITE LOG DMA EXT commands.
    pub gpl_dma: Option<bool>,
    /// WRITE UNCORRECTABLE EXT command.
    pub write_uncorrectable_ext: Option<bool>,
    /// Write-Read-Verify feature set.
    pub write_read_verify: Option<bool>,
}

impl Features {
    /// Parse validity bits.
    fn validate_word(word: Option<u16>) -> Option<u16> {
        word.filter(|x| x >> 14 == 0b01)
    }

    /// Parse from words, supported (82-84 & 119) or enabled (85-87 & 120).
    fn parse(
        word82_or_85: Option<u16>,
        word83_or_86: Option<u16>,
        word84_or_87: Option<u16>,
        word119_or_120: Option<u16>,
    ) -> Self {
        let nop = word82_or_85.map(|x| x & (1 << 14) != 0);
        let read_buffer = word82_or_85.map(|x| x & (1 << 13) != 0);
        let write_buffer = word82_or_85.map(|x| x & (1 << 12) != 0);
        let device_reset = word82_or_85.map(|x| x & (1 << 9) != 0);
        let read_lookahead = word82_or_85.map(|x| x & (1 << 6) != 0);
        let volatile_write_cache = word82_or_85.map(|x| x & (1 << 5) != 0);
        let packet = word82_or_85.map(|x| x & (1 << 4) != 0);
        let power_management = word82_or_85.map(|x| x & (1 << 3) != 0);
        let security = word82_or_85.map(|x| x & (1 << 1) != 0);
        let smart = word82_or_85.map(|x| x & (1 << 0) != 0);

        let flush_cache_ext = word83_or_86.map(|x| x & (1 << 13) != 0);
        let flush_cache = word83_or_86.map(|x| x & (1 << 12) != 0);
        let address_48bit = word83_or_86.map(|x| x & (1 << 10) != 0);
        let set_features_required = word83_or_86.map(|x| x & (1 << 6) != 0);
        let puis = word83_or_86.map(|x| x & (1 << 5) != 0);
        let apm = word83_or_86.map(|x| x & (1 << 3) != 0);
        let cfa = word83_or_86.map(|x| x & (1 << 2) != 0);
        let dlmc = word83_or_86.map(|x| x & (1 << 0) != 0);

        let idle_immediate_unload = word84_or_87.map(|x| x & (1 << 13) != 0);
        let wwn = word84_or_87.map(|x| x & (1 << 8) != 0);
        let write_dma_fua_ext = word84_or_87.map(|x| x & (1 << 6) != 0);
        let gpl = word84_or_87.map(|x| x & (1 << 5) != 0);
        let smart_self_test = word84_or_87.map(|x| x & (1 << 1) != 0);
        let smart_error_log = word84_or_87.map(|x| x & (1 << 0) != 0);

        let dsn = word119_or_120.map(|x| x & (1 << 9) != 0);
        let epc = word119_or_120.map(|x| x & (1 << 7) != 0);
        let sense_data_reporting = word119_or_120.map(|x| x & (1 << 6) != 0);
        let freefall_control = word119_or_120.map(|x| x & (1 << 5) != 0);
        let dlmc_mode3 = word119_or_120.map(|x| x & (1 << 4) != 0);
        let gpl_dma = word119_or_120.map(|x| x & (1 << 3) != 0);
        let write_uncorrectable_ext = word119_or_120.map(|x| x & (1 << 2) != 0);
        let write_read_verify = word119_or_120.map(|x| x & (1 << 1) != 0);

        Self {
            nop,
            read_buffer,
            write_buffer,
            device_reset,
            read_lookahead,
            volatile_write_cache,
            packet,
            power_management,
            security,
            smart,
            flush_cache_ext,
            flush_cache,
            address_48bit,
            set_features_required,
            puis,
            apm,
            cfa,
            dlmc,
            idle_immediate_unload,
            wwn,
            write_dma_fua_ext,
            gpl,
            smart_self_test,
            smart_error_log,
            dsn,
            epc,
            sense_data_reporting,
            freefall_control,
            dlmc_mode3,
            gpl_dma,
            write_uncorrectable_ext,
            write_read_verify,
        }
    }
}

/// Features supported from words 82-84 and 119.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeaturesSupported {
    /// Shared supported/enabled feature fields.
    pub features: Features,
    /// Streaming feature set.
    pub streaming: Option<bool>,
    /// Accessible Max Address Configuration feature set.
    pub amac: Option<bool>,
}

impl FeaturesSupported {
    /// Parse from words 82-84 and 119.
    fn parse(word82: u16, word83: u16, word84: u16, word119: Option<u16>) -> Self {
        let word83 = Features::validate_word(Some(word83));
        let words_82_83_valid = word83.is_some();
        let word82 = words_82_83_valid.then_some(word82);

        let word84 = Features::validate_word(Some(word84));
        let word119 = Features::validate_word(word119);

        let features = Features::parse(word82, word83, word84, word119);
        let streaming = word84.map(|x| x & (1 << 4) != 0);
        let amac = word119.map(|x| x & (1 << 8) != 0);

        Self {
            features,
            streaming,
            amac,
        }
    }
}

/// Lets `FeaturesSupported` be used directly as the inner `Features`.
impl std::ops::Deref for FeaturesSupported {
    type Target = Features;

    fn deref(&self) -> &Features {
        &self.features
    }
}

/// Features enabled from words 85-87 and 120.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FeaturesEnabled {
    /// Shared supported/enabled feature fields.
    pub features: Features,
    /// Words 119 and 120 are valid.
    pub words_119_120_valid: Option<bool>,
    /// Current media serial number (words 176-205) is valid.
    pub media_serial_valid: Option<bool>,
}

impl FeaturesEnabled {
    /// Parse from words 85-87 and 120.
    fn parse(word85: u16, word86: u16, word87: u16, word120: u16) -> Self {
        let word87 = Features::validate_word(Some(word87));
        let words_85_to_87_valid = word87.is_some();
        let word85 = words_85_to_87_valid.then_some(word85);
        let word86 = words_85_to_87_valid.then_some(word86);
        let mut word120 = Features::validate_word(Some(word120));

        let words_119_120_valid = word86.map(|x| x & (1 << 15) != 0);
        word120 = word120.filter(|_| words_119_120_valid == Some(true));

        let features = Features::parse(word85, word86, word87, word120);

        let media_serial_valid = word87.map(|x| x & (1 << 2) != 0);

        Self {
            features,
            words_119_120_valid,
            media_serial_valid,
        }
    }
}

impl std::ops::Deref for FeaturesEnabled {
    type Target = Features;

    fn deref(&self) -> &Features {
        &self.features
    }
}

/// Physical/logical sector geometry decoded from word 106.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SectorSize {
    /// A physical sector contains multiple logical sectors.
    pub multiple_logical_per_physical: bool,
    /// Logical sector size is larger than 256 words (512 bytes).
    pub logical_gt_256_words: bool,
    /// Exponent x giving 2^x logical sectors per physical sector.
    pub relationship: u8,
}

impl SectorSize {
    /// Parse from word 106.
    pub(crate) fn parse(word106: u16) -> Option<Self> {
        if word106 >> 14 != 0b01 {
            return None;
        }

        Some(Self {
            multiple_logical_per_physical: word106 & (1 << 13) != 0,
            logical_gt_256_words: word106 & (1 << 12) != 0,
            relationship: (word106 & 0b1111) as _,
        })
    }
}

/// WWN Network Address Authority.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WwnNaa {
    /// IEEE.
    Ieee = 1,
    /// IEEE extended.
    IeeeExtended = 2,
    /// IEEE registered.
    IeeeRegistered = 5,
}

impl WwnNaa {
    /// Parse from raw field.
    fn parse(value: u8) -> Result<Self, Error> {
        match value {
            x if x == Self::Ieee as u8 => Ok(Self::Ieee),
            x if x == Self::IeeeExtended as u8 => Ok(Self::IeeeExtended),
            x if x == Self::IeeeRegistered as u8 => Ok(Self::IeeeRegistered),
            x => Err(Error::WwnNaa(x)),
        }
    }
}

/// World Wide Name from words 108-111.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Wwn {
    /// Network Address Authority.
    pub naa: WwnNaa,
    /// IEEE Organizationally Unique Identifier.
    pub oui: u32,
    /// Vendor-specific.
    pub vendor_specific: u64,
}

impl Wwn {
    /// Size in bytes.
    pub(crate) const SIZE: usize = 8;

    /// Parse from raw bytes.
    fn parse(data: [u8; Self::SIZE]) -> Result<Self, Error> {
        let value = u64::from_be_bytes(swap_word_bytes(&data).as_ref().try_into().unwrap());
        let naa = WwnNaa::parse(((value >> 60) & 0xF) as u8)?;

        let (oui, vendor_specific) = match naa {
            WwnNaa::Ieee => (((value >> 24) & 0xFF_FFFF) as u32, value & 0xFF_FFFF),
            WwnNaa::IeeeExtended => (
                ((value >> 24) & 0xFF_FFFF) as u32,
                (((value >> 48) & 0xFFF) << 24) | (value & 0xFF_FFFF),
            ),
            WwnNaa::IeeeRegistered => (((value >> 36) & 0xFF_FFFF) as u32, value & 0xF_FFFF_FFFF),
        };

        Ok(Self {
            naa,
            oui,
            vendor_specific,
        })
    }
}

impl std::fmt::Display for Wwn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:x} {:06X} {:09X}",
            self.naa as u8, self.oui, self.vendor_specific
        )
    }
}

/// Nominal device form factor from word 168 bits 3:0.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FormFactor {
    /// 5.25 inch.
    Inch5_25 = 0x1,
    /// 3.5 inch.
    Inch3_5 = 0x2,
    /// 2.5 inch.
    Inch2_5 = 0x3,
    /// 1.8 inch.
    Inch1_8 = 0x4,
    /// Less than 1.8-inch.
    LessThan1_8 = 0x5,
    /// mSATA.
    MSata = 0x6,
    /// M.2.
    M2 = 0x7,
    /// `MicroSSD`.
    MicroSsd = 0x8,
    /// `CFast`.
    CFast = 0x9,
}

impl FormFactor {
    /// Parse from raw field.
    pub(crate) fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[FormFactor] = &[
            FormFactor::Inch5_25,
            FormFactor::Inch3_5,
            FormFactor::Inch2_5,
            FormFactor::Inch1_8,
            FormFactor::LessThan1_8,
            FormFactor::MSata,
            FormFactor::M2,
            FormFactor::MicroSsd,
            FormFactor::CFast,
        ];

        if value == 0 {
            return Ok(None);
        }

        Ok(Some(
            VARIANTS
                .iter()
                .find(|&&x| x as u8 == value)
                .copied()
                .ok_or(Error::FormFactor(value))?,
        ))
    }
}

impl std::fmt::Display for FormFactor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Inch5_25 => write!(f, "5.25 inch"),
            Self::Inch3_5 => write!(f, "3.5 inch"),
            Self::Inch2_5 => write!(f, "2.5 inch"),
            Self::Inch1_8 => write!(f, "1.8 inch"),
            Self::LessThan1_8 => write!(f, "less than 1.8 inch"),
            Self::MSata => write!(f, "mSATA"),
            Self::M2 => write!(f, "M.2"),
            Self::MicroSsd => write!(f, "MicroSSD"),
            Self::CFast => write!(f, "CFast"),
        }
    }
}

/// SMART Command Transport supported, from word 206.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct SctSupported {
    /// SCT data tables.
    pub data_tables: bool,
    /// SCT feature control.
    pub feature_control: bool,
    /// SCT error recovery control.
    pub error_recovery_control: bool,
    /// SCT Write Same.
    pub write_same: bool,
    /// SMART Command Transport.
    pub sct: bool,
}

impl SctSupported {
    /// Parse SMART Command Transport supported from raw word 206.
    fn parse(word206: u16) -> Self {
        Self {
            data_tables: word206 & (1 << 5) != 0,
            feature_control: word206 & (1 << 4) != 0,
            error_recovery_control: word206 & (1 << 3) != 0,
            write_same: word206 & (1 << 2) != 0,
            sct: word206 & (1 << 0) != 0,
        }
    }
}

/// Nominal media rotation rate from word 217.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RotationRate {
    /// Non-rotating media, e.g. a solid-state device.
    NonRotating,
    /// Spinning media with this rotations per minute.
    Rpm(u16),
}

impl RotationRate {
    /// Parse from word 217.
    fn parse(word217: u16) -> Result<Option<Self>, Error> {
        match word217 {
            0 => Ok(None),
            1 => Ok(Some(Self::NonRotating)),
            x @ 0x401..=0xFFFE => Ok(Some(Self::Rpm(x))),
            x => Err(Error::RotationRate(x)),
        }
    }
}

/// Identify device response.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct IdentifyDevice {
    /// Drive serial number.
    pub serial: String,
    /// Firmware revision.
    pub firmware: String,
    /// Model.
    pub model: String,
    /// Free-fall sensitivity level,
    pub freefall_sensitivity: Option<u8>,
    /// User-addressable sector count (28-bit).
    pub sectors_28bit: u32,
    /// Additional-support fields.
    pub additional_supported: Option<AdditionalSupported>,
    /// SATA capability fields.
    pub sata_capabilities: Option<SataCapabilities>,
    /// Standards-conformance major versions fields.
    pub major_version: Option<MajorVersion>,
    /// Features supported.
    pub features_supported: FeaturesSupported,
    /// Features enabled.
    pub features_enabled: FeaturesEnabled,
    /// User-addressable sector count (48-bit).
    pub sectors_48bit: Option<u64>,
    /// Logical/physical sector relationship.
    pub sector_size: Option<SectorSize>,
    /// World Wide Name.
    pub wwn: Option<Wwn>,
    /// Logical sector size in bytes.
    pub logical_sector_size: u64,
    /// Vendor-specific data.
    pub vendor_specific: [u8; 62],
    /// Nominal device form factor.
    pub form_factor: Option<FormFactor>,
    /// SMART command transport supported fields.
    pub sct_supported: SctSupported,
    /// Nominal media rotation rate.
    pub rotation_rate: Option<RotationRate>,
    /// Extended user-addressable sector count.
    pub sectors_ext: Option<u64>,
    /// Minimum DOWNLOAD MICROCODE transfer size in 512-byte blocks.
    pub dlmc_min_blocks: Option<u16>,
    /// Maximum DOWNLOAD MICROCODE transfer size in 512-byte blocks.
    pub dlmc_max_blocks: Option<u16>,
}

impl IdentifyDevice {
    /// Size in bytes.
    pub(crate) const SIZE: usize = SECTOR_SIZE;

    /// Get correct user-addressable sector count from appropriate field.
    pub(crate) fn sectors(&self) -> u64 {
        self.sectors_ext
            .or(self.sectors_48bit)
            .unwrap_or(self.sectors_28bit.into())
    }

    /// Get total user capacity in bytes.
    pub(crate) fn capacity(&self) -> u64 {
        self.sectors().saturating_mul(self.logical_sector_size)
    }
}

impl TryFrom<&[u8; IdentifyDevice::SIZE]> for IdentifyDevice {
    type Error = Error;

    fn try_from(data: &[u8; IdentifyDevice::SIZE]) -> Result<Self, Error> {
        const WORD_SIZE: usize = size_of::<u16>();
        const SERIAL_OFFSET: usize = 20;
        const SERIAL_SIZE: usize = 20;
        const FIRMWARE_OFFSET: usize = 46;
        const FIRMWARE_SIZE: usize = 8;
        const MODEL_OFFSET: usize = 54;
        const MODEL_SIZE: usize = 40;
        const WWN_OFFSET: usize = 216;
        const VENDOR_SPECIFIC_OFFSET: usize = 258;
        const DLMC_BLOCKS_RANGE: RangeInclusive<u16> = 1..=0xFFFE;

        let words: [_; Self::SIZE / WORD_SIZE] = std::array::from_fn(|i| {
            u16::from_le_bytes([data[i * WORD_SIZE], data[i * WORD_SIZE + 1]])
        });

        let serial = parse_string(&data[SERIAL_OFFSET..SERIAL_OFFSET + SERIAL_SIZE])?;
        let firmware = parse_string(&data[FIRMWARE_OFFSET..FIRMWARE_OFFSET + FIRMWARE_SIZE])?;
        let model = parse_string(&data[MODEL_OFFSET..MODEL_OFFSET + MODEL_SIZE])?;

        let features_enabled = FeaturesEnabled::parse(words[85], words[86], words[87], words[120]);
        let words_119_120_valid = features_enabled.words_119_120_valid == Some(true);
        let word119 = words_119_120_valid.then(|| words[119]);
        let features_supported = FeaturesSupported::parse(words[82], words[83], words[84], word119);

        let word53 = words[53];
        let freefall_valid = features_supported.freefall_control == Some(true)
            && features_enabled.freefall_control == Some(true);
        let freefall_sensitivity = freefall_valid.then_some((word53 >> 8) as _);
        let _word88_valid = word53 & (1 << 2) != 0; // For future use when word 88 is implemented
        let words64_to_70_valid = word53 & (1 << 1) != 0;

        let sectors_28bit = (u32::from(words[61]) << 16) | u32::from(words[60]);

        let additional_supported = if words64_to_70_valid {
            Some(AdditionalSupported::try_from(words[69])?)
        } else {
            None
        };

        let sata_capabilities = SataCapabilities::parse(words[76]);
        let major_version = MajorVersion::parse(words[80]);

        let sectors_48bit = (features_supported.address_48bit == Some(true)).then(|| {
            (u64::from(words[103]) << 48)
                | (u64::from(words[102]) << 32)
                | (u64::from(words[101]) << 16)
                | u64::from(words[100])
        });

        let sector_size = SectorSize::parse(words[106]);

        let wwn_bytes = data[WWN_OFFSET..WWN_OFFSET + Wwn::SIZE].try_into().unwrap();
        let wwn_valid = features_supported.wwn == Some(true)
            && features_enabled.wwn == Some(true)
            && wwn_bytes != [0; Wwn::SIZE];
        let wwn = wwn_valid.then(|| Wwn::parse(wwn_bytes)).transpose()?;

        let logical_sector_size = if sector_size.is_some_and(|x| x.logical_gt_256_words) {
            let word_count = (u64::from(words[118]) << 16) | u64::from(words[117]);
            word_count * u64::try_from(WORD_SIZE).unwrap()
        } else {
            SECTOR_SIZE as _
        };
        if logical_sector_size < SECTOR_SIZE as _ {
            return Err(Error::LogicalSectorSize(logical_sector_size));
        }

        let vendor_specific = std::array::from_fn(|i| data[VENDOR_SPECIFIC_OFFSET + i]);

        let form_factor = FormFactor::parse((words[168] & 0xF) as _)?;
        let sct_supported = SctSupported::parse(words[206]);
        let rotation_rate = RotationRate::parse(words[217])?;

        let sectors_ext = additional_supported.and_then(|s| {
            s.sector_count_ext.then(|| {
                (u64::from(words[233]) << 48)
                    | (u64::from(words[232]) << 32)
                    | (u64::from(words[231]) << 16)
                    | (u64::from(words[230]))
            })
        });

        let dlmc_blocks_valid = (features_supported.dlmc == Some(true)
            || additional_supported.is_some_and(|x| x.dlmc_dma))
            && features_supported.dlmc_mode3 == Some(true);
        let dlmc_min_blocks = dlmc_blocks_valid
            .then_some(words[234])
            .filter(|x| DLMC_BLOCKS_RANGE.contains(x));
        let dlmc_max_blocks = dlmc_blocks_valid
            .then_some(words[235])
            .filter(|x| DLMC_BLOCKS_RANGE.contains(x));

        Ok(Self {
            serial,
            firmware,
            model,
            freefall_sensitivity,
            sectors_28bit,
            additional_supported,
            sata_capabilities,
            major_version,
            features_supported,
            features_enabled,
            sectors_48bit,
            sector_size,
            wwn,
            logical_sector_size,
            vendor_specific,
            form_factor,
            sct_supported,
            rotation_rate,
            sectors_ext,
            dlmc_min_blocks,
            dlmc_max_blocks,
        })
    }
}

impl std::fmt::Display for IdentifyDevice {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let wwn = self
            .wwn
            .as_ref()
            .map_or_else(|| "N/A".to_string(), ToString::to_string);

        let drive_type = match self.rotation_rate {
            None => String::from("unknown"),
            Some(RotationRate::NonRotating) => String::from("SSD"),
            Some(RotationRate::Rpm(x)) => format!("HDD {x} RPM"),
        };

        let form = self
            .form_factor
            .as_ref()
            .map_or_else(|| "unknown".to_string(), ToString::to_string);

        write!(
            f,
            "(model: {}, serial: {}, WWN: {wwn}, firmware: {}, type: {drive_type}, size: {}, \
             form: {form})",
            self.model.trim(),
            self.serial.trim(),
            self.firmware.trim(),
            output::format_byte_size_decimal(self.capacity()),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn string_parse() {
        assert_eq!(parse_string(&[0x42, 0x41, 0x44, 0x43]).unwrap(), "ABCD");
        std::assert_matches!(parse_string(&[0xFF, 0xFF]), Err(Error::String(_)));
    }

    #[test]
    fn identify_parse() {
        const DATA_VALID: &[&[u8; IdentifyDevice::SIZE]] = &[
            test_data::westerndigital_caviarse::IDENTIFY,
            test_data::seagate_barracudapro::IDENTIFY,
            test_data::samsung_ss410::IDENTIFY,
            test_data::skhynix_sc308::IDENTIFY,
            test_data::kingston_dc500r::IDENTIFY,
        ];
        const DATA_INVALID: &[&[u8; IdentifyDevice::SIZE]] = &[&[0x01; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(IdentifyDevice::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(IdentifyDevice::try_from(data).is_err());
        }
    }
}
