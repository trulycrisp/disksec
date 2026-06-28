//! Seagate Momentus gen-5 ST9750420AS.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC firmware information response.
pub(crate) const FIRMWARE_INFO: &[u8; 512] = include_bytes!("firmware_info.bin");
/// Exception vector in memory.
pub(crate) const EXCEPTION_VECTOR: &[u8; 512] = include_bytes!("exception_vector.bin");
