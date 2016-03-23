//! Evaluate R script
//!

use error::{rraise, rerror, SEXPResult};
use ::error::REKind::*;
use ::rdll::*;
use ::protect::stackp::*;

use util::*;
use std::result::Result::Ok;
use ::traits::*;


// SEXP Rcpp_eval(SEXP expr_, SEXP env = R_GlobalEnv);

#[inline]
pub fn rustr_eval(expr: SEXP, env: SEXP) -> SEXPResult {

    // define the evalq call -- the actual R evaluation we
    // want to execute
    unsafe {
        // 'identity' function used to capture errors, interrupts
        let identity = Rf_findFun(cstr_sym("identity"), R_BaseNamespace);

        if identity == R_UnboundValue {
            return rraise::<SEXP, _>("Failed to find 'base::identity()'");
        }


        let evalq_call = Shield::new(Rf_lang3(Rf_install(c_str("evalq").as_ptr()), expr, env));

        // define the call -- enclose with `tryCatch` so we can record
        // and later forward error messages
        let call = Shield::new(Rf_lang4(cstr_sym("tryCatch"), evalq_call.s(), identity, identity));

        SET_TAG(CDDR(call.s()), Rf_install(c_str("error").as_ptr()));
        SET_TAG(CDDR(CDR(call.s())), Rf_install(c_str("interrupt").as_ptr()));

        // execute the call
        let res = Shield::new(Rf_eval(call.s(), R_GlobalEnv));

        // check for condition results (errors, interrupts)
        if Rf_inherits(res.s(), c_str("condition").as_ptr()) == Rboolean::TRUE {

            if Rf_inherits(res.s(), c_str("error").as_ptr()) == Rboolean::TRUE {

                let condition_message_call = Shield::new(Rf_lang2(Rf_install(c_str("conditionM\
                                                                                    essage")
                                                                                 .as_ptr()),
                                                                  res.s()));

                let condition_message = Shield::new(Rf_eval(condition_message_call.s(),
                                                            R_GlobalEnv));

                // throw eval_error(CHAR(STRING_ELT(conditionMessage, 0)));
                let ss = try!(Vec::<String>::rnew(condition_message.s()));
                return rerror(EvalError(ss[0].clone().into()));

            }

            if Rf_inherits(res.s(), c_str("interrupt").as_ptr()) == Rboolean::TRUE {
                return rerror(Interrupted("in rustr_eval".into()));
            }
        }

        Ok(res.s())
    }


}
