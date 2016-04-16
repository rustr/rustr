//! Traits for R type
//!
//!
//!



use ::rdll::*;
use rtype::{Rtype, RTYPEOF};
use error::RResult;
use macros::rbool;


pub mod intor;
pub mod attr;
pub mod slot;
pub mod name;
pub mod rfield;
pub mod dim;
pub mod dimname;
#[cfg(feature = "extra")]
pub mod rlist;

pub use self::intor::*;
pub use self::attr::*;
pub use self::slot::*;
pub use self::name::*;
pub use self::rfield::*;
pub use self::dim::*;
pub use self::dimname::*;
#[cfg(feature = "extra")]
pub use self::rlist::*;

use util::c_str;
pub trait Args: Named {}

// RFun and n_("",&sdsd)

pub struct Rtemp {
    data: SEXP,
}
pub trait RSize {
    fn rsize(&self) -> R_xlen_t;
}

pub trait SetSEXP {
    fn set_s(&mut self, X: SEXP);
}

pub trait SetSelf {
    fn set(&mut self, x: Self);
}

pub trait ToExpr {
    unsafe fn expr(&self) -> SEXP;
}

pub trait ToSEXP {
    unsafe fn s(&self) -> SEXP;

    fn rtype(&self) -> Rtype {
        unsafe { RTYPEOF(self.s()) }
    }
    unsafe fn t(&self) -> Rtemp {
        Rtemp { data: self.s() }
    }
    fn print(&self) {
        unsafe {
            Rf_PrintValue(self.s());
        }
    }
    fn is_null(&self) -> bool {
        unsafe { rbool(Rf_isNull(self.s())) }
    }
    fn is_object(&self) -> bool {
        unsafe { rbool(Rf_isObject(self.s())) }
    }
    fn is_s4(&self) -> bool {
        unsafe { rbool(Rf_isS4(self.s())) }
    }
    fn is_array(&self) -> bool {
        unsafe { rbool(Rf_isArray(self.s())) }
    }
    fn is_factor(&self) -> bool {
        unsafe { rbool(Rf_isFactor(self.s())) }
    }
    fn is_frame(&self) -> bool {
        unsafe { rbool(Rf_isFrame(self.s())) }
    }
    fn is_function(&self) -> bool {
        unsafe { rbool(Rf_isFunction(self.s())) }
    }
    fn is_integer(&self) -> bool {
        unsafe { rbool(Rf_isInteger(self.s())) }
    }
    fn is_language(&self) -> bool {
        unsafe { rbool(Rf_isLanguage(self.s())) }
    }
    fn is_list(&self) -> bool {
        unsafe { rbool(Rf_isList(self.s())) }
    }
    fn is_matrix(&self) -> bool {
        unsafe { rbool(Rf_isMatrix(self.s())) }
    }
    fn is_newlist(&self) -> bool {
        unsafe { rbool(Rf_isNewList(self.s())) }
    }
    fn is_number(&self) -> bool {
        unsafe { rbool(Rf_isNumber(self.s())) }
    }
    fn is_numeric(&self) -> bool {
        unsafe { rbool(Rf_isNumeric(self.s())) }
    }
    fn is_pairlist(&self) -> bool {
        unsafe { rbool(Rf_isPairList(self.s())) }
    }
    fn is_primitive(&self) -> bool {
        unsafe { rbool(Rf_isPrimitive(self.s())) }
    }
    fn is_ts(&self) -> bool {
        unsafe { rbool(Rf_isTs(self.s())) }
    }
    fn is_userbinop(&self) -> bool {
        unsafe { rbool(Rf_isUserBinop(self.s())) }
    }
    fn is_validstring(&self) -> bool {
        unsafe { rbool(Rf_isValidString(self.s())) }
    }
    fn is_validstringf(&self) -> bool {
        unsafe { rbool(Rf_isValidStringF(self.s())) }
    }
    fn is_vector(&self) -> bool {
        unsafe { rbool(Rf_isVector(self.s())) }
    }
    fn is_vectoratomic(&self) -> bool {
        unsafe { rbool(Rf_isVectorAtomic(self.s())) }
    }
    fn is_vectorlist(&self) -> bool {
        unsafe { rbool(Rf_isVectorList(self.s())) }
    }
    fn is_vectorizable(&self) -> bool {
        unsafe { rbool(Rf_isVectorizable(self.s())) }
    }
    fn is_ordered(&self) -> bool {
        unsafe { rbool(Rf_isOrdered(self.s())) }
    }
    fn is_unordered(&self) -> bool {
        unsafe { rbool(Rf_isUnordered(self.s())) }
    }
    fn is_data_frame(&self)->bool{
    	unsafe {rbool(Rf_inherits( self.s(), c_str("data.frame").as_ptr() ))}
    }
}


impl ToSEXP for Rtemp {
    unsafe fn s(&self) -> SEXP {
        self.data
    }
}

impl ToSEXP for SEXP {
    unsafe fn s(&self) -> SEXP {
        *self
    }
}

pub trait NoneSEXP {}

pub trait FromR<T> {
    fn fromr(&self) -> RResult<T>;
}

pub trait RNew
    where Self: Sized
{
    fn rnew(x: SEXP) -> RResult<Self>;
}

pub trait URNew
    where Self: Sized
{
    unsafe fn urnew(x: SEXP) -> Self;
}

pub trait IntoR {
    fn intor(&self) -> RResult<SEXP>;
}

pub trait UIntoR {
    unsafe fn uintor(&self) -> SEXP;
}

// pub trait Frombucket<T>{
//    fn from(SEXPbucket) -> RResult<T>;
// }

impl FromR<SEXP> for SEXP {
    fn fromr(&self) -> RResult<SEXP> {
        Ok(*self)
    }
}

impl RNew for SEXP {
    fn rnew(x: SEXP) -> RResult<SEXP> {
        Ok(x)
    }
}

impl IntoR for SEXP {
    fn intor(&self) -> RResult<SEXP> {
        Ok(*self)
    }
}

pub trait Moves {
    fn moves(x: Self) -> Self;
}

pub trait Others {
    fn others(x: &Self) -> Self;
}

pub trait Empty {
    fn empty() -> Self;
}

pub trait Gettable {
    type Res;
    fn get(self) -> Self::Res;
}

pub trait ErrGettable {
    type Res;
    fn get_err(self) -> Self::Res;
}

pub trait RObjClass: ToSEXP {
    type Origin;
}

use storage::*;
use robject::*;

pub trait FromRObj<T:SEXPbucket, E:ToSEXP> : Sized+NewRObj{
    fn r(x: RObjM<T>) -> RResult<Self> {
        Self::new(x)
    }
}

pub trait NewRObj : Sized{
    fn new<E: ToSEXP>(x: E) -> RResult<Self>;
    unsafe fn unew<E: ToSEXP>(x: E) -> Self;
}

pub trait Shallow: Sized + ToSEXP {
    fn sc(&self) -> Self;
    fn sclone_from(&mut self, source: &Self) {
        *self = source.sc()
    }
}


pub trait Named: ToSEXP {
    fn named(&self) -> bool {
        false
    }
    fn name(&self) -> SEXP {
        unsafe { R_NilValue }
    }
}

impl Named for SEXP {}

impl<T: Named> Args for T {}

pub trait FromCastSEXP: Sized {
    fn casts(x: SEXP) -> RResult<Self>;
}
