//! Kingston DC500R SEDC500R480G.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC system info response.
pub(crate) const SYSTEM_INFO: &[u8; 4_096] = include_bytes!("system_info.bin");
/// VUC read info block response.
pub(crate) const INFO_BLOCK: &[u8; 4_096] = include_bytes!("info_block.bin");
/// CPU exception vector table in memory.
pub(crate) const EXCEPTION_VECTOR_TABLE: &[u8; 512] = include_bytes!("exception_vector_table.bin");
