//! SCSI INQUIRY command functionality.

/// INQUIRY error.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// Data too short.
    Truncated,
    /// Invalid string.
    String(Box<[u8]>),
    /// Invalid peripheral qualifier field value.
    InvalidPeripheralQualifier(u8),
    /// Invalid peripheral device type field value.
    InvalidPeripheralDeviceType(u8),
    /// Invalid hot-pluggable field value.
    InvalidHotPluggable(u8),
    /// Invalid version field value.
    InvalidVersion(u8),
    /// Invalid response data format field value.
    InvalidFormat(u8),
    /// Invalid Target Port Group support field value.
    InvalidTpgSupport(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated"),
            Self::InvalidPeripheralQualifier(x) => {
                write!(f, "invalid peripheral qualifier {x:#b}")
            },
            Self::InvalidPeripheralDeviceType(x) => {
                write!(f, "invalid peripheral device type {x:#x}")
            },
            Self::InvalidHotPluggable(x) => {
                write!(f, "invalid hot pluggable {x:#b}")
            },
            Self::InvalidVersion(x) => {
                write!(f, "invalid version {x:#x}")
            },
            Self::InvalidFormat(x) => {
                write!(f, "invalid format {x:#x}")
            },
            Self::InvalidTpgSupport(x) => {
                write!(f, "invalid TPG support {x:#x}")
            },
            Self::String(_) => write!(f, "invalid string"),
        }
    }
}

/// Peripheral qualifier field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralQualifier {
    /// Device of the reported type is connected to this logical unit.
    Connected = 0b0,
    /// Device of the reported type is supported but not currently connected.
    NotConnected = 0b1,
    /// Target does not support a device on this logical unit.
    NotSupported = 0b11,
    /// Vendor-specific.
    VendorSpecific4 = 0b100,
    /// Vendor-specific.
    VendorSpecific5 = 0b101,
    /// Vendor-specific.
    VendorSpecific6 = 0b110,
    /// Vendor-specific.
    VendorSpecific7 = 0b111,
}

impl TryFrom<u8> for PeripheralQualifier {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[PeripheralQualifier] = &[
            PeripheralQualifier::Connected,
            PeripheralQualifier::NotConnected,
            PeripheralQualifier::NotSupported,
            PeripheralQualifier::VendorSpecific4,
            PeripheralQualifier::VendorSpecific5,
            PeripheralQualifier::VendorSpecific6,
            PeripheralQualifier::VendorSpecific7,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidPeripheralQualifier(value))
    }
}

impl std::fmt::Display for PeripheralQualifier {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Connected => write!(f, "accessible"),
            Self::NotConnected => write!(f, "not accessible at this time"),
            Self::NotSupported => write!(f, "not accessible"),
            &x => write!(f, "vendor specific {:#b}", x as u8),
        }
    }
}

/// Peripheral device field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PeripheralDeviceType {
    /// Direct-access block device.
    DirectAccessBlock = 0x0,
    /// Sequential-access device (e.g. tape).
    SequentialAccess = 0x1,
    /// Processor device.
    Processor = 0x3,
    /// CD/DVD device.
    CdDvd = 0x5,
    /// Optical memory device.
    OpticalMemory = 0x7,
    /// Media changer (e.g. jukebox/autoloader).
    MediaChanger = 0x8,
    /// Storage array controller (e.g. RAID).
    StorageArrayController = 0xC,
    /// Enclosure services device (SES).
    EnclosureServices = 0xD,
    /// Simplified direct-access (reduced block command set) device.
    SimplifiedDirectAccess = 0xE,
    /// Optical card reader/writer device.
    OpticalCardReaderWriter = 0xF,
    /// Object-based storage device (OSD).
    ObjectBasedStorage = 0x11,
    /// Automation/drive interface.
    AutomationDriveInterface = 0x12,
    /// Host-managed zoned block device (ZBC).
    HostManagedZonedBlock = 0x14,
    /// Well-known logical unit (addresses a target service, not media).
    WellKnownLogicalUnit = 0x1E,
    /// Unknown or no device type.
    Unknown = 0x1F,
}

impl TryFrom<u8> for PeripheralDeviceType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[PeripheralDeviceType] = &[
            PeripheralDeviceType::DirectAccessBlock,
            PeripheralDeviceType::SequentialAccess,
            PeripheralDeviceType::Processor,
            PeripheralDeviceType::CdDvd,
            PeripheralDeviceType::OpticalMemory,
            PeripheralDeviceType::MediaChanger,
            PeripheralDeviceType::StorageArrayController,
            PeripheralDeviceType::EnclosureServices,
            PeripheralDeviceType::SimplifiedDirectAccess,
            PeripheralDeviceType::OpticalCardReaderWriter,
            PeripheralDeviceType::ObjectBasedStorage,
            PeripheralDeviceType::AutomationDriveInterface,
            PeripheralDeviceType::HostManagedZonedBlock,
            PeripheralDeviceType::WellKnownLogicalUnit,
            PeripheralDeviceType::Unknown,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidPeripheralDeviceType(value))
    }
}

impl std::fmt::Display for PeripheralDeviceType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DirectAccessBlock => write!(f, "direct access block device"),
            Self::SequentialAccess => write!(f, "sequential access device"),
            Self::Processor => write!(f, "processor device"),
            Self::CdDvd => write!(f, "CD/DVD device"),
            Self::OpticalMemory => write!(f, "optical memory device"),
            Self::MediaChanger => write!(f, "media changer device"),
            Self::StorageArrayController => write!(f, "storage array controller device"),
            Self::EnclosureServices => write!(f, "enclosure services device"),
            Self::SimplifiedDirectAccess => write!(f, "simplified direct access device"),
            Self::OpticalCardReaderWriter => write!(f, "optical card reader/writer device"),
            Self::ObjectBasedStorage => write!(f, "object-based storage device"),
            Self::AutomationDriveInterface => write!(f, "automation/drive interface"),
            Self::HostManagedZonedBlock => write!(f, "host managed zoned block device"),
            Self::WellKnownLogicalUnit => write!(f, "well known logical unit"),
            Self::Unknown => write!(f, "unknown or no device type"),
        }
    }
}

/// SCSI Primary Commands (SPC) version conformance.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Version {
    /// Does not claim conformance to any standard.
    NoConformance = 0x0,
    /// SPC.
    Spc = 0x3,
    /// SPC-2.
    Spc2 = 0x4,
    /// SPC-3.
    Spc3 = 0x5,
    /// SPC-4.
    Spc4 = 0x6,
    /// SPC-5.
    Spc5 = 0x7,
    /// SPC-6.
    Spc6 = 0xD,
    /// SPC-7.
    Spc7 = 0xE,
}

impl TryFrom<u8> for Version {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[Version] = &[
            Version::NoConformance,
            Version::Spc,
            Version::Spc2,
            Version::Spc3,
            Version::Spc4,
            Version::Spc5,
            Version::Spc6,
            Version::Spc7,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidVersion(value))
    }
}

impl std::fmt::Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoConformance => write!(f, "no conformance"),
            Self::Spc => write!(f, "SPC"),
            Self::Spc2 => write!(f, "SPC-2"),
            Self::Spc3 => write!(f, "SPC-3"),
            Self::Spc4 => write!(f, "SPC-4"),
            Self::Spc5 => write!(f, "SPC-5"),
            Self::Spc6 => write!(f, "SPC-6"),
            Self::Spc7 => write!(f, "SPC-7"),
        }
    }
}

/// Target Port Group Support field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TpgSupport {
    /// Not supported.
    NotSupported = 0b0,
    /// Only implicit (target-initiated) access state changes.
    ImplicitOnly = 0b1,
    /// Only explicit (host-commanded) access state changes.
    ExplicitOnly = 0b10,
    /// Both implicit and explicit access state changes.
    ImplicitExplicit = 0b11,
}

impl std::fmt::Display for TpgSupport {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NotSupported => write!(f, "not supported"),
            Self::ImplicitOnly => write!(f, "implicit only"),
            Self::ExplicitOnly => write!(f, "explicit only"),
            Self::ImplicitExplicit => write!(f, "implicit and explicit"),
        }
    }
}

impl TryFrom<u8> for TpgSupport {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[TpgSupport] = &[
            TpgSupport::NotSupported,
            TpgSupport::ImplicitOnly,
            TpgSupport::ExplicitOnly,
            TpgSupport::ImplicitExplicit,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidTpgSupport(value))
    }
}

/// INQUIRY command response.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Inquiry {
    /// Whether a device is connected to this logical unit.
    pub peripheral_qualifier: PeripheralQualifier,
    /// SCSI device class of the logical unit.
    pub peripheral_device_type: PeripheralDeviceType,
    /// Medium is removable.
    pub removable_medium: bool,
    /// Logical unit is part of a conglomerate.
    pub lu_conglomerate: bool,
    /// Device is hot-pluggable.
    pub hot_pluggable: Option<bool>,
    /// SPC version conformance.
    pub version: Version,
    /// Normal Auto Contingent Allegiance support for CONTROL command.
    pub normal_aca: bool,
    /// Hierarchical addressing support.
    pub hierarchical_support: bool,
    /// Device contains an embedded storage array controller.
    pub scc_support: bool,
    /// Target Port Group support.
    pub tpg_support: TpgSupport,
    /// Third-party copy (EXTENDED COPY) command support.
    pub third_party_copy: bool,
    /// Device supports protection information.
    pub protect: bool,
    /// Device contains an embedded enclosure services component.
    pub enclosure_services: bool,
    /// Vendor-specific bit (byte 6 bit 5).
    pub vendor_specific1: bool,
    /// Device has multiple SCSI ports.
    pub multiple_port: bool,
    /// Device supports command queuing.
    pub command_queuing: bool,
    /// Vendor-specific bit (byte 7 bit 0).
    pub vendor_specific2: bool,
    /// Vendor identification.
    pub vendor_id: String,
    /// Product identification.
    pub product_id: String,
    /// Product revision.
    pub product_revision: String,
    /// Vendor-specific bytes 36:55.
    pub vendor_specific3: Option<[u8; 20]>,
    /// Version descriptor codes.
    pub version_descriptors: Box<[u16]>,
    /// Trailing vendor-specific bytes.
    pub vendor_specific4: Box<[u8]>,
}

impl Inquiry {
    /// Maximum size in bytes.
    pub const MAX_SIZE: usize = 260;

    /// Parse string.
    fn parse_string(data: &[u8]) -> Result<String, Error> {
        let string = str::from_utf8(data).map_err(|_| Error::String(data.into()))?;

        // Trim trailing whitespace and null bytes. Not standards-compliant, but some
        // shitty devices null-pad strings
        let string = string.trim_end_matches(|x: char| x == '\0' || x.is_ascii_whitespace());

        if !string
            .chars()
            .all(|x| x.is_ascii_graphic() || x.is_ascii_whitespace())
        {
            return Err(Error::String(data.into()));
        }

        Ok(string.into())
    }
}

impl TryFrom<&[u8]> for Inquiry {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        const ADDITIONAL_OFFSET: usize = 5;
        const HEADER_SIZE: usize = 36;
        const HOT_PLUGGABLE_NONE: u8 = 0b0;
        const HOT_PLUGGABLE_TRUE: u8 = 0b1;
        const HOT_PLUGGABLE_FALSE: u8 = 0b10;
        const FORMAT_STANDARD: u8 = 0x2;

        let header = data
            .first_chunk::<{ HEADER_SIZE }>()
            .ok_or(Error::Truncated)?;

        let peripheral_qualifier = (header[0] >> 5).try_into()?;
        let peripheral_device_type = (header[0] & 0b11111).try_into()?;
        let removable_medium = (header[1] & (1 << 7)) != 0;
        let lu_conglomerate = (header[1] & (1 << 6)) != 0;

        let hot_pluggable = match (header[1] >> 4) & 0b11 {
            HOT_PLUGGABLE_NONE => None,
            HOT_PLUGGABLE_TRUE => Some(true),
            HOT_PLUGGABLE_FALSE => Some(false),
            x => return Err(Error::InvalidHotPluggable(x)),
        };

        let version = header[2].try_into()?;
        let normal_aca = (header[3] & (1 << 5)) != 0;
        let hierarchical_support = (header[3] & (1 << 4)) != 0;

        let format = header[3] & 0b1111;
        if format != FORMAT_STANDARD {
            return Err(Error::InvalidFormat(format));
        }

        let additional_length = header[4] as usize;
        let data_size = ADDITIONAL_OFFSET + additional_length;
        if data_size < HEADER_SIZE {
            return Err(Error::Truncated);
        }
        let data = data.get(..data_size).ok_or(Error::Truncated)?;

        let scc_support = (header[5] & (1 << 7)) != 0;
        let tpg_support = TpgSupport::try_from((header[5] >> 4) & 0b11)?;
        let third_party_copy = (header[5] & (1 << 3)) != 0;
        let protect = (header[5] & (1 << 0)) != 0;
        let enclosure_services = (header[6] & (1 << 6)) != 0;
        let vendor_specific1 = (header[6] & (1 << 5)) != 0;
        let multiple_port = (header[6] & (1 << 4)) != 0;
        let command_queuing = (header[7] & (1 << 1)) != 0;
        let vendor_specific2 = (header[7] & (1 << 0)) != 0;
        let vendor_id = Self::parse_string(&header[8..16])?;
        let product_id = Self::parse_string(&header[16..32])?;
        let product_revision = Self::parse_string(&header[32..36])?;
        let vendor_specific3 = data.get(36..).and_then(<[_]>::first_chunk).copied();

        let version_descriptors = data
            .get(58..)
            .unwrap_or_default()
            .chunks_exact(size_of::<u16>())
            .take(8)
            .map(|x| u16::from_be_bytes([x[0], x[1]]))
            .filter(|&x| x != 0)
            .collect();

        let vendor_specific4 = data.get(96..).unwrap_or_default().into();

        Ok(Self {
            peripheral_qualifier,
            peripheral_device_type,
            removable_medium,
            lu_conglomerate,
            hot_pluggable,
            version,
            normal_aca,
            hierarchical_support,
            scc_support,
            tpg_support,
            third_party_copy,
            protect,
            enclosure_services,
            vendor_specific1,
            multiple_port,
            command_queuing,
            vendor_specific2,
            vendor_id,
            product_id,
            product_revision,
            vendor_specific3,
            version_descriptors,
            vendor_specific4,
        })
    }
}

impl std::fmt::Display for Inquiry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "inquiry (vendor: {}, product: {}, revision: {})",
            self.vendor_id, self.product_id, self.product_revision
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_inquiry() {
        const DATA_VALID: &[&[u8]] = &[
            test_data::kingston_datatraveler3::INQUIRY,
            test_data::asmedia_asmt2235::INQUIRY,
        ];
        const DATA_INVALID: &[&[u8]] = &[&[0; 128], &[0xFF; 256]];

        for &data in DATA_VALID {
            assert!(Inquiry::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(Inquiry::try_from(data).is_err());
        }
    }
}
