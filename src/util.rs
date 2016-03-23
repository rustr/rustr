//! Utilities
//!
//!
use ::rdll::*;
use ::std::ffi::*;

use ::protect::stackp::*;
use traits::ToSEXP;

pub unsafe extern "C" fn check_interrupt_fn(_ptr: *mut ::std::os::raw::c_void /* dummy */) {
    R_CheckUserInterrupt();
}

/// For CString
/// 
#[macro_export]
macro_rules! cstring {
 	($x:expr) => (
 	::std::ffi::CString::new($x).unwrap().as_ptr()
 		)
 }

pub unsafe fn cstr_sym(x: &str) -> SEXP {
    Rf_install(c_str(x).as_ptr())
}

#[inline]
pub fn cstring_user<T: Into<Vec<u8>>>(string: T) -> Result<CString, NulError> {

    ::std::ffi::CString::new(string)

}

pub fn check_user_interrupt() -> bool {
    unsafe { R_ToplevelExec(Some(check_interrupt_fn), ::std::ptr::null_mut()) != Rboolean::TRUE }
}

#[inline]
pub fn c_str(x: &str) -> CString {
    match CString::new(x) {
        Ok(some) => some,
        Err(err) => {
            unsafe {
                Rf_warning(cstring!(format!("{:?} from string '{:?}': safe string \
                                             transformation failed, use unknown",
                                            err,
                                            x)));
            }
            CString::new("unknown").unwrap()
        }
    }
}


#[inline]
pub fn c_str2(x: &str, unknown: &str) -> CString {
    CString::new(x).unwrap_or({
        unsafe {
            Rf_warning(cstring!("safe string transformation failed, use unknown"));
        }
        CString::new(unknown).unwrap()
    })
}

#[inline]
pub fn derives_from(cl: SEXP, clazz: CString) -> bool {
    unsafe {
        if cl == R_NilValue {
            return false;
        }
        let sstring = clazz;
        // simple test for exact match
        if sstring == CStr::from_ptr(R_CHAR(STRING_ELT(cl, 0))).to_owned() {
            return true;
        }
        let contains_sym = cstr_sym("contains");
        let contains = Shield::new(R_do_slot(R_getClassDef(R_CHAR(Rf_asChar(cl))), contains_sym));
        let res = Rf_getAttrib(contains.s(), R_NamesSymbol);
        if res == R_NilValue {
            return false;
        }
        for ii in 0..Rf_xlength(res) + 1 {

            if CStr::from_ptr(R_CHAR(*VECTOR_PTR(res).offset(ii as isize))).to_owned() == sstring {
                return true;
            }
        }
    }
    false
}

pub fn rprint<T: ToSEXP>(s: T) {
    unsafe {
        Rf_PrintValue(s.s());
    }

}
