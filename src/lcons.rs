//! R lcons methods
//!
//!

use rdll::*;
use protect::stackp::*;
use traits::*;

#[inline]
pub fn rustr_lcons<T: ToSEXP>(car: T, cdr: T) -> SEXP {
    unsafe { Shield::new(Rf_lcons(car.s(), cdr.s())).s() }
}

#[inline]
pub fn rustr_list1<T: ToSEXP>(x: T) -> SEXP {
    unsafe { Shield::new(Rf_list1(x.s())).s() }
}

#[inline]
pub fn rustr_lang1<T: ToSEXP>(x: T) -> SEXP {
    unsafe { Shield::new(Rf_lang1(x.s())).s() }
}

#[inline]
pub fn rustr_list2<T: ToSEXP>(x0: T, x1: T) -> SEXP {
    unsafe { Shield::new(Rf_cons(x0.s(), rustr_list1(x1.s()))).s() }
}

#[inline]
pub fn rustr_lang2<T: ToSEXP>(x0: T, x1: T) -> SEXP {
    unsafe { Shield::new(Rf_lcons(x0.s(), rustr_list1(x1.s()))).s() }
}

#[inline]
pub fn rustr_list3<T: ToSEXP>(x0: T, x1: T, x2: T) -> SEXP {
    unsafe { Shield::new(Rf_cons(x0.s(), rustr_list2(x1.s(), x2.s()))).s() }
}

#[inline]
pub fn rustr_lang3<T: ToSEXP>(x0: T, x1: T, x2: T) -> SEXP {
    unsafe { Shield::new(Rf_lcons(x0.s(), rustr_list2(x1.s(), x2.s()))).s() }
}

#[inline]
pub fn rustr_list4<T: ToSEXP>(x0: T, x1: T, x2: T, x3: T) -> SEXP {
    unsafe { Shield::new(Rf_cons(x0.s(), rustr_list3(x1.s(), x2.s(), x3.s()))).s() }
}

#[inline]
pub fn rustr_lang4<T: ToSEXP>(x0: T, x1: T, x2: T, x3: T) -> SEXP {
    unsafe { Shield::new(Rf_lcons(x0.s(), rustr_list3(x1.s(), x2.s(), x3.s()))).s() }
}

// .... if sb needed this, we can add rustr_lang20
