//! Western Digital Blue Mobile WD5000LPCX-60VHAT0.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC native information response.
pub(crate) const NATIVE_INFO: &[u8; 512] = include_bytes!("native_info.bin");
/// System area file ID 0x1 (directory).
pub(crate) const FILE_1H: &[u8; 16_384] = include_bytes!("file_1h.bin");
/// Exception vector in memory.
pub(crate) const EXCEPTION_VECTOR: &[u8; 512] = include_bytes!("exception_vector.bin");
