//! ADATA ISSS316 ISSS316-128GCTB5.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
