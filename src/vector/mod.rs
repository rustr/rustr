//! R Vector with boolean and number
//!
//!
//!


/// For Vectors
/// 
#[macro_export]
macro_rules! gen_named_vec{
($NumVecM:ident,$REALSXP:ident,$NumVecIter:ident,$RAW:ident,$Rbyte:ident)=> (

impl<T: SEXPbucket> $NumVecM<T> {
    pub fn new(x: SEXP) -> RResult<$NumVecM<T>> {
            if RTYPEOF(x.clone()) != $REALSXP {
                return rerror(NotCompatible(concat!("expecting a ",stringify!($REALSXP)).into()));
            }
        Ok($NumVecM { data: T::new(x.clone()) })
    }
	pub fn alloc(x: R_xlen_t)->$NumVecM<T>{
    	$NumVecM { data: T::new(unsafe{Rf_allocVector($REALSXP,x)}) }
    }
    pub fn alloc_matrix(x: ::std::os::raw::c_int,y: ::std::os::raw::c_int)->$NumVecM<T>{
    	$NumVecM { data: T::new(unsafe{Rf_allocMatrix($REALSXP,x ,y)}) }
    }
	pub fn at(&self,  ind: usize) -> Option<$Rbyte> {
		unsafe{
        if Rf_xlength(self.s())<=ind as R_xlen_t {
        	return None
        }
        let ptr = $RAW(self.s());

        Some(* ptr.offset(ind as isize) as $Rbyte)}
    }
	pub fn set(&mut self,  ind: usize, value: $Rbyte) -> RResult<()> {
		unsafe{
        if Rf_xlength(self.s())<ind  as R_xlen_t || ind ==0 {
        	return rraise("index out of bound");
        }
        let ptr = $RAW(self.s());
        * ptr.offset((ind-1) as isize)  = value as $Rbyte;
        Ok(())}
    }
	pub unsafe fn uat(&self, ind: usize) -> $Rbyte {
            let ptr = $RAW(self.s());
            *ptr.offset(ind as isize) as $Rbyte
    }
    pub unsafe fn uset(&mut self, ind: usize, value: $Rbyte) {
            let ptr = $RAW(self.s());
            *ptr.offset(ind as isize) = value as $Rbyte;
    }
	pub fn range(&self,  ind: Range<usize>) -> Option<Vec<$Rbyte>> {
		unsafe{
        if Rf_xlength(self.s())<= ind.end as R_xlen_t {
        	return None
        }
		let mut vecs = Vec::with_capacity(ind.len() as usize);
		let ptr = $RAW(self.s());
		for ii in ind {
                vecs.push(*ptr.offset(ii as isize) as $Rbyte);
		}
        Some(vecs)}
    }
	pub fn is_duplicated(&self,from_last:bool)->R_xlen_t{
    	let last = match from_last{
    		true => Rboolean::TRUE,
    		false => Rboolean::FALSE
    	};
    	unsafe{
    		Rf_any_duplicated(self.s(),last)
    	}
    }
}


impl<T: SEXPbucket, E: Into<$Rbyte> + Clone> From<Vec<E>> for $NumVecM<T> {
    fn from(x: Vec<E>) -> $NumVecM<T> {
        let size_x = x.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector($REALSXP, size_x as R_xlen_t));
            let rptr = $RAW(rvec.s());
            for ii in 0..size_x {
                *rptr.offset(ii as isize) = (*x.get_unchecked(ii as usize)).clone().into() as $Rbyte;
            }
            $NumVecM { data: T::new(rvec.s()) }
        }
    }
}

impl<T: SEXPbucket> From<$NumVecM<T>> for Vec<$Rbyte> {
    fn from(x: $NumVecM<T>) -> Vec<$Rbyte> {
        unsafe {
            let lens = Rf_xlength(x.s());
            let mut vecs = Vec::with_capacity(lens as usize);
            let rptr = $RAW(x.s());
            for ii in 0..lens {
                vecs.push(*rptr.offset(ii as isize) as $Rbyte);
            }
            vecs
        }
    }
}


#[derive(Debug)]
pub struct $NumVecIter< T: SEXPbucket> {
    data: *mut $Rbyte,
    size: R_xlen_t,
    next: R_xlen_t,
    ty: $NumVecM<T>
}

impl<T: SEXPbucket> Iterator for $NumVecIter<T> {
// we will be counting with usize
    type Item = $Rbyte;

// next() is the only required method
    fn next(&mut self) -> Option<$Rbyte> {
// increment our count. This is why we started at zero.
// check to see if we've finished counting or not.
        if self.next < self.size {
        	self.next += 1;
        	unsafe{
            Some(*self.data.offset((self.next-1) as isize))}
        } else {
            None
        }

    }
}






impl<T: SEXPbucket> RNew for $NumVecM<T> {
	fn rnew(x:SEXP)->RResult<Self>{
		Self::new(x)
	}
}

impl<T: SEXPbucket> RSize for $NumVecM<T>{
	fn rsize(&self)->R_xlen_t{
		unsafe{Rf_xlength(self.s())}
	}
}


impl<T: SEXPbucket> RName for $NumVecM<T>{}
impl<T: SEXPbucket> RDim for $NumVecM<T>{
	type Output = $Rbyte;
}


impl<T: SEXPbucket> ExactSizeIterator  for $NumVecIter<T> {
// We already have the number of iterations, so we can use it directly.
    fn len(&self) -> usize {
        self.size as usize
    }
}

impl<T: SEXPbucket> IntoIterator for $NumVecM<T>{
    type Item = $Rbyte;
    type IntoIter = $NumVecIter<T>;

    fn into_iter(self) -> Self::IntoIter {
         unsafe{
         	$NumVecIter{data: $RAW(self.s()),size:self.rsize(),next:0,ty:self}
    	}
    }

}



)
}


pub mod numvec;
pub mod intvec;
pub mod boolvec;
pub mod rawvec;
pub mod cplvec;

pub use self::numvec::*;
pub use self::intvec::*;
pub use self::boolvec::*;
pub use self::rawvec::*;
pub use self::cplvec::*;
