use ::rdll::*;
use traits::*;
use std::ffi::{CString, CStr};
use storage::*;
use util::c_str;

use symbol::*;
use error::*;

pub trait RAttribute : ToSEXP{
    fn get_attr<D: RNew, S: SEXPbucket, EE: Into<SymbolM<S>>>(&self, name: EE) -> RResult<D> {
        unsafe { D::rnew(Rf_getAttrib(self.s(), name.into().s())) }
    }
    fn set_attr<T: ToSEXP, EE: Into<SymbolM<S>>, S: SEXPbucket>(&self, name: EE, attr: T) {
        unsafe {
            Rf_setAttrib(self.s(), name.into().s(), attr.s());
        }
    }
    fn attribute_names(&self) -> Vec<CString> {
        let mut v: Vec<CString> = Vec::new();

        let mut attrs = unsafe { ATTRIB(self.s()) };
        unsafe {
            while attrs != R_NilValue {
                v.push(CStr::from_ptr(R_CHAR(PRINTNAME(TAG(attrs)))).to_owned());
                attrs = CDR(attrs);
            }
            v
        }
    }
    fn has_attribute(&self, attr: &str) -> bool {
        let mut attrs = unsafe { ATTRIB(self.s()) };
        unsafe {
            while attrs != R_NilValue {
                if c_str(attr) == CStr::from_ptr(R_CHAR(PRINTNAME(TAG(attrs)))).to_owned() {
                    return true;
                }
                attrs = CDR(attrs);
            }
        }
        false 
    }
    fn attributte<D: RNew>(&self) -> RResult<D> {
        unsafe { RNew::rnew(ATTRIB(self.s())) }
    }
}
