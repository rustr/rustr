use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;
use std::os::raw::c_int;
pub type BoolVec = BoolVecM<Preserve>;

impl<T: SEXPbucket> BoolVecM<T> {
    pub fn new(x: SEXP) -> RResult<BoolVecM<T>> {
        if RTYPEOF(x) != LGLSXP {
            return rerror(NotCompatible("expecting a numeric vector".into()));
        }
        Ok(BoolVecM { data: T::new(x) })
    }
    pub fn alloc(x: R_xlen_t) -> BoolVecM<T> {
        BoolVecM { data: T::new(unsafe { Rf_allocVector(LGLSXP, x) }) }
    }
    pub fn alloc_matrix(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> BoolVecM<T> {
        BoolVecM { data: T::new(unsafe { Rf_allocMatrix(LGLSXP, x, y) }) }
    }

    pub fn at(&self, ind: usize) -> Option<bool> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return None;
            }
            let ptr = LOGICAL(self.s());

            Some(*ptr.offset(ind as isize) == 1)
        }
    }
    pub unsafe fn uat(&self, ind: usize) -> bool {
        let ptr = LOGICAL(self.s());
        *ptr.offset(ind as isize) == 1
    }
    pub unsafe fn uset(&mut self, ind: usize, value: bool) {
        let ptr = LOGICAL(self.s());
        *ptr.offset(ind as isize) = value as c_int;
    }
    pub fn set(&mut self, ind: usize, value: bool) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) < ind as R_xlen_t || ind == 0 {
                return rraise("index out of bound");
            }
            let ptr = LOGICAL(self.s());
            *ptr.offset((ind - 1) as isize) = value as c_int;
            Ok(())
        }
    }
    pub fn range(&self, ind: Range<usize>) -> Option<Vec<bool>> {
        unsafe {
            if Rf_xlength(self.s()) <= ind.end as R_xlen_t {
                return None;
            }
            let mut vecs = Vec::with_capacity(ind.len() as usize);
            let ptr = LOGICAL(self.s());
            for ii in ind {
                vecs.push(*ptr.offset(ii as isize) == 1);
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



impl<T: SEXPbucket, E: Into<bool> + Clone> From<Vec<E>> for BoolVecM<T> {
    fn from(x: Vec<E>) -> BoolVecM<T> {
        let size_x = x.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(LGLSXP, size_x as R_xlen_t));
            let rptr = LOGICAL(rvec.s());
            for ii in 0..size_x {
                *rptr.offset(ii as isize) =
                    (*x.get_unchecked(ii as usize)).clone().into() as ::std::os::raw::c_int;
            }
            BoolVecM { data: T::new(rvec.s()) }
        }
    }
}

impl<T: SEXPbucket> From<BoolVecM<T>> for Vec<bool> {
    fn from(x: BoolVecM<T>) -> Vec<bool> {
        unsafe {
            let lens = Rf_xlength(x.s());
            let mut vecs = Vec::with_capacity(lens as usize);
            let rptr = LOGICAL(x.s());
            for ii in 0..lens {
                vecs.push(*rptr.offset(ii as isize) == 1);
            }
            vecs
        }
    }
}


impl<T: SEXPbucket> RNew for BoolVecM<T> {
    fn rnew(x: SEXP) -> RResult<Self> {
        Self::new(x)
    }
}

impl<T: SEXPbucket> RSize for BoolVecM<T> {
    fn rsize(&self) -> R_xlen_t {
        unsafe { Rf_xlength(self.s()) }
    }
}

gen_traits_sexp!(BoolVecM);


impl<T: SEXPbucket> RName for BoolVecM<T> {}
impl<T: SEXPbucket> RDim for BoolVecM<T> {
    type Output = bool;
}

#[derive(Debug)]
pub struct BoolVecIter<T: SEXPbucket> {
    data: *mut ::std::os::raw::c_int,
    size: R_xlen_t,
    next: R_xlen_t,
    ty: BoolVecM<T>,
}

impl<T: SEXPbucket> Iterator for BoolVecIter<T> {
    // we will be counting with usize
    type Item = bool;

    // next() is the only required method
    fn next(&mut self) -> Option<bool> {
        // increment our count. This is why we started at zero.
        // check to see if we've finished counting or not.
        if self.next < self.size {
            self.next += 1;
            unsafe { Some(*self.data.offset((self.next - 1) as isize) == 1) }
        } else {
            None
        }

    }
}

impl<T: SEXPbucket> IntoIterator for BoolVecM<T> {
    type Item = bool;
    type IntoIter = BoolVecIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        unsafe {
            BoolVecIter {
                data: LOGICAL(self.s()),
                size: self.rsize(),
                next: 0,
                ty: self,
            }
        }
    }
}

impl<T: SEXPbucket> ExactSizeIterator for BoolVecIter<T> {
    // We already have the number of iterations, so we can use it directly.
    fn len(&self) -> usize {
        self.size as usize
    }
}
