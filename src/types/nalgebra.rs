use ::rdll::*;
use ::rtype::*;
use error::*;
use traits::*;
use ::protect::stackp::*;
use std::ffi::*;
use nalgebra::*;

macro_rules! gen_fromr_vec {
    ($sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for DVec<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
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
                vecs[ii] = *rptr.offset(ii as isize) as $x;
            }
            vecs
        }
    }


impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector($sexp, size_x as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as $into ;
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

gen_fromr_vec!(INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec!(REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);

// u8

impl RNew for DVec<u8> {
    fn rnew(x:SEXP) -> RResult<DVec<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for DVec<u8> {
    unsafe fn urnew(x:SEXP) -> DVec<u8> {
            let lens = Rf_xlength(x);
            let mut vecs: DVec<u8> = DVec::new_uninitialized(lens as usize);
            if RTYPEOF(x) ==INTSXP{
	            let rptr = INTEGER(x);
	            for ii in 0..lens {
	                vecs[ii] = *rptr.offset(ii as isize) as u8;
	            }
	             return vecs;
            }
            let rptr = RAW(x);
            for ii in 0..lens {
				vecs[ii] = *rptr.offset(ii as isize) as u8;
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
            for ii in self {
                *rptr.offset(index) = ii.clone() as ::std::os::raw::c_int ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for DVec<u8> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

