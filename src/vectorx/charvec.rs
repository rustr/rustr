use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;
use std::ffi::*;
use util::c_str;


pub type CharVec = CharVecM<Preserve>;

impl<T: SEXPbucket> CharVecM<T> {
    pub fn new(x: SEXP) -> RResult<CharVecM<T>> {
        if RTYPEOF(x) != STRSXP {
            return rerror(NotCompatible("expecting a string vector".into()));
        }
        Ok(CharVecM { data: T::new(x) })
    }
    pub fn alloc(x: usize) -> CharVecM<T> {
        CharVecM { data: T::new(unsafe { Rf_allocVector(STRSXP, x as R_xlen_t) }) }
    }
    pub fn alloc_matrix(x: usize, y: usize) -> CharVecM<T> {
        CharVecM {
            data: T::new(unsafe {
                Rf_allocMatrix(STRSXP,
                               x as ::std::os::raw::c_int,
                               y as ::std::os::raw::c_int)
            }),
        }
    }
    pub unsafe fn uat(&self, ind: usize) -> Result<String, IntoStringError> {
        CStr::from_ptr(R_CHAR(STRING_ELT(self.s(), ind as R_xlen_t))).to_owned().into_string()
    }
    pub unsafe fn uset(&mut self, ind: usize, value: &str) {
        SET_STRING_ELT(self.s(),
                       ind as R_xlen_t,
                       Shield::new(Rf_mkChar(c_str(value).as_ptr())).s())
    }
    pub fn at(&self, ind: usize) -> RResult<String> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return rraise("index out of bound");
            }
            Ok(try!(CStr::from_ptr(R_CHAR(STRING_ELT(self.s(), ind as R_xlen_t)))
                        .to_owned()
                        .into_string()))
        }
    }
    pub fn set(&mut self, ind: usize, value: &str) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return rraise("index out of bound");
            }

            SET_STRING_ELT(self.s(),
                           ind as R_xlen_t,
                           Shield::new(Rf_mkChar(c_str(value).as_ptr())).s());
            Ok(())
        }
    }
    pub unsafe fn atc(&self, ind: usize) -> RResult<CString> {
        if Rf_xlength(self.s()) <= ind as R_xlen_t {
            return rraise("index out of bound");
        }
        Ok(CStr::from_ptr(R_CHAR(STRING_ELT(self.s(), ind as R_xlen_t))).to_owned())
    }
    pub unsafe fn uatc(&self, ind: usize) -> CString {
        CStr::from_ptr(R_CHAR(STRING_ELT(self.s(), ind as R_xlen_t))).to_owned()
    }
    pub unsafe fn usetc<D:ToSEXP>(&mut self, ind: usize, value: D) {
        SET_STRING_ELT(self.s(),
                       ind as R_xlen_t,
                      value.s())
    }
    pub fn setc<D:ToSEXP>(&mut self, ind: usize, value: D) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return rraise("index out of bound");
            }
            if RTYPEOF(self.s()) == CHARSXP  || self.s() == R_NaString{
                return rraise("not a CHARSXP");
            }
            SET_STRING_ELT(self.s(),
                           ind as R_xlen_t,
                           value.s());
            Ok(())
        }
    }
    pub fn range(&self, ind: Range<usize>) -> Option<Vec<CString>> {
        unsafe {
            if Rf_xlength(self.s()) <= ind.end as R_xlen_t {
                return None;
            }
            let mut vecs = Vec::with_capacity((ind.end - ind.start) as usize);
            for ii in ind {
                vecs.push(CStr::from_ptr(R_CHAR(STRING_ELT(self.s(), ii as R_xlen_t))).to_owned());
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
}


#[cfg_attr(feature = "dev",allow(explicit_counter_loop))]
impl<T: SEXPbucket, E: Into<CString> + Clone> From<Vec<E>> for CharVecM<T> {
    fn from(x: Vec<E>) -> CharVecM<T> {
        let size_x = x.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let mut xs = 0;
            for ii in x {
                SET_STRING_ELT(rvec.s(), xs, Shield::new(Rf_mkChar(ii.into().as_ptr())).s());
                xs += 1;
            }
            CharVecM { data: T::new(rvec.s()) }
        }
    }
}

impl<T: SEXPbucket> From<CharVecM<T>> for Vec<CString> {
    fn from(x: CharVecM<T>) -> Vec<CString> {
        unsafe {
            let lens = Rf_xlength(x.s());
            let mut vecs = Vec::with_capacity(lens as usize);
            for ii in 0..lens {
                vecs.push(CString::urnew(STRING_ELT(x.s(), ii)));
            }
            vecs
        }
    }
}

impl<T: SEXPbucket> URNew for CharVecM<T> {
    unsafe fn urnew(x: SEXP) -> Self {
        CharVecM { data: T::new(x) }
    }
}

impl<T: SEXPbucket> RNew for CharVecM<T> {
    fn rnew(x: SEXP) -> RResult<Self> {
        Self::new(x)
    }
}

impl<T: SEXPbucket> RSize for CharVecM<T> {
    fn rsize(&self) -> R_xlen_t {
        unsafe { Rf_xlength(self.s()) }
    }
}

gen_traits_sexp!(CharVecM);


impl<T: SEXPbucket> RName for CharVecM<T> {}
impl<T: SEXPbucket> RDim for CharVecM<T> {
    type Output = CString;
}

#[derive(Debug)]
pub struct CharVecIter<T: SEXPbucket> {
    size: R_xlen_t,
    next: R_xlen_t,
    ty: CharVecM<T>,
}

impl<T: SEXPbucket> Iterator for CharVecIter<T> {
    // we will be counting with usize
    type Item = CString;

    // next() is the only required method
    fn next(&mut self) -> Option<CString> {
        // increment our count. This is why we started at zero.
        // check to see if we've finished counting or not.
        if self.next < self.size {
            self.next += 1;
            unsafe { Some(CString::urnew(STRING_ELT(self.ty.s(), self.next - 1))) }
        } else {
            None
        }

    }
}

impl<T: SEXPbucket> IntoIterator for CharVecM<T> {
    type Item = CString;
    type IntoIter = CharVecIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        CharVecIter {
            size: self.rsize(),
            next: 0,
            ty: self,
        }
    }
}

impl<T: SEXPbucket> ExactSizeIterator for CharVecIter<T> {
    // We already have the number of iterations, so we can use it directly.
    fn len(&self) -> usize {
        self.size as usize
    }
}


/// Create a CharVec
///
#[macro_export]
macro_rules! charvec {
    ($($tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = CharVec::alloc(size as usize);
	  unsafe{
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
			let xi = try!(::std::ffi::CString::new($tts));
			let tmp : SEXP = $crate::protect::stackp::Shield::new(RCharM::<NoProtect>::from(xi).s()).s(); 
      		res.usetc(x-1,tmp);
      		
      )*      
	}
      res
      }
    };
    
    ($($id:ident ~ $tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = CharVec::alloc(size as usize);
	  let mut name = CharVec::alloc(size as usize);
	  unsafe{
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
			let xi = try!(::std::ffi::CString::new($tts));
			let tmp : SEXP = $crate::protect::stackp::Shield::new(RCharM::<NoProtect>::from(xi).s()).s(); 
      		res.usetc(x-1,tmp);
      		name.uset(x-1, stringify!($id));
      )*
	}
	  unsafe{Rf_setAttrib(res.s(), R_NamesSymbol,name.s());}
      res
      }
    }
}

/// Create a CharVec with unsafe code
///
#[macro_export]
macro_rules! ucharvec {
    ($($tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = CharVec::alloc(size as usize);
      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1, $tts);
      		
      )*      
      res
      }
    };
        ($($id:ident ~ $tts:expr),*) => {
      // count macro parameter learn from rust macro book	
      {let size = <[()]>::len(&[$(replace_expr!($tts, ())),*]);
      	
      // init 
      let mut res = CharVec::alloc(size as usize);
	  let mut name = CharVec::alloc(size as usize);

      let mut x = 0;
      $(
			// skip a warning message 
			x += 1;
      		res.uset(x-1, $tts);
      		name.uset(x-1, stringify!($id));
      )*
	
	  Rf_setAttrib(res.s(), R_NamesSymbol,name.s());
      res
      }
    }
}
