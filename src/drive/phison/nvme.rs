//! Common NVMe controller functionality.

use crate::protocol::{Transfer, nvme};

pub mod e13;

/// VUC operation code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum VucOperation {
    /// "AP key" preparing for following VUC
    ApKey = 0x0,
    /// Read system information.
    SystemInfo = 0x80,
    /// Read data from memory address (intended for SRAM).
    ReadSram = 0xA0,
    /// Read drive info block (configuration data).
    ReadInfoBlock = 0xB1,
}

impl std::fmt::Display for VucOperation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ApKey => write!(f, "AP key"),
            Self::SystemInfo => write!(f, "system info"),
            Self::ReadSram => write!(f, "read SRAM"),
            Self::ReadInfoBlock => write!(f, "read info block"),
        }
    }
}

/// CRC16-CCITT checksum used for VUC submission queue entry.
fn crc16_ccitt(data: &[u8]) -> u16 {
    const POLYNOMIAL: u16 = 0x1021;
    const CRC16_MSB: u16 = 1 << (u16::BITS - 1);

    let mut crc = 0;

    for &byte in data {
        crc ^= u16::from(byte) << u8::BITS;

        for _ in 0..8 {
            crc = if crc & CRC16_MSB != 0 {
                (crc << 1) ^ POLYNOMIAL
            } else {
                crc << 1
            };
        }
    }

    crc
}

/// Construct VUC submission queue entry.
pub(super) fn vuc(
    transfer: &Transfer,
    operation: VucOperation,
    parameters: [u32; 4],
) -> nvme::command::AdminCommand {
    const OPCODE_NONE: u8 = 0xd0;
    const OPCODE_WRITE: u8 = 0xd1;
    const OPCODE_READ: u8 = 0xd2;

    let transfer_size = transfer.size();
    let opcode = match transfer {
        Transfer::None => OPCODE_NONE,
        _ if transfer_size == 0 => OPCODE_NONE,
        Transfer::Write(_) => OPCODE_WRITE,
        Transfer::Read(_) => OPCODE_READ,
    };

    // CDW10 is transfer size in dwords
    let cdw10 = transfer_size.div_ceil(size_of::<u32>()).try_into().unwrap();

    let [cdw11, _, cdw13, cdw14] = parameters;

    // CDW12 is operation code and 24-bit parameter
    let cdw12 = ((parameters[1] & 0xFF_FFFF) << 8) | (operation as u32);

    let mut command = nvme::command::AdminCommand {
        opcode: nvme::command::AdminOpcode::VendorSpecific(opcode),
        cdw10,
        cdw11,
        cdw12,
        cdw13,
        cdw14,
        ..Default::default()
    };

    // CDW15 is a checksum of the command SQE itself.
    let checksum = crc16_ccitt(&<[u8; _]>::from(&command));
    command.cdw15 = u32::from(checksum.swap_bytes()) << u16::BITS;

    command
}

/// Construct "AP key" VUC preparation command.
fn vuc_ap_key(hosin: bool) -> nvme::command::AdminCommand {
    const MAGIC_PHISON: u32 = 0xFAEF_FE6F;
    const MAGIC_HOSIN: u32 = 0x4846_4353;

    let magic = if hosin { MAGIC_HOSIN } else { MAGIC_PHISON };
    vuc(&Transfer::None, VucOperation::ApKey, [0, 0, magic, 0])
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::protocol::nvme::command::{AdminCommand, AdminOpcode};

    #[test]
    fn crc16_ccitt_known_results() {
        // CRC-16/XMODEM: poly 0x1021, init 0x0000, no reflection, no final xor
        assert_eq!(crc16_ccitt(b""), 0x0000);
        assert_eq!(crc16_ccitt(b"123456789"), 0x31C3);
        assert_eq!(crc16_ccitt(&[0xFF]), 0x1EF0);
        assert_eq!(crc16_ccitt(&[0xDE, 0xAD, 0xBE, 0xEF]), 0xC457);
        assert_eq!(crc16_ccitt(&[0x5A, 0x00, 0x00, 0x00]), 0x1D90);
        assert_eq!(
            crc16_ccitt(b"The quick brown fox jumps over the lazy dog."),
            0xE2B3
        );
    }

    #[test]
    fn vuc_composition() {
        let none = vuc(&Transfer::None, VucOperation::SystemInfo, [0, 0, 0, 0]);
        assert_eq!(none.opcode, AdminOpcode::VendorSpecific(0xD0));
        assert_eq!(none.cdw10, 0);
        assert_eq!(none.cdw11, 0);
        assert_eq!(none.cdw12, 0x0000_0080);
        assert_eq!(none.cdw13, 0);
        assert_eq!(none.cdw14, 0);
        assert_eq!(none.cdw15, 0x815A_0000);

        let mut expected = [0; AdminCommand::SIZE];
        expected[0x00] = 0xD0;
        expected[0x30] = 0x80;
        expected[0x3E] = 0x5A;
        expected[0x3F] = 0x81;
        assert_eq!(<[u8; AdminCommand::SIZE]>::from(&none), expected);

        let mut read_buffer = [0; 30];
        let read = vuc(
            &Transfer::Read(&mut read_buffer),
            VucOperation::ReadInfoBlock,
            [0x1111_2222, 0xFFAB_CDEF, 0x3333_4444, 0x5555_6666],
        );
        assert_eq!(read.opcode, AdminOpcode::VendorSpecific(0xD2));
        assert_eq!(read.cdw10, 8);
        assert_eq!(read.cdw11, 0x1111_2222);
        assert_eq!(read.cdw12, 0xABCD_EFB1);
        assert_eq!(read.cdw13, 0x3333_4444);
        assert_eq!(read.cdw14, 0x5555_6666);
        assert_eq!(read.cdw15, 0x6305_0000);

        let write_buffer = [0; 16];
        let write = vuc(
            &Transfer::Write(&write_buffer),
            VucOperation::ReadSram,
            [0x0102_0304, 0x0012_3456, 0x0A0B_0C0D, 0x0E0F_1011],
        );
        assert_eq!(write.opcode, AdminOpcode::VendorSpecific(0xD1));
        assert_eq!(write.cdw10, 4);
        assert_eq!(write.cdw11, 0x0102_0304);
        assert_eq!(write.cdw12, 0x1234_56A0);
        assert_eq!(write.cdw13, 0x0A0B_0C0D);
        assert_eq!(write.cdw14, 0x0E0F_1011);
        assert_eq!(write.cdw15, 0xCA67_0000);

        let empty = vuc(
            &Transfer::Write(&[]),
            VucOperation::SystemInfo,
            [0, 0, 0, 0],
        );
        assert_eq!(empty.opcode, AdminOpcode::VendorSpecific(0xD0));
        assert_eq!(empty.cdw10, 0);
        assert_eq!(empty.cdw15, 0x815A_0000);
    }

    #[test]
    fn checksum_taken_before_cdw15_assignment() {
        let command = vuc(&Transfer::None, VucOperation::SystemInfo, [0, 0, 0, 0]);

        let mut cleared = command;
        cleared.cdw15 = 0;
        let over_cleared = crc16_ccitt(&<[u8; AdminCommand::SIZE]>::from(&cleared));
        let over_final = crc16_ccitt(&<[u8; AdminCommand::SIZE]>::from(&command));

        assert_ne!(over_cleared, over_final);
        assert_eq!(
            command.cdw15,
            u32::from(over_cleared.swap_bytes()) << u16::BITS
        );
    }

    #[test]
    fn vuc_to_bytes_read_transfer() {
        const PARAMETERS: [u32; 4] = [0x1234_5678, 0x89AB_CDEF, 0xFEDC_BA98, 0x0F1E_2D3C];
        const TRANSFER_SIZE: usize = 13;
        // CDW10 rounds the transfer up to whole dwords, CDW12 keeps 24 bits
        // of the second parameter above the operation code
        const COMMAND_DATA: &[u8] = &[
            0xD2, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x04, 0x00,
            0x00, 0x00, 0x78, 0x56, 0x34, 0x12, 0xA0, 0xEF, 0xCD, 0xAB, 0x98, 0xBA, 0xDC, 0xFE,
            0x3C, 0x2D, 0x1E, 0x0F, 0x00, 0x00, 0xA1, 0xCA,
        ];

        let mut data = [0; TRANSFER_SIZE];
        let transfer = Transfer::Read(&mut data);
        let command = vuc(&transfer, VucOperation::ReadSram, PARAMETERS);

        assert_eq!(<[u8; _]>::from(&command).as_ref(), COMMAND_DATA);
    }
}
