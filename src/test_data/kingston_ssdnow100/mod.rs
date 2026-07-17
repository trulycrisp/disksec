//! Kingston SSDNow-100 RBU-SC100S37/256GD.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC system info response.
pub(crate) const SYSTEM_INFO: &[u8; 512] = include_bytes!("system_info.bin");
/// VUC read info block response.
pub(crate) const INFO_BLOCK: &[u8; 512] = include_bytes!("info_block.bin");
/// CPU vector table in memory.
pub(crate) const VECTOR_TABLE: &[u8; 512] = include_bytes!("vector_table.bin");
