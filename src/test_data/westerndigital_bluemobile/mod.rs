//! Western Digital Blue Mobile WD5000LPCX-60VHAT0.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC physical parameters table.
pub(crate) const PHYSICAL_PARAMETERS: &[u8; 512] = include_bytes!("physical_parameters.bin");
/// System area file ID 0x1 (directory).
pub(crate) const FILE_1H: &[u8; 16_384] = include_bytes!("file_1h.bin");
/// CPU exception vector table in memory.
pub(crate) const EXCEPTION_VECTOR_TABLE: &[u8; 512] = include_bytes!("exception_vector_table.bin");
