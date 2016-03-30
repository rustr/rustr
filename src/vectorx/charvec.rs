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
    pub fn alloc(x: R_xlen_t) -> CharVecM<T> {
        CharVecM { data: T::new(unsafe { Rf_allocVector(STRSXP, x) }) }
    }
    pub fn alloc_matrix(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> CharVecM<T> {
        CharVecM { data: T::new(unsafe { Rf_allocMatrix(STRSXP, x, y) }) }
    }
    pub fn atc(&self, ind: R_xlen_t) -> Option<CString> {
        unsafe {
            if Rf_xlength(self.s()) <= ind {
                return None;
            }
            Some(CString::urnew(STRING_ELT(self.s(), ind)))
        }
    }
    pub unsafe fn uat(&self, ind: R_xlen_t) -> Result<String, IntoStringError> {
        CString::urnew(STRING_ELT(self.s(), ind)).into_string()
    }
    pub unsafe fn uset(&mut self, ind: R_xlen_t, value: &str) {
        SET_STRING_ELT(self.s(), ind, Shield::new(Rf_mkString(c_str(value).as_ptr())).s())
    }
    pub unsafe fn uatc(&self, ind: R_xlen_t) -> CString {
        CString::urnew(STRING_ELT(self.s(), ind))
    }
    pub unsafe fn usetc(&mut self, ind: R_xlen_t, value: CString) {
        SET_STRING_ELT(self.s(), ind, Shield::new(Rf_mkString(value.as_ptr())).s())
    }
    pub fn setc(&mut self, ind: R_xlen_t, value: CString) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) < ind as R_xlen_t || ind == 0 {
                return rraise("index out of bound");
            }
            SET_STRING_ELT(self.s(),
                           ind - 1,
                           Shield::new(Rf_mkString(value.as_ptr())).s());
            Ok(())
        }
    }
    pub fn range(&self, ind: Range<R_xlen_t>) -> Option<Vec<CString>> {
        unsafe {
            if Rf_xlength(self.s()) <= ind.end {
                return None;
            }
            let mut vecs = Vec::with_capacity((ind.end - ind.start) as usize);
            for ii in ind {
                vecs.push(CString::urnew(STRING_ELT(self.s(), ii)));
            }
            Some(vecs)
        }
    }
    pub fn is_duplicated(&self, from_last: bool) -> R_xlen_t {
        let last = if from_last { Rboolean::TRUE} else { Rboolean::FALSE };
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
                SET_STRING_ELT(rvec.s(),
                               xs,
                               Shield::new(Rf_mkString(ii.into().as_ptr())).s());
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

impl<T: SEXPbucket> URNew for  CharVecM<T> {
    unsafe fn urnew(x: SEXP) -> Self {
		CharVecM{ data: T::new(x) }
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
