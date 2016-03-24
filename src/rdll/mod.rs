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

#[cfg(feature = "engine")]
pub use self::engine::*;

#[cfg(feature = "engine")]
pub mod engine {
#[link(name = "R")]
extern "C" {
	pub static mut R_CStackLimit : super::uintptr_t;
	pub static mut R_CStackStart : super::uintptr_t;
	pub static mut R_SignalHandlers: ::std::os::raw::c_int;
	pub static mut R_Interactive: super::Rboolean;
    pub static mut R_Slave: super::Rboolean;
    pub static mut R_GUIType: *mut ::std::os::raw::c_char;
    pub static mut R_HistoryFile: *mut ::std::os::raw::c_char;
    pub static mut R_HistorySize: ::std::os::raw::c_int;
    pub static mut R_RestoreHistory: ::std::os::raw::c_int;
    pub static mut R_Home: *mut ::std::os::raw::c_char;
    pub static mut R_GlobalContext: *mut ::std::os::raw::c_void;
    pub static mut R_Consolefile: *mut super::FILE;
    pub static mut R_Outputfile: *mut super::FILE;
    pub static mut R_timeout_val: ::std::os::raw::c_long;
    pub static mut R_running_as_main_program: ::std::os::raw::c_int;
    
	#[cfg(target_os = "windows")] 
    pub fn get_R_HOME() -> *const ::std::os::raw::c_char;
    #[cfg(target_os = "windows")] 
    pub fn getRUser()-> *const ::std::os::raw::c_char;
    #[cfg(target_os = "windows")] 
    pub fn getDLLVersion()-> *const ::std::os::raw::c_char;
 	}

}
