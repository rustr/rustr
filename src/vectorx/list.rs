use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;


pub type RList = RListM<Preserve>;


impl<T: SEXPbucket> RListM<T> {
    pub fn new(x: SEXP) -> RResult<RListM<T>> {

        if RTYPEOF(x) != VECSXP {
            return rerror(NotCompatible("expecting a list".into()));
        }

        Ok(RListM { data: T::new(x) })
    }
    pub fn alloc(x: R_xlen_t) -> RListM<T> {
        RListM { data: T::new(unsafe { Rf_allocVector(VECSXP, x) }) }
    }
    pub fn alloc_matrix(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> RListM<T> {
        RListM { data: T::new(unsafe { Rf_allocMatrix(VECSXP, x, y) }) }
    }
    pub fn atc(&self, ind: R_xlen_t) -> Option<SEXP> {
        unsafe {
            if Rf_xlength(self.s()) <= ind {
                return None;
            }
            Some(VECTOR_ELT(self.s(), ind))
        }
    }
    pub unsafe fn uatc(&self, ind: R_xlen_t) -> SEXP {
        VECTOR_ELT(self.s(), ind)
    }
    pub unsafe fn usetc<TT: ToSEXP>(&mut self, ind: R_xlen_t, value: TT) {
        SET_VECTOR_ELT(self.s(), ind, value.s());
    }
    pub fn setc<TT: ToSEXP>(&mut self, ind: R_xlen_t, value: TT) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) < ind as R_xlen_t || ind == 0 {
                return rraise("index out of bound");
            }
            SET_VECTOR_ELT(self.s(), ind - 1, value.s());
            Ok(())
        }
    }
    pub fn range(&self, ind: Range<R_xlen_t>) -> Option<Vec<SEXP>> {
        unsafe {
            if Rf_xlength(self.s()) <= ind.end {
                return None;
            }
            let mut vecs = Vec::with_capacity((ind.end - ind.start) as usize);
            for ii in ind {
                vecs.push(VECTOR_ELT(self.s(), ii));
            }
            Some(vecs)
        }
    }
    pub fn is_duplicated(&self, from_last: bool) -> R_xlen_t {
        let last = if from_last { Rboolean::TRUE} else { Rboolean::FALSE };
        unsafe { Rf_any_duplicated(self.s(), last) }
    }
}

#[allow(explicit_counter_loop)]
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
