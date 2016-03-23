//! Preserve and Unprotect SEXP struct
//!

use ::rdll::*;
use ::protect::*;
use ::traits::*;
use ::util::*;
use ::error::*;
use ::error::REKind::*;

// pub enum RType{
// 	CHAR(Preserve),
// 	LGL(Preserve),
// 	INT(Preserve),
// 	REAL(Preserve),
// 	CPLX(Preserve),
// 	STR(Preserve),
// 	VEC(Preserve),
// 	S4(Preserve),
// 	RAW(Preserve),
// 	EXPR(Preserve),
// }

pub type Pre = Preserve;
pub type Nop = NoProtect;

#[derive(PartialEq, Eq, Debug)]
pub struct Preserve {
    data: SEXP,
}

pub trait SEXPbucket :Drop+ToSEXP {
    fn init() -> Self;
    fn new(x: SEXP) -> Self;
    fn invalidate(&mut self) -> SEXP;
    fn inherits(&self, class_: &str) -> RResult<bool>;
    fn set(&mut self, x: SEXP);
    fn copy<'a, 'b>(&'a mut self, other: &'b Self) -> &'a Self;
}

impl SEXPbucket for Preserve {
    fn init() -> Preserve {
        unsafe { Preserve { data: R_NilValue } }

    }

    fn new(x: SEXP) -> Preserve {
        unsafe {
            rustr_preserve_object(x);
        }

        Preserve { data: x }
    }

    fn invalidate(&mut self) -> SEXP {
        let out = self.data;
        unsafe {
            self.data = R_NilValue;
        }
        out
    }

    fn inherits(&self, class_: &str) -> RResult<bool> {
        let class = match cstring_user(class_) {
            Ok(x) => x,
            Err(y) => return rerror(Other(y.into())),
        };

        unsafe { Ok(Rf_inherits((self.data), class.as_ptr()) == Rboolean::TRUE) }
    }


    fn set(&mut self, x: SEXP) {

        unsafe {
            self.data = rustr_replace_object((self.data), x);
        }
        // calls the update method of CLASS
        // this is where to react to changes in the underlying SEXP

        // todo
        // static_cast<CLASS&>(*this).update(data) ; 调用父类方法
        //
    }

    // template <typename T>
    //
    // life time example result have the same life time as self
    fn copy<'a, 'b>(&'a mut self, other: &'b Preserve) -> &'a Preserve {
        if self != other {
            self.set(unsafe { other.s() });
        }
        self
    }
}

impl Drop for Preserve {
    fn drop(&mut self) {
        unsafe {
            rustr_release_object(self.data);
            self.data = R_NilValue;
        }
    }
}

impl ToSEXP for Preserve {
    unsafe fn s(&self) -> SEXP {
        self.data
    }
}

impl From<SEXP> for Preserve {
    fn from(x: SEXP) -> Preserve {
        Preserve::new(x)
    }
}

/// ///////////////// noprotect
#[derive(PartialEq, Eq, Debug)]
pub struct NoProtect {
    data: SEXP,
}

pub type RInput = NoProtect;

impl SEXPbucket for NoProtect {
    fn init() -> NoProtect {
        unsafe { NoProtect { data: R_NilValue } }

    }

    fn new(x: SEXP) -> NoProtect {
        NoProtect { data: x }
    }

    fn invalidate(&mut self) -> SEXP {
        let out = self.data;
        unsafe {
            self.data = R_NilValue;
        }
        out
    }
    fn inherits(&self, class_: &str) -> RResult<bool> {
        let class = match cstring_user(class_) {
            Ok(x) => x,
            Err(y) => return rerror(Other(y.into())), // nul string error
        };

        unsafe { Ok(Rf_inherits((self.data), class.as_ptr()) == Rboolean::TRUE) }
    }


    fn set(&mut self, x: SEXP) {

        self.data = x;

        // calls the update method of CLASS
        // this is where to react to changes in the underlying SEXP

        // todo
        // static_cast<CLASS&>(*this).update(data) ; 调用父类方法
        //
    }

    // template <typename T>
    //
    // life time example result have the same life time as self
    fn copy<'a, 'b>(&'a mut self, other: &'b NoProtect) -> &'a NoProtect {
        if self != other {
            self.set(unsafe { other.s() });
        }
        self
    }
}

impl Drop for NoProtect {
    fn drop(&mut self) {

        unsafe {
            self.data = R_NilValue;
        }

    }
}

impl ToSEXP for NoProtect {
    unsafe fn s(&self) -> SEXP {
        self.data
    }
}

impl From<SEXP> for NoProtect {
    fn from(x: SEXP) -> NoProtect {
        NoProtect::new(x)
    }
}
