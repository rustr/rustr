//! R Function type
//!
//!
//!


use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::error::*;
use std::convert::*;

use symbol::*;

use environment::*;
use protect::stackp::*;
use eval::*;
use grow::*;


pub type RFun = RFunM<Preserve>;


impl<T: SEXPbucket> NewRObj for RFunM<T> {
    fn new<E: ToSEXP>(x: E) -> RResult<Self> {
        match unsafe { RTYPEOF(x.s()) } {
            CLOSXP | SPECIALSXP | BUILTINSXP => unsafe {
                 Ok(RFunM { data: T::new(x.s()) })
            },
            _ => rraise("cannot convert to function")
        }
    }
    unsafe fn unew<E: ToSEXP>(x: E) -> Self {
			RFunM { data: T::new(x.s()) }
    }
}

impl<T: SEXPbucket> RFunM<T> {
    pub fn set_sexp<S: ToSEXP>(&mut self, x: S) -> RResult<()> {

        match unsafe { RTYPEOF(x.s()) } {
            CLOSXP | SPECIALSXP | BUILTINSXP => {
                self.data.set(unsafe { x.s() });
                Ok(())
            }
            _ => rraise("cannot convert to function"),
        }
    }
    pub fn from_str_global(x: &str) -> RResult<RFunM<T>> {
        let sym = Symbol::from(x);
        unsafe { RFunM::new(Rf_findFun(sym.s(), R_GlobalEnv)) }
    }
    pub fn from_str<EE: SEXPbucket>(x: &str, env: EnvirM<EE>) -> RResult<RFunM<T>> {
        let sym = Symbol::from(x);
        unsafe { RFunM::new(Rf_findFun(sym.s(), env.s())) }
    }
    pub fn ufrom_str_global(x: &str) -> RFunM<T> {
        let sym = Symbol::from(x);
        unsafe { RFunM::unew(Rf_findFun(sym.s(), R_GlobalEnv)) }
    }
    pub fn ufrom_str<EE: SEXPbucket>(x: &str, env: EnvirM<EE>) -> RFunM<T> {
        let sym = Symbol::from(x);
        unsafe { RFunM::unew(Rf_findFun(sym.s(), env.s())) }
    }
    pub fn envir<S: SEXPbucket>(&self) -> RResult<EnvirM<S>> {
        unsafe {
            if RTYPEOF(self.s()) != CLOSXP {
                return rraise(format!("not a closure, type = {:?}",
                                      match_rtype(RTYPEOF(self.s()))));
            }
            Ok(EnvirM::from_sexp_envir(CLOENV(self.s())))
        }
    }
    pub fn body(&self) -> SEXP {
        unsafe { BODY(self.s()) }
    }
    pub fn eval<D: RNew>(&self, args: &[&Args]) -> RResult<D> {
        let call = Shield::new(try!(language1(self, args)));
        D::rnew(try!(rustr_eval(unsafe { call.s() }, unsafe { R_GlobalEnv })))
    }
    pub fn eval_env<EE: SEXPbucket, D: RNew>(&self,
                                             args: &[&Args],
                                             envir: EnvirM<EE>)
                                             -> RResult<D> {
        let call = Shield::new(try!(language1(self, args)));
        D::rnew(try!(rustr_eval(unsafe { call.s() }, unsafe { envir.s() })))
    }
}

// impl<'a, T: SEXPbucket> From<&'a str> for RFunM<T> {
//    fn from(x: &'a str) -> RFunM<T> {
//        RFunM::from_str_global(x)
//    }
// }




impl<T: SEXPbucket> RNew for RFunM<T> {
    fn rnew(x: SEXP) -> RResult<Self> {
        match unsafe { RTYPEOF(x.s()) } {
            CLOSXP | SPECIALSXP | BUILTINSXP => {
                Ok(RFunM { data: T::new(unsafe { x.s() }) })
            }
            _ =>  rraise("cannot convert to function"),
        }
    }
}

gen_traits_sexp!(RFunM);
