use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;
use std::os::raw::c_int;

pub type IntVec = IntVecM<Preserve>;

gen_traits_sexp!(IntVecM);
gen_named_vec!(IntVecM, INTSXP, IntVecIter, INTEGER, c_int);
