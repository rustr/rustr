//! R Memory Protection Functions
//!
//!

use ::rdll::*;

#[inline]
pub unsafe fn rustr_release_object(x: SEXP) -> SEXP {
    if x != R_NilValue {
        R_ReleaseObject(x);
    }
    x
}


#[inline]
pub unsafe fn rustr_preserve_object(x: SEXP) -> SEXP {
    if x != R_NilValue {
        R_PreserveObject(x);
    }
    x
}

#[inline]
pub unsafe fn rustr_replace_object(x: SEXP, y: SEXP) -> SEXP {
    if Rf_isNull(x) == Rboolean::TRUE {
        rustr_preserve_object(y);
    } else if Rf_isNull(y) == Rboolean::TRUE {
        rustr_release_object(x);
    } else {
        // if we are setting to the same SEXP as we already have, do nothing
        if x != y {

            // the previous SEXP was not NULL, so release it
            rustr_release_object(x);

            // the new SEXP is not NULL, so preserve it
            rustr_preserve_object(y);

        }
    }
    y
}

pub mod stackp;
pub mod indexp;
pub mod countp;
