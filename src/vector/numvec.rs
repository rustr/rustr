use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;


use std::os::raw::c_double;
pub type NumVec = NumVecM<Preserve>;


gen_traits_sexp!(NumVecM);
gen_named_vec!(NumVecM, REALSXP, NumVecIter, REAL, c_double);
