//! Error Type
//!
//!
use std::result::Result;
use ::rdll::*;
use std::error::*;
use std::result::Result::Err;
pub type RResult<T> = Result<T, RError>;
pub type SEXPResult = RResult<SEXP>;
use self::REKind::*;
use std::fmt;
use std::convert::From;
use util::*;
use ::protect::stackp::*;

use ::traits::*;

use ::rtype::*;


#[derive(Debug)]
pub struct RError {
    kind: REKind,
}

macro_rules! gen_fmt {
    ($($x:ident),*) => (

    impl fmt::Display for RError{
        fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
                match self.kind() {
                    &UnknownError(ref info) => write!(fmt,"UnknownError: {}",info),
                    &UnreachableError(ref info) => write!(fmt,"UnreachableError: {}",info),
                    &ForceStopError(ref info) => write!(fmt,"ForceStopError: {}",info),
                    &Other(ref c) =>  write!(fmt, "rustr error: {}",c.description()),
                    $(&$x(ref c) => write!(fmt, "{}: {}", stringify!($x), c.description()))
                        ,*

                }

        }
    }

	impl RError {
        pub fn kind_info(&self) -> &str {
	        match self.kind() {
	            &UnknownError(..) => "UnknownError",
	            &UnreachableError(..) => "UnreachableError",
	            &ForceStopError(..) => "ForceStopError",
	            &Other(ref c) =>  c.description(),
	            $(&$x(..) => stringify!($x))
	                ,*
	        }
	    }
	}

    impl Error for RError {

        fn description(&self) -> &str {
            match self.kind() {
                &UnknownError(..) => "UnknownError",
                &UnreachableError(..) => "UnreachableError",
                &ForceStopError(..) => "ForceStopError",
                &Other(ref c) => c.description(),
                $(&$x(ref c) => c.description())
                    ,*

            }

        }

    fn cause(&self) -> Option<&Error> {
        match self.kind() {
            &UnknownError(..) => None,
            &UnreachableError(..) => None,
            &ForceStopError(..) => None,
            &Other(ref c) => c.cause(),
            $(&$x(ref c))
                    |* => c.cause(),
        }

    }
    }

    #[derive(Debug)]
    pub enum REKind{
        $($x(Box<Error+Send+Sync>)),*
        ,UnknownError(String), UnreachableError(String), ForceStopError(String),Other(Box<Error+Send+Sync>)

    }
    )
}

gen_fmt!(Interrupted,
         NotAMatrix,
         IndexOutOfBounds,
         ParseError,
         NotS4,
         NotReference,
         NotInitialized,
         NoSuchSlot,
         NoSuchField,
         NotAClosure,
         NoSuchFunction,
         UnevaluatedPromise,
         NoSuchEnv,
         FileIoError,
         FileNotFound,
         FileExists,
         NotCompatible,
         S4CreationError,
         ReferenceCreationError,
         NoSuchBinding,
         BindingNotFound,
         BindingIsLocked,
         NoSuchNamespace,
         FunctionNotExported,
         EvalError);


impl From<::std::ffi::NulError> for RError {
    fn from(x: ::std::ffi::NulError) -> Self {
        RError::new(REKind::Other(x.into()))
    }
}


impl RError {
    /// Creates a new RError from a known kind of error as well as an
    /// arbitrary error payload.
    ///
    /// # Examples
    ///
    /// ```txt
    /// use rustr::error::{RError, MessageKind};
    ///
    /// // errors can be created from strings
    /// let custom_error = RError::message(MessageKind::Other, "oh no!");
    ///
    /// // errors can also be created from other errors
    /// let custom_error2 = RError::message(MessageKind::IndexOutOfBounds, custom_error);
    /// ```
    #[inline]
    pub fn new(kind_: REKind) -> RError {
        RError { kind: kind_ }
    }
    #[inline]
    pub fn unknown(x: String) -> RError {
        RError { kind: REKind::UnknownError(x) }
    }
    #[inline]
    pub fn forcestop(x: String) -> RError {
        RError { kind: REKind::ForceStopError(x) }
    }
    #[inline]
    pub fn unreachable(x: String) -> RError {
        RError { kind: REKind::UnreachableError(x) }
    }
    #[inline]
    pub fn other<E>(x: E) -> RError
        where E: Into<Box<Error + Send + Sync>>
    {
        RError { kind: REKind::Other(x.into()) }
    }
    pub fn kind(&self) -> &REKind {
        &(self.kind)
    }
}


#[inline]
pub fn rraise<T, E>(error: E) -> RResult<T>
    where E: Into<Box<Error + Send + Sync>>
{

    Err(RError::other(error))

}

#[inline]
pub fn rerror<T>(kind_: REKind) -> RResult<T> {
    Err(RError { kind: kind_ })
}

#[inline]
pub fn rerr<T, E>(x: E) -> RResult<T>
    where E: Into<Box<Error + Send + Sync>>
{
    Err(RError { kind: REKind::Other(x.into()) })
}

pub fn get_current_call() -> SEXP {
    unsafe {
        let sys_calls_symbol = Rf_install(c_str("sys.calls").as_ptr());
        let sys_calls_expr = Shield::new(Rf_lang1(sys_calls_symbol));
        let calls = Shield::new(Rf_eval(sys_calls_expr.s(), R_GlobalEnv));
        let mut res = calls.s();
        while CDR(res) != R_NilValue {
            res = CDR(res);
        }
        return CAR(res);
    }

}

pub fn get_exception_classes(ex: &RError) -> SEXP {
    unsafe {

        let kind_info_c = c_str(ex.kind_info());

        let res = Shield::new(Rf_allocVector(STRSXP as ::std::os::raw::c_uint, 4));
        SET_STRING_ELT(res.s(), 0, Rf_mkChar(kind_info_c.as_ptr()));
        SET_STRING_ELT(res.s(), 1, Rf_mkChar(c_str("RustError").as_ptr()));
        SET_STRING_ELT(res.s(), 2, Rf_mkChar(c_str("error").as_ptr()));
        SET_STRING_ELT(res.s(), 3, Rf_mkChar(c_str("condition").as_ptr()));
        res.s()
    }

}

pub fn error_to_r_condition(x: RError) -> SEXP {
    // todo
    unsafe {
        let class_sym = Shield::new(get_exception_classes(&x));

        let cond = Shield::new(Rf_allocVector(VECSXP as ::std::os::raw::c_uint, 2));

        let mess_c = c_str(format!("{}", x).as_ref());

        let message = Shield::new(Rf_mkString(mess_c.as_ptr()));

        SET_VECTOR_ELT(cond.s(), 0, message.s());
        SET_VECTOR_ELT(cond.s(), 1, get_current_call());

        let names = Shield::new(Rf_allocVector(STRSXP, 2));

        SET_STRING_ELT(names.s(), 0, Rf_mkChar(c_str("message").as_ptr()));
        SET_STRING_ELT(names.s(), 1, Rf_mkChar(c_str("call").as_ptr()));

        Rf_setAttrib(cond.s(), R_NamesSymbol, names.s());
        Rf_setAttrib(cond.s(), R_ClassSymbol, class_sym.s());

        return cond.s();
    }

}

pub fn forward_exception_to_r(ex: RError) -> SEXP {
    unsafe {

        let stop_sym = cstr_sym("stop");
        let condition = Shield::new(error_to_r_condition(ex));

        let expr = Shield::new(Rf_lang2(stop_sym, condition.s()));

        Rf_eval(expr.s(), R_GlobalEnv);
        return R_NilValue;
    }
}


// other error kind ///////////////////////


impl From<::std::ffi::IntoStringError> for RError {
    fn from(x: ::std::ffi::IntoStringError) -> RError {
        RError::new(REKind::Other(x.into()))
    }
}
