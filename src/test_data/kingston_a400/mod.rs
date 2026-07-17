//! Kingston A400 SA400S37/120G.

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
/// VUC system info response.
pub(crate) const SYSTEM_INFO: &[u8; 512] = include_bytes!("system_info.bin");
/// VUC read info block response.
pub(crate) const INFO_BLOCK: &[u8; 512] = include_bytes!("info_block.bin");
/// Firmware header on flash.
pub(crate) const FIRMWARE_FLASH_HEADER: &[u8; 4096] = include_bytes!("firmware_flash_header.bin");
/// CPU exception vector table in memory.
pub(crate) const EXCEPTION_VECTOR_TABLE: &[u8; 512] = include_bytes!("exception_vector_table.bin");
