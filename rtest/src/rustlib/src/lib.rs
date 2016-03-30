#[macro_use]
extern crate rustr;
pub mod export;
pub use rustr::*;

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

