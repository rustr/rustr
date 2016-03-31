use ::rdll::*;
use ::rtype::*;
use error::*;
use traits::*;
use ::protect::stackp::*;
use nalgebra::{DVec, Vec1, Vec2, Vec3, Vec4, Vec5, Vec6, DVec1, DVec2, DVec3, DVec4, DVec5, DVec6,
               Iterable, IterableMut};
use num::Zero;

macro_rules! gen_fromr_vec {
    ($sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for DVec<$x> {
    fn rnew(x:SEXP) -> RResult<DVec<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for DVec<$x> {
    unsafe fn urnew(x:SEXP) -> DVec<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: DVec<$x> = DVec::new_uninitialized(lens as usize);
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs[ii as usize] = *rptr.offset(ii as isize) as $x;
            }
            vecs
        }
    }


impl UIntoR for DVec<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector($sexp, size_x as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self.iter() {
                *rptr.offset(index) = ii.clone() as $into ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for DVec<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_vec!(INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec!(REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);

// u8

impl RNew for DVec<u8> {
    fn rnew(x: SEXP) -> RResult<DVec<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for DVec<u8> {
    unsafe fn urnew(x: SEXP) -> DVec<u8> {
        let lens = Rf_xlength(x);
        let mut vecs: DVec<u8> = DVec::new_uninitialized(lens as usize);
        if RTYPEOF(x) == INTSXP {
            let rptr = INTEGER(x);
            for ii in 0..lens {
                vecs[ii as usize] = *rptr.offset(ii as isize) as u8;
            }
            return vecs;
        }
        let rptr = RAW(x);
        for ii in 0..lens {
            vecs[ii as usize] = *rptr.offset(ii as isize) as u8;
        }
        vecs
    }
}


impl UIntoR for DVec<u8> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();
        let rvec = Shield::new(Rf_allocVector(INTSXP, size_x as R_xlen_t));
        let rptr = INTEGER(rvec.s());
        let mut index = 0;
        for ii in self.iter() {
            *rptr.offset(index) = *ii as ::std::os::raw::c_int;
            index = index + 1;
        }
        rvec.s()

    }
}

impl IntoR for DVec<u8> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe { Ok(Self::uintor(self)) }
    }
}

// stack vec

macro_rules! gen_fromr_vec1 {
    ($dvec:ident; $lens:expr; $sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $dvec<$x> {
    fn rnew(x:SEXP) -> RResult<$dvec<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            if Rf_xlength(x) != $lens{
            	return rerror(REKind::NotCompatible(concat!("expecting length",$lens).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $dvec<$x> {
    unsafe fn urnew(x:SEXP) -> $dvec<$x> {
            let mut vecs: $dvec<$x> = $dvec::zero();
            let rptr = $sexpget(x);
            for (ii,item) in vecs.iter_mut().enumerate().take(1) {
                *item = *rptr.offset(ii as isize) as $x;
            }
            vecs
        }
    }


impl UIntoR for $dvec<$x> {
    unsafe fn uintor(&self) -> SEXP {
            let rvec = Shield::new(Rf_allocVector($sexp, $lens as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self.iter() {
                *rptr.offset(index) = ii.clone() as $into ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $dvec<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_vec1!(Vec1;1;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Vec1;1;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Vec2;2;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Vec2;2;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Vec3;3;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Vec3;3;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Vec4;4;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Vec4;4;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Vec5;5;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Vec5;5;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Vec6;6;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Vec6;6;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);


// u8 stack vec

// u8
macro_rules! gen_fromr_vec1_u8 {
    ($dvec:ident; $lens:expr) => (

impl RNew for $dvec<u8> {
    fn rnew(x:SEXP) -> RResult<$dvec<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            if Rf_xlength(x) != $lens{
            	return rerror(REKind::NotCompatible(concat!("expecting length",$lens).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $dvec<u8> {
    unsafe fn urnew(x:SEXP) -> $dvec<u8> {
            let mut vecs = $dvec::zero();
            if RTYPEOF(x) ==INTSXP{
	            let rptr = INTEGER(x);
	            for (ii,item) in vecs.iter_mut().enumerate().take(1) {
	                *item = *rptr.offset(ii as isize) as u8;
	            }
	             return vecs;
            }
            let rptr = RAW(x);
            for (ii,item) in vecs.iter_mut().enumerate().take(1) {
                *item = *rptr.offset(ii as isize) as u8;
            }
            vecs
        }
    }


impl UIntoR for $dvec<u8> {
    unsafe fn uintor(&self) -> SEXP {
            let rvec = Shield::new(Rf_allocVector(INTSXP, $lens));
            let rptr = INTEGER(rvec.s());
            let mut index = 0;
            for ii in self.iter() {
                *rptr.offset(index) = *ii as ::std::os::raw::c_int ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $dvec<u8> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block

    )
}

gen_fromr_vec1_u8!(Vec1;1);
gen_fromr_vec1_u8!(Vec2;2);
gen_fromr_vec1_u8!(Vec3;3);
gen_fromr_vec1_u8!(Vec4;4);
gen_fromr_vec1_u8!(Vec5;5);
gen_fromr_vec1_u8!(Vec6;6);


macro_rules! gen_fromr_dvec1 {
    ($dvec:ident; $lens:expr; $sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $dvec<$x> {
    fn rnew(x:SEXP) -> RResult<$dvec<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            if Rf_xlength(x) > $lens{
            	return rerror(REKind::NotCompatible(concat!("expecting length <",$lens).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $dvec<$x> {
    unsafe fn urnew(x:SEXP) -> $dvec<$x> {
    		let xx: Vec<$x> = Vec::urnew(x);
             $dvec::from_slice(xx.len(),&xx)
        }
    }


impl UIntoR for $dvec<$x> {
    unsafe fn uintor(&self) -> SEXP {
            let rvec = Shield::new(Rf_allocVector($sexp, $lens as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self.iter() {
                *rptr.offset(index) = ii.clone() as $into ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $dvec<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_dvec1!(DVec1;1;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_dvec1!(DVec1;1;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_dvec1!(DVec2;2;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_dvec1!(DVec2;2;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_dvec1!(DVec3;3;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_dvec1!(DVec3;3;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_dvec1!(DVec4;4;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_dvec1!(DVec4;4;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_dvec1!(DVec5;5;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_dvec1!(DVec5;5;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_dvec1!(DVec6;6;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_dvec1!(DVec6;6;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);

macro_rules! gen_fromr_dvec1_u8 {
    ($dvec:ident; $lens:expr) => (

// main block
impl RNew for $dvec<u8> {
    fn rnew(x:SEXP) -> RResult<$dvec<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            if Rf_xlength(x) > $lens{
            	return rerror(REKind::NotCompatible(concat!("expecting length <",$lens).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $dvec<u8> {
    unsafe fn urnew(x:SEXP) -> $dvec<u8> {
    		let xx: Vec<u8> = Vec::urnew(x);
			$dvec::from_slice(xx.len(),&xx)
        }
    }


impl UIntoR for $dvec<u8> {
    unsafe fn uintor(&self) -> SEXP {
            let rvec = Shield::new(Rf_allocVector(INTSXP, $lens as R_xlen_t));
            let rptr = INTEGER(rvec.s());
            let mut index = 0;
            for ii in self.iter() {
                *rptr.offset(index) = *ii as ::std::os::raw::c_int ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $dvec<u8> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block

    )
}


gen_fromr_dvec1_u8!(DVec1;1);
gen_fromr_dvec1_u8!(DVec2;2);
gen_fromr_dvec1_u8!(DVec3;3);
gen_fromr_dvec1_u8!(DVec4;4);
gen_fromr_dvec1_u8!(DVec5;5);
gen_fromr_dvec1_u8!(DVec6;6);
