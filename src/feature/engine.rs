#![feature(const_fn)]

use rdll::*;
use environment::*;
use std::ptr::null_mut;


pub static mut REng : * mut REngine = null_mut();

pub struct REngine{
    verbose_m : bool,	 // constructor, or setter
	interactive_m: bool,	// constructor only
	global_env : EnvirN
}