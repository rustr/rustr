//! R RC type
//!
//!
//!



use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::error::*;
use std::ffi::*;
use ::util::*;


pub type Reference = ReferenceM<Preserve>;

gen_traits_sexp!(ReferenceM);


impl<T: SEXPbucket> ReferenceM<T> {
    pub fn new<E: ToSEXP>(x: E) -> RResult<ReferenceM<T>> {
        unsafe {
            if Rf_isS4(x.s()) != Rboolean::TRUE {
                return rraise("not an Reference object");
            }
            Ok(ReferenceM { data: T::new(x.s()) })

        }
    }
    pub fn from_cstring(x: CString) -> RResult<ReferenceM<T>> {
        unsafe {
            let res = ReferenceM { data: T::new(R_do_new_object(R_do_MAKE_CLASS(x.as_ptr()))) };
            if Rf_inherits(res.data.s(), x.as_ptr()) != Rboolean::TRUE {
                return rraise(format!("error creating Reference object of class: {:?}", x));
            }
            Ok(res)
        }

    }

    pub fn is(&self, clazz: CString) -> bool {
        unsafe { derives_from(Rf_getAttrib(self.data.s(), R_ClassSymbol), clazz) }
    }
}
