use rdll::*;
use std::ffi::CString;
use util::*;
use error::*;
use traits::*;
use protect::stackp::*;


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

pub unsafe fn rf_warn_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    Rf_warning(cstr.as_ptr());
    Ok(())
}

// may longjmp
pub unsafe fn rf_warn(arg1: &str) {
    Rf_warning(c_str(arg1).as_ptr());
}

pub fn r_warn(arg1: &str) {
    unsafe {
        let warning = Shield::new(Rf_lang2(::Rf_install(c_str("warning").as_ptr()),
                                           c_str(arg1).uintor()));
        let _ = Rf_eval(warning.s(), R_GlobalEnv);
    }
}

pub fn r_warn_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    unsafe {
        let warning = Shield::new(Rf_lang2(::Rf_install(c_str("warning").as_ptr()),
                                           cstr.uintor()));
        let _ = Rf_eval(warning.s(), R_GlobalEnv);
         Ok(())
    }
}

pub fn r_message(arg1: &str) {
    unsafe {
        let warning = Shield::new(Rf_lang2(::Rf_install(c_str("message").as_ptr()),
                                           c_str(arg1).uintor()));
        let _ = Rf_eval(warning.s(), R_GlobalEnv);
    }
}

pub fn r_message_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    unsafe {
        let warning = Shield::new(Rf_lang2(::Rf_install(c_str("message").as_ptr()),
                                           cstr.uintor()));
        let _ = Rf_eval(warning.s(), R_GlobalEnv);
         Ok(())
    }
}


pub fn r_message_box_user(arg1: &str) -> RResult<()> {
    let cstr = try!(CString::new(arg1));
    unsafe {
        R_ShowMessage(cstr.as_ptr());
        Ok(())
    }
}

pub fn r_message_box(arg1: &str) {
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
