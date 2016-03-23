use rdll::{SEXP, R_NilValue, Rf_protect, Rf_unprotect};
use traits::ToSEXP;


/// Shelter
///
/// Shelter count the protect in scrope
pub struct Shelter {
    nprotected: ::std::os::raw::c_int,
}

impl Shelter {
    pub fn new() -> Shelter {
        // println!("{:?}", "protecting \n");
        Shelter { nprotected: 0 }
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
