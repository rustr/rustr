use ::traits::*;
use ::rdll::*;

// use ::std::borrow::Borrow;
#[inline]
pub unsafe fn rustr_protect(x: SEXP) -> SEXP {
    if x != R_NilValue {
        Rf_protect(x);
    }
    x
}

// use ::std::borrow::Borrow;
#[inline]
pub unsafe fn rustr_unprotect(x: SEXP) {
    if x != R_NilValue {
        Rf_unprotect(1)
    }
}
/// Shield
///
///Shield is safe to use
pub struct Shield {
    t: SEXP,
}


impl Shield {
    pub fn new<T: ToSEXP>(t_: T) -> Shield {
        unsafe {
            // println!("{:?}", "protecting \n");
            Shield { t: rustr_protect(t_.s()) }
        }
    }
}

impl Drop for Shield {
    fn drop(&mut self) {
        unsafe {
            // println!("{:?}", "droping \n");
            if self.t != R_NilValue {
                Rf_unprotect(1)
            };
        }
    }
}

impl ToSEXP for Shield {
    unsafe fn s(&self) -> SEXP {
        (self.t).clone()
    }
}

impl Moves for Shield {
    fn moves(mut x: Self) -> Shield {
        let res = Shield { t: x.t };
        unsafe {
            x.t = R_NilValue;
        }

        res
    }
}


impl Others for Shield {
    fn others(x: &Shield) -> Shield {
        unsafe { Shield { t: rustr_protect(x.t.clone()) } }
    }
}
