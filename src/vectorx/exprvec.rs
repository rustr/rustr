use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
// use ::protect::stackp::*;
use error::*;
use error::REKind::NotCompatible;
use std::ops::Range;


pub type ExprVec = ExprVecM<Preserve>;


impl<T: SEXPbucket> ExprVecM<T> {
    pub fn new(x: SEXP) -> RResult<ExprVecM<T>> {

        if RTYPEOF(x) != EXPRSXP {
            return rerror(NotCompatible("expecting a expression vector".into()));
        }
        Ok(ExprVecM { data: T::new(x) })
    }
    pub fn alloc(x: usize) -> ExprVecM<T> {
        ExprVecM { data: T::new(unsafe { Rf_allocVector(EXPRSXP, x as R_xlen_t) }) }
    }
    pub fn alloc_matrix(x: ::std::os::raw::c_int, y: ::std::os::raw::c_int) -> ExprVecM<T> {
        ExprVecM { data: T::new(unsafe { Rf_allocMatrix(EXPRSXP, x, y) }) }
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
    pub unsafe fn uset<TT: ToExpr>(&mut self, ind: usize, value: TT) {
        SET_VECTOR_ELT(self.s(), ind as R_xlen_t, value.expr());
    }
    pub fn set<TT: ToExpr>(&mut self, ind: usize, value: TT) -> RResult<()> {
        unsafe {
            if Rf_xlength(self.s()) <= ind as R_xlen_t {
                return rraise("index out of bound");
            }
            SET_VECTOR_ELT(self.s(), (ind - 1) as R_xlen_t, value.expr());
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
}

// impl<T: SEXPbucket, E: UIntoR + Clone> From<Vec<E>> for ExprVecM<T> {
//    fn from(x: Vec<E>) -> ExprVecM<T> {
//        let size_x = x.len();
//        unsafe {
//            let rvec = Shield::new(Rf_allocVector(VECSXP, size_x as R_xlen_t));
//            let mut xs =0;
//            for ii in x {
// 				SET_VECTOR_ELT(rvec.s(),  xs, Shield::new(ii.uintor()).s());
//            	xs+=1;
//            }
//            ExprVecM { data: T::new(rvec.s()) }
//        }
//    }
// }

// impl<T: SEXPbucket,D:URNew> From<ExprVecM<T>> for Vec<D> {
//    fn from(x: ExprVecM<T>) -> Vec<D> {
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

impl<T: SEXPbucket> URNew for ExprVecM<T> {
    unsafe fn urnew(x: SEXP) -> Self {
        ExprVecM { data: T::new(x) }
    }
}
impl<T: SEXPbucket> RNew for ExprVecM<T> {
    fn rnew(x: SEXP) -> RResult<Self> {
        Self::new(x)
    }
}

impl<T: SEXPbucket> RSize for ExprVecM<T> {
    fn rsize(&self) -> R_xlen_t {
        unsafe { Rf_xlength(self.s()) }
    }
}

gen_traits_sexp!(ExprVecM);


impl<T: SEXPbucket> RName for ExprVecM<T> {}
impl<T: SEXPbucket> RDim for ExprVecM<T> {
    type Output = SEXP;
}

#[derive(Debug)]
pub struct ExprIter<T: SEXPbucket> {
    size: R_xlen_t,
    next: R_xlen_t,
    ty: ExprVecM<T>,
}

impl<T: SEXPbucket> Iterator for ExprIter<T> {
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

impl<T: SEXPbucket> IntoIterator for ExprVecM<T> {
    type Item = SEXP;
    type IntoIter = ExprIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        ExprIter {
            size: self.rsize(),
            next: 0,
            ty: self,
        }
    }
}

impl<T: SEXPbucket> ExactSizeIterator for ExprIter<T> {
    // We already have the number of iterations, so we can use it directly.
    fn len(&self) -> usize {
        self.size as usize
    }
}
