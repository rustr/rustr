//! R Language type
//!
//!

use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::error::*;
use std::convert::*;
use rcast::*;
use symbol::*;
use eval::*;
use environment::*;

use grow::*;

use rfunction::*;

pub type RLang = RLangM<Preserve>;

impl<T: SEXPbucket> From<SymbolM<T>> for RLangM<T> {
    fn from(x: SymbolM<T>) -> RLangM<T> {
        unsafe { RLangM { data: T::new(Rf_lang1(x.s())) } }
    }
}

impl<T: SEXPbucket> From<RFunM<T>> for RLangM<T> {
    fn from(x: RFunM<T>) -> RLangM<T> {
        unsafe { RLangM { data: T::new(Rf_lang1(x.s())) } }
    }
}

impl<T: SEXPbucket> RLangM<T> {
    pub fn new(x: &[&Args]) -> RResult<Self> {
        Ok(RLangM { data: T::new(try!(pairlist(x))) })
    }
    pub fn set_symbol(&mut self, symbol: &Symbol) {
        unsafe {
            SETCAR(self.data.s(), symbol.s());
            SET_TAG(self.data.s(), R_NilValue);
        }
    }
    pub fn set_fun(&mut self, fun: RFun) {
        unsafe {
            SETCAR(self.data.s(), fun.s());
            SET_TAG(self.data.s(), R_NilValue);
        }
    }
    pub fn eval_env<D: RNew>(&self, envir: EnvirM<T>) -> RResult<D> {
        D::rnew(try!(rustr_eval(unsafe { self.s() }, unsafe { envir.s() })))
    }
    pub fn eval<D: RNew>(&self) -> RResult<D> {
        D::rnew(try!(unsafe { rustr_eval(self.s(), R_GlobalEnv) }))
    }
    pub unsafe fn fast_eval(&self) -> SEXP {
        Rf_eval(self.s(), R_GlobalEnv)
    }
    pub unsafe fn fast_eval_env(&self, envir: EnvirM<T>) -> SEXP {
        Rf_eval(self.s(), envir.s())
    }
}
impl<T: SEXPbucket> FromCastSEXP for RLangM<T> {
    fn casts(x: SEXP) -> RResult<RLangM<T>> {
        unsafe { Ok(RLangM { data: T::new(Rf_lang1(try!(r_cast(x, LANGSXP)))) }) }
    }
}


gen_traits_sexp!(RLangM);
