use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;

pub type CplVec = CplVecM<Preserve>;

gen_traits_sexp!(CplVecM);
gen_named_vec!(CplVecM, CPLXSXP, CplVecIter, COMPLEX, Rcomplex);
