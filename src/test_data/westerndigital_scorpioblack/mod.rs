//! Western Digital Scorpio Black WD3200BEKT-60V5T1.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// SMART READ DATA response.
pub(crate) const SMART_DATA: &[u8; 512] = include_bytes!("smart_data.bin");
/// SMART log 0x0 (directory).
pub(crate) const SMART_LOG_0H: &[u8; 512] = include_bytes!("smart_log_0h.bin");
/// GPL log 0x0 (directory).
pub(crate) const GP_LOG_0H: &[u8; 512] = include_bytes!("gp_log_0h.bin");
/// GPL log 0x11 (SATA PHY event counters).
pub(crate) const GP_LOG_11H: &[u8; 512] = include_bytes!("gp_log_11h.bin");
/// VUC native information response.
pub(crate) const NATIVE_INFO: &[u8; 512] = include_bytes!("native_info.bin");
/// System area file ID 0x1 (directory).
pub(crate) const FILE_1H: &[u8; 12_288] = include_bytes!("file_1h.bin");
/// CPU exception vector table in memory.
pub(crate) const EXCEPTION_VECTOR_TABLE: &[u8; 512] = include_bytes!("exception_vector_table.bin");
