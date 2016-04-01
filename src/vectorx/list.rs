use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;
use util::*;
use super::CharVec;
use vector::IntVec;

pub type RList = RListM<Preserve>;


impl<T: SEXPbucket> RListM<T> {
    pub fn new(x: SEXP) -> RResult<RListM<T>> {

        if RTYPEOF(x) != VECSXP {
            return rerror(NotCompatible("expecting a list".into()));
        }

        Ok(RListM { data: T::new(x) })
    }
    pub fn alloc(x: usize) -> RListM<T> {
        RListM { data: T::new(unsafe { Rf_allocVector(VECSXP, x as R_xlen_t) }) }
    }
    pub fn alloc_matrix(x: usize, y: usize) -> RListM<T> {
        RListM {
            data: T::new(unsafe {
                Rf_allocMatrix(VECSXP,
                               x as ::std::os::raw::c_int,
                               y as ::std::os::raw::c_int)
            }),
        }
    }
    pub fn at(&self, ind: usize) -> Option<SEXP> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return None;
            }
            Some(VECTOR_ELT(self.s(), ind as R_xlen_t))
        }
    }
    pub unsafe fn uat(&self, ind: usize) -> SEXP {
        VECTOR_ELT(self.s(), ind as R_xlen_t)
    }
    pub unsafe fn uset<TT: ToSEXP>(&mut self, ind: usize, value: TT) {
        SET_VECTOR_ELT(self.s(), ind as R_xlen_t, value.s());
    }
    pub fn set<TT: ToSEXP>(&mut self, ind: usize, value: TT) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return rraise("index out of bound");
            }
            SET_VECTOR_ELT(self.s(), ind as R_xlen_t, value.s());
            Ok(())
        }
    }
    pub fn range(&self, ind: Range<usize>) -> Option<Vec<SEXP>> {
        unsafe {
            if Rf_xlength(self.s()) <= ind.end as R_xlen_t {
                return None;
            }
            let mut vecs = Vec::with_capacity((ind.end - ind.start) as usize);
            for ii in ind {
                vecs.push(VECTOR_ELT(self.s(), ii as R_xlen_t));
            }
            Some(vecs)
        }
    }
    pub fn is_duplicated(&self, from_last: bool) -> R_xlen_t {
        let last = if from_last {
            Rboolean::TRUE
        } else {
            Rboolean::FALSE
        };
        unsafe { Rf_any_duplicated(self.s(), last) }
    }
    pub fn as_data_frame(&mut self)->RResult<()>{
    	if self.rsize() == 0{
    		unsafe{
            Rf_setAttrib(self.s(), R_NamesSymbol, Rf_allocVector(STRSXP, 0));
            Rf_setAttrib(self.s(), R_RowNamesSymbol, Rf_allocVector(INTSXP, 0));
            Rf_setAttrib(self.s(), R_ClassSymbol, Rf_mkString(c_str("data.frame").as_ptr()));
    		}
            return Ok(());
    	}
    	unsafe{
    		if self.namesexp() == R_NilValue{
    			let mut colname = CharVec::alloc(self.rsize() as usize);
    			for ii in 0..(self.rsize() as usize){
    				colname.uset(ii, &format!("col{}", ii+1));
    			}
    			self.uset_name(&colname);
    		}
 			let colsize : usize = Rf_xlength(self.uat(0)) as usize ;
 			for ii in 0..(self.rsize() as usize){
    			if colsize  != Rf_xlength(self.uat(ii)) as usize {
    				return rraise("not all colunm are the same length.");
    			}
			}
 			let mut rowname = IntVec::alloc(colsize as usize);
			for ii in 0..(self.rsize() as usize){
    				rowname.uset(ii, (ii as ::std::os::raw::c_int)+1);
			}
            Rf_setAttrib(self.s(), R_RowNamesSymbol, rowname.s());
			Rf_setAttrib(self.s(), R_ClassSymbol, Rf_mkString(c_str("data.frame").as_ptr()));
    		Ok(())
    	}
    }
}

#[cfg_attr(feature = "dev",allow(explicit_counter_loop))]
impl<T: SEXPbucket, E: UIntoR + Clone> From<Vec<E>> for RListM<T> {
    fn from(x: Vec<E>) -> RListM<T> {
        let size_x = x.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(VECSXP, size_x as R_xlen_t));
            let mut xs = 0;
            for ii in x {
                SET_VECTOR_ELT(rvec.s(), xs, Shield::new(ii.uintor()).s());
                xs += 1;
            }
            RListM { data: T::new(rvec.s()) }
        }
    }
}

// impl<T: SEXPbucket,D:URNew> From<RListM<T>> for Vec<D> {
//    fn from(x: RListM<T>) -> Vec<D> {
//        unsafe {
//            let lens = Rf_xlength(x.s());
//            let mut vecs = Vec::with_capacity(lens as usize);
//            for ii in 0..lens {
//                vecs.push(D::urnew(VECTOR_ELT(x.s(), ii)));
//            }
//            vecs
//        }
//    }
// }


impl<T: SEXPbucket> URNew for RListM<T> {
    unsafe fn urnew(x: SEXP) -> Self {
        RListM { data: T::new(x) }
    }
}

impl<T: SEXPbucket> RNew for RListM<T> {
    fn rnew(x: SEXP) -> RResult<Self> {
        Self::new(x)
    }
}

impl<T: SEXPbucket> RSize for RListM<T> {
    fn rsize(&self) -> R_xlen_t {
        unsafe { Rf_xlength(self.s()) }
    }
}

gen_traits_sexp!(RListM);


impl<T: SEXPbucket> RName for RListM<T> {}
impl<T: SEXPbucket> RDim for RListM<T> {
    type Output = SEXP;
}

#[derive(Debug)]
pub struct RListIter<T: SEXPbucket> {
    size: R_xlen_t,
    next: R_xlen_t,
    ty: RListM<T>,
}

impl<T: SEXPbucket> Iterator for RListIter<T> {
    // we will be counting with usize
    type Item = SEXP;

    // next() is the only required method
    fn next(&mut self) -> Option<SEXP> {
        // increment our count. This is why we started at zero.
        // check to see if we've finished counting or not.
        if self.next < self.size {
            self.next += 1;
            unsafe { Some(VECTOR_ELT(self.ty.s(), self.next - 1)) }
        } else {
            None
        }

    }
}

impl<T: SEXPbucket> IntoIterator for RListM<T> {
    type Item = SEXP;
    type IntoIter = RListIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        RListIter {
            size: self.rsize(),
            next: 0,
            ty: self,
        }
    }
}

impl<T: SEXPbucket> ExactSizeIterator for RListIter<T> {
    // We already have the number of iterations, so we can use it directly.
    fn len(&self) -> usize {
        self.size as usize
    }
}

/// Helper for the number of arguments 
///
#[macro_export]
macro_rules! replace_expr {
    ($_t:expr, $sub:expr) => {$sub};
}

/// Create a RList
///
#[macro_export]
macro_rules! rlist {
    ($($tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = RList::alloc(size as usize);
	  unsafe{
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1, try!($tts.intor()));
      		
      )*      
	}
      res
      }
    };
    
    ($($id:ident ~ $tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = RList::alloc(size as usize);
	  let mut name = CharVec::alloc(size as usize);
	  unsafe{
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1, try!($tts.intor()));
      		name.uset(x-1, stringify!($id));
      )*
	}
	  unsafe{Rf_setAttrib(res.s(), R_NamesSymbol,name.s());}
      res
      }
    }
}

/// Create a rlist with unsafe code
/// 

#[macro_export]
macro_rules! urlist {
    ($($tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = RList::alloc(size as usize);
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1, $tts.uintor());
      		
      )*      
      res
      }
    };
        ($($id:ident ~ $tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = RList::alloc(size as usize);
	  let mut name = CharVec::alloc(size as usize);

      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1, $tts.uintor());
      		name.uset(x-1, stringify!($id));
      )*
	
	  Rf_setAttrib(res.s(), R_NamesSymbol,name.s());
      res
      }
    }
}
