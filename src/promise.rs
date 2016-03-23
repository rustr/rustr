//! R Promise type
//!
//!


use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::error::*;
use std::convert::*;
// use std::ffi::*;
// use ::moredll::*;

pub type Promise = PromiseM<Preserve>;

gen_traits_sexp!(PromiseM);

impl<T: SEXPbucket> PromiseM<T> {
    pub fn new<E: ToSEXP>(x: E) -> RResult<PromiseM<T>> {
        unsafe {
            if RTYPEOF(x.s()) != PROMSXP {
                return rraise("not a promise");
            }

            let mut re = x.s();

            while RTYPEOF(PRCODE(re)) == PROMSXP {
                re = PRCODE(re);
            }

            Ok(PromiseM { data: T::new(re) })
        }
    }
    pub fn seen(&self) -> ::std::os::raw::c_int {
        unsafe { PRSEEN(self.data.s()) }
    }


    pub fn value<S: SEXPbucket, D: RNew>(&self) -> RResult<D> {
        unsafe {
            let val = PRVALUE(self.data.s());
            if val == R_UnboundValue {
                return rerror(REKind::UnevaluatedPromise("an Unevaluated promise value".into()));
            }
            D::rnew(val)
        }
    }

    pub fn was_evaluated(&self) -> bool {
        unsafe { PRVALUE(self.data.s()) != R_UnboundValue }
    }
    // todo return ExprVec
    pub fn expression(&self) -> SEXP {
        unsafe { PRCODE(self.data.s()) }
    }
    pub fn environment<S: SEXPbucket>(&self) -> EnvirM<S> {
        unsafe { EnvirM::from_sexp_envir(PRENV(self.data.s())) }
    }
}

use environment::*;
