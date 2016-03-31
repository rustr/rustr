use rdll::*;
use num::{BigInt,Complex,BigUint};
use rtype::*;
use error::*;
use traits::*;
use protect::stackp::*;
use util::*;

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
