//! Phison S9 SSE032GTTC7-S9A-1.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC system info response.
pub(crate) const SYSTEM_INFO: &[u8; 512] = include_bytes!("system_info.bin");
/// VUC read info block response.
pub(crate) const INFO_BLOCK: &[u8; 512] = include_bytes!("info_block.bin");
/// Firmware header on flash.
pub(crate) const FIRMWARE_FLASH_HEADER: &[u8; 2048] = include_bytes!("firmware_flash_header.bin");
/// CPU vector table in memory.
pub(crate) const VECTOR_TABLE: &[u8; 512] = include_bytes!("vector_table.bin");
