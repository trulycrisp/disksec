//! Seagate Momentus gen-5 ST9750420AS.

/// Identify device response.
pub(crate) const IDENTIFY: &[u8; 512] = include_bytes!("identify.bin");
/// VUC ID Page 0 response.
pub(crate) const ID_PAGE_0: &[u8; 512] = include_bytes!("id_page_0.bin");
/// VUC ID Page 11 response.
pub(crate) const ID_PAGE_11: &[u8; 512] = include_bytes!("id_page_11.bin");
/// CPU exception vector table in memory.
pub(crate) const EXCEPTION_VECTOR_TABLE: &[u8; 512] = include_bytes!("exception_vector_table.bin");
