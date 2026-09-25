//! Phison vendor.

pub mod ata;
pub mod nvme;

/// Info block data magic prefix.
const INFO_BLOCK_MAGIC: &[u8] = b"PhIsOn";

/// VUC lock state.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum VucLockState {
    /// Locked.
    Locked = 1,
    /// Engineering.
    Engineering = 2,
    /// Unlocked.
    Unlocked = 3,
    /// No lock (no key configured).
    NoLock = 4,
}

impl VucLockState {
    /// Parse from raw byte.
    fn parse(value: u8) -> Result<Option<Self>, u8> {
        const VARIANTS: &[VucLockState] = &[
            VucLockState::Locked,
            VucLockState::Engineering,
            VucLockState::Unlocked,
            VucLockState::NoLock,
        ];

        if value == 0 {
            return Ok(None);
        }

        VARIANTS
            .iter()
            .find(|&&x| x as u8 == value)
            .copied()
            .ok_or(value)
            .map(Some)
    }
}

impl std::fmt::Display for VucLockState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Locked => write!(f, "locked"),
            Self::Engineering => write!(f, "engineering"),
            Self::Unlocked => write!(f, "unlocked"),
            Self::NoLock => write!(f, "no lock"),
        }
    }
}
