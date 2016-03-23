//! Helper macro
//!
//!

use error::*;
use rdll::{R_ToplevelExec, Rboolean};

#[inline]
pub fn rbool(x: Rboolean) -> bool {
    x == Rboolean::TRUE
}

/// Check User Interrupt
/// 
#[macro_export]
macro_rules! check_interrupt_return {
    () => (
        unsafe{
            return rerror($crate::error::REKind::Interrupted("user interrpted".into()))
        }
        	)
}

pub fn check_interrupt() -> RResult<()> {
    unsafe {
        if R_ToplevelExec(::std::option::Option::Some(::util::check_interrupt_fn),
                          ::std::ptr::null_mut()) == Rboolean::FALSE {
            // NULL=0
            rerror(REKind::Interrupted("user interrpted".into()))
        } else {
            Ok(())
        }
    }
}


/// For rustinr codegen, it moves error to R
/// 
#[macro_export]
macro_rules! unwrapr_void {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
        	$crate::error::forward_exception_to_r(err);
            return;
        }
    	}
    )
}

/// For rustinr codegen, it moves error to R
/// 
#[macro_export]
macro_rules! rtry {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
        	return Err($crate::error::RError::other(err));
        }
    	}
    )
}

/// For rustinr codegen, it moves error to R
/// 
#[macro_export]
macro_rules! unwrapr {
    ($expr:expr) => (match $expr {
        Ok(val) => val,
        Err(err) => {
        	$crate::error::forward_exception_to_r(err);
        	unsafe{
				return R_NilValue;
        	}
        }
    })
}

/// For RPtr
/// 
#[macro_export]
macro_rules! gen_traits_rptr{
	($typ:ident)=>(

impl<Obj:Any, T: SEXPbucket> IntoR for $typ<Obj, T> {
    fn intor(&self) -> RResult<SEXP> {
        Ok(unsafe{self.data.s()})
    }
}


impl<Obj:Any, T: SEXPbucket> UIntoR for $typ<Obj, T> {
    unsafe fn uintor(&self) -> SEXP {
        {self.data.s()}
    }
}

impl<Obj:Any, T: SEXPbucket> RAttribute for $typ<Obj, T> {}
impl<Obj:Any, T: SEXPbucket> RSlot for $typ<Obj, T> {}

impl<Obj:Any, T: SEXPbucket> ToSEXP for $typ<Obj, T> {
    unsafe fn s(&self) -> SEXP {
        self.data.s()
    }
}

impl<Obj:Any, T: SEXPbucket> SetSelf for $typ<Obj, T> {
    fn set(&mut self, x: Self) {
        unsafe{self.data.set(x.s());}
    }
}

impl<Obj:Any,T: SEXPbucket> Shallow for $typ<Obj,T> {
    fn sc(&self) ->Self{
        unsafe{ $typ{data: T::new(self.s()),obj:PhantomData, tag:self.tag}}
    }
}

impl<Obj:Any,T: SEXPbucket> Clone for $typ<Obj,T> {
    fn clone(&self) -> Self {
        unsafe { $typ { data: T::new(Rf_duplicate(self.data.s())), obj:PhantomData,tag:self.tag } }
    }
}
impl<Obj:Any,T: SEXPbucket> RObjClass for $typ<Obj,T> {
    type Origin = $typ<Obj,T> ;
}
	
	)
}

/// For SEXP
/// 
#[macro_export]
macro_rules! gen_traits_sexp{
	($typ:ident)=>(
		
#[derive(Debug)]
pub struct $typ<T: SEXPbucket> {
    data: T,
}

impl< T: SEXPbucket> IntoR for $typ< T> {
    fn intor(&self) -> RResult<SEXP> {
        Ok(unsafe{self.data.s()})
    }
}

impl<T: SEXPbucket> RAttribute for $typ<T> {}
impl<T: SEXPbucket> RSlot for $typ<T> {}

impl<T: SEXPbucket> ToSEXP for $typ<T> {
    unsafe fn s(&self) -> SEXP {
        self.data.s()
    }
}

impl<T: SEXPbucket> SetSelf for $typ<T> {
    fn set(&mut self, x: Self) {
        unsafe{self.data.set(x.s());}
    }
}

impl<T: SEXPbucket> Shallow for $typ<T> {
    fn sc(&self) ->Self{
        unsafe{ $typ{data: T::new(self.s())}}
    }
}

impl<T: SEXPbucket> Clone for $typ<T> {
    fn clone(&self) -> Self {
        unsafe { $typ { data: T::new(Rf_duplicate(self.data.s()))} }
    }
}
impl<T: SEXPbucket> RObjClass for $typ<T> {
    type Origin = $typ<T>;
}

impl<T: SEXPbucket> Named for $typ<T>{}

	)
}
