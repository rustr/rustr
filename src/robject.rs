//! R Object type
//!
//!
//!

use ::rdll::*;
use ::storage::*;
// use ::rdefined::*;
use error::*;

use ::traits::*;

pub type RObj = RObjM<Preserve>;

impl<T: SEXPbucket> RObjM<T> {
    pub fn new<E: ToSEXP>(x: E) -> RObjM<T> {
        RObjM { data: T::new(unsafe { x.s() }) }
    }
}

gen_traits_sexp!(RObjM);
