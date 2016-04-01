use ::rdll::*;
use traits::*;
use error::*;
use vectorx::*;

pub trait RName: ToSEXP + RSize {
    fn get_name<D: RNew>(&self) -> RResult<D> {
        D::rnew(unsafe { Rf_getAttrib(self.s(), R_NamesSymbol) })
    }
    fn name(&self) -> CharVec {
        unsafe { CharVec::urnew(Rf_getAttrib(self.s(), R_NamesSymbol) )}
    }
    unsafe fn namesexp(&self) -> SEXP {
        Rf_getAttrib(self.s(), R_NamesSymbol) 
    }
    fn set_name(&self, attr: &CharVec) -> RResult<()> {
        unsafe {
            if self.rsize() == Rf_xlength(self.s()) {
                Rf_setAttrib(self.s(), R_NamesSymbol, attr.s());
                return Ok(());
            }
             rraise("CharVec length is not the same as the vector.")
        }
    }
    unsafe fn uset_name(&self, attr: &CharVec) {
        Rf_setAttrib(self.s(), R_NamesSymbol, attr.s());
    }
}
