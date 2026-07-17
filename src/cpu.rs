//! CPU architecture functionality.

pub mod arcompact;
pub mod arm32;

/// Standard vector table data size in bytes.
pub const VECTOR_TABLE_SIZE: usize = 512;
