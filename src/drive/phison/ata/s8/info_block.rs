//! Info block parsing.

use super::Error;
use crate::drive::phison::INFO_BLOCK_MAGIC;
use crate::protocol::ata::{SECTOR_SIZE, identify};

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
    pub(super) const SIZE: usize = SECTOR_SIZE;
}

impl TryFrom<&[u8; Self::SIZE]> for InfoBlock {
    type Error = Error;

    fn try_from(data: &[u8; Self::SIZE]) -> Result<Self, Self::Error> {
        if !data.starts_with(INFO_BLOCK_MAGIC) {
            return Err(Error::InvalidInfoBlock);
        }

        let serial = identify::parse_string(&data[16..36]).or(Err(Error::InvalidInfoBlock))?;
        let model = identify::parse_string(&data[36..76]).or(Err(Error::InvalidInfoBlock))?;

        Ok(Self { serial, model })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn parse_info_block() {
        const DATA_VALID: &[&[u8; InfoBlock::SIZE]] = &[
            test_data::kingston_ssdnow100::INFO_BLOCK,
            test_data::apacer_sfd25hm::INFO_BLOCK,
        ];
        const DATA_INVALID: &[&[u8; InfoBlock::SIZE]] = &[&[0; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(InfoBlock::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(InfoBlock::try_from(data).is_err());
        }
    }
}
