//! Drive command set protocols.

pub mod ata;
pub mod scsi;

/// Data transfer.
#[derive(Debug, PartialEq, Eq)]
pub enum Transfer<'a> {
    /// No data.
    None,
    /// Device-to-host read.
    Read(&'a mut [u8]),
    /// Host-to-device write.
    Write(&'a [u8]),
}

impl Transfer<'_> {
    /// Get transfer size in bytes.
    pub const fn size(&self) -> usize {
        match self {
            Self::None => 0,
            Self::Read(x) => x.len(),
            Self::Write(x) => x.len(),
        }
    }
}

impl std::fmt::Display for Transfer<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => write!(f, "none"),
            Self::Read(x) => write!(f, "read {}", x.len()),
            Self::Write(x) => write!(f, "write {}", x.len()),
        }
    }
}
