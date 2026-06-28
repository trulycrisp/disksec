//! SATA PHY Event Counters log.

use super::PAGE_SIZE;

/// PHY Event Counters error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Invalid counter ID value.
    InvalidCounterId(u16),
    /// Invalid counter entry size value.
    InvalidEntrySize(u8),
    /// Data too short (truncated).
    Truncated,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCounterId(x) => {
                write!(f, "invalid counter id {x:#x}")
            },
            Self::InvalidEntrySize(x) => {
                write!(f, "invalid entry size code {x}")
            },
            Self::Truncated => {
                write!(f, "truncated")
            },
        }
    }
}

/// PHY event counter ID.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum CounterId {
    /// Interface CRC error.
    IcrcError = 0x1,
    /// `R_ERR` response for all data FISes (both directions).
    RErrData = 0x2,
    /// `R_ERR` response for device-to-host data FISes.
    RErrDataD2H = 0x3,
    /// `R_ERR` response for host-to-device data FISes.
    RErrDataH2D = 0x4,
    /// `R_ERR` response for all non-data FISes (both directions).
    RErrNondata = 0x5,
    /// `R_ERR` response for device-to-host non-data FISes.
    RErrNondataD2H = 0x6,
    /// `R_ERR` response for host-to-device non-data FISes.
    RErrNondataH2D = 0x7,
    /// Non-Data FIS retries.
    NondataFisRetries = 0x8,
    /// PHY transitioned from `PhyRdy` to `PhyNRdy` (link drop-out).
    PhyrdyToPhynrdy = 0x9,
    /// Signature device-to-host Register FIS received after COMRESET.
    ComresetFis = 0xA,
    /// CRC error on host-to-device FIS.
    H2DCrcError = 0xB,
    /// Non-CRC error on host-to-device FIS.
    H2DNonCrcError = 0xD,
    /// `R_ERR` response for host-to-device Data FIS due to CRC error.
    RErrDataH2DCrc = 0xF,
    /// `R_ERR` response for host-to-device Data FIS due to non-CRC error.
    RErrDataH2DNonCrc = 0x10,
    /// `R_ERR` response for host-to-device non-Data FIS due to CRC error.
    RErrNondataH2DCrc = 0x12,
    /// `R_ERR` response for host-to-device non-Data FIS due to non-CRC error.
    RErrNondataH2DNonCrc = 0x13,
    /// Vendor-specific counter.
    VendorSpecific(u16),
}

impl CounterId {
    /// Parse from raw value.
    fn parse(value: u16) -> Result<Option<Self>, Error> {
        const ID_MASK: u16 = 0xFFF;
        const VENDOR_SPECIFIC_MASK: u16 = 1 << 15;
        const CONST_VARIANTS: &[CounterId] = &[
            CounterId::IcrcError,
            CounterId::RErrData,
            CounterId::RErrDataD2H,
            CounterId::RErrDataH2D,
            CounterId::RErrNondata,
            CounterId::RErrNondataD2H,
            CounterId::RErrNondataH2D,
            CounterId::NondataFisRetries,
            CounterId::PhyrdyToPhynrdy,
            CounterId::ComresetFis,
            CounterId::H2DCrcError,
            CounterId::H2DNonCrcError,
            CounterId::RErrDataH2DCrc,
            CounterId::RErrDataH2DNonCrc,
            CounterId::RErrNondataH2DCrc,
            CounterId::RErrNondataH2DNonCrc,
        ];

        let id = value & ID_MASK;

        if id == 0 {
            return Ok(None);
        }

        if value & VENDOR_SPECIFIC_MASK != 0 {
            return Ok(Some(Self::VendorSpecific(id | VENDOR_SPECIFIC_MASK)));
        }

        if let Some(x) = CONST_VARIANTS
            .iter()
            .find(|&&y| Into::<u16>::into(y) == id)
            .copied()
        {
            return Ok(Some(x));
        }

        Err(Error::InvalidCounterId(id))
    }
}

impl From<CounterId> for u16 {
    fn from(value: CounterId) -> Self {
        match value {
            CounterId::VendorSpecific(x) => x,
            ref x => unsafe { std::mem::transmute_copy(x) },
        }
    }
}

/// Counter entry.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Counter {
    /// Counter ID.
    pub id: CounterId,
    /// Counter value.
    pub value: u64,
}

/// PHY Event Counters log.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct PhyEventCounters(Box<[Counter]>);

impl TryFrom<&[u8; PAGE_SIZE]> for PhyEventCounters {
    type Error = super::Error;

    fn try_from(data: &[u8; PAGE_SIZE]) -> Result<Self, Self::Error> {
        const ENTRIES_START: usize = 4;
        const ENTRIES_END: usize = 508;
        const ENTRY_HEADER_SIZE: usize = size_of::<u16>();
        const ENTRY_SIZE_CODES: std::ops::Range<u8> = 1..5;

        super::validate_checksum(data)?;

        let mut entries_data = &data[ENTRIES_START..ENTRIES_END];
        let mut entries = Vec::new();

        while !entries_data.is_empty() {
            let header;
            (header, entries_data) = entries_data
                .split_first_chunk::<{ ENTRY_HEADER_SIZE }>()
                .ok_or(Error::Truncated)?;
            let header = u16::from_le_bytes(*header);

            let Some(id) = CounterId::parse(header)? else {
                break;
            };

            let size_code = ((header >> 12) & 0b111) as _;

            if !ENTRY_SIZE_CODES.contains(&size_code) {
                return Err(Error::InvalidEntrySize(size_code).into());
            }

            let size = (size_code as usize) * 2;
            let value;
            (value, entries_data) = entries_data
                .split_at_checked(size)
                .ok_or(Error::Truncated)?;
            let mut value_buf = [0u8; size_of::<u64>()];
            value_buf[..value.len()].copy_from_slice(value);
            let value = u64::from_le_bytes(value_buf);

            entries.push(Counter { id, value });
        }

        Ok(Self(entries.into()))
    }
}

impl std::ops::Deref for PhyEventCounters {
    type Target = [Counter];

    fn deref(&self) -> &[Counter] {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test_data;

    #[test]
    fn counter_id_parse() {
        assert_eq!(CounterId::parse(0), Ok(None));
        assert_eq!(CounterId::parse(0xF000), Ok(None));
        assert_eq!(
            CounterId::parse(0x9234),
            Ok(Some(CounterId::VendorSpecific(0x8234)))
        );
        assert_eq!(CounterId::parse(0xC), Err(Error::InvalidCounterId(0xC)));
        assert_eq!(CounterId::parse(0x100C), Err(Error::InvalidCounterId(0xC)));
    }

    #[test]
    fn phy_event_counters_parse() {
        const DATA_VALID: &[&[u8; PAGE_SIZE]] = &[
            test_data::westerndigital_scorpioblack::GP_LOG_11H,
            test_data::kingston_a400::GP_LOG_11H,
        ];
        const DATA_INVALID: &[&[u8; PAGE_SIZE]] = &[&[0x01; _], &[0xFF; _]];

        for &data in DATA_VALID {
            assert!(PhyEventCounters::try_from(data).is_ok());
        }

        for &data in DATA_INVALID {
            assert!(PhyEventCounters::try_from(data).is_err());
        }
    }
}
