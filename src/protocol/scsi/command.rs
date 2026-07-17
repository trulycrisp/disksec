//! SCSI command functionality.

use crate::protocol::{Transfer, ata};

/// Command operation code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum OpCode {
    /// INQUIRY.
    Inquiry = 0x12,
    /// SAT ATA PASS-THROUGH (16).
    AtaPassThrough16 = 0x85,
}

/// Command Descriptor Block.
pub trait Cdb {
    /// Convert to bytes.
    fn to_bytes(&self) -> Box<[u8]>;
}

impl std::fmt::Display for dyn Cdb + '_ {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", crate::output::format_bytes(&self.to_bytes()))
    }
}

impl From<&dyn Cdb> for Box<[u8]> {
    fn from(value: &dyn Cdb) -> Self {
        value.to_bytes()
    }
}

/// INQUIRY CDB.
#[derive(Default, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Inquiry {
    /// Vital Product Data page code.
    pub page_code: Option<u8>,
    /// Maximum response data size.
    pub allocation_length: u16,
    /// CONTROL byte.
    pub control: u8,
}

impl Cdb for Inquiry {
    fn to_bytes(&self) -> Box<[u8]> {
        let byte1 = self.page_code.is_some().into();
        let page_code = self.page_code.unwrap_or_default();
        let allocation_length = self.allocation_length.to_be_bytes();

        Box::new([
            OpCode::Inquiry as _,
            byte1,
            page_code,
            allocation_length[0],
            allocation_length[1],
            self.control,
        ])
    }
}

/// SCSI ATA Translation CDB protocol field.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatProtocol {
    /// No data transfer.
    NonData = 3,
    /// PIO data-in (device to host).
    PioIn = 4,
    /// PIO data-out (host to device).
    PioOut = 5,
    /// DMA transfer.
    Dma = 6,
}

impl std::fmt::Display for SatProtocol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::NonData => write!(f, "non-data"),
            Self::PioIn => write!(f, "PIO data-in"),
            Self::PioOut => write!(f, "PIO data-out"),
            Self::Dma => write!(f, "DMA"),
        }
    }
}

/// SAT CDB field for which ATA register contains the data transfer length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatTransferLength {
    /// No data transfer.
    None = 0,
    /// Use `count` ATA register.
    Count = 2,
}

impl std::fmt::Display for SatTransferLength {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Count => write!(f, "count"),
        }
    }
}

/// SAT CDB field for data transfer direction.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SatTransferDirection {
    /// Host to device (write).
    ToDevice = 0,
    /// Device to host (read).
    FromDevice = 1,
}

impl std::fmt::Display for SatTransferDirection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ToDevice => write!(f, "to device"),
            Self::FromDevice => write!(f, "from device"),
        }
    }
}

/// SAT ATA PASS-THROUGH (16) CDB.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AtaPassThrough16 {
    /// Data transfer protocol.
    pub protocol: SatProtocol,
    /// Extended LBA-48 command.
    pub extend: bool,
    /// Maximum seconds the device may be offline during the command.
    pub off_line: u8,
    /// Request the SATL always return the ATA result registers in sense.
    pub ck_cond: bool,
    /// When transfer length uses blocks (`byt_blok`) selects block size, device
    /// logical blocks (true) or 512-byte sectors (false).
    pub t_type: bool,
    /// Data transfer direction.
    pub t_dir: SatTransferDirection,
    /// Transfer length counts blocks (true) rather than bytes (false).
    pub byt_blok: bool,
    /// Which ATA register supplies the transfer length.
    pub t_length: SatTransferLength,
    /// The ATA taskfile (feature/count/lba/device/command) to execute.
    pub registers: ata::command::CommandRegisters,
    /// SCSI CONTROL byte (CDB last byte); normally 0.
    pub control: u8,
}

impl AtaPassThrough16 {
    /// Construct CDB.
    pub fn new(
        registers: ata::command::CommandRegisters,
        transfer: &Transfer,
        extend: bool,
        dma: bool,
    ) -> Self {
        let protocol = match transfer {
            Transfer::None => SatProtocol::NonData,
            Transfer::Read(_) => {
                if dma {
                    SatProtocol::Dma
                } else {
                    SatProtocol::PioIn
                }
            },
            Transfer::Write(_) => {
                if dma {
                    SatProtocol::Dma
                } else {
                    SatProtocol::PioOut
                }
            },
        };

        let t_dir = match transfer {
            Transfer::None | Transfer::Write(_) => SatTransferDirection::ToDevice,
            Transfer::Read(_) => SatTransferDirection::FromDevice,
        };

        let t_length = match transfer {
            Transfer::None => SatTransferLength::None,
            Transfer::Read(_) | Transfer::Write(_) => SatTransferLength::Count,
        };

        Self {
            protocol,
            extend,
            off_line: 0,
            ck_cond: true,
            t_type: false,
            t_dir,
            byt_blok: true,
            t_length,
            registers,
            control: 0,
        }
    }
}

impl Cdb for AtaPassThrough16 {
    fn to_bytes(&self) -> Box<[u8]> {
        let byte1 = ((self.protocol as u8) << 1) | u8::from(self.extend);

        let byte2 = ((self.off_line & 0b11) << 6)
            | (u8::from(self.ck_cond) << 5)
            | (u8::from(self.t_type) << 4)
            | ((self.t_dir as u8) << 3)
            | (u8::from(self.byt_blok) << 2)
            | (self.t_length as u8);

        let feature = self.registers.feature.to_be_bytes();
        let count = self.registers.count.to_be_bytes();
        let [_, _, lba @ ..] = self.registers.lba.to_be_bytes();

        Box::new([
            OpCode::AtaPassThrough16 as _,
            byte1,
            byte2,
            feature[0],
            feature[1],
            count[0],
            count[1],
            lba[2],
            lba[5],
            lba[1],
            lba[4],
            lba[0],
            lba[3],
            self.registers.device,
            self.registers.command.into(),
            self.control,
        ])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn inquiry_to_bytes() {
        const PAGE_CODE: u8 = 0xAB;
        const ALLOCATION_LENGTH: u16 = 0x1234;
        const CONTROL: u8 = 0xCD;
        const DATA: &[u8] = &[
            0x12,
            0x01,
            PAGE_CODE,
            (ALLOCATION_LENGTH >> 8) as _,
            (ALLOCATION_LENGTH & 0xFF) as _,
            CONTROL,
        ];

        let inquiry = Inquiry {
            page_code: Some(PAGE_CODE),
            allocation_length: ALLOCATION_LENGTH,
            control: CONTROL,
        };

        assert_eq!(inquiry.to_bytes().as_ref(), DATA);
    }

    #[test]
    fn ata_pass_through16_to_bytes() {
        const FEATURE: u16 = 0x1234;
        const COUNT: u16 = 0x5678;
        const LBA: u64 = 0xA1B2_C3D4_E5F6;
        const DEVICE: u8 = 0xAB;
        const COMMAND: u8 = 0xEC;
        const DATA: &[u8] = &[
            0x85,
            0x0D,
            0x2E,
            (FEATURE >> 8) as _,
            (FEATURE & 0xFF) as _,
            (COUNT >> 8) as _,
            (COUNT & 0xFF) as _,
            ((LBA >> 24) & 0xFF) as _,
            (LBA & 0xFF) as _,
            ((LBA >> 32) & 0xFF) as _,
            ((LBA >> 8) & 0xFF) as _,
            ((LBA >> 40) & 0xFF) as _,
            ((LBA >> 16) & 0xFF) as _,
            DEVICE,
            COMMAND,
            0x00,
        ];

        let registers = ata::command::CommandRegisters {
            feature: FEATURE,
            count: COUNT,
            lba: LBA,
            device: DEVICE,
            command: COMMAND.into(),
        };

        let cdb = AtaPassThrough16 {
            protocol: SatProtocol::Dma,
            extend: true,
            off_line: 0,
            ck_cond: true,
            t_type: false,
            t_dir: SatTransferDirection::FromDevice,
            byt_blok: true,
            t_length: SatTransferLength::Count,
            registers,
            control: 0,
        };

        assert_eq!(cdb.to_bytes().as_ref(), DATA);
    }
}
