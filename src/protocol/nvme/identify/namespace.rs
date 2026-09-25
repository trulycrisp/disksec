//! Identify namespace result parsing.

use crate::protocol::nvme::PAGE_SIZE;

/// Identify namespace error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid protection information type value.
    ProtectionType(u8),
    /// Invalid deallocated logical block read behavior value.
    ReadBehavior(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ProtectionType(x) => write!(f, "invalid protection type {x:#x}"),
            Self::ReadBehavior(x) => write!(f, "invalid read behavior {x:#x}"),
        }
    }
}

/// Namespace features field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct NamespaceFeatures {
    /// Thin provisioning is supported.
    pub thin_provisioning: bool,
    /// Namespace specific atomic boundary and power fail fields are defined.
    pub atomic_boundary: bool,
    /// Deallocated or unwritten logical block error is supported.
    pub deallocated_error: bool,
    /// Namespace identifiers are reused after the namespace is deleted.
    pub uid_reuse: bool,
    /// Optimal write alignment and granularity fields are defined.
    pub optimal_performance: u8,
    /// Multiple atomicity mode applies to writes to this namespace.
    pub multiple_atomicity_mode: bool,
    /// Optimal read alignment and granularity fields are defined.
    pub optimal_read_performance: bool,
}

impl From<u8> for NamespaceFeatures {
    fn from(value: u8) -> Self {
        Self {
            thin_provisioning: (value & (1 << 0)) != 0,
            atomic_boundary: (value & (1 << 1)) != 0,
            deallocated_error: (value & (1 << 2)) != 0,
            uid_reuse: (value & (1 << 3)) != 0,
            optimal_performance: (value >> 4) & 0b11,
            multiple_atomicity_mode: (value & (1 << 6)) != 0,
            optimal_read_performance: (value & (1 << 7)) != 0,
        }
    }
}

/// Metadata capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct MetadataCapabilities {
    /// Metadata may be transferred at the end of the logical block.
    pub extended_lba: bool,
    /// Metadata may be transferred in a separate contiguous buffer.
    pub separate_buffer: bool,
}

impl From<u8> for MetadataCapabilities {
    fn from(value: u8) -> Self {
        Self {
            extended_lba: (value & (1 << 0)) != 0,
            separate_buffer: (value & (1 << 1)) != 0,
        }
    }
}

/// End-to-end data protection capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtectionCapabilities {
    /// Protection information type 1 is supported.
    pub type1: bool,
    /// Protection information type 2 is supported.
    pub type2: bool,
    /// Protection information type 3 is supported.
    pub type3: bool,
    /// Protection information may be in the first bytes of metadata.
    pub first_bytes: bool,
    /// Protection information may be in the last bytes of metadata.
    pub last_bytes: bool,
}

impl From<u8> for ProtectionCapabilities {
    fn from(value: u8) -> Self {
        Self {
            type1: (value & (1 << 0)) != 0,
            type2: (value & (1 << 1)) != 0,
            type3: (value & (1 << 2)) != 0,
            first_bytes: (value & (1 << 3)) != 0,
            last_bytes: (value & (1 << 4)) != 0,
        }
    }
}

/// Protection information type field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ProtectionType {
    /// Type 1 protection.
    Type1 = 0x1,
    /// Type 2 protection.
    Type2 = 0x2,
    /// Type 3 protection.
    Type3 = 0x3,
}

impl ProtectionType {
    /// Parse protection information type field, absent when disabled.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[ProtectionType] = &[
            ProtectionType::Type1,
            ProtectionType::Type2,
            ProtectionType::Type3,
        ];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::ProtectionType(value))
            .map(Some)
    }
}

impl std::fmt::Display for ProtectionType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Type1 => write!(f, "type 1"),
            Self::Type2 => write!(f, "type 2"),
            Self::Type3 => write!(f, "type 3"),
        }
    }
}

/// End-to-end data protection type settings field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ProtectionSettings {
    /// Enabled protection information type.
    pub protection_type: Option<ProtectionType>,
    /// Protection information is in the first instead of last bytes of
    /// metadata.
    pub first_bytes: bool,
}

impl TryFrom<u8> for ProtectionSettings {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            protection_type: ProtectionType::parse(value & 0b111)?,
            first_bytes: (value & (1 << 3)) != 0,
        })
    }
}

/// Reservation capabilities field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ReservationCapabilities {
    /// Reservations persist across power loss.
    pub persist_through_power_loss: bool,
    /// Write Exclusive reservation type is supported.
    pub write_exclusive: bool,
    /// Exclusive Access reservation type is supported.
    pub exclusive_access: bool,
    /// Write Exclusive - Registrants Only reservation type is supported.
    pub write_exclusive_registrants: bool,
    /// Exclusive Access - Registrants Only reservation type is supported.
    pub exclusive_access_registrants: bool,
    /// Write Exclusive - All Registrants reservation type is supported.
    pub write_exclusive_all_registrants: bool,
    /// Exclusive Access - All Registrants reservation type is supported.
    pub exclusive_access_all_registrants: bool,
    /// Ignore Existing Key is used as defined in revision 1.3 or later.
    pub ignore_existing_key: bool,
}

impl From<u8> for ReservationCapabilities {
    fn from(value: u8) -> Self {
        Self {
            persist_through_power_loss: (value & (1 << 0)) != 0,
            write_exclusive: (value & (1 << 1)) != 0,
            exclusive_access: (value & (1 << 2)) != 0,
            write_exclusive_registrants: (value & (1 << 3)) != 0,
            exclusive_access_registrants: (value & (1 << 4)) != 0,
            write_exclusive_all_registrants: (value & (1 << 5)) != 0,
            exclusive_access_all_registrants: (value & (1 << 6)) != 0,
            ignore_existing_key: (value & (1 << 7)) != 0,
        }
    }
}

/// Read behavior of a deallocated logical block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ReadBehavior {
    /// All bytes read as zero.
    Zeroes = 0x1,
    /// All bytes read as one.
    Ones = 0x2,
}

impl ReadBehavior {
    /// Parse read behavior field, absent when not reported.
    fn parse(value: u8) -> Result<Option<Self>, Error> {
        const VARIANTS: &[ReadBehavior] = &[ReadBehavior::Zeroes, ReadBehavior::Ones];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::ReadBehavior(value))
            .map(Some)
    }
}

impl std::fmt::Display for ReadBehavior {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Zeroes => write!(f, "zeroes"),
            Self::Ones => write!(f, "ones"),
        }
    }
}

/// Deallocated logical block features field.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DeallocateFeatures {
    /// Read behavior of a deallocated logical block.
    pub read_behavior: Option<ReadBehavior>,
    /// Deallocate bit is supported in the WRITE ZEROES command.
    pub write_zeroes: bool,
    /// Guard field of a deallocated logical block is set to its CRC.
    pub guard_crc: bool,
}

impl TryFrom<u8> for DeallocateFeatures {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        Ok(Self {
            read_behavior: ReadBehavior::parse(value & 0b111)?,
            write_zeroes: (value & (1 << 3)) != 0,
            guard_crc: (value & (1 << 4)) != 0,
        })
    }
}

/// Relative performance of an LBA format.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RelativePerformance {
    /// Best performance.
    Best,
    /// Better performance.
    Better,
    /// Good performance.
    Good,
    /// Degraded performance.
    Degraded,
}

impl From<u8> for RelativePerformance {
    fn from(value: u8) -> Self {
        match value & 0b11 {
            0x0 => Self::Best,
            0x1 => Self::Better,
            0x2 => Self::Good,
            _ => Self::Degraded,
        }
    }
}

impl std::fmt::Display for RelativePerformance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Best => write!(f, "best"),
            Self::Better => write!(f, "better"),
            Self::Good => write!(f, "good"),
            Self::Degraded => write!(f, "degraded"),
        }
    }
}

/// LBA format data structure.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LbaFormat {
    /// Metadata bytes provided per logical block.
    pub metadata_size: u16,
    /// Logical block data size in bytes.
    pub data_size: Option<u32>,
    /// Relative performance of the format.
    pub relative_performance: RelativePerformance,
}

impl From<u32> for LbaFormat {
    fn from(value: u32) -> Self {
        const MIN_DATA_SIZE_POWER: u8 = 9;

        let [metadata_size @ .., data_size, relative_performance] = value.to_le_bytes();

        // Data size is a power of two
        let data_size = (data_size >= MIN_DATA_SIZE_POWER)
            .then(|| 1u32.checked_shl(data_size.into()))
            .flatten();

        Self {
            metadata_size: u16::from_le_bytes(metadata_size),
            data_size,
            relative_performance: relative_performance.into(),
        }
    }
}

impl LbaFormat {
    /// Parse supported LBA formats from an identify namespace data structure.
    pub(crate) fn parse_list(data: &[u8; Identify::SIZE]) -> Box<[Self]> {
        const DWORD_SIZE: usize = size_of::<u32>();
        const LBA_FORMAT_OFFSET: usize = 128;
        const LBA_FORMAT_MAX: usize = 64;

        // Number of formats with common attributes
        let common_count = usize::from(data[25]) + 1;
        // Number of formats with unique attributes
        let unique_count = usize::from(data[82]);
        // Total count of all formats, limited by maximum
        let count = (common_count + unique_count).min(LBA_FORMAT_MAX);

        data[LBA_FORMAT_OFFSET..]
            .as_chunks::<DWORD_SIZE>()
            .0
            .iter()
            .take(count)
            .map(|&x| u32::from_le_bytes(x).into())
            .collect()
    }
}

impl std::fmt::Display for LbaFormat {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let data_size = self
            .data_size
            .map_or_else(|| "N/A".to_string(), |x| x.to_string());

        write!(
            f,
            "(data: {data_size}, metadata: {}, performance: {})",
            self.metadata_size, self.relative_performance
        )
    }
}

/// Identify namespace data structure.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Identify {
    /// Total size of the namespace in logical blocks.
    pub size: u64,
    /// Maximum number of logical blocks that may be allocated.
    pub capacity: u64,
    /// Number of logical blocks currently allocated.
    pub utilization: u64,
    /// Namespace features.
    pub features: NamespaceFeatures,
    /// Index of the LBA format the namespace is formatted with.
    pub lba_format_index: u8,
    /// Metadata is transferred at the end of the logical block.
    pub metadata_extended: bool,
    /// Metadata capabilities.
    pub metadata: MetadataCapabilities,
    /// End-to-end data protection capabilities.
    pub protection_capabilities: ProtectionCapabilities,
    /// End-to-end data protection settings.
    pub protection: ProtectionSettings,
    /// Namespace may be attached to two or more controllers.
    pub shared: bool,
    /// Reservation capabilities.
    pub reservations: ReservationCapabilities,
    /// Percentage of the namespace remaining to be formatted.
    pub format_remaining: Option<u8>,
    /// Deallocated logical block features.
    pub deallocate: DeallocateFeatures,
    /// Atomic write size in logical blocks, `None` when the controller value
    /// applies.
    pub atomic_write: Option<u32>,
    /// Atomic write size on power failure in logical blocks, `None` when the
    /// controller value applies.
    pub atomic_write_power_fail: Option<u32>,
    /// Atomic compare and write size in logical blocks, `None` when the
    /// controller value applies.
    pub atomic_compare_write: Option<u32>,
    /// Atomic boundary size in logical blocks, `None` when the controller
    /// value applies.
    pub atomic_boundary_size: Option<u32>,
    /// Atomic boundary offset in logical blocks, `None` when the controller
    /// value applies.
    pub atomic_boundary_offset: Option<u16>,
    /// Atomic boundary size on power failure in logical blocks, `None` when
    /// the controller value applies.
    pub atomic_boundary_size_power_fail: Option<u32>,
    /// Optimal I/O boundary in logical blocks.
    pub optimal_io_boundary: Option<u16>,
    /// Total capacity of the namespace in bytes.
    pub nvm_capacity: Option<u128>,
    /// Preferred write granularity in logical blocks.
    pub preferred_write_granularity: Option<u32>,
    /// Preferred write alignment in logical blocks.
    pub preferred_write_alignment: Option<u32>,
    /// Preferred deallocate granularity in logical blocks.
    pub preferred_deallocate_granularity: Option<u32>,
    /// Preferred deallocate alignment in logical blocks.
    pub preferred_deallocate_alignment: Option<u32>,
    /// Optimal write size in logical blocks.
    pub optimal_write_size: Option<u32>,
    /// Maximum length of a single COPY source range in logical blocks.
    pub max_source_range_size: Option<u16>,
    /// Maximum length of a COPY command in logical blocks.
    pub max_copy_size: Option<u32>,
    /// Maximum number of COPY source ranges.
    pub max_source_ranges: Option<u16>,
    /// Asymmetric Namespace Access Group Identifier.
    pub ana_group_id: u32,
    /// Namespace is currently write protected.
    pub write_protected: bool,
    /// NVM Set Identifier.
    pub nvm_set_id: u16,
    /// Endurance Group Identifier.
    pub endurance_group_id: u16,
    /// Namespace Globally Unique Identifier.
    pub guid: [u8; 16],
    /// IEEE Extended Unique Identifier.
    pub eui64: [u8; 8],
    /// Supported LBA formats.
    pub lba_formats: Box<[LbaFormat]>,
}

impl Identify {
    /// Size in bytes.
    pub(crate) const SIZE: usize = PAGE_SIZE;

    /// Get the active LBA format.
    pub(crate) fn lba_format(&self) -> Option<LbaFormat> {
        self.lba_formats
            .get(usize::from(self.lba_format_index))
            .copied()
    }
}

impl TryFrom<&[u8; Identify::SIZE]> for Identify {
    type Error = Error;

    fn try_from(data: &[u8; Identify::SIZE]) -> Result<Self, Self::Error> {
        let size = u64::from_le_bytes(*data[0..8].first_chunk().unwrap());
        let capacity = u64::from_le_bytes(*data[8..16].first_chunk().unwrap());
        let utilization = u64::from_le_bytes(*data[16..24].first_chunk().unwrap());
        let features = NamespaceFeatures::from(data[24]);

        // Format index is split, with the upper bits above the metadata bit
        let lba_format_index = (data[26] & 0b1111) | (((data[26] >> 5) & 0b11) << 4);
        let metadata_extended = (data[26] & (1 << 4)) != 0;

        let metadata = data[27].into();
        let protection_capabilities = data[28].into();
        let protection = data[29].try_into()?;
        let shared = (data[30] & (1 << 0)) != 0;
        let reservations = data[31].into();

        // Percentage is only valid when the indicator is supported
        let format_remaining = ((data[32] & (1 << 7)) != 0).then(|| data[32] & 0b111_1111);

        let deallocate = data[33].try_into()?;

        // Atomic fields are only defined when reported by features
        let atomic = |low: usize| {
            features
                .atomic_boundary
                .then(|| u16::from_le_bytes([data[low], data[low + 1]]))
                .filter(|&x| x != 0)
                .map(|x| u32::from(x) + 1)
        };

        let atomic_write = atomic(34);
        let atomic_write_power_fail = atomic(36);
        let atomic_compare_write = atomic(38);
        let atomic_boundary_size = atomic(40);

        // Boundary offset is not a zero-based count
        let atomic_boundary_offset = features
            .atomic_boundary
            .then(|| u16::from_le_bytes([data[42], data[43]]));

        let atomic_boundary_size_power_fail = atomic(44);
        let optimal_io_boundary =
            Some(u16::from_le_bytes([data[46], data[47]])).filter(|&x| x != 0);

        let nvm_capacity =
            Some(u128::from_le_bytes(*data[48..64].first_chunk().unwrap())).filter(|&x| x != 0);

        // Preferred granularity fields are only defined when reported by
        // features
        let preferred = |low: usize| {
            (features.optimal_performance != 0)
                .then(|| u32::from(u16::from_le_bytes([data[low], data[low + 1]])) + 1)
        };

        let preferred_write_granularity = preferred(64);
        let preferred_write_alignment = preferred(66);
        let preferred_deallocate_granularity = matches!(features.optimal_performance, 0b01 | 0b11)
            .then(|| u32::from(u16::from_le_bytes([data[68], data[69]])) + 1);
        let preferred_deallocate_alignment = preferred(70);
        let optimal_write_size = preferred(72);
        // Copy fields are only defined when the copy command is supported,
        // indicated by a non-zero source range size
        let source_range_size = u16::from_le_bytes([data[74], data[75]]);
        let copy = source_range_size != 0;

        let max_source_range_size = copy.then_some(source_range_size);
        let max_copy_size = copy.then(|| u32::from_le_bytes(*data[76..80].first_chunk().unwrap()));
        let max_source_ranges = copy.then(|| u16::from(data[80]) + 1);

        let ana_group_id = u32::from_le_bytes(*data[92..96].first_chunk().unwrap());
        let write_protected = (data[99] & (1 << 0)) != 0;
        let nvm_set_id = u16::from_le_bytes([data[100], data[101]]);
        let endurance_group_id = u16::from_le_bytes([data[102], data[103]]);
        let guid = *data[104..120].first_chunk().unwrap();
        let eui64 = *data[120..128].first_chunk().unwrap();

        let lba_formats = LbaFormat::parse_list(data);

        Ok(Self {
            size,
            capacity,
            utilization,
            features,
            lba_format_index,
            metadata_extended,
            metadata,
            protection_capabilities,
            protection,
            shared,
            reservations,
            format_remaining,
            deallocate,
            atomic_write,
            atomic_write_power_fail,
            atomic_compare_write,
            atomic_boundary_size,
            atomic_boundary_offset,
            atomic_boundary_size_power_fail,
            optimal_io_boundary,
            nvm_capacity,
            preferred_write_granularity,
            preferred_write_alignment,
            preferred_deallocate_granularity,
            preferred_deallocate_alignment,
            optimal_write_size,
            max_source_range_size,
            max_copy_size,
            max_source_ranges,
            ana_group_id,
            write_protected,
            nvm_set_id,
            endurance_group_id,
            guid,
            eui64,
            lba_formats,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn identify_parse() {
        const DATA_VALID: &[&[u8; Identify::SIZE]] = &[test_data::patriot_p300::IDENTIFY_NAMESPACE];
        const DATA_INVALID: &[&[u8; Identify::SIZE]] = &[&[0xFF; _]];

        for &data in DATA_VALID {
            assert!(Identify::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(Identify::try_from(data).is_err());
        }
    }
}
