use ::rdll::*;
use traits::*;
use protect::stackp::Shield;
use rtype::*;
use util::c_str;
use storage::*;
use error::*;

pub trait RName : ToSEXP + RSize{
    fn get_name<D: RNew>(&self) -> RResult<D> {
        D::rnew(unsafe { Rf_getAttrib(self.s(), R_NamesSymbol) })
    }
    fn set_name<T: ToSEXP>(&self, attr: T) {
        unsafe {
            if RTYPEOF(self.s()) == STRSXP && self.rsize() == Rf_xlength(self.s()) {
                Rf_setAttrib(self.s(), R_NamesSymbol, attr.s());
            } else {
                // slower, and R may throw error
                let names_sym = Rf_install(c_str("names<-").as_ptr());
                let new_vec = Shield::new(Rf_eval(Rf_lang3(names_sym, self.s(), attr.s()),
                                                  R_GlobalEnv));
                Rf_setAttrib(self.s(), R_NamesSymbol, new_vec.s());
            }
        }
    }
}
