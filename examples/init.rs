#[macro_use]
extern crate rustr;

extern crate libc;

use std::ffi::CString;
use libc::{c_char,c_int};
use rustr::rdll::*;
use std::iter::*;
use rustr::protect::shield::*;
use rustr::rdefined::*;


fn main(){
	let argv:Vec<CString>  = vec![CString::new("REmbeddedPostgres").unwrap(), CString::new("--gui=none").unwrap(), CString::new("--silent").unwrap()];
	let dd = argv.len();
	let args:Vec<*mut c_char> = argv.into_iter().map(|arg| { arg.as_ptr() as *mut c_char } ).collect();

	unsafe{

	let d = Rf_initEmbeddedR(dd as c_int,args.as_ptr() as *mut *mut c_char);
	let fun = Shield::new(Rf_findFun(Rf_install(cstring!("print")),  R_GlobalEnv));
	let sdd = Shield::new(Rf_allocVector(INTSXP as SEXPTYPE,10));
	println!("{:?}", "here");
	Rf_endEmbeddedR(0);
	println!("{:?}", d);

	}

}
