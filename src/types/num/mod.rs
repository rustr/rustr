use rdll::*;
use num::{BigInt,Complex,BigUint};
use rtype::*;
use error::*;
use traits::*;
use protect::stackp::*;
use util::*;
use std::collections::{VecDeque,  BTreeSet, BinaryHeap, HashSet, LinkedList};


impl From<Rcomplex> for Complex<::std::os::raw::c_double>{
	fn from(x: Rcomplex)->Self{
		Complex { re: x.r, im:x.i}
	}
}

impl IntoR for BigInt {
    fn intor(&self) -> RResult<SEXP> {

        unsafe { Ok(Self::uintor(self)) }
    }
}

impl UIntoR for BigInt {
    unsafe fn uintor(&self) -> SEXP {
        let rvec = Shield::new(Rf_allocVector(STRSXP, 1));
        SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(&self.to_str_radix(10)).as_ptr()));
        rvec.s()
    }
}


impl IntoR for BigUint {
    fn intor(&self) -> RResult<SEXP> {

        unsafe { Ok(Self::uintor(self)) }
    }
}

impl UIntoR for BigUint {
    unsafe fn uintor(&self) -> SEXP {
        let rvec = Shield::new(Rf_allocVector(STRSXP, 1));
        SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(&self.to_str_radix(10)).as_ptr()));
        rvec.s()
    }
}

impl RNew for BigInt {
    fn rnew(x:SEXP) -> RResult<Self> {
        if RTYPEOF(x) != INTSXP || unsafe{Rf_xlength(x) != 1} {
            return rerror(REKind::NotCompatible("expecting a int value".into()));
        }
        unsafe { Ok(Self::urnew(x)) }
    }
}

impl URNew for BigInt {
    unsafe fn urnew(x:SEXP) -> Self {
        let rvec = INTEGER(x);
        BigInt::from(*rvec.offset(0))
    }
}

impl RNew for BigUint {
    fn rnew(x:SEXP) -> RResult<Self> {
        if RTYPEOF(x) != INTSXP || unsafe{Rf_xlength(x) != 1} {
            return rerror(REKind::NotCompatible("expecting a int value".into()));
        }
        unsafe { Ok(Self::urnew(x)) }
    }
}

impl URNew for BigUint {
    unsafe fn urnew(x:SEXP) -> Self {
        let rvec = INTEGER(x);
        BigUint::from(*rvec.offset(0) as usize)
    }
}

macro_rules! gen_fromr_vec {
    ($collection:ident; $push:ident; $sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::with_capacity(lens as usize);
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push( BigInt::from(*rptr.offset(ii as isize)));
            }
            vecs
        }
    }


impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let mut index = 0;
            for ii in self {
				SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(&ii.to_str_radix(10)).as_ptr()));
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_vec!(Vec; push; INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigInt);
gen_fromr_vec!(BinaryHeap; push;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigInt);
gen_fromr_vec!(VecDeque; push_back; INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigInt);
gen_fromr_vec!(HashSet; insert;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigInt);



macro_rules! gen_fromr_vec_uint {
    ($collection:ident; $push:ident; $sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::with_capacity(lens as usize);
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push( BigUint::from(*rptr.offset(ii as isize) as usize));
            }
            vecs
        }
    }


impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let mut index = 0;
            for ii in self {
				SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(&ii.to_str_radix(10)).as_ptr()));
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_vec_uint!(Vec; push; INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigUint);
gen_fromr_vec_uint!(BinaryHeap; push;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigUint);
gen_fromr_vec_uint!(VecDeque; push_back; INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigUint);
gen_fromr_vec_uint!(HashSet; insert;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigUint);

macro_rules! gen_fromr_linklist {
    ($collection:ident; $push:ident;$sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::new();
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push( BigInt::from(*rptr.offset(ii as isize)) );
            }
            vecs
    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let mut index = 0;
            for ii in self {
				SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(&ii.to_str_radix(10)).as_ptr()));
				index = index + 1;
            }
            rvec.s()

    }
}


// end main block
		)*
    )
}

gen_fromr_linklist!(LinkedList;push_front;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigInt);
gen_fromr_linklist!(BTreeSet;insert;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigInt);

macro_rules! gen_fromr_linklist_uint {
    ($collection:ident; $push:ident;$sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::new();
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push( BigUint::from(*rptr.offset(ii as isize) as usize) );
            }
            vecs
    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let mut index = 0;
            for ii in self {
				SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(&ii.to_str_radix(10)).as_ptr()));
				index = index + 1;
            }
            rvec.s()

    }
}


// end main block
		)*
    )
}

gen_fromr_linklist_uint!(LinkedList;push_front;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigUint);
gen_fromr_linklist_uint!(BTreeSet;insert;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; BigUint);
