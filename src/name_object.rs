//! Named Object Proxy
//!
//!

use ::rdll::*;
use ::traits::*;
use ::util::*;
use error::*;

pub type NameObject<'a> = NameObj<'a, SEXP>;

pub struct NameObj<'a, T: 'a + ToSEXP> {
    name: SEXP,
    obj: &'a T,
}

impl<'a, S: ToSEXP> ToSEXP for NameObj<'a, S> {
    unsafe fn s(&self) -> SEXP {
        self.obj.s()
    }
}

impl<'a, T: ToSEXP> IntoR for NameObj<'a, T> {
    fn intor(&self) -> RResult<SEXP> {
        Ok(unsafe { self.obj.s() })
    }
}

impl<'a, T: ToSEXP> UIntoR for NameObj<'a, T> {
    unsafe fn uintor(&self) -> SEXP {
        self.obj.s()
    }
}

impl<'a, T: ToSEXP> Named for NameObj<'a, T> {
    fn named(&self) -> bool {
        true
    }
    fn name(&self) -> SEXP {
        self.name
    }
}

impl<'a, T: 'a + ToSEXP> NameObj<'a, T> {
    pub fn from_str(x: &str, obj_: &'a T) -> NameObj<'a, T> {
        NameObj {
            name: unsafe { Rf_install(c_str(x).as_ptr()) },
            obj: obj_,
        }
    }

    pub fn from_named_obj(x: &'a NameObj<T>) -> NameObj<'a, T> {
        NameObj {
            name: x.name.clone(),
            obj: x.obj,
        }
    }
    #[allow(non_snake_case)]
    pub fn from_SEXP(x: SEXP, obj_: &'a T) -> NameObj<'a, T> {
        NameObj {
            name: x,
            obj: obj_,
        }
    }
    pub fn from_ref<E: AsRef<T>>(x: SEXP, obj_: &'a E) -> NameObj<'a, T> {
        NameObj {
            name: x,
            obj: obj_.as_ref(),
        }
    }
}

pub fn n_<'a, T: ToSEXP>(x: &str, obj_: &'a T) -> NameObj<'a, T> {
    NameObj::from_str(x, obj_)
}

// create a named object n_("name",obj)
