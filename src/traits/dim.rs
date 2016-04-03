use rdll::*;
use protect::stackp::*;
use traits::*;
use error::*;
use std::ffi::CString;
use rtype::*;

pub trait RDim: RAttribute+RSize {
    type Output;
    fn dim(&self) -> Vec<usize> {
        if !self.is_matrix() && !self.is_array() {
            return Vec::new();
        }
        unsafe {
            let res = Shield::new(Rf_getAttrib(self.s(), R_DimSymbol));
            Vec::urnew(res.s())
        }
    }
    fn set_dim(&mut self, x: &[usize]) -> RResult<()> {
		if self.rsize() == 0{
			return rraise("can not set 0 length vector");
        }
    	let lens = x.iter().fold(1,|sum,i| sum * i );
        if self.rsize() as usize != lens {
			return rraise(format!("length is {} , can not set dimension {:?}",self.rsize(),lens));
        }
        unsafe {
            Rf_setAttrib(self.s(), R_DimSymbol, x.uintor());
        }
        Ok(())
    }
    fn nrow(&self) -> usize {
        if !self.is_matrix() {
            return 0;
        }
        self.dim()[0]
    }
    fn ncol(&self) -> usize {
        if !self.is_matrix() {
            return 0;
        }
        self.dim()[1]
    }
    fn dimnamec(&self, side: usize) -> Vec<CString> {
        if !self.is_matrix() && !self.is_array() {
            return Vec::new();
        }
        unsafe {
            let dims = self.dim();
            if dims.len() < side {
                return Vec::new();
            }
            let dimnames = Rf_getAttrib(self.s(), R_DimNamesSymbol);
            if Rf_isNull(dimnames) == Rboolean::TRUE {
                return Vec::new();
            }
            let res = VECTOR_ELT(dimnames, (side - 1) as R_xlen_t);
            Vec::urnew(res.s())
        }
    }
    fn dimname(&self, side: usize) -> RResult<Vec<String>> {
        if !self.is_matrix() && !self.is_array() {
            return Ok(Vec::new());
        }
        unsafe {
            let dims = self.dim();
            if dims.len() < side {
                return Ok(Vec::new());
            }
            let dimnames = Rf_getAttrib(self.s(), R_DimNamesSymbol);
            if Rf_isNull(dimnames) == Rboolean::TRUE {
                return Ok(Vec::new());
            }
            let res = VECTOR_ELT(dimnames, (side - 1) as R_xlen_t);
            Vec::rnew(res.s())
        }
    }
    // CharVec
    fn set_dimname(&mut self, side: usize, x: &[String]) -> RResult<()> {
        unsafe {
            if x.len() == 0 {
                Rf_setAttrib(self.s(), R_DimNamesSymbol, R_NilValue);
                Ok(())
            } else {
                let dims = self.dim();
                if dims.len() < side || dims[side - 1] != x.len() {
                    return rraise(format!("dimension extent is {} the length of names is {}",
                                          dims.len(),
                                          x.len()));
                }

                let dimnames = Rf_getAttrib(self.s(), R_DimNamesSymbol);
                if Rf_isNull(dimnames) == Rboolean::TRUE {
                    let new_dimnames = Shield::new(Rf_allocVector(VECSXP, dims.len() as R_xlen_t))
                                           .s();
                    SET_VECTOR_ELT(new_dimnames, (side - 1) as R_xlen_t, try!(x.intor()));
                    Rf_setAttrib(self.s(), R_DimNamesSymbol, new_dimnames);
                } else {
                    SET_VECTOR_ELT(dimnames, (side - 1) as R_xlen_t, try!(x.intor()));
                }
                Ok(())
            }
        }
    }
}
