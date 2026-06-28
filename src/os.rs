//! OS-specific interface.

#[cfg(not(any(windows, target_os = "linux")))]
compile_error!("Unsupported target OS");

#[cfg(windows)]
mod windows;
#[cfg(windows)]
pub use windows::*;

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "linux")]
pub use linux::*;
