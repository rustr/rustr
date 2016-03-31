use rdll::{SEXP, R_NilValue, Rf_protect, Rf_unprotect};
use traits::ToSEXP;
use std::default::Default;

/// Shelter
///
/// Shelter count the protect in scrope
pub struct Shelter {
    nprotected: ::std::os::raw::c_int,
}

impl Default for Shelter {
    fn default() -> Self {
        Shelter { nprotected: 0 }
    }
}

impl Shelter {
    pub fn new() -> Shelter {
        // println!("{:?}", "protecting \n");
        Self::default()
    }

    pub fn add<T: ToSEXP>(&mut self, a: T) -> SEXP {
        let sexp = unsafe { a.s() };
        unsafe {
            if sexp != R_NilValue {
                Rf_protect(sexp);
                self.nprotected = self.nprotected + 1;
            }
        }

        sexp
    }
}

impl Drop for Shelter {
    fn drop(&mut self) {
        unsafe { Rf_unprotect(self.nprotected) }
    }
}
