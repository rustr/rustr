use ::rdll::*;
use traits::{ToSEXP, SetSEXP, RNew};
use protect::stackp::Shield;

use util::c_str;
use storage::*;
use error::*;

pub trait RField: ToSEXP+ SetSEXP{
    fn get<D: RNew>(&self, field_name: &str) -> RResult<D> {
        unsafe {
            let call = Shield::new(Rf_lang3(R_DollarSymbol,
                                            self.s(),
                                            Rf_mkString(c_str(field_name).as_ptr())));
            RNew::rnew(Rf_eval(call.s(), R_GlobalEnv))
        }
    }
    fn set<T: ToSEXP>(&mut self, field_name: &str, x: T) {
        unsafe {
            let dollar = Rf_install(c_str(field_name).as_ptr());
            let call = Shield::new(Rf_lang4(dollar,
                                            self.s(),
                                            Rf_mkString(c_str(field_name).as_ptr()),
                                            x.s()));
            self.set_s(Rf_eval(call.s(), R_GlobalEnv));
        }
    }
}
