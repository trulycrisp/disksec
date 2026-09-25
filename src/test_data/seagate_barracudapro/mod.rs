//! Seagate BarraCuda Pro ST500LM035-2GJ17A.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC ID Page 0 response.
pub(crate) const ID_PAGE_0: &[u8; 512] = include_bytes!("id_page_0.bin");
/// VUC ID Page 11 response.
pub(crate) const ID_PAGE_11: &[u8; 512] = include_bytes!("id_page_11.bin");
