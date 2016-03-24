#![cfg_attr(feature = "dev", feature(plugin))]
#![cfg_attr(feature = "dev", plugin(clippy))]

/// rustr version
/// 
#[macro_export]
macro_rules! rustr_version{
    () => (
	{
		let mar = 0 * 10000000;
    	let min = 1 * 100000;
    	let rev = 0 * 1000;
    	let dev = 0 * 1;
    	mar + min + rev + dev
	})
}

// ///////////////////// begin mod

#[macro_use]
pub mod macros;

#[allow(non_camel_case_types,non_upper_case_globals,non_snake_case)]
#[cfg_attr(feature = "dev",allow(type_complexity,expl_impl_clone_on_copy,used_underscore_binding))]
pub mod rdll;
pub mod protect;
pub mod storage;
pub mod rtype;
#[macro_use]
pub mod util;

pub mod traits;
pub mod grow;
pub mod lcons;
pub mod rlang;
pub mod formula;
pub mod dotpair;

pub mod impls;



#[macro_use]
#[allow(non_camel_case_types,non_upper_case_globals,non_snake_case)]
pub mod rwrap;
pub mod complex;
pub mod rptr;
pub mod rweak;
pub mod result;
pub mod error;
pub mod eval;
pub mod rcast;

pub mod vector;
pub mod vectorx;

pub mod name_object;
// pub mod caster_complex; //num crate

pub mod robject;
pub mod symbol;
pub mod promise;
pub mod s4;
pub mod reference;
pub mod rmath;
pub mod environment;
pub mod rfunction;
// TODO: RList
pub mod dataframe;

pub mod dll;
pub use dll::*;
pub mod rstatic;

// begin feature

#[cfg(feature = "date")]
extern crate chrono;

pub mod feature {

    //! Features
    //!
    //!
    //!

    // TODO: Function Date Date meat
    #[cfg(feature = "date")]
    pub mod date;
    #[cfg(feature = "date")]
    pub mod datetime;
	#[cfg(feature = "engine")]
    pub mod engine;
}


pub mod sugar {
    //! Sugar for rustr
    use error::RResult;
    pub type RR<T> = RResult<T>;
}

pub use traits::*;
pub use error::*;
pub use macros::*;
pub use vector::*;
pub use rtype::*;
pub use rdll::*;
pub use protect::stackp::*;
pub use storage::*;
pub use rfunction::*;
pub use environment::*;
pub use robject::*;
pub use vectorx::*;
