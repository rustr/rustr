static mut RNGScopeCounter: usize = 0;

use rand::Rng;
use rdll::*;
use std::marker::PhantomData;
use std::os::raw::c_double as cd;
pub struct RRand {
    data: PhantomData<()>,
}

impl RRand {
    pub fn new() -> RRand {
        unsafe {
            if RNGScopeCounter == 0 {
                GetRNGstate();

            }
            RNGScopeCounter += 1;
            RRand { data: PhantomData }
        }
    }
    pub fn norm_rand(&mut self) -> cd {
        unsafe { ::rmath::norm_rand() }
    }
    pub fn unif_rand(&mut self) -> cd {
        unsafe { ::rmath::unif_rand() }
    }
    pub fn exp_rand(&mut self) -> cd {
        unsafe { ::rmath::exp_rand() }
    }
    pub fn rnorm(&mut self, mu: cd, sigma: cd) -> cd {
        unsafe { ::rmath::rnorm(mu, sigma) }
    }
    pub fn runif(&mut self, a: cd, b: cd) -> cd {
        unsafe { ::rmath::runif(a, b) }
    }
    pub fn rgamma(&mut self, a: cd, scl: cd) -> cd {
        unsafe { ::rmath::rgamma(a, scl) }
    }
    pub fn rbeta(&mut self, a: cd, b: cd) -> cd {
        unsafe { ::rmath::rbeta(a, b) }
    }
    pub fn rlnorm(&mut self, ml: cd, sl: cd) -> cd {
        unsafe { ::rmath::rlnorm(ml, sl) }
    }
    pub fn rchisq(&mut self, df: cd) -> cd {
        unsafe { ::rmath::rchisq(df) }
    }
    pub fn rnchisq(&mut self, df: cd, lb: cd) -> cd {
        unsafe { ::rmath::rnchisq(df, lb) }
    }
    pub fn rf(&mut self, df1: cd, df2: cd) -> cd {
        unsafe { ::rmath::rf(df1, df2) }
    }
    pub fn rt(&mut self, n: cd) -> cd {
        unsafe { ::rmath::rt(n) }
    }
    pub fn rbinom(&mut self, n: cd, p: cd) -> cd {
        unsafe { ::rmath::rbinom(n, p) }
    }
    // todo: multinorm
    pub fn rcauchy(&mut self, lc: cd, sl: cd) -> cd {
        unsafe { ::rmath::rcauchy(lc, sl) }
    }
    pub fn rexp(&mut self, sl: cd) -> cd {
        unsafe { ::rmath::rexp(sl) }
    }
    pub fn rgeom(&mut self, p: cd) -> cd {
        unsafe { ::rmath::rgeom(p) }
    }
    pub fn rhyper(&mut self, r: cd, b: cd, n: cd) -> cd {
        unsafe { ::rmath::rhyper(r, b, n) }
    }
    pub fn rnbinom(&mut self, sz: cd, p: cd) -> cd {
        unsafe { ::rmath::rnbinom(sz, p) }
    }
    pub fn rnbinom_mu(&mut self, sz: cd, mu: cd) -> cd {
        unsafe { ::rmath::rnbinom_mu(sz, mu) }
    }
    pub fn rpois(&mut self, mu: cd) -> cd {
        unsafe { ::rmath::rpois(mu) }
    }
    pub fn rweibull(&mut self, sh: cd, sl: cd) -> cd {
        unsafe { ::rmath::rweibull(sh, sl) }
    }
    pub fn rlogis(&mut self, lc: cd, sl: cd) -> cd {
        unsafe { ::rmath::rlogis(lc, sl) }
    }
    pub fn rwilcox(&mut self, m: cd, n: cd) -> cd {
        unsafe { ::rmath::rwilcox(m, n) }
    }
    pub fn rsignrank(&mut self, n: cd) -> cd {
        unsafe { ::rmath::rsignrank(n) }
    }
}

impl Rng for RRand {
    fn next_u32(&mut self) -> u32 {
        // learn from do_sample R source
        (u32::max_value() as cd * unsafe { unif_rand() } + 1.0) as u32
    }
}

impl Drop for RRand {
    fn drop(&mut self) {
        unsafe {
            RNGScopeCounter -= 1;
            if RNGScopeCounter == 0 {
                PutRNGstate()
            }
        }
    }
}
