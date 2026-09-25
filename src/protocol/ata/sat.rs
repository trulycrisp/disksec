//! SCSI ATA pass-through drive interface.

use std::path::Path;

use super::{
    Error,
    command::{CommandRegisters, ResultRegisters},
};
use crate::protocol::{Transfer, scsi};

/// ATA drive interface over SCSI ATA pass-through.
#[derive(Debug)]
pub struct Interface {
    /// SCSI drive interface.
    scsi_drive: scsi::Drive,
}

impl Interface {
    /// Construct interface over SCSI drive.
    pub(crate) fn new(scsi_drive: scsi::Drive) -> Self {
        Self { scsi_drive }
    }

    /// Handle SCSI sense data returned from command.
    fn handle_sense(sense: scsi::sense::Sense) -> Result<Option<ResultRegisters>, Error> {
        let registers = match &sense {
            scsi::sense::Sense::Fixed(s) => s.ata_return().map(|r| r.registers),
            scsi::sense::Sense::Descriptor(s) => s.descriptors.iter().find_map(|d| match d {
                scsi::sense::Descriptor::AtaReturn(r) => Some(*r),
            }),
        };

        if let Some(registers) = registers {
            // Check ATA registers for error
            let status = registers.status;

            if status.error() || status.device_fault() {
                return Err(Error::Command(registers));
            }
        }

        if sense.sense_key().is_error() {
            // Sense reports error, but ATA registers are non-error or absent,
            // return SCSI sense error
            return Err(Error::Scsi(scsi::Error::Sense(sense)));
        }

        Ok(registers)
    }
}

impl super::Interface for Interface {
    fn path(&self) -> &Path {
        self.scsi_drive.path()
    }

    fn command(
        &self,
        registers: CommandRegisters,
        transfer: Transfer,
        extend: bool,
        dma: bool,
        timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, Error> {
        let transfer_size = transfer.size();
        let cdb = scsi::command::AtaPassThrough16::new(registers, &transfer, extend, dma);

        let (transferred_size, sense) =
            match self.scsi_drive.command(&cdb, transfer, timeout, false) {
                Ok(x) => x,
                Err(scsi::Error::UnsupportedDrive) => return Err(Error::UnsupportedDrive),
                Err(x) => return Err(x.into()),
            };

        let out_registers = sense.map(Self::handle_sense).transpose()?.flatten();

        // Check transfer residual
        let residual = transfer_size
            .checked_signed_diff(transferred_size)
            .unwrap_or(isize::MAX);
        if residual != 0 {
            return Err(Error::Residual(residual));
        }

        Ok(out_registers)
    }
}
