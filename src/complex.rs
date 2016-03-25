//! R Rcomplex methods
//!
//!

use rdll::Rcomplex;
use std::ops::*;
use std::fmt::{Debug, Formatter, Result};
use std::cmp::PartialEq;

impl Add for Rcomplex {
    type Output=Rcomplex;
    #[inline]
    fn add(self, rhs: Rcomplex) -> Rcomplex {
        Rcomplex {
            r: (self.r + rhs.r),
            i: (self.i + rhs.i),
        }
    }
}

impl Sub for Rcomplex {
    type Output=Rcomplex;
    #[inline]
    fn sub(self, rhs: Rcomplex) -> Rcomplex {
        Rcomplex {
            r: (self.r - rhs.r),
            i: (self.i - rhs.i),
        }
    }
}

impl Neg for Rcomplex {
    type Output=Rcomplex;
    #[inline]
    fn neg(self) -> Rcomplex {
        Rcomplex {
            r: -self.r,
            i: -self.i,
        }
    }
}

impl Not for Rcomplex {
    type Output=bool;
    #[inline]
    fn not(self) -> bool {
        (self.r != 0.0) && (self.i != 0.0)
    }
}

impl Debug for Rcomplex {
    #[inline]
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "({}+{}i)", self.r, self.i)
    }
}

impl PartialEq for Rcomplex {
    #[inline]
    fn eq(&self, other: &Rcomplex) -> bool {
        self.i == other.i && self.r == other.r
    }
}


impl Div for Rcomplex {
    type Output=Rcomplex;
    #[inline]
    fn div(self, rhs: Rcomplex) -> Rcomplex {

        let abr = if rhs.r < 0.0 {
            -rhs.r
        } else {
            rhs.r
        };
        let abis = if rhs.i < 0.0 {
            -rhs.i
        } else {
            rhs.i
        };
        let ratio: ::std::os::raw::c_double;
        let den: ::std::os::raw::c_double;

        if abr <= abis {
            ratio = rhs.r / rhs.i;
            den = rhs.i * (1.0 + ratio * ratio);
            Rcomplex {
                r: (self.r * ratio + self.i) / den,
                i: (self.i * ratio - self.r) / den,
            }
        } else {
            ratio = rhs.i / rhs.r;
            den = rhs.r * (1.0 + ratio * ratio);
            Rcomplex {
                r: (self.r + self.i * ratio) / den,
                i: (self.i - self.r * ratio) / den,
            }
        }

    }
}
