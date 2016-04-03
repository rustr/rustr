use storage::*;
use traits::*;
use error::*;
use rtype::*;
use rdll::*;

pub struct RCharM<T:SEXPbucket>{
	data:T
}
pub type RChar = RCharM<Preserve>;

impl<T:SEXPbucket>  RCharM<T>{
	pub fn new<E: ToSEXP>(x:SEXP)->RResult<Self>{
		if RTYPEOF(x) != CHARSXP {
            return rraise("not a CHARSXP");
        }
		Ok(RCharM { data: T::new(x) })
	}
	pub fn na_char()->RCharM<NoProtect>{
		RCharM { data: NoProtect::new(unsafe { R_NaString }) }
	}
	pub fn na()->SEXP{
		unsafe { R_NaString }
	}
}


impl<T:SEXPbucket> RNew for RCharM<T>{
	fn rnew(x:SEXP)->RResult<Self>{
				if RTYPEOF(x) != CHARSXP {
            return rraise("not a CHARSXP");
        }
		Ok(RCharM { data: T::new(x) })
	}
}

impl<T:SEXPbucket> URNew for RCharM<T>{
	unsafe fn urnew(x:SEXP)->Self{
		RCharM { data: T::new(x) }
	}
}

impl<T:SEXPbucket> IntoR for RCharM<T>{
	fn intor(&self)->RResult<SEXP>{
		Ok(unsafe{self.data.s()})
	}
}

impl<T:SEXPbucket> UIntoR for RCharM<T>{
	unsafe fn uintor(&self)-> SEXP{
		self.data.s()
	}
}

impl<T: SEXPbucket> ToSEXP for RCharM<T> {
    unsafe fn s(&self) -> SEXP {
        self.data.s()
    }
}

impl<T: SEXPbucket> SetSelf for RCharM<T> {
    fn set(&mut self, x: Self) {
        unsafe{self.data.set(x.s());}
    }
}

impl<T: SEXPbucket> Shallow for RCharM<T> {
    fn sc(&self) ->Self{
        unsafe{ RCharM{data: T::new(self.s())}}
    }
}

impl<T: SEXPbucket> Clone for RCharM<T> {
    fn clone(&self) -> Self {
        unsafe { RCharM { data: T::new(Rf_duplicate(self.data.s()))} }
    }
}
impl<T: SEXPbucket> RObjClass for RCharM<T> {
    type Origin = RCharM<T>;
}

use std::ffi::*;

impl<T: SEXPbucket,D:Into<CString>>  From<D> for RCharM<T>{
	fn from(x:D)->RCharM<T>{
		unsafe {RCharM { data: T::new(Rf_mkChar(x.into().as_ptr()))}}
	}
}
