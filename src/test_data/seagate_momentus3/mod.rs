//! Seagate Momentus gen-3 ST9160821AS.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
