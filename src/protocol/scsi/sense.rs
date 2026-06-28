//! SCSI sense data parsing: fixed- and descriptor-format sense buffers, sense
//! keys, and extraction of SAT ATA result registers returned via sense.

pub mod asc;

use std::num::NonZero;

use crate::protocol::ata;

/// Sense error.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Error {
    /// Data too short.
    Truncated,
    /// Invalid response code value.
    InvalidResponseCode(u8),
    /// Invalid sense key value.
    InvalidSenseKey(u8),
    /// Invalid descriptor type value.
    InvalidDescriptorType(u8),
}

impl std::error::Error for Error {}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Truncated => write!(f, "truncated"),
            Self::InvalidResponseCode(x) => {
                write!(f, "invalid response code {x:#x}")
            },
            Self::InvalidSenseKey(x) => write!(f, "invalid sense key {x:#x}"),
            Self::InvalidDescriptorType(x) => {
                write!(f, "invalid descriptor type {x:#x}")
            },
        }
    }
}

/// Sense response code.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ResponseCode {
    /// Fixed-format sense for the command just issued.
    FixedCurrent = 0x70,
    /// Fixed-format sense for an earlier, deferred error.
    FixedDeferred = 0x71,
    /// Descriptor-format sense for the command just issued.
    DescriptorCurrent = 0x72,
    /// Descriptor-format sense for an earlier, deferred error.
    DescriptorDeferred = 0x73,
}

impl ResponseCode {
    /// Mask for response code values.
    pub const MASK: u8 = 0b111_1111;
}

impl std::fmt::Display for ResponseCode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::FixedCurrent => write!(f, "fixed current"),
            Self::FixedDeferred => write!(f, "fixed deferred"),
            Self::DescriptorCurrent => write!(f, "descriptor current"),
            Self::DescriptorDeferred => write!(f, "descriptor deferred"),
        }
    }
}

impl TryFrom<u8> for ResponseCode {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[ResponseCode] = &[
            ResponseCode::FixedCurrent,
            ResponseCode::FixedDeferred,
            ResponseCode::DescriptorCurrent,
            ResponseCode::DescriptorDeferred,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidResponseCode(value))
    }
}

/// Sense key.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SenseKey {
    /// No error to report (or only non-key fields are meaningful).
    NoSense = 0x0,
    /// Command succeeded after device-side recovery.
    RecoveredError = 0x1,
    /// Device not ready (e.g. spinning up, becoming ready).
    NotReady = 0x2,
    /// Unrecoverable error reading/writing the medium.
    MediumError = 0x3,
    /// Non-recoverable hardware failure.
    HardwareError = 0x4,
    /// Bad CDB / parameters; the most common rejection for an unsupported VUC.
    IllegalRequest = 0x5,
    /// State change since last access (reset, media change, etc.).
    UnitAttention = 0x6,
    /// Access blocked by write/read protection.
    DataProtect = 0x7,
    /// Blank or formatted-but-empty region encountered.
    BlankCheck = 0x8,
    /// Vendor-defined condition.
    VendorSpecific = 0x9,
    /// COPY/COMPARE/COPY AND VERIFY aborted.
    CopyAborted = 0xA,
    /// Command aborted by the device; often the response to a failed VUC.
    AbortedCommand = 0xB,
    /// Buffered data would overflow the medium.
    VolumeOverflow = 0xD,
    /// VERIFY/COMPARE data miscompare.
    Miscompare = 0xE,
    /// Command completed (used to report completion of a prior operation).
    Completed = 0xF,
}

impl SenseKey {
    /// If sense key represents an error.
    pub fn is_error(self) -> bool {
        matches!(
            self,
            Self::NotReady
                | Self::MediumError
                | Self::HardwareError
                | Self::IllegalRequest
                | Self::DataProtect
                | Self::CopyAborted
                | Self::AbortedCommand
                | Self::VolumeOverflow
                | Self::Miscompare
        )
    }
}

impl TryFrom<u8> for SenseKey {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[SenseKey] = &[
            SenseKey::NoSense,
            SenseKey::RecoveredError,
            SenseKey::NotReady,
            SenseKey::MediumError,
            SenseKey::HardwareError,
            SenseKey::IllegalRequest,
            SenseKey::UnitAttention,
            SenseKey::DataProtect,
            SenseKey::BlankCheck,
            SenseKey::VendorSpecific,
            SenseKey::CopyAborted,
            SenseKey::AbortedCommand,
            SenseKey::VolumeOverflow,
            SenseKey::Miscompare,
            SenseKey::Completed,
        ];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidSenseKey(value))
    }
}

impl std::fmt::Display for SenseKey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NoSense => write!(f, "NO SENSE"),
            Self::RecoveredError => write!(f, "RECOVERED ERROR"),
            Self::NotReady => write!(f, "NOT READY"),
            Self::MediumError => write!(f, "MEDIUM ERROR"),
            Self::HardwareError => write!(f, "HARDWARE ERROR"),
            Self::IllegalRequest => write!(f, "ILLEGAL REQUEST"),
            Self::UnitAttention => write!(f, "UNIT ATTENTION"),
            Self::DataProtect => write!(f, "DATA PROTECT"),
            Self::BlankCheck => write!(f, "BLANK CHECK"),
            Self::VendorSpecific => write!(f, "VENDOR SPECIFIC"),
            Self::CopyAborted => write!(f, "COPY ABORTED"),
            Self::AbortedCommand => write!(f, "ABORTED COMMAND"),
            Self::VolumeOverflow => write!(f, "VOLUME OVERFLOW"),
            Self::Miscompare => write!(f, "MISCOMPARE"),
            Self::Completed => write!(f, "COMPLETED"),
        }
    }
}

/// SAT ATA return response in fixed-format sense data.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtaReturnFixed {
    /// ATA registers.
    pub registers: ata::command::ResultRegisters,
    /// LBA-48 extended command.
    pub extend: bool,
    /// Upper bits of count register are non-zero.
    pub count_upper_nonzero: bool,
    /// Upper bits of LBA register are non-zero.
    pub lba_upper_nonzero: bool,
    /// Relevant log index in ATA PASS-THROUGH Results log page.
    pub log_index: Option<NonZero<u8>>,
}

/// Fixed-format sense data.
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FixedSense {
    /// Raw bytes.
    raw: Box<[u8]>,
    /// Information field is valid (not vendor-specific).
    pub valid: bool,
    /// Response code was deferred.
    pub deferred: bool,
    /// Tape FILEMARK flag.
    pub filemark: bool,
    /// End-of-medium flag (sequential-access devices).
    pub eom: bool,
    /// Incorrect Length Indicator, requested length differed from actual.
    pub ili: bool,
    /// Sense data overflowed the allocated buffer.
    pub sdat_ovfl: bool,
    /// Sense key.
    pub sense_key: SenseKey,
    /// Information.
    pub information: [u8; 4],
    /// Command-specific information field, when present.
    pub csi: Option<[u8; 4]>,
    /// Additional sense code/qualifier (ASC/ASCQ), when present.
    pub asc: Option<asc::AdditionalSenseCode>,
    /// Field-replaceable-unit code, when present.
    pub fru: Option<u8>,
    /// Sense-key-specific bytes; meaning depends on the sense key (and `sksv`).
    pub sks: Option<[u8; 3]>,
    /// Sense-key-specific field is valid (not vendor-specific).
    pub sksv: Option<bool>,
}

impl FixedSense {
    /// Parse from raw bytes.
    fn parse(data: &[u8]) -> Result<Self, Error> {
        const HEADER_SIZE: usize = 8;

        let header = data.first_chunk::<HEADER_SIZE>().ok_or(Error::Truncated)?;

        let response_code = ResponseCode::try_from(header[0] & ResponseCode::MASK)?;
        let deferred = match response_code {
            ResponseCode::FixedCurrent => false,
            ResponseCode::FixedDeferred => true,
            x => return Err(Error::InvalidResponseCode(x as _)),
        };

        let valid = (header[0] & (1 << 7)) != 0;
        let filemark = (header[2] & (1 << 7)) != 0;
        let eom = (header[2] & (1 << 6)) != 0;
        let ili = (header[2] & (1 << 5)) != 0;
        let sdat_ovfl = (header[2] & (1 << 4)) != 0;
        let sense_key = SenseKey::try_from(header[2] & 0xF)?;
        let information = std::array::from_fn(|i| header[3 + i]);

        // Resize to defined size (header + additional)
        let additional_length = header[7] as usize;
        let data = data
            .get(..HEADER_SIZE + additional_length)
            .ok_or(Error::Truncated)?;

        let csi = data.get(8..).and_then(<[_]>::first_chunk).copied();
        let asc = data
            .get(12..14)
            .map(|x| asc::AdditionalSenseCode::parse(x[0], x[1]));
        let fru = data.get(14).copied();
        let sks = data.get(15..).and_then(<[_]>::first_chunk).copied();
        let sksv = sks.map(|x| x[0] & (1 << 7) != 0);

        Ok(Self {
            raw: data.into(),
            valid,
            deferred,
            filemark,
            eom,
            ili,
            sdat_ovfl,
            sense_key,
            information,
            csi,
            asc,
            fru,
            sks,
            sksv,
        })
    }

    /// Get SAT ATA return information.
    pub fn ata_return(&self) -> Option<AtaReturnFixed> {
        let csi = self.csi?;

        // Don't check valid before parsing information, some SATLs seem not to set it
        let [error, status, device, count] = self.information;
        let count = count.into();

        // valid ATA status register should never be 0
        if status == 0 {
            return None;
        }

        let lba = u32::from_le_bytes([csi[1], csi[2], csi[3], 0]).into();
        let status = status.into();

        let registers = ata::command::ResultRegisters {
            error,
            count,
            lba,
            device,
            status,
        };

        let csi_byte0 = csi[0];
        let extend = (csi_byte0 & (1 << 7)) != 0;
        let count_upper_nonzero = (csi_byte0 & (1 << 6)) != 0;
        let lba_upper_nonzero = (csi_byte0 & (1 << 5)) != 0;
        let log_index = NonZero::new(csi_byte0 & 0xF);

        Some(AtaReturnFixed {
            registers,
            extend,
            count_upper_nonzero,
            lba_upper_nonzero,
            log_index,
        })
    }
}

impl std::fmt::Display for FixedSense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .raw
                .iter()
                .map(|x| format!("{x:02x}"))
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

/// Descriptor type codes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DescriptorType {
    /// SAT ATA Return descriptor.
    AtaReturn = 0x9,
}

impl TryFrom<u8> for DescriptorType {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        const VARIANTS: &[DescriptorType] = &[DescriptorType::AtaReturn];

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(Error::InvalidDescriptorType(value))
    }
}

/// Descriptor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Descriptor {
    /// SAT ATA return descriptor.
    AtaReturn(ata::command::ResultRegisters),
}

impl Descriptor {
    /// Parse SAT ATA return descriptor from raw bytes.
    fn ata_return_parse(data: &[u8]) -> Result<ata::command::ResultRegisters, Error> {
        const FLAGS_OFFSET: usize = 0;
        const FLAGS_MASK_EXTEND: u8 = 1 << 0;
        const ERROR_OFFSET: usize = FLAGS_OFFSET + 1;
        const COUNT_OFFSET: usize = ERROR_OFFSET + 1;
        const LBA_OFFSET: usize = COUNT_OFFSET + 2;
        const LBA_SIZE: usize = 6;
        const DEVICE_OFFSET: usize = LBA_OFFSET + LBA_SIZE;
        const STATUS_OFFSET: usize = DEVICE_OFFSET + 1;
        const SIZE: usize = STATUS_OFFSET + 1;

        if data.len() < SIZE {
            return Err(Error::Truncated);
        }

        let extend = (data[FLAGS_OFFSET] & FLAGS_MASK_EXTEND) != 0;
        let error = data[ERROR_OFFSET];
        let mut count = u16::from_be_bytes(std::array::from_fn(|i| data[COUNT_OFFSET + i]));
        let lba = &data[LBA_OFFSET..LBA_OFFSET + LBA_SIZE];
        let mut lba = u64::from_be_bytes([0, 0, lba[4], lba[2], lba[0], lba[5], lba[3], lba[1]]);

        if !extend {
            count &= 0xFF;
            lba &= 0xFF_FFFF;
        }

        let device = data[DEVICE_OFFSET];
        let status = data[STATUS_OFFSET].into();

        Ok(ata::command::ResultRegisters {
            error,
            count,
            lba,
            device,
            status,
        })
    }

    /// Parse descriptor and descriptor size from raw bytes.
    fn parse(data: &[u8]) -> Result<(Option<Self>, usize), Error> {
        const HEADER_SIZE: usize = 2;

        let (header, additional) = data
            .split_first_chunk::<{ HEADER_SIZE }>()
            .ok_or(Error::Truncated)?;

        let descriptor_type_raw = header[0];
        let additional_length = header[1] as _;
        let size = HEADER_SIZE + additional_length;

        let additional = additional
            .get(..additional_length)
            .ok_or(Error::Truncated)?;

        let descriptor_type = match DescriptorType::try_from(descriptor_type_raw) {
            Ok(x) => x,
            // Skip unknown descriptor type
            Err(Error::InvalidDescriptorType(_)) => return Ok((None, size)),
            Err(x) => return Err(x),
        };

        let descriptor = match descriptor_type {
            DescriptorType::AtaReturn => Descriptor::AtaReturn(Self::ata_return_parse(additional)?),
        };

        Ok((Some(descriptor), size))
    }
}

/// Descriptor-format sense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DescriptorSense {
    /// Raw bytes.
    raw: Box<[u8]>,
    /// Response code was deferred.
    pub deferred: bool,
    /// Sense key.
    pub sense_key: SenseKey,
    /// Additional sense code/qualifier.
    pub asc: asc::AdditionalSenseCode,
    /// Descriptors.
    pub descriptors: Box<[Descriptor]>,
}

impl DescriptorSense {
    /// Parse a descriptor-format sense buffer and its trailing descriptor list.
    fn parse(data: &[u8]) -> Result<Self, Error> {
        let (header, mut additional) = data.split_first_chunk::<8>().ok_or(Error::Truncated)?;

        let response_code = ResponseCode::try_from(header[0] & ResponseCode::MASK)?;
        let deferred = match response_code {
            ResponseCode::DescriptorCurrent => false,
            ResponseCode::DescriptorDeferred => true,
            x => return Err(Error::InvalidResponseCode(x as _)),
        };

        let sense_key = SenseKey::try_from(header[1] & 0xF)?;
        let asc = asc::AdditionalSenseCode::parse(header[2], header[3]);
        let additional_length = header[7] as _;

        // Resize additional to supplied size
        additional = additional
            .get(..additional_length)
            .ok_or(Error::Truncated)?;

        let mut descriptors = Vec::new();

        while !additional.is_empty() {
            let (descriptor, descriptor_size) = Descriptor::parse(additional)?;
            additional = &additional[descriptor_size..];

            if let Some(descriptor) = descriptor {
                descriptors.push(descriptor);
            }
        }

        Ok(Self {
            raw: data.into(),
            deferred,
            sense_key,
            asc,
            descriptors: descriptors.into(),
        })
    }
}

impl std::fmt::Display for DescriptorSense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(
            &self
                .raw
                .iter()
                .map(|x| format!("{x:02x}"))
                .collect::<Vec<_>>()
                .join(" "),
        )
    }
}

/// Sense.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Sense {
    /// Fixed-format sense.
    Fixed(FixedSense),
    /// Descriptor-format sense.
    Descriptor(DescriptorSense),
}

impl Sense {
    /// Get sense key.
    pub fn sense_key(&self) -> SenseKey {
        match self {
            Self::Fixed(x) => x.sense_key,
            Self::Descriptor(x) => x.sense_key,
        }
    }
}

impl TryFrom<&[u8]> for Sense {
    type Error = Error;

    fn try_from(data: &[u8]) -> Result<Self, Error> {
        let response_code = *data.first().ok_or(Error::Truncated)? & ResponseCode::MASK;
        let response_code = ResponseCode::try_from(response_code)?;

        Ok(match response_code {
            ResponseCode::FixedCurrent | ResponseCode::FixedDeferred => {
                Sense::Fixed(FixedSense::parse(data)?)
            },
            ResponseCode::DescriptorCurrent | ResponseCode::DescriptorDeferred => {
                Sense::Descriptor(DescriptorSense::parse(data)?)
            },
        })
    }
}

impl std::fmt::Display for Sense {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Fixed(x) => x.fmt(f),
            Self::Descriptor(x) => x.fmt(f),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_fixed_sense_ata_return() {
        const SENSE_KEY: SenseKey = SenseKey::RecoveredError;
        const ASC: u8 = 0x3;
        const ASCQ: u8 = 0x1;
        const FRU: u8 = 0x89;
        const ERROR: u8 = 0x12;
        const COUNT: u16 = 0xAB;
        const LBA: u64 = 0xC3_B2_A1;
        const DEVICE: u8 = 0xEF;
        const STATUS: u8 = 0x50;
        const DATA: &[u8] = &[
            ResponseCode::FixedCurrent as _,
            0x00,
            (SENSE_KEY as u8) & 0xF,
            ERROR,
            STATUS,
            DEVICE,
            (COUNT & 0xFF) as _,
            0x0A,
            0x80,
            (LBA & 0xFF) as _,
            ((LBA >> 8) & 0xFF) as _,
            ((LBA >> 16) & 0xFF) as _,
            ASC,
            ASCQ,
            FRU,
            0x00,
            0x00,
            0x00,
        ];

        let Sense::Fixed(sense) = Sense::try_from(DATA).unwrap() else {
            panic!("expected fixed sense");
        };

        assert!(!sense.valid);
        assert!(!sense.deferred);
        assert_eq!(sense.sense_key, SENSE_KEY);
        assert_eq!(sense.asc, Some(asc::AdditionalSenseCode::parse(ASC, ASCQ)));
        assert_eq!(sense.fru, Some(FRU));
        assert_eq!(sense.sksv, Some(false));

        let ata_return = sense.ata_return().unwrap();
        assert_eq!(ata_return.registers.error, ERROR);
        assert_eq!(ata_return.registers.count, COUNT);
        assert_eq!(ata_return.registers.lba, LBA);
        assert_eq!(ata_return.registers.device, DEVICE);
        assert_eq!(ata_return.registers.status, STATUS.into());
        assert!(ata_return.extend);
        assert!(!ata_return.count_upper_nonzero);
        assert!(!ata_return.lba_upper_nonzero);
        assert_eq!(ata_return.log_index, None);

        std::assert_matches!(Sense::try_from(&DATA[..8]), Err(Error::Truncated));
    }

    #[test]
    fn parse_descriptor_sense() {
        const SENSE_KEY: SenseKey = SenseKey::AbortedCommand;
        const ASC: u8 = 0x3;
        const ASCQ: u8 = 0x1;
        const DATA: &[u8] = &[
            ResponseCode::DescriptorDeferred as _,
            SENSE_KEY as _,
            ASC,
            ASCQ,
            0,
            0,
            0,
            0,
        ];

        let Sense::Descriptor(sense) = Sense::try_from(DATA).unwrap() else {
            panic!("expected descriptor sense");
        };

        assert!(sense.deferred);
        assert_eq!(sense.sense_key, SENSE_KEY);
        assert_eq!(sense.asc, asc::AdditionalSenseCode::parse(ASC, ASCQ));
        assert!(sense.descriptors.is_empty());
    }

    #[test]
    fn parse_ata_return_descriptor() {
        const ERROR: u8 = 0x51;
        const COUNT: u16 = 0x1234;
        const LBA: u64 = 0xA1B2_C3D4_E5F6;
        const DEVICE: u8 = 0xEF;
        const STATUS: u8 = 0x50;
        const DATA: &[u8] = &[
            0x01,
            ERROR,
            ((COUNT >> 8) & 0xFF) as _,
            (COUNT & 0xFF) as _,
            ((LBA >> 24) & 0xFF) as _,
            (LBA & 0xFF) as _,
            ((LBA >> 32) & 0xFF) as _,
            ((LBA >> 8) & 0xFF) as _,
            ((LBA >> 40) & 0xFF) as _,
            ((LBA >> 16) & 0xFF) as _,
            DEVICE,
            STATUS,
        ];

        let registers = Descriptor::ata_return_parse(DATA).unwrap();
        assert_eq!(registers.error, ERROR);
        assert_eq!(registers.count, COUNT);
        assert_eq!(registers.lba, LBA);
        assert_eq!(registers.device, DEVICE);
        assert_eq!(registers.status, STATUS.into());

        std::assert_matches!(
            Descriptor::ata_return_parse(&DATA[..10]),
            Err(Error::Truncated)
        );
    }
}
