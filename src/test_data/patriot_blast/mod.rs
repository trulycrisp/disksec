//! Patriot Blast PBT120GS25SSDR.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC system info response.
pub(crate) const SYSTEM_INFO: &[u8; 512] = include_bytes!("system_info.bin");
/// VUC read info block response.
pub(crate) const INFO_BLOCK: &[u8; 512] = include_bytes!("info_block.bin");
/// Firmware header on flash.
pub(crate) const FIRMWARE_FLASH_HEADER: &[u8; 4096] = include_bytes!("firmware_flash_header.bin");
