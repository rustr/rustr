//! Some C APIs for R coded in Rust.
//!
//!

pub use std::mem::transmute;

// begin --------------- get SEXP ptr

/// C Macro DATAPTR()
/// 
#[macro_export]
macro_rules! DATAPTR{

    ($v:ident) => (transmute::<SEXP,*mut SEXPREC_ALIGN>($v));

}
