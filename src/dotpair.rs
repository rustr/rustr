//! ... type
//!
//!

use ::rdll::*;
use ::storage::*;
use ::traits::*;

use ::error::*;


pub type RDot = RDotM<Preserve>;

impl<T: SEXPbucket> RDotM<T> {
    pub fn new(x: SEXP) -> Self {
        RDotM { data: T::new(x) }
    }
}

gen_traits_sexp!(RDotM);
