//! R FFI to Rust
//!
//!
//!



pub mod message;
pub use self::message::*;
use rdll::*;

pub fn r_isna(arg1: ::std::os::raw::c_double) -> bool {
    unsafe { R_IsNA(arg1) == 1 }
}

pub fn r_isnan(arg1: ::std::os::raw::c_double) -> bool {
    unsafe { R_IsNaN(arg1) == 1 }
}

pub fn r_finite(arg1: ::std::os::raw::c_double) -> bool {
    unsafe { R_finite(arg1) == 1 }
}

use symbol::*;
use storage::*;
use robject::*;
use traits::*;

pub fn r_option<T: SEXPbucket>(x: SymbolM<T>) -> RObj {
    unsafe { RObj::new(Rf_GetOption1(x.s())) }
}

pub fn r_option_digits() -> ::std::os::raw::c_int {
    unsafe { Rf_GetOptionDigits() }
}

pub fn r_option_width() -> ::std::os::raw::c_int {
    unsafe { Rf_GetOptionWidth() }
}

pub fn r_flushconsole() {
    unsafe { R_FlushConsole() }
}
