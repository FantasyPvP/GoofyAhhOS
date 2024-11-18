#[cfg(target_arch = "x86_64")]
pub mod x86_64;

#[cfg(target_arch = "x86_64")]
pub use x86_64::*;

pub mod io;

pub use io::*;

// Add common CPU traits/interfaces here that all architectures must implement
