//! Linux ATA interface.

use std::path::Path;

use crate::protocol::{
    Transfer, ata,
    ata::command::{CommandRegisters, ResultRegisters},
};

/// ATA drive interface, no native interface exists on Linux.
#[derive(Debug)]
pub enum Interface {}

impl Interface {
    /// Open drive.
    pub(crate) fn open(_path: &Path) -> Result<Self, ata::Error> {
        Err(ata::Error::UnsupportedDrive)
    }
}

impl ata::Interface for Interface {
    fn path(&self) -> &Path {
        match *self {}
    }

    fn command(
        &self,
        _registers: CommandRegisters,
        _transfer: Transfer,
        _extend: bool,
        _dma: bool,
        _timeout: Option<u32>,
    ) -> Result<Option<ResultRegisters>, ata::Error> {
        match *self {}
    }
}
