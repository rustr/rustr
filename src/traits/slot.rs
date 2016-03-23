use ::rdll::*;
use traits::*;
use error::{rerror, RResult};
use error::REKind::{NotS4, NoSuchSlot};
use symbol::*;


pub trait RSlot :ToSEXP{
    fn get_slot<D: RNew>(&self, slot: &str) -> RResult<D> {
        unsafe {
            let name = Symbol::from(slot);
            if Rf_isS4(self.s()) == Rboolean::FALSE {
                return rerror(NotS4("fail to get slot".into()));
            }
            if R_has_slot(self.s(), name.s()) == 0 {
                return rerror(NoSuchSlot("fail to get slot".into()));
            }
            RNew::rnew(R_do_slot(self.s(), name.s()))
        }
    }
    fn set_slot<T: ToSEXP, D: RNew>(&self, slot: &str, x: T) -> RResult<D> {
        unsafe {
            let name = Symbol::from(slot);
            if Rf_isS4(self.s()) == Rboolean::FALSE {
                return rerror(NotS4("fail to get slot".into()));
            }
            if R_has_slot(self.s(), name.s()) == 0 {
                return rerror(NoSuchSlot("fail to get slot".into()));
            }
            RNew::rnew(R_do_slot_assign(self.s(), name.s(), x.s()))
        }
    }
    fn has_slot(&self, slot: &str) -> RResult<bool> {
        unsafe {
            let name = Symbol::from(slot);
            if Rf_isS4(self.s()) == Rboolean::FALSE {
                return rerror(NotS4("fail to get slot".into()));
            }
            if R_has_slot(self.s(), name.s()) == 0 {
                return Ok(false);
            }
            Ok(true)

        }
    }
}
