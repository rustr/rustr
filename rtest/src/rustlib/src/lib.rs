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