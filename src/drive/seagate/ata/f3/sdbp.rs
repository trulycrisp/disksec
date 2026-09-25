//! Seagate Diagnostic Bridge Protocol functionality.

use crate::protocol::scsi::sense;

/// SDBP error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Error {
    /// Data truncated.
    Truncated,
    /// Invalid port value.
    InvalidPort(u16),
    /// Invalid DITS Diagnostic Status Block.
    InvalidDitsDsb,
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated"),
            Self::InvalidPort(x) => write!(f, "invalid port {x}"),
            Self::InvalidDitsDsb => write!(f, "invalid DITS DSB"),
        }
    }
}

/// SDBP port.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Port {
    /// Diagnostic Internal Test Service.
    Dits = 1,
    /// Diagnostic External Test Service.
    Dets = 7,
}

impl TryFrom<u16> for Port {
    type Error = Error;

    fn try_from(value: u16) -> Result<Self, Self::Error> {
        match value {
            x if x == Self::Dits as _ => Ok(Self::Dits),
            x if x == Self::Dets as _ => Ok(Self::Dets),
            x => Err(Error::InvalidPort(x)),
        }
    }
}

impl std::fmt::Display for Port {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dits => write!(f, "DITS"),
            Self::Dets => write!(f, "DETS"),
        }
    }
}

/// Diagnostic Function Block.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub(super) struct Dfb<'a> {
    /// Function ID.
    pub(super) function: u16,
    /// Function revision ID.
    pub(super) revision: u16,
    /// Function-specific parameter data.
    pub(super) data: &'a [u8],
}

impl Dfb<'_> {
    /// Header size in bytes.
    pub(super) const HEADER_SIZE: usize = 4;
}

impl From<&Dfb<'_>> for Box<[u8]> {
    fn from(value: &Dfb) -> Self {
        let mut dfb = Vec::with_capacity(Dfb::HEADER_SIZE + value.data.len());

        dfb.extend_from_slice(&value.function.to_le_bytes());
        dfb.extend_from_slice(&value.revision.to_le_bytes());
        dfb.extend_from_slice(value.data);

        dfb.into()
    }
}

/// DITS Diagnostic Status Block.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct DitsDsb<'a> {
    /// SCSI sense key.
    pub(super) sense_key: sense::SenseKey,
    /// SCSI ASC/ASCQ.
    pub(super) asc: sense::asc::AdditionalSenseCode,
    /// Function-specific status data.
    pub(super) data: &'a [u8],
}

impl DitsDsb<'_> {
    /// Header size in bytes.
    pub(super) const HEADER_SIZE: usize = 16;
}

impl<'a> TryFrom<&'a [u8]> for DitsDsb<'a> {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let (header, data) = value
            .split_first_chunk::<{ DitsDsb::HEADER_SIZE }>()
            .ok_or(Error::Truncated)?;

        let response_code = sense::ResponseCode::try_from(header[0] & sense::ResponseCode::MASK)
            .or(Err(Error::InvalidDitsDsb))?;
        if response_code != sense::ResponseCode::DescriptorCurrent {
            return Err(Error::InvalidDitsDsb);
        }

        let sense_key =
            sense::SenseKey::try_from(header[1] & 0xF).or(Err(Error::InvalidDitsDsb))?;
        let asc = sense::asc::AdditionalSenseCode::parse(header[2], header[3])
            .or(Err(Error::InvalidDitsDsb))?;

        Ok(Self {
            sense_key,
            asc,
            data,
        })
    }
}

/// SDBP packet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct Packet<'a> {
    /// Source port.
    pub(super) from: Port,
    /// Destination port.
    pub(super) to: Port,
    /// Packet payload.
    pub(super) data: &'a [u8],
}

impl Packet<'_> {
    /// Header size in bytes.
    pub(super) const HEADER_SIZE: usize = 8;
}

impl<'a> TryFrom<&'a [u8]> for Packet<'a> {
    type Error = Error;

    fn try_from(value: &'a [u8]) -> Result<Self, Self::Error> {
        let (header, data) = value
            .split_first_chunk::<{ Packet::HEADER_SIZE }>()
            .ok_or(Error::Truncated)?;

        let from = u16::from_le_bytes([header[0], header[1]]).try_into()?;
        let to = u16::from_le_bytes([header[2], header[3]]).try_into()?;
        let data_size = u32::from_le_bytes([header[4], header[5], header[6], header[7]]);

        let data = data.get(..data_size as _).ok_or(Error::Truncated)?;

        Ok(Self { from, to, data })
    }
}

impl From<&Packet<'_>> for Box<[u8]> {
    fn from(value: &Packet) -> Self {
        let data_size = u32::try_from(value.data.len()).unwrap();

        let mut header = [0u8; Packet::HEADER_SIZE];
        header[..2].copy_from_slice(&(value.from as u16).to_le_bytes());
        header[2..4].copy_from_slice(&(value.to as u16).to_le_bytes());
        header[4..].copy_from_slice(&data_size.to_le_bytes());

        let mut data = Vec::with_capacity(header.len() + value.data.len());
        data.extend_from_slice(&header);
        data.extend_from_slice(value.data);

        data.into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::scsi::sense;

    #[test]
    fn packet_to_bytes() {
        const PORT: Port = Port::Dits;
        const PAYLOAD: &[u8] = &[0, 1, 2, 3];
        const PACKET_DATA: &[u8] = &[
            0x01, 0x00, 0x01, 0x00, 0x04, 0x00, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03,
        ];

        let packet = Packet {
            from: PORT,
            to: PORT,
            data: PAYLOAD,
        };

        let packet_data = <Box<[_]>>::from(&packet);

        assert_eq!(packet_data.as_ref(), PACKET_DATA);
    }

    #[test]
    fn packet_from_bytes() {
        const PACKET_DATA: &[u8] = &[
            0x01, 0x00, 0x07, 0x00, 0x04, 0x00, 0x00, 0x00, 0x01, 0x02, 0x03, 0x04,
        ];
        const PORT_FROM: Port = Port::Dits;
        const PORT_TO: Port = Port::Dets;
        const PAYLOAD: &[u8] = &[1, 2, 3, 4];

        let packet = Packet::try_from(PACKET_DATA).unwrap();

        assert_eq!(packet.from, PORT_FROM);
        assert_eq!(packet.to, PORT_TO);
        assert_eq!(packet.data, PAYLOAD);
    }

    #[test]
    fn packet_bytes_symmetric() {
        const PORT: Port = Port::Dits;
        const PAYLOAD: &[u8] = &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

        let packet_in = Packet {
            from: PORT,
            to: PORT,
            data: PAYLOAD,
        };

        let packet_data = <Box<[_]>>::from(&packet_in);

        let packet_out = Packet::try_from(packet_data.as_ref()).unwrap();

        assert_eq!(packet_in, packet_out);
    }

    #[test]
    fn dfb_to_bytes() {
        const FUNCTION: u16 = 1;
        const REVISION: u16 = 2;
        const PAYLOAD: &[u8] = &[0xAA, 0xBB, 0xCC, 0xDD];
        const DFB_DATA: &[u8] = &[0x01, 0x00, 0x02, 0x0, 0xAA, 0xBB, 0xCC, 0xDD];

        let dfb = Dfb {
            function: FUNCTION,
            revision: REVISION,
            data: PAYLOAD,
        };

        let dfb_data = <Box<[_]>>::from(&dfb);

        assert_eq!(dfb_data.as_ref(), DFB_DATA);
    }

    #[test]
    fn dits_dsb_from_bytes() {
        const DSB_DATA: &[u8] = &[
            0x72, 0x07, 0x20, 0x02, 0x00, 0x00, 0x00, 0x08, 0x03, 0x02, 0x00, 0x00, 0x80, 0x02,
            0x00, 0x00, 0x12, 0x34, 0x56, 0x78,
        ];
        const PAYLOAD: &[u8] = &[0x12, 0x34, 0x56, 0x78];

        let dsb = DitsDsb::try_from(DSB_DATA).unwrap();

        assert_eq!(dsb.sense_key, sense::SenseKey::DataProtect);
        assert_eq!(
            dsb.asc,
            sense::asc::AdditionalSenseCode::AccessDeniedNoAccessRights
        );
        assert_eq!(dsb.data, PAYLOAD);
    }
}
