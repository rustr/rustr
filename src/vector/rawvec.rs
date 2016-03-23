use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;

pub type RawVec = RawVecM<Preserve>;

gen_traits_sexp!(RawVecM);
gen_named_vec!(RawVecM, RAWSXP, RawVecIter, RAW, Rbyte);
