//! VUC definitions.

pub(super) mod debug_stop;
pub(super) mod error;
pub(super) mod sense;
pub(super) mod status;

pub(super) use debug_stop::DebugStopCode;
pub(super) use error::ErrorCode;

/// VUC action code.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum Action {
    /// File (system area module).
    File = 8,
    /// Information table.
    Table = 13,
    /// Controller memory.
    Memory = 19,
}

impl std::fmt::Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::File => write!(f, "file"),
            Self::Table => write!(f, "table"),
            Self::Memory => write!(f, "memory"),
        }
    }
}
