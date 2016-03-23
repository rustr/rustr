//! Cast R Object type
//!
//!

use protect::indexp::Armor;
use ::error::*;
use ::rdll::*;
use eval::*;
use ::traits::*;
use ::error::REKind::*;
use ::rtype::*;
use util::c_str;
use ::protect::stackp::*;

#[inline]
pub fn convert_using_rfunction(x: SEXP, fun: &str) -> SEXPResult {
    unsafe {
        let fun_r = try!(::std::ffi::CString::new(fun));
        let fun_sym = Rf_install(fun_r.as_ptr());

        let res: Armor;

        // match one
        res = match rustr_eval(Rf_lang2(fun_sym, x), R_GlobalEnv) {
            Ok(x) => Armor::new(x),

            // match two
            Err(y) => {
                match *y.kind() {
                    // match two case one
                    EvalError(_) => {
                        return rerror(NotCompatible(format!("could not convert using R function \
                                                             : {}",
                                                            fun)
                                                        .into()))
                    }

                    // match two case two
                    _ => return rraise("unreachable error in convert_using_rfunction."),
                }
            } // end match two

        };//end match one assign to res

        Ok(res.s())
    }

}

#[inline]
pub fn r_true_cast(x: SEXP, into: Rtype) -> SEXPResult {
    unsafe {
        match into {

            STRSXP => {
                if RTYPEOF(x) == into {
                    return Ok(x);
                };
                match RTYPEOF(x) {
                    // 2 m 1 c
                    CPLXSXP | RAWSXP | LGLSXP | REALSXP | INTSXP => {

                        // return Rf_coerceVector( x, STRSXP );
                        // coerceVector does not work for some reason
                        let call = Shield::new(Rf_lang2(Rf_install(c_str("as.character")
                                                                       .as_ptr()),
                                                        x));
                        let res = Shield::new(Rf_eval(call.s(), R_GlobalEnv));
                        return Ok(res.s());

                    }
                    // 2 m 2 c
                    CHARSXP => return Ok(Rf_ScalarString(x)),
                    // 2 m 3 c
                    SYMSXP => return Ok(Rf_ScalarString(PRINTNAME(x))),
                    // 2 m 4 c
                    _ => return rerror(NotCompatible("not compatible with STRSXP".into())),
                }
                // end m1 c1 mm

            } //end m1 c1

            REALSXP | RAWSXP | LGLSXP | CPLXSXP | INTSXP => {
                if RTYPEOF(x) == into {
                    return Ok(x);
                };

                match RTYPEOF(x) {
                    REALSXP | RAWSXP | LGLSXP | CPLXSXP | INTSXP => {
                        return Ok(Rf_coerceVector(x, into as ::std::os::raw::c_uint))
                    }
                    _ => return rerror(NotCompatible("not compatible with requested type".into())), 
                }

            } // end m1 c2

            VECSXP => {
                return convert_using_rfunction(x, "as.list");
            }

            EXPRSXP => {
                return convert_using_rfunction(x, "as.expression");
            }

            LANGSXP => {
                return convert_using_rfunction(x, "as.call");
            }

            LISTSXP => {
                match RTYPEOF(x) {
                    LANGSXP => {
                        let y = Shield::new(Rf_duplicate(x));
                        SET_RTYPEOF(y.s(), LISTSXP);
                        return Ok(y.s());
                    }
                    _ => {
                        return convert_using_rfunction(x, "as.pairlist");
                    }
                }
            }

            _ => return rerror(UnreachableError(format!("r_true_cast: {:?} to {}", x, into))),

        }

    } // unsafe
}

#[inline]
pub fn r_cast(x: SEXP, into: Rtype) -> SEXPResult {
    unsafe {
        if RTYPEOF(x) == into {
            return Ok(x);
        } else {
            // #ifdef RCPP_WARN_ON_COERCE
            let result = match r_true_cast(x, into) {
                Ok(ins) => Shield::new(ins),
                Err(er) => return rerror(Other(er.into())),
            };

            Rf_warning(c_str("coerced object from '%s' to '%s'").as_ptr(),
                       R_CHAR(Rf_type2str(TYPEOF(x) as ::std::os::raw::c_uint)),
                       R_CHAR(Rf_type2str(into as ::std::os::raw::c_uint)));

            return Ok(result.s());
            // #else
            // return internal::r_true_cast<TARGET>(x);
            // #endif
        }
    }

}
