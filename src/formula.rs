//! R Formula type
//!
//!

use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::error::*;

use rcast::*;

use util::*;
use std::ffi::CString;

pub type RFml = RFmlM<Preserve>;

gen_traits_sexp!(RFmlM);

impl<T: SEXPbucket> RFmlM<T> {
    pub fn new(x: SEXP) -> RResult<Self> {
        unsafe {

            let res: SEXP = match RTYPEOF(x) {
                LANGSXP => {
                    if Rf_inherits(x, c_str("formula").as_ptr()) == Rboolean::TRUE {
                        x
                    } else {
                        let zz = try!(convert_using_rfunction(x, "as.formula"));
                        zz
                    }
                }
                EXPRSXP | VECSXP => {
                    // lists or expression
                    if Rf_length(x) > 0 {
                        let y = VECTOR_ELT(x, 0);
                        if Rf_inherits(y, c_str("formula").as_ptr()) == Rboolean::TRUE {
                            y
                        } else {
                            let rr = try!(convert_using_rfunction(y, "as.formula"));
                            rr
                        }
                    } else {
                        return rraise("cannot create formula from empty list or expression");
                    }
                }
                _ => {
                    let dd = try!(convert_using_rfunction(x, "as.formula"));
                    dd
                }
            };
            Ok(RFmlM { data: T::new(res) })
        }
    }
    pub fn from_str(string: &str) -> RResult<Self> {
        let char = try!(CString::new(string));
        let dd = try!(convert_using_rfunction(unsafe { Rf_mkString(char.as_ptr()) }, "as.formula"));
        Ok(RFmlM { data: T::new(dd) })
    }
}
// impl<T: SEXPbucket> FromCastSEXP for RFmlM<T> {
//    fn casts(x: SEXP) -> RResult<RLangImpl<T>> {
//        unsafe { Ok(RLangImpl { data: T::new(Rf_lang1(try!(r_cast(x, LANGSXP)))) }) }
//    }
// }


impl<T: SEXPbucket> RNew for RFmlM<T> {
    fn rnew(x: SEXP) -> RResult<Self> {
        Self::new(x)
    }
}
