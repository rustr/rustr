//! R Symbol type
//!
//!

use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::error::*;
use std::convert::*;
use std::ffi::*;

use ::util::*;

pub type Symbol = SymbolM<NoProtect>;

impl<T: SEXPbucket> SymbolM<T> {
    pub fn new<E: ToSEXP>(x: E) -> RResult<SymbolM<T>> {
        unsafe {
            let types = RTYPEOF(x.s());
            match types {
                SYMSXP => {
                    return Ok(SymbolM { data: T::new(x.s()) });
                }

                CHARSXP => {
                    return Ok(SymbolM { data: (T::new(Rf_installChar(x.s()))) });
                }

                STRSXP => {
                    // todo: check that there is at least one element
                    return Ok(SymbolM { data: T::new(Rf_installChar(STRING_ELT(x.s(), 0))) });
                }

                _ => {
                    return rraise("cannot convert to symbol (SYMSXP)");
                }
            }
        }
    }

    pub fn cstring(&self) -> CString {
        unsafe { CStr::from_ptr(R_CHAR(PRINTNAME(self.data.s()))).to_owned() }
    }
}

impl<T: SEXPbucket> From<CString> for SymbolM<T> {
    fn from(x: CString) -> SymbolM<T> {
        unsafe { SymbolM { data: T::new(Rf_install(x.as_ptr())) } }

    }
}

impl<'a, T: SEXPbucket> From<&'a str> for SymbolM<T> {
    fn from(x: &'a str) -> SymbolM<T> {
        unsafe { SymbolM { data: T::new(Rf_install(c_str(x).as_ptr())) } }
    }
}

impl<T: SEXPbucket> From<SymbolM<T>> for CString {
    fn from(x: SymbolM<T>) -> CString {
        unsafe { CStr::from_ptr(R_CHAR(PRINTNAME(x.data.s()))).to_owned() }
    }
}


gen_traits_sexp!(SymbolM);
