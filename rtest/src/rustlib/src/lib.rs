#[macro_use]
extern crate rustr;
extern crate nalgebra;

pub mod export;
pub use rustr::*;
pub use std::os::raw::*;

pub mod nalg;
pub use self::nalg::*;
pub use nalgebra::{DVec,Vec3,Vec4,DVec1,DVec2,DVec3};
pub use nalgebra::{DMat,Mat3,Mat4};
pub use rustr::types::nalgebra::*;

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

// #[rustr_export]
pub fn dll_option(x:String)->RResult<SEXP>{
    r_option::<SEXP>(x.into())
}

// #[rustr_export]
pub fn clist()-> RResult<RList>{

    Ok(rlist!("sd","Sd"))
}

// #[rustr_export]
pub fn nlist()-> RResult<RList>{

    Ok(rlist!(sd ~ "sd", Sd ~ "Sd"))
}

// #[rustr_export]
pub fn uclist()-> RList{

    unsafe{urlist!("sd","Sd")}
}

// #[rustr_export]
pub fn unlist()-> RList{

    unsafe{urlist!(sd ~ "sd", Sd ~ "Sd")}
}
