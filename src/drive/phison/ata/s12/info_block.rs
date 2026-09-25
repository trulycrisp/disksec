//! Info block parsing.

use super::Error;
use crate::drive::phison::INFO_BLOCK_MAGIC;

/// Parse string.
fn parse_str(data: &[u8]) -> Option<&str> {
    data.iter()
        .all(|&x| x == b' ' || x.is_ascii_graphic())
        .then(|| std::str::from_utf8(data).unwrap())
}

/// Info block (drive configuration).
#[derive(Clone, Debug, PartialEq, Eq)]
pub(super) struct InfoBlock {
    /// Serial number.
    pub(super) serial: String,
    /// Model.
    pub(super) model: String,
}

impl InfoBlock {
    /// Size in bytes.
    pub(super) const SIZE: usize = 4_096;

    /// Parse string.
    fn parse_string(data: &[u8]) -> Result<String, Error> {
        parse_str(data)
            .map(|x| x.trim_end().to_string())
            .ok_or(Error::InvalidInfoBlock)
    }
}

impl TryFrom<&[u8; Self::SIZE]> for InfoBlock {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        if !data.starts_with(INFO_BLOCK_MAGIC) {
            return Err(Error::InvalidInfoBlock);
        }

        let serial = Self::parse_string(&data[6..26])?;
        let model = Self::parse_string(&data[34..74])?;

        Ok(Self { serial, model })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_info_block() {
        const DATA_VALID: &[&[u8; InfoBlock::SIZE]] = &[test_data::kingston_dc500r::INFO_BLOCK];
        const DATA_INVALID: &[&[u8; InfoBlock::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(InfoBlock::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(InfoBlock::try_from(data).is_err());
        }
    }
}
