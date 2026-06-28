//! Western Digital Caviar SE WD1600JD-00GBB0.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
