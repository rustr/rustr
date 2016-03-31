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


#[macro_export]
macro_rules! intvec {
    ($($tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = IntVec::alloc(size as usize);
	  unsafe{
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1 as usize, $tts);
      		
      )*      
	}
      res
      }
    };
    
    ($($id:ident ~ $tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = IntVec::alloc(size as usize);
	  let mut name = CharVec::alloc(size as usize);
	  unsafe{
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1 as usize , $tts);
      		name.uset(x-1, stringify!($id));
      )*
	}
	  unsafe{Rf_setAttrib(res.s(), R_NamesSymbol,name.s());}
      res
      }
    }
}
