use rdll::*;
use std::ffi::CString;
use util::*;
use error::*;

pub unsafe fn r_error_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    Rf_error(cstr.as_ptr());
    Ok(())
}

pub unsafe fn r_error(arg1: &str) {
    Rf_error(c_str(arg1).as_ptr());
}

pub unsafe fn r_unimplemented_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    UNIMPLEMENTED(cstr.as_ptr());
    Ok(())
}

pub unsafe fn r_unimplemented(arg1: &str) {
    UNIMPLEMENTED(c_str(arg1).as_ptr());
}

pub fn r_warn_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    unsafe {
        Rf_warning(cstr.as_ptr());
        Ok(())
    }
}

pub fn r_warn(arg1: &str) {
    unsafe {
        Rf_warning(c_str(arg1).as_ptr());
    }
}

pub fn r_message_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    unsafe {
        R_ShowMessage(cstr.as_ptr());
        Ok(())
    }
}

pub fn r_message(arg1: &str) {
    unsafe {
        R_ShowMessage(c_str(arg1).as_ptr());
    }
}

pub fn r_printf(x: &str) {
    unsafe {
        Rprintf(c_str(x).as_ptr());
    }
}

pub fn r_printf_user(x: &str) -> RResult<()> {
    let cstr = try!(CString::new(x));
    unsafe {
        Rprintf(cstr.as_ptr());
        Ok(())
    }
}
