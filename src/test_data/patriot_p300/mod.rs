//! Patriot M.2 P300 256GB.

/// Identify controller response.
pub(crate) const IDENTIFY_CONTROLLER: &[u8; 4096] = include_bytes!("identify_controller.bin");
/// Identify namespace response.
pub(crate) const IDENTIFY_NAMESPACE: &[u8; 4096] = include_bytes!("identify_namespace.bin");
/// Log 0x5 (commands supported and effects).
pub(crate) const LOG_5H: &[u8; 4096] = include_bytes!("log_5h.bin");
/// VUC system info response.
pub(crate) const SYSTEM_INFO: &[u8; 4_096] = include_bytes!("system_info.bin");
/// Identify namespace list response.
pub(crate) const IDENTIFY_NAMESPACE_LIST: &[u8; 4096] = include_bytes!("identify_namespace_list.bin");
/// VUC read info block response.
pub(crate) const INFO_BLOCK: &[u8; 6_144] = include_bytes!("info_block.bin");
