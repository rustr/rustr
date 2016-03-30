#[macro_use]
extern crate rustr;
pub mod export;
pub use rustr::*;

// #[rustr_export]
pub fn say_hi()->RResult<String>{
	Ok("hello world".into())
}
