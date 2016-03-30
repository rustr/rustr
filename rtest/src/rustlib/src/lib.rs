#[macro_use]
extern crate rustr;
pub mod export;
pub use rustr::*;
pub use std::os::raw::*;

// #[rustr_export]
pub fn say_hi()->RResult<String>{
	Ok("hello world".into())
}

// message --------------------------

// #[rustr_export]
pub fn message_warning(){
    r_warn("this is warning from rust")
}

// #[rustr_export]
pub fn message_message(){
    r_message("this is message from rust")
}

// #[rustr_export]
pub fn message_r_printf(){
    r_printf("this is printf from rust")
}

// dll --------------------------

// #[rustr_export]
pub fn dll_r_finite(x:c_double)->bool{
    r_finite(x)
}
// #[rustr_export]
pub fn dll_is_na(x:c_double)->bool{
    r_isna(x)
}
// #[rustr_export]
pub fn dll_is_nan(x:c_double)->bool{
    r_isnan(x)
}
