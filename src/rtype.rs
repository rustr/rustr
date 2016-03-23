//! C Macro constant for R
//!
//!
//!

use rdll::{TYPEOF, SEXP, SET_TYPEOF};

pub type Rtype = ::std::os::raw::c_uint;

#[derive(Copy, PartialEq, Eq, Clone, Debug)]
pub enum RTypeEnum {
    NIL,
    SYM,
    LIST,
    CLO,
    ENV,
    PROM,
    LANG,
    SPECIAL,
    BUILTIN,
    /// ////////////VECTOR BEGIN
    CHAR,
    LGL,
    INT,
    REAL,
    CPLX,
    STR,
    VEC,
    S4,
    RAW,
    EXPR,
    /// ///////////VECTOR END
    DOT,
    ANY,
    BCODE,
    EXTPTR,
    WEAKREF,
    NEW,
    FREE,
    FUN,
}

#[inline]
pub fn match_rtype(x: Rtype) -> RTypeEnum {
    match x {
        NILSXP => RTypeEnum::NIL,
        SYMSXP => RTypeEnum::SYM,
        LISTSXP => RTypeEnum::LIST,
        CLOSXP => RTypeEnum::CLO,
        ENVSXP => RTypeEnum::ENV,
        PROMSXP => RTypeEnum::PROM,
        LANGSXP => RTypeEnum::LANG,
        SPECIALSXP => RTypeEnum::SPECIAL,
        BUILTINSXP => RTypeEnum::BUILTIN,
        /// ////////////VECTOR BEGIN
        CHARSXP => RTypeEnum::CHAR,
        LGLSXP => RTypeEnum::LGL,
        INTSXP => RTypeEnum::INT,
        REALSXP => RTypeEnum::REAL,
        CPLXSXP => RTypeEnum::CPLX,
        STRSXP => RTypeEnum::STR,
        VECSXP => RTypeEnum::VEC,
        S4SXP => RTypeEnum::S4,
        RAWSXP => RTypeEnum::RAW,
        EXPRSXP => RTypeEnum::EXPR,
        /// ///////////VECTOR END
        DOTSXP => RTypeEnum::DOT,
        ANYSXP => RTypeEnum::ANY,
        BCODESXP => RTypeEnum::BCODE,
        EXTPTRSXP => RTypeEnum::EXTPTR,
        WEAKREFSXP => RTypeEnum::WEAKREF,
        NEWSXP => RTypeEnum::NEW,
        FREESXP => RTypeEnum::FREE,
        FUNSXP => RTypeEnum::FUN,
        _ => RTypeEnum::NIL, // unreachable but to keep safe
    }
}

#[allow(non_snake_case)]
#[inline]
pub fn RTYPEOF(x: SEXP) -> ::std::os::raw::c_uint {
    unsafe { TYPEOF(x) as ::std::os::raw::c_uint }
}

#[allow(non_snake_case)]
#[inline]
pub unsafe fn SET_RTYPEOF(x: SEXP, v: ::std::os::raw::c_uint) {
    SET_TYPEOF(x, v as ::std::os::raw::c_int);
}



pub const NILSXP: ::std::os::raw::c_uint = 0;	  /* nil = NULL */
pub struct RNIL;

pub const SYMSXP: ::std::os::raw::c_uint = 1;	  /* symbols */
pub struct RSYM;

pub const LISTSXP: ::std::os::raw::c_uint = 2;	  /* lists of dotted pairs */
pub struct RLIST;
pub const CLOSXP: ::std::os::raw::c_uint = 3;	  /* closures */
pub struct RCLOS;
pub const ENVSXP: ::std::os::raw::c_uint = 4;  /* environments */
pub struct RENV;
pub const PROMSXP: ::std::os::raw::c_uint = 5;  /* promises: [un]evaluated closure arguments */
pub struct RPROM;
pub const LANGSXP: ::std::os::raw::c_uint = 6;	  /* language constructs (special lists) */
pub struct RLANG;
pub const SPECIALSXP: ::std::os::raw::c_uint = 7;  /* special forms */
pub struct RSPECIAL;
pub const BUILTINSXP: ::std::os::raw::c_uint = 8;  /* builtin non-special forms */
// pub struct
pub const CHARSXP: ::std::os::raw::c_uint = 9;  /* "scalar" string type (internal only)*/
pub struct RCHAR;
pub const LGLSXP: ::std::os::raw::c_uint = 10;  /* logical vectors */
// 11 and 12 were factors and ordered factors in the 1990s
pub struct RLGL;
pub const INTSXP: ::std::os::raw::c_uint = 13;  /* integer vectors */
pub struct RINT;
pub const REALSXP: ::std::os::raw::c_uint = 14;  /* real variables */
pub struct RREAL;
pub const CPLXSXP: ::std::os::raw::c_uint = 15;  /* complex variables */
pub struct RCPLX;
pub const STRSXP: ::std::os::raw::c_uint = 16;  /* string vectors */
pub struct RSTR;
pub const DOTSXP: ::std::os::raw::c_uint = 17;  /* dot-dot-dot object */
pub struct RDOT;
pub const ANYSXP: ::std::os::raw::c_uint = 18;
pub struct RANY;
// make "any" args work.
// Used in specifying types for symbol
// registration to mean anything is okay

pub const VECSXP: ::std::os::raw::c_uint = 19;  /* generic vectors */
pub struct RVEC;
pub const EXPRSXP: ::std::os::raw::c_uint = 20;  /* expressions vectors */
pub struct REXPR;
pub const BCODESXP: ::std::os::raw::c_uint = 21; /* byte code */
pub struct RBCODESXP;
pub const EXTPTRSXP: ::std::os::raw::c_uint = 22;  /* external pointer */
pub struct REXTPTR;
pub const WEAKREFSXP: ::std::os::raw::c_uint = 23;  /* weak reference */
pub struct RWEAKREF;
pub const RAWSXP: ::std::os::raw::c_uint = 24; /* raw bytes */
pub struct RRAW;
pub const S4SXP: ::std::os::raw::c_uint = 25;  /* S4, non-vector */
pub struct RS4;

// used for detecting PROTECT issues in memory.c
pub const NEWSXP: ::std::os::raw::c_uint = 30;  /* fresh node created in new page */
pub struct RNEW;
pub const FREESXP: ::std::os::raw::c_uint = 31;  /* node released by GC */
pub struct RFREE;
pub const FUNSXP: ::std::os::raw::c_uint = 99;  /* Closure or Builtin or Special */
pub struct RFUN;
