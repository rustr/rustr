use ::rdll::*;
use ::traits::*;

/// Armor
///
/// save index in Armor

pub struct Armor {
    data: SEXP,
    index: PROTECT_INDEX,
}

impl Armor {
    /// return R_NilValue 
    pub fn empty() -> Armor {
        unsafe {
            Armor {
                data: R_NilValue,
                index: -1,
            }
        }
    }

    /// create from a SEXP
    pub fn new<T: ToSEXP>(x: T) -> Armor {
        let mut res = Armor::empty();
        unsafe {
            res.init(x.s());
        }

        res
    }

    /// protect with index in new, and will be asigned later
    pub unsafe fn init(&mut self, x: SEXP) {
        let index: *mut PROTECT_INDEX = &mut (self.index);
        R_ProtectWithIndex(x, index);
    }

    pub fn reassign<T: ToSEXP>(&mut self, x: T) -> &SEXP {
        unsafe {
            self.data = x.s();
            R_Reprotect(self.data, self.index);
            &(self.data)
        }
    }
}

impl Drop for Armor {
    fn drop(&mut self) {
        unsafe {
            if self.index >= 0 {
                Rf_unprotect(1);
            }
        }
    }
}

impl Moves for Armor {
    fn moves(mut other: Armor) -> Armor {
        let res = Armor {
            data: other.data,
            index: other.index,
        };
        other.index = -1;
        unsafe {
            other.data = R_NilValue;
        }

        res
    }
}

impl ToSEXP for Armor {
    unsafe fn s(&self) -> SEXP {
        self.data
    }
}
