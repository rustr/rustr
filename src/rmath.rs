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

    return ::rdll::norm_rand();

}
pub unsafe fn unif_rand() -> cd {

    return ::rdll::unif_rand();

}
pub unsafe fn exp_rand() -> cd {

    return ::rdll::exp_rand();

}

// Normal Distribution
pub unsafe fn dnorm(x: cd, mu: cd, sigma: cd, lg: c_int) -> cd {

    return Rf_dnorm4(x, mu, sigma, lg);

}
pub unsafe fn pnorm(x: cd, mu: cd, sigma: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnorm5(x, mu, sigma, lt, lg);

}
pub unsafe fn qnorm(p: cd, mu: cd, sigma: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnorm5(p, mu, sigma, lt, lg);

}
pub unsafe fn rnorm(mu: cd, sigma: cd) -> cd {

    return Rf_rnorm(mu, sigma);

}

pub unsafe fn pnorm_both(x: cd, cum: *mut cd, ccum: *mut cd, lt: c_int, lg: c_int) {

    return Rf_pnorm_both(x, cum, ccum, lt, lg);

}

// Uniform Distribution
pub unsafe fn dunif(x: cd, a: cd, b: cd, lg: c_int) -> cd {

    return Rf_dunif(x, a, b, lg);

}
pub unsafe fn punif(x: cd, a: cd, b: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_punif(x, a, b, lt, lg);

}
pub unsafe fn qunif(p: cd, a: cd, b: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qunif(p, a, b, lt, lg);

}
pub unsafe fn runif(a: cd, b: cd) -> cd {

    return Rf_runif(a, b);

}

// Gamma Distribution
pub unsafe fn dgamma(x: cd, shp: cd, scl: cd, lg: c_int) -> cd {

    return Rf_dgamma(x, shp, scl, lg);

}
pub unsafe fn pgamma(x: cd, al: cd, scl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pgamma(x, al, scl, lt, lg);

}
pub unsafe fn qgamma(p: cd, al: cd, scl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qgamma(p, al, scl, lt, lg);

}
pub unsafe fn rgamma(a: cd, scl: cd) -> cd {

    return Rf_rgamma(a, scl);

}

pub unsafe fn log1pmx(x: cd) -> cd {

    return Rf_log1pmx(x);

}
// pub unsafe fn rf_log1pexp(x: cd) -> cd {
//
//    return log1pexp(x);
//
// }  // <-- ../nmath/plogis.c
pub unsafe fn lgamma1p(a: cd) -> cd {

    return Rf_lgamma1p(a);

}
pub unsafe fn logspace_add(lx: cd, ly: cd) -> cd {

    return Rf_logspace_add(lx, ly);

}
pub unsafe fn logspace_sub(lx: cd, ly: cd) -> cd {

    return Rf_logspace_sub(lx, ly);

}

// Beta Distribution
pub unsafe fn dbeta(x: cd, a: cd, b: cd, lg: c_int) -> cd {

    return Rf_dbeta(x, a, b, lg);

}
pub unsafe fn pbeta(x: cd, p: cd, q: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pbeta(x, p, q, lt, lg);

}
pub unsafe fn qbeta(a: cd, p: cd, q: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qbeta(a, p, q, lt, lg);

}
pub unsafe fn rbeta(a: cd, b: cd) -> cd {

    return Rf_rbeta(a, b);

}

// Lognormal Distribution
pub unsafe fn dlnorm(x: cd, ml: cd, sl: cd, lg: c_int) -> cd {

    return Rf_dlnorm(x, ml, sl, lg);

}
pub unsafe fn plnorm(x: cd, ml: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_plnorm(x, ml, sl, lt, lg);

}
pub unsafe fn qlnorm(p: cd, ml: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qlnorm(p, ml, sl, lt, lg);

}
pub unsafe fn rlnorm(ml: cd, sl: cd) -> cd {

    return Rf_rlnorm(ml, sl);

}

// Chi-squared Distribution
pub unsafe fn dchisq(x: cd, df: cd, lg: c_int) -> cd {

    return Rf_dchisq(x, df, lg);

}
pub unsafe fn pchisq(x: cd, df: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pchisq(x, df, lt, lg);

}
pub unsafe fn qchisq(p: cd, df: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qchisq(p, df, lt, lg);

}
pub unsafe fn rchisq(df: cd) -> cd {

    return Rf_rchisq(df);

}

// Non-central Chi-squared Distribution
pub unsafe fn dnchisq(x: cd, df: cd, n: cd, lg: c_int) -> cd {

    return Rf_dnchisq(x, df, n, lg);

}
pub unsafe fn pnchisq(x: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnchisq(x, df, n, lt, lg);

}
pub unsafe fn qnchisq(p: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnchisq(p, df, n, lt, lg);

}
pub unsafe fn rnchisq(df: cd, lb: cd) -> cd {

    return Rf_rnchisq(df, lb);

}

// F Distibution
pub unsafe fn df(x: cd, df1: cd, df2: cd, lg: c_int) -> cd {

    return Rf_df(x, df1, df2, lg);

}
pub unsafe fn pf(x: cd, df1: cd, df2: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pf(x, df1, df2, lt, lg);

}
pub unsafe fn qf(p: cd, df1: cd, df2: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qf(p, df1, df2, lt, lg);

}
pub unsafe fn rf(df1: cd, df2: cd) -> cd {

    return Rf_rf(df1, df2);

}

// Student t Distibution
pub unsafe fn dt(x: cd, n: cd, lg: c_int) -> cd {

    return Rf_dt(x, n, lg);

}
pub unsafe fn pt(x: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pt(x, n, lt, lg);

}
pub unsafe fn qt(p: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qt(p, n, lt, lg);

}
pub unsafe fn rt(n: cd) -> cd {

    return Rf_rt(n);

}

// Binomial Distribution
pub unsafe fn dbinom(x: cd, n: cd, p: cd, lg: c_int) -> cd {

    return Rf_dbinom(x, n, p, lg);

}
pub unsafe fn pbinom(x: cd, n: cd, p: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pbinom(x, n, p, lt, lg);

}
pub unsafe fn qbinom(p: cd, n: cd, m: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qbinom(p, n, m, lt, lg);

}
pub unsafe fn rbinom(n: cd, p: cd) -> cd {

    return Rf_rbinom(n, p);

}

// Multnomial Distribution
pub unsafe fn rmultinom(n: c_int, prob: *mut cd, k: c_int, rn: *mut c_int) {

    return Rf_rmultinom(n, prob, k, rn);

}

// Cauchy Distribution
pub unsafe fn dcauchy(x: cd, lc: cd, sl: cd, lg: c_int) -> cd {

    return Rf_dcauchy(x, lc, sl, lg);

}
pub unsafe fn pcauchy(x: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pcauchy(x, lc, sl, lt, lg);

}
pub unsafe fn qcauchy(p: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qcauchy(p, lc, sl, lt, lg);

}
pub unsafe fn rcauchy(lc: cd, sl: cd) -> cd {

    return Rf_rcauchy(lc, sl);

}

// Exponential Distribution
pub unsafe fn dexp(x: cd, sl: cd, lg: c_int) -> cd {

    return Rf_dexp(x, sl, lg);

}
pub unsafe fn pexp(x: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pexp(x, sl, lt, lg);

}
pub unsafe fn qexp(p: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qexp(p, sl, lt, lg);

}
pub unsafe fn rexp(sl: cd) -> cd {

    return Rf_rexp(sl);

}

// Geometric Distribution
pub unsafe fn dgeom(x: cd, p: cd, lg: c_int) -> cd {

    return Rf_dgeom(x, p, lg);

}
pub unsafe fn pgeom(x: cd, p: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pgeom(x, p, lt, lg);

}
pub unsafe fn qgeom(p: cd, pb: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qgeom(p, pb, lt, lg);

}
pub unsafe fn rgeom(p: cd) -> cd {

    return Rf_rgeom(p);

}

// Hypergeometric Distibution
pub unsafe fn dhyper(x: cd, r: cd, b: cd, n: cd, lg: c_int) -> cd {

    return Rf_dhyper(x, r, b, n, lg);

}
pub unsafe fn phyper(x: cd, r: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_phyper(x, r, b, n, lt, lg);

}
pub unsafe fn qhyper(p: cd, r: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qhyper(p, r, b, n, lt, lg);

}
pub unsafe fn rhyper(r: cd, b: cd, n: cd) -> cd {

    return Rf_rhyper(r, b, n);

}

// Negative Binomial Distribution
pub unsafe fn dnbinom(x: cd, sz: cd, p: cd, lg: c_int) -> cd {

    return Rf_dnbinom(x, sz, p, lg);

}
pub unsafe fn pnbinom(x: cd, sz: cd, p: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnbinom(x, sz, p, lt, lg);

}
pub unsafe fn qnbinom(p: cd, sz: cd, pb: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnbinom(p, sz, pb, lt, lg);

}
pub unsafe fn rnbinom(sz: cd, p: cd) -> cd {

    return Rf_rnbinom(sz, p);

}



pub unsafe fn dnbinom_mu(x: cd, sz: cd, mu: cd, lg: c_int) -> cd {

    return Rf_dnbinom_mu(x, sz, mu, lg);

}
pub unsafe fn pnbinom_mu(x: cd, sz: cd, mu: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnbinom_mu(x, sz, mu, lt, lg);

}
pub unsafe fn qnbinom_mu(x: cd, sz: cd, mu: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnbinom_mu(x, sz, mu, lt, lg);

}

// inline r :cdnbinom_mu(sz :cd, mu: cd)				{ unsafe{ return Rf_rnbinom_mu(sz, mu); } }


// Poisson Distribution
pub unsafe fn dpois(x: cd, lb: cd, lg: c_int) -> cd {

    return Rf_dpois(x, lb, lg);

}
pub unsafe fn ppois(x: cd, lb: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_ppois(x, lb, lt, lg);

}
pub unsafe fn qpois(p: cd, lb: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qpois(p, lb, lt, lg);

}
pub unsafe fn rpois(mu: cd) -> cd {

    return Rf_rpois(mu);

}

// Weibull Distribution
pub unsafe fn dweibull(x: cd, sh: cd, sl: cd, lg: c_int) -> cd {

    return Rf_dweibull(x, sh, sl, lg);

}
pub unsafe fn pweibull(x: cd, sh: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pweibull(x, sh, sl, lt, lg);

}
pub unsafe fn qweibull(p: cd, sh: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qweibull(p, sh, sl, lt, lg);

}
pub unsafe fn rweibull(sh: cd, sl: cd) -> cd {

    return Rf_rweibull(sh, sl);

}

// Logistic Distribution
pub unsafe fn dlogis(x: cd, lc: cd, sl: cd, lg: c_int) -> cd {

    return Rf_dlogis(x, lc, sl, lg);

}
pub unsafe fn plogis(x: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_plogis(x, lc, sl, lt, lg);

}
pub unsafe fn qlogis(p: cd, lc: cd, sl: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qlogis(p, lc, sl, lt, lg);

}
pub unsafe fn rlogis(lc: cd, sl: cd) -> cd {

    return Rf_rlogis(lc, sl);

}

// Non-central Beta Distribution
pub unsafe fn dnbeta(x: cd, a: cd, b: cd, n: cd, lg: c_int) -> cd {

    return Rf_dnbeta(x, a, b, n, lg);

}
pub unsafe fn pnbeta(x: cd, a: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnbeta(x, a, b, n, lt, lg);

}
pub unsafe fn qnbeta(p: cd, a: cd, b: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnbeta(p, a, b, n, lt, lg);

}
// pub unsafe fn rnbeta(a: cd, b: cd, np: cd) -> cd {
//
//    return Rf_rnbeta(a, b, np);
//
// }

// Non-central F Distribution
pub unsafe fn dnf(x: cd, df1: cd, df2: cd, n: cd, lg: c_int) -> cd {

    return Rf_dnf(x, df1, df2, n, lg);

}
pub unsafe fn pnf(x: cd, df1: cd, df2: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnf(x, df1, df2, n, lt, lg);

}
pub unsafe fn qnf(p: cd, df1: cd, df2: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnf(p, df1, df2, n, lt, lg);

}

// Non-central Student t Distribution
pub unsafe fn dnt(x: cd, df: cd, n: cd, lg: c_int) -> cd {

    return Rf_dnt(x, df, n, lg);

}
pub unsafe fn pnt(x: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pnt(x, df, n, lt, lg);

}
pub unsafe fn qnt(p: cd, df: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qnt(p, df, n, lt, lg);

}

// Studentized Range Distribution
pub unsafe fn ptukey(q: cd, rr: cd, cc: cd, df: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_ptukey(q, rr, cc, df, lt, lg);

}
pub unsafe fn qtukey(p: cd, rr: cd, cc: cd, df: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qtukey(p, rr, cc, df, lt, lg);

}

// Wilcoxon Rank Sum Distribution
pub unsafe fn dwilcox(x: cd, m: cd, n: cd, lg: c_int) -> cd {

    return Rf_dwilcox(x, m, n, lg);

}
pub unsafe fn pwilcox(q: cd, m: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_pwilcox(q, m, n, lt, lg);

}
pub unsafe fn qwilcox(x: cd, m: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qwilcox(x, m, n, lt, lg);

}
pub unsafe fn rwilcox(m: cd, n: cd) -> cd {

    return Rf_rwilcox(m, n);

}

// Wilcoxon Signed Rank Distribution
pub unsafe fn dsignrank(x: cd, n: cd, lg: c_int) -> cd {

    return Rf_dsignrank(x, n, lg);

}
pub unsafe fn psignrank(x: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_psignrank(x, n, lt, lg);

}
pub unsafe fn qsignrank(x: cd, n: cd, lt: c_int, lg: c_int) -> cd {

    return Rf_qsignrank(x, n, lt, lg);

}
pub unsafe fn rsignrank(n: cd) -> cd {

    return Rf_rsignrank(n);

}

// Gamma and Related Functions
pub unsafe fn gammafn(x: cd) -> cd {

    return Rf_gammafn(x);

}
pub unsafe fn lgammafn(x: cd) -> cd {

    return Rf_lgammafn(x);

}
pub unsafe fn lgammafn_sign(x: cd, sgn: *mut c_int) -> cd {

    return Rf_lgammafn_sign(x, sgn);

}

pub unsafe fn dpsifn(x: cd,
                     n: c_int,
                     k: c_int,
                     m: c_int,
                     ans: *mut cd,
                     nz: *mut c_int,
                     ierr: *mut c_int) {

    return Rf_dpsifn(x, n, k, m, ans, nz, ierr);

}
pub unsafe fn psigamma(x: cd, deriv: cd) -> cd {

    return Rf_psigamma(x, deriv);

}
pub unsafe fn digamma(x: cd) -> cd {

    return Rf_digamma(x);

}
pub unsafe fn trigamma(x: cd) -> cd {

    return Rf_trigamma(x);

}
pub unsafe fn tetragamma(x: cd) -> cd {

    return Rf_tetragamma(x);

}
pub unsafe fn pentagamma(x: cd) -> cd {

    return Rf_pentagamma(x);

}

pub unsafe fn beta(a: cd, b: cd) -> cd {

    return Rf_beta(a, b);

}
pub unsafe fn lbeta(a: cd, b: cd) -> cd {

    return Rf_lbeta(a, b);

}

pub unsafe fn choose(n: cd, k: cd) -> cd {

    return Rf_choose(n, k);

}
pub unsafe fn lchoose(n: cd, k: cd) -> cd {

    return Rf_lchoose(n, k);

}

// Bessel Functions
pub unsafe fn bessel_i(x: cd, al: cd, ex: cd) -> cd {

    return Rf_bessel_i(x, al, ex);

}
pub unsafe fn bessel_j(x: cd, al: cd) -> cd {

    return Rf_bessel_j(x, al);

}
pub unsafe fn bessel_k(x: cd, al: cd, ex: cd) -> cd {

    return Rf_bessel_k(x, al, ex);

}
pub unsafe fn bessel_y(x: cd, al: cd) -> cd {

    return Rf_bessel_y(x, al);

}
pub unsafe fn bessel_i_ex(x: cd, al: cd, ex: cd, bi: *mut cd) -> cd {

    return Rf_bessel_i_ex(x, al, ex, bi);

}
pub unsafe fn bessel_j_ex(x: cd, al: cd, bj: *mut cd) -> cd {

    return Rf_bessel_j_ex(x, al, bj);

}
pub unsafe fn bessel_k_ex(x: cd, al: cd, ex: cd, bk: *mut cd) -> cd {

    return Rf_bessel_k_ex(x, al, ex, bk);

}
pub unsafe fn bessel_y_ex(x: cd, al: cd, by: *mut cd) -> cd {

    return Rf_bessel_y_ex(x, al, by);

}


// General Support Functions
// #ifndef HAVE_HYPOT
// pub unsafe fn hypot(a: cd, b: cd) -> cd {
//
//        return Rf_hypot(a, b);
//
// }
// #endif

// pub unsafe fn pythag(a: cd, b: cd) -> cd {
//
//    return Rf_pythag(a, b);
//
// }

// #ifndef HAVE_EXPM1
// pub unsafe fn expm1(x: cd) -> cd /* = exp(x)-1 {care for small x} */ {
//
//        return Rf_expm1(x);
//
// }
// #endif

// #ifndef HAVE_LOG1P
// pub unsafe fn log1p(x: cd) -> cd /* = log(1+x) {care for small x} */ {
//
//        return Rf_log1p(x);
//
// }
// #endif

pub unsafe fn imax2(x: c_int, y: c_int) -> c_int {

    return Rf_imax2(x, y);

}
pub unsafe fn imin2(x: c_int, y: c_int) -> c_int {

    return Rf_imin2(x, y);

}

pub unsafe fn fmax2(x: cd, y: cd) -> cd {

    return Rf_fmax2(x, y);

}
pub unsafe fn fmin2(x: cd, y: cd) -> cd {

    return Rf_fmin2(x, y);

}
pub unsafe fn sign(x: cd) -> cd {

    return Rf_sign(x);

}
pub unsafe fn fprec(x: cd, dg: cd) -> cd {

    return Rf_fprec(x, dg);

}
pub unsafe fn fround(x: cd, dg: cd) -> cd {

    return Rf_fround(x, dg);

}
pub unsafe fn fsign(x: cd, y: cd) -> cd {

    return Rf_fsign(x, y);

}
pub unsafe fn ftrunc(x: cd) -> cd {

    return Rf_ftrunc(x);

}
