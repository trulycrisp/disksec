//! Seagate `BarraCuda` Pro ST500LM035-2GJ17A.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC firmware information response.
pub(crate) const FIRMWARE_INFO: &[u8; 512] = include_bytes!("firmware_info.bin");
