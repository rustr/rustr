//! R FFI
//!
//!
//!


#[cfg(all(any(target_os="linux",target_os="macos"), target_pointer_width = "64"))]
pub mod unix64;
#[cfg(all(any(target_os="linux",target_os="macos"), target_pointer_width = "64"))]
pub use self::unix64::*;

#[cfg(all(target_os="linux", target_pointer_width = "32"))]
pub mod unix32;
#[cfg(all(target_os="linux", target_pointer_width = "32"))]
pub use self::unix32::*;

#[cfg(all(target_os="windows", target_pointer_width = "64"))]
pub mod win64;
#[cfg(all(target_os="windows", target_pointer_width = "64"))]
pub use self::win64::*;

#[cfg(all(target_os="windows", target_pointer_width = "32"))]
pub mod win32;
#[cfg(all(target_os="windows", target_pointer_width = "32"))]
pub use self::win32::*;

pub use super::complex::*;
