//! VUC sense definitions.

use super::ErrorCode;

/// VUC sense error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid version.
    Version(u16),
    /// Invalid additional size.
    AdditionalSize(u16),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Version(x) => write!(f, "invalid version {x:#x}"),
            Self::AdditionalSize(x) => write!(f, "invalid additional size {x}"),
        }
    }
}

/// VUC sense.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Sense {
    /// Main error code.
    pub principal_error: Option<ErrorCode>,
    /// Additional error code.
    pub supplemental_error: Option<ErrorCode>,
    /// Pending transfer size in 512-byte sectors.
    pub sector_count: u32,
    /// Command-specific return data.
    pub return_data: [u8; 8],
    /// Additional data with unknown purpose, always observed empty.
    pub additional: Box<[u8]>,
}

impl Sense {
    /// Size in bytes.
    pub(crate) const SIZE: usize = crate::protocol::ata::SECTOR_SIZE;
}

impl TryFrom<&[u8; Sense::SIZE]> for Sense {
    type Error = Error;

    fn try_from(value: &[u8; Sense::SIZE]) -> Result<Self, Self::Error> {
        const VERSION: u16 = 0x100;
        const ADDITIONAL_OFFSET: usize = 34;

        let version = u16::from_le_bytes([value[0], value[1]]);
        if version != VERSION {
            return Err(Error::Version(version));
        }

        let principal_error =
            ErrorCode::parse(u32::from_le_bytes([value[2], value[3], value[4], value[5]]));
        let supplemental_error =
            ErrorCode::parse(u32::from_le_bytes([value[6], value[7], value[8], value[9]]));
        let sector_count = u32::from_le_bytes([value[10], value[11], value[12], value[13]]);
        let return_data = std::array::from_fn(|i| value[14 + i]);

        let additional_size = u16::from_le_bytes([value[32], value[33]]);
        if ADDITIONAL_OFFSET + usize::from(additional_size) > value.len() {
            return Err(Error::AdditionalSize(additional_size));
        }

        let additional =
            (&value[ADDITIONAL_OFFSET..ADDITIONAL_OFFSET + usize::from(additional_size)]).into();

        Ok(Self {
            principal_error,
            supplemental_error,
            sector_count,
            return_data,
            additional,
        })
    }
}
