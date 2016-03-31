use ::rdll::*;
use ::rtype::*;
use error::*;
use traits::*;
use ::protect::stackp::*;
use nalgebra::{DMat, Mat1, Mat2, Mat3, Mat4, Mat5, Mat6};
use num::Zero;
// use ::rprint;

macro_rules! gen_fromr_vec {
    ($sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for DMat<$x> {
    fn rnew(x:SEXP) -> RResult<DMat<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            if !x.is_matrix() {
                return rerror(REKind::NotCompatible(concat!("expecting a ","matrix").into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for DMat<$x> {
    unsafe fn urnew(x:SEXP) -> DMat<$x> {
			let res = Shield::new(Rf_getAttrib(x, R_DimSymbol));
            let dims = Vec::urnew(res.s());
            let mut vecs: DMat<$x> = DMat::new_uninitialized(dims[0],dims[1]);
            let rptr = $sexpget(x);
            for ii in 0..dims[1] {
            	for jj in 0..dims[0]{
					vecs[(jj as usize, ii as usize)] = *rptr.offset((jj*dims[1]+ii) as isize) as $x;
            	}
            }
            vecs
        }
    }

impl UIntoR for DMat<$x> {
    unsafe fn uintor(&self) -> SEXP {
            let rvec = Shield::new(Rf_allocMatrix($sexp, self.nrows() as ::std::os::raw::c_int, self.ncols()  as ::std::os::raw::c_int));
            let rptr = $sexpget(rvec.s());
            for ii in 0..self.ncols() {
            	for jj in 0..self.nrows(){
					*rptr.offset((jj*self.ncols()+ii) as isize) = self[(jj as usize, ii as usize)] as $into;
            	}
            }
            rvec.s()
    }
}

impl IntoR for DMat<$x> {
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

impl RNew for DMat<u8> {
    fn rnew(x: SEXP) -> RResult<DMat<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            if !x.is_matrix() {
                return rerror(REKind::NotCompatible(concat!("expecting a ", "matrix").into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for DMat<u8> {
    unsafe fn urnew(x: SEXP) -> DMat<u8> {
        let res = Shield::new(Rf_getAttrib(x, R_DimSymbol));
        let dims = Vec::urnew(res.s());
        let mut vecs: DMat<u8> = DMat::new_uninitialized(dims[0], dims[1]);
        if RTYPEOF(x) == INTSXP {
            let rptr = INTEGER(x);
            for ii in 0..dims[1] {
                for jj in 0..dims[0] {
                    vecs[(jj as usize, ii as usize)] =
                        *rptr.offset((jj * dims[0] + ii) as isize) as u8;
                }
            }
            return vecs;
        }
        let rptr = RAW(x);
        for ii in 0..dims[1] {
            for jj in 0..dims[0] {
                vecs[(jj as usize, ii as usize)] = *rptr.offset((jj * dims[1] + ii) as isize) as u8;
            }
        }
        vecs
    }
}


impl UIntoR for DMat<u8> {
    unsafe fn uintor(&self) -> SEXP {
        let rvec = Shield::new(Rf_allocMatrix(INTSXP,
                                              self.nrows() as ::std::os::raw::c_int,
                                              self.ncols() as ::std::os::raw::c_int));
        let rptr = INTEGER(rvec.s());
        for ii in 0..self.ncols() {
            for jj in 0..self.nrows() {
                *rptr.offset((jj * self.ncols() + ii) as isize) =
                    self[(jj as usize, ii as usize)] as ::std::os::raw::c_int;
            }
        }
        rvec.s()
    }
}

impl IntoR for DMat<u8> {
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
			if !x.is_matrix() {
                return rerror(REKind::NotCompatible(concat!("expecting a ","matrix").into()));
            }
			let res = Shield::new(Rf_getAttrib(x, R_DimSymbol));
			let dims : Vec<usize>= Vec::urnew(res.s());
            if dims[0] != $lens || dims[1] != $lens {
            	return rerror(REKind::NotCompatible(concat!("expecting row: ",$lens,", cols: ", $lens).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $dvec<$x> {
    unsafe fn urnew(x:SEXP) -> $dvec<$x> {
			let res = Shield::new(Rf_getAttrib(x, R_DimSymbol));
            let dims = Vec::urnew(res.s());
            let mut vecs: $dvec<$x> = $dvec::zero();
            let rptr = $sexpget(x);
            for ii in 0..dims[1] {
            	for jj in 0..dims[0]{
					vecs[(jj as usize, ii as usize)] = *rptr.offset((jj*dims[0]+ii) as isize) as $x;
            	}
            }
            vecs
        }
    }


impl UIntoR for $dvec<$x> {
    unsafe fn uintor(&self) -> SEXP {
 			let rvec = Shield::new(Rf_allocMatrix($sexp,  $lens , $lens));
            let rptr = $sexpget(rvec.s());
            for ii in 0..$lens {
            	for jj in 0..$lens{
					*rptr.offset((jj* $lens+ii) as isize) = self[(jj as usize, ii as usize)] as $into;
            	}
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

gen_fromr_vec1!(Mat1;1;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Mat1;1;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Mat2;2;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Mat2;2;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Mat3;3;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Mat3;3;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Mat4;4;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Mat4;4;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Mat5;5;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Mat5;5;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec1!(Mat6;6;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec1!(Mat6;6;REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);


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
			if !x.is_matrix() {
                return rerror(REKind::NotCompatible(concat!("expecting a ","matrix").into()));
            }
			let res = Shield::new(Rf_getAttrib(x, R_DimSymbol));
			let dims : Vec<usize>= Vec::urnew(res.s());
            if dims[0] != $lens || dims[1] != $lens {
            	return rerror(REKind::NotCompatible(concat!("expecting row: ",$lens,", cols: ", $lens).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $dvec<u8> {
    unsafe fn urnew(x:SEXP) -> $dvec<u8> {
			let res = Shield::new(Rf_getAttrib(x, R_DimSymbol));
            let dims = Vec::urnew(res.s());
            let mut vecs: $dvec<u8> = $dvec::zero();
            if RTYPEOF(x) ==INTSXP{
	            let rptr = INTEGER(x);
            for ii in 0..dims[1] {
            	for jj in 0..dims[0]{
					vecs[(jj as usize, ii as usize)] = *rptr.offset((jj*dims[0]+ii) as isize) as u8;
            	}
            }
	             return vecs;
            }
            let rptr = RAW(x);
            for ii in 0..dims[1] {
            	for jj in 0..dims[0]{
					vecs[(jj as usize, ii as usize)] = *rptr.offset((jj*dims[0]+ii) as isize) as u8;
            	}
            }
            vecs
        }
    }


impl UIntoR for $dvec<u8> {
    unsafe fn uintor(&self) -> SEXP {

 			let rvec = Shield::new(Rf_allocMatrix(INTSXP,  $lens , $lens));
            let rptr = INTEGER(rvec.s());
            for ii in 0..$lens {
            	for jj in 0..$lens{
					*rptr.offset((jj* $lens+ii) as isize) = self[(jj as usize, ii as usize)] as  ::std::os::raw::c_int;
            	}
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

gen_fromr_vec1_u8!(Mat1;1);
gen_fromr_vec1_u8!(Mat2;2);
gen_fromr_vec1_u8!(Mat3;3);
gen_fromr_vec1_u8!(Mat4;4);
gen_fromr_vec1_u8!(Mat5;5);
gen_fromr_vec1_u8!(Mat6;6);
