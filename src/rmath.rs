//! R Math Functions
//!
//!
//!


// see R's Rmath.h as well as Writing R Extension
use std::os::raw::{c_double as cd, c_int};
use rdll::*;

// should call get set RNG first

// Random Number Generators
pub unsafe fn norm_rand() -> cd {

    ::rdll::norm_rand()

}
pub unsafe fn unif_rand() -> cd {

    ::rdll::unif_rand()

}
pub unsafe fn exp_rand() -> cd {

    ::rdll::exp_rand()

}

// Normal Distribution
pub fn dnorm(x: cd, mu: cd, sigma: cd, lg: c_int) -> cd {
    unsafe { Rf_dnorm4(x, mu, sigma, lg) }
}
pub fn pnorm(x: cd, mu: cd, sigma: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnorm5(x, mu, sigma, lt, lg) }
}
pub fn qnorm(p: cd, mu: cd, sigma: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnorm5(p, mu, sigma, lt, lg) }
}
pub unsafe fn rnorm(mu: cd, sigma: cd) -> cd {

    Rf_rnorm(mu, sigma)

}

pub fn pnorm_both(x: cd, cum: *mut cd, ccum: *mut cd, lt: c_int, lg: c_int) {
    unsafe { Rf_pnorm_both(x, cum, ccum, lt, lg) }
}

// Uniform Distribution
pub fn dunif(x: cd, a: cd, b: cd, lg: c_int) -> cd {
    unsafe { Rf_dunif(x, a, b, lg) }
}
pub fn punif(x: cd, a: cd, b: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_punif(x, a, b, lt, lg) }
}
pub fn qunif(p: cd, a: cd, b: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qunif(p, a, b, lt, lg) }
}
pub unsafe fn runif(a: cd, b: cd) -> cd {
    {
        Rf_runif(a, b)
    }
}

// Gamma Distribution
pub fn dgamma(x: cd, shp: cd, scl: cd, lg: c_int) -> cd {
    unsafe { Rf_dgamma(x, shp, scl, lg) }
}
pub fn pgamma(x: cd, al: cd, scl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pgamma(x, al, scl, lt, lg) }
}
pub fn qgamma(p: cd, al: cd, scl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qgamma(p, al, scl, lt, lg) }
}
pub unsafe fn rgamma(a: cd, scl: cd) -> cd {
    {
        Rf_rgamma(a, scl)
    }
}

pub fn log1pmx(x: cd) -> cd {
    unsafe { Rf_log1pmx(x) }
}
// pub unsafe fn rf_log1pexp(x: cd) -> cd {
//
//     log1pexp(x)
//
// }  // <-- ../nmath/plogis.c
pub fn lgamma1p(a: cd) -> cd {
    unsafe { Rf_lgamma1p(a) }
}
pub fn logspace_add(lx: cd, ly: cd) -> cd {
    unsafe { Rf_logspace_add(lx, ly) }
}
pub fn logspace_sub(lx: cd, ly: cd) -> cd {
    unsafe { Rf_logspace_sub(lx, ly) }
}

// Beta Distribution
pub fn dbeta(x: cd, a: cd, b: cd, lg: c_int) -> cd {
    unsafe { Rf_dbeta(x, a, b, lg) }
}
pub fn pbeta(x: cd, p: cd, q: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pbeta(x, p, q, lt, lg) }
}
pub fn qbeta(a: cd, p: cd, q: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qbeta(a, p, q, lt, lg) }
}
pub unsafe fn rbeta(a: cd, b: cd) -> cd {
    {
        Rf_rbeta(a, b)
    }
}

// Lognormal Distribution
pub fn dlnorm(x: cd, ml: cd, sl: cd, lg: c_int) -> cd {
    unsafe { Rf_dlnorm(x, ml, sl, lg) }
}
pub fn plnorm(x: cd, ml: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_plnorm(x, ml, sl, lt, lg) }
}
pub fn qlnorm(p: cd, ml: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qlnorm(p, ml, sl, lt, lg) }
}
pub unsafe fn rlnorm(ml: cd, sl: cd) -> cd {
    {
        Rf_rlnorm(ml, sl)
    }
}

// Chi-squared Distribution
pub fn dchisq(x: cd, df: cd, lg: c_int) -> cd {
    unsafe { Rf_dchisq(x, df, lg) }
}
pub fn pchisq(x: cd, df: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pchisq(x, df, lt, lg) }
}
pub fn qchisq(p: cd, df: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qchisq(p, df, lt, lg) }
}
pub unsafe fn rchisq(df: cd) -> cd {
    {
        Rf_rchisq(df)
    }
}

// Non-central Chi-squared Distribution
pub fn dnchisq(x: cd, df: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dnchisq(x, df, n, lg) }
}
pub fn pnchisq(x: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnchisq(x, df, n, lt, lg) }
}
pub fn qnchisq(p: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnchisq(p, df, n, lt, lg) }
}
pub unsafe fn rnchisq(df: cd, lb: cd) -> cd {
    {
        Rf_rnchisq(df, lb)
    }
}

// F Distibution
pub fn df(x: cd, df1: cd, df2: cd, lg: c_int) -> cd {
    unsafe { Rf_df(x, df1, df2, lg) }
}
pub fn pf(x: cd, df1: cd, df2: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pf(x, df1, df2, lt, lg) }
}
pub fn qf(p: cd, df1: cd, df2: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qf(p, df1, df2, lt, lg) }
}
pub unsafe fn rf(df1: cd, df2: cd) -> cd {

    Rf_rf(df1, df2)

}

// Student t Distibution
pub fn dt(x: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dt(x, n, lg) }
}
pub fn pt(x: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pt(x, n, lt, lg) }
}
pub fn qt(p: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qt(p, n, lt, lg) }
}
pub unsafe fn rt(n: cd) -> cd {

    Rf_rt(n)

}

// Binomial Distribution
pub fn dbinom(x: cd, n: cd, p: cd, lg: c_int) -> cd {
    unsafe { Rf_dbinom(x, n, p, lg) }
}
pub fn pbinom(x: cd, n: cd, p: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pbinom(x, n, p, lt, lg) }
}
pub fn qbinom(p: cd, n: cd, m: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qbinom(p, n, m, lt, lg) }
}
pub unsafe fn rbinom(n: cd, p: cd) -> cd {

    Rf_rbinom(n, p)

}

// Multnomial Distribution
pub unsafe fn rmultinom(n: c_int, prob: *mut cd, k: c_int, rn: *mut c_int) {

    Rf_rmultinom(n, prob, k, rn)

}

// Cauchy Distribution
pub fn dcauchy(x: cd, lc: cd, sl: cd, lg: c_int) -> cd {
    unsafe { Rf_dcauchy(x, lc, sl, lg) }
}
pub fn pcauchy(x: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pcauchy(x, lc, sl, lt, lg) }
}
pub fn qcauchy(p: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qcauchy(p, lc, sl, lt, lg) }
}
pub unsafe fn rcauchy(lc: cd, sl: cd) -> cd {

    Rf_rcauchy(lc, sl)

}

// Exponential Distribution
pub fn dexp(x: cd, sl: cd, lg: c_int) -> cd {
    unsafe { Rf_dexp(x, sl, lg) }
}
pub fn pexp(x: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pexp(x, sl, lt, lg) }
}
pub fn qexp(p: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qexp(p, sl, lt, lg) }
}
pub unsafe fn rexp(sl: cd) -> cd {

    Rf_rexp(sl)

}

// Geometric Distribution
pub fn dgeom(x: cd, p: cd, lg: c_int) -> cd {
    unsafe { Rf_dgeom(x, p, lg) }
}
pub fn pgeom(x: cd, p: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pgeom(x, p, lt, lg) }
}
pub fn qgeom(p: cd, pb: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qgeom(p, pb, lt, lg) }
}
pub unsafe fn rgeom(p: cd) -> cd {

    Rf_rgeom(p)

}

// Hypergeometric Distibution
pub fn dhyper(x: cd, r: cd, b: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dhyper(x, r, b, n, lg) }
}
pub fn phyper(x: cd, r: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_phyper(x, r, b, n, lt, lg) }
}
pub fn qhyper(p: cd, r: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qhyper(p, r, b, n, lt, lg) }
}
pub unsafe fn rhyper(r: cd, b: cd, n: cd) -> cd {

    Rf_rhyper(r, b, n)

}

// Negative Binomial Distribution
pub fn dnbinom(x: cd, sz: cd, p: cd, lg: c_int) -> cd {
    unsafe { Rf_dnbinom(x, sz, p, lg) }
}
pub fn pnbinom(x: cd, sz: cd, p: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnbinom(x, sz, p, lt, lg) }
}
pub fn qnbinom(p: cd, sz: cd, pb: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnbinom(p, sz, pb, lt, lg) }
}
pub unsafe fn rnbinom(sz: cd, p: cd) -> cd {

    Rf_rnbinom(sz, p)

}



pub fn dnbinom_mu(x: cd, sz: cd, mu: cd, lg: c_int) -> cd {
    unsafe { Rf_dnbinom_mu(x, sz, mu, lg) }
}
pub fn pnbinom_mu(x: cd, sz: cd, mu: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnbinom_mu(x, sz, mu, lt, lg) }
}
pub fn qnbinom_mu(x: cd, sz: cd, mu: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnbinom_mu(x, sz, mu, lt, lg) }
}

pub unsafe fn rnbinom_mu(sz: cd, mu: cd) -> cd {
    {
        Rf_rnbinom_mu(sz, mu)
    }
}


// Poisson Distribution
pub fn dpois(x: cd, lb: cd, lg: c_int) -> cd {
    unsafe { Rf_dpois(x, lb, lg) }
}
pub fn ppois(x: cd, lb: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_ppois(x, lb, lt, lg) }
}
pub fn qpois(p: cd, lb: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qpois(p, lb, lt, lg) }
}
pub unsafe fn rpois(mu: cd) -> cd {

    Rf_rpois(mu)

}

// Weibull Distribution
pub fn dweibull(x: cd, sh: cd, sl: cd, lg: c_int) -> cd {
    unsafe { Rf_dweibull(x, sh, sl, lg) }
}
pub fn pweibull(x: cd, sh: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pweibull(x, sh, sl, lt, lg) }
}
pub fn qweibull(p: cd, sh: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qweibull(p, sh, sl, lt, lg) }
}
pub unsafe fn rweibull(sh: cd, sl: cd) -> cd {

    Rf_rweibull(sh, sl)

}

// Logistic Distribution
pub fn dlogis(x: cd, lc: cd, sl: cd, lg: c_int) -> cd {
    unsafe { Rf_dlogis(x, lc, sl, lg) }
}
pub fn plogis(x: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_plogis(x, lc, sl, lt, lg) }
}
pub fn qlogis(p: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qlogis(p, lc, sl, lt, lg) }
}
pub unsafe fn rlogis(lc: cd, sl: cd) -> cd {

    Rf_rlogis(lc, sl)

}

// Non-central Beta Distribution
pub fn dnbeta(x: cd, a: cd, b: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dnbeta(x, a, b, n, lg) }
}
pub fn pnbeta(x: cd, a: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnbeta(x, a, b, n, lt, lg) }
}
pub fn qnbeta(p: cd, a: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnbeta(p, a, b, n, lt, lg) }
}
// pub  fn rnbeta(a: cd, b: cd, np: cd) -> cd {
//
//     Rf_rnbeta(a, b, np)
//
// }

// Non-central F Distribution
pub fn dnf(x: cd, df1: cd, df2: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dnf(x, df1, df2, n, lg) }
}
pub fn pnf(x: cd, df1: cd, df2: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnf(x, df1, df2, n, lt, lg) }
}
pub fn qnf(p: cd, df1: cd, df2: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnf(p, df1, df2, n, lt, lg) }
}

// Non-central Student t Distribution
pub fn dnt(x: cd, df: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dnt(x, df, n, lg) }
}
pub fn pnt(x: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pnt(x, df, n, lt, lg) }
}
pub fn qnt(p: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qnt(p, df, n, lt, lg) }
}

// Studentized Range Distribution
pub fn ptukey(q: cd, rr: cd, cc: cd, df: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_ptukey(q, rr, cc, df, lt, lg) }
}
pub fn qtukey(p: cd, rr: cd, cc: cd, df: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qtukey(p, rr, cc, df, lt, lg) }
}

// Wilcoxon Rank Sum Distribution
pub fn dwilcox(x: cd, m: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dwilcox(x, m, n, lg) }
}
pub fn pwilcox(q: cd, m: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_pwilcox(q, m, n, lt, lg) }
}
pub fn qwilcox(x: cd, m: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qwilcox(x, m, n, lt, lg) }
}
pub unsafe fn rwilcox(m: cd, n: cd) -> cd {

    Rf_rwilcox(m, n)

}

// Wilcoxon Signed Rank Distribution
pub fn dsignrank(x: cd, n: cd, lg: c_int) -> cd {
    unsafe { Rf_dsignrank(x, n, lg) }
}
pub fn psignrank(x: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_psignrank(x, n, lt, lg) }
}
pub fn qsignrank(x: cd, n: cd, lt: c_int, lg: c_int) -> cd {
    unsafe { Rf_qsignrank(x, n, lt, lg) }
}
pub unsafe fn rsignrank(n: cd) -> cd {

    Rf_rsignrank(n)

}

// Gamma and Related Functions
pub fn gammafn(x: cd) -> cd {
    unsafe { Rf_gammafn(x) }
}
pub fn lgammafn(x: cd) -> cd {
    unsafe { Rf_lgammafn(x) }
}
pub fn lgammafn_sign(x: cd, sgn: *mut c_int) -> cd {
    unsafe { Rf_lgammafn_sign(x, sgn) }
}

pub fn dpsifn(x: cd,
              n: c_int,
              k: c_int,
              m: c_int,
              ans: *mut cd,
              nz: *mut c_int,
              ierr: *mut c_int) {
    unsafe { Rf_dpsifn(x, n, k, m, ans, nz, ierr) }
}
pub fn psigamma(x: cd, deriv: cd) -> cd {
    unsafe { Rf_psigamma(x, deriv) }
}
pub fn digamma(x: cd) -> cd {
    unsafe { Rf_digamma(x) }
}
pub fn trigamma(x: cd) -> cd {
    unsafe { Rf_trigamma(x) }
}
pub fn tetragamma(x: cd) -> cd {
    unsafe { Rf_tetragamma(x) }
}
pub fn pentagamma(x: cd) -> cd {
    unsafe { Rf_pentagamma(x) }
}

pub fn beta(a: cd, b: cd) -> cd {
    unsafe { Rf_beta(a, b) }
}
pub fn lbeta(a: cd, b: cd) -> cd {
    unsafe { Rf_lbeta(a, b) }
}

pub fn choose(n: cd, k: cd) -> cd {
    unsafe { Rf_choose(n, k) }
}
pub fn lchoose(n: cd, k: cd) -> cd {
    unsafe { Rf_lchoose(n, k) }
}

// Bessel Functions
pub fn bessel_i(x: cd, al: cd, ex: cd) -> cd {
    unsafe { Rf_bessel_i(x, al, ex) }
}
pub fn bessel_j(x: cd, al: cd) -> cd {
    unsafe { Rf_bessel_j(x, al) }
}
pub fn bessel_k(x: cd, al: cd, ex: cd) -> cd {
    unsafe { Rf_bessel_k(x, al, ex) }
}
pub fn bessel_y(x: cd, al: cd) -> cd {
    unsafe { Rf_bessel_y(x, al) }
}
pub fn bessel_i_ex(x: cd, al: cd, ex: cd, bi: *mut cd) -> cd {
    unsafe { Rf_bessel_i_ex(x, al, ex, bi) }
}
pub fn bessel_j_ex(x: cd, al: cd, bj: *mut cd) -> cd {
    unsafe { Rf_bessel_j_ex(x, al, bj) }
}
pub fn bessel_k_ex(x: cd, al: cd, ex: cd, bk: *mut cd) -> cd {
    unsafe { Rf_bessel_k_ex(x, al, ex, bk) }
}
pub fn bessel_y_ex(x: cd, al: cd, by: *mut cd) -> cd {
    unsafe { Rf_bessel_y_ex(x, al, by) }
}


// General Support Functions
// #ifndef HAVE_HYPOT
// pub unsafe fn hypot(a: cd, b: cd) -> cd {
//
//         Rf_hypot(a, b)
//
// }
// #endif

// pub unsafe fn pythag(a: cd, b: cd) -> cd {
//
//     Rf_pythag(a, b)
//
// }

// #ifndef HAVE_EXPM1
// pub unsafe fn expm1(x: cd) -> cd /* = exp(x)-1 {care for small x} */ {
//
//         Rf_expm1(x)
//
// }
// #endif

// #ifndef HAVE_LOG1P
// pub unsafe fn log1p(x: cd) -> cd /* = log(1+x) {care for small x} */ {
//
//         Rf_log1p(x)
//
// }
// #endif

pub fn imax2(x: c_int, y: c_int) -> c_int {
    unsafe { Rf_imax2(x, y) }
}
pub fn imin2(x: c_int, y: c_int) -> c_int {
    unsafe { Rf_imin2(x, y) }
}

pub fn fmax2(x: cd, y: cd) -> cd {
    unsafe { Rf_fmax2(x, y) }
}
pub fn fmin2(x: cd, y: cd) -> cd {
    unsafe { Rf_fmin2(x, y) }
}
pub fn sign(x: cd) -> cd {
    unsafe { Rf_sign(x) }
}
pub fn fprec(x: cd, dg: cd) -> cd {
    unsafe { Rf_fprec(x, dg) }
}
pub fn fround(x: cd, dg: cd) -> cd {
    unsafe { Rf_fround(x, dg) }
}
pub fn fsign(x: cd, y: cd) -> cd {
    unsafe { Rf_fsign(x, y) }
}
pub fn ftrunc(x: cd) -> cd {
    unsafe { Rf_ftrunc(x) }
}
