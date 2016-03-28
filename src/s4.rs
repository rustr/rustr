//! R S4 type
//!
//!


use ::rdll::*;
use ::storage::*;
use ::traits::*;

use ::error::*;

use std::ffi::*;

use ::util::*;


pub type S4 = S4M<Preserve>;

impl<T: SEXPbucket> NewRObj for S4M<T> {
    fn new<E: ToSEXP>(x: E) -> RResult<Self> {
        unsafe {
            if Rf_isS4(x.s()) != Rboolean::TRUE {
                return rraise("not an S4 object");
            }
            Ok(S4M { data: T::new(x.s()) })
        }
    }
    unsafe fn unew<E: ToSEXP>(x: E) -> Self{
    	S4M { data: T::new(x.s()) }
    }
}


impl<T: SEXPbucket> S4M<T> {
    pub fn from_cstring(x: CString) -> RResult<S4M<T>> {
        unsafe {
            let res = S4M { data: T::new(R_do_new_object(R_do_MAKE_CLASS(x.as_ptr()))) };
            if Rf_inherits(res.data.s(), x.as_ptr()) != Rboolean::TRUE {
                return rraise(format!("error creating S4 object of class: {:?}", x));
            }
            Ok(res)
        }
    }

    pub fn is(&self, clazz: CString) -> bool {
        unsafe { derives_from(Rf_getAttrib(self.data.s(), R_ClassSymbol), clazz) }
    }
}


gen_traits_sexp!(S4M);
