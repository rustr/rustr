//! R pairlist methods
//!
//!

use rdll::*;
use traits::*;
use error::*;
use protect::stackp::*;
use rtype::*;
use util::rprint as rp;

pub fn pairlist(xs: &[&Args]) -> RResult<SEXP> {
    let mut res: Shield = Shield::new(unsafe { R_NilValue });
    let mut it = xs.iter().rev();
    let head: &Args = match it.next() {
        None => return rraise("need at list one input args"),
        Some(x) => *x,
    };
    unsafe {
        if head.named() {
            let y = Shield::new(head.s());
            let x = Shield::new(Rf_cons(y.s(), res.s()));
            SET_TAG(x.s(), head.name().s());
            res = x;
        } else {
            res = Shield::new(Rf_cons(Shield::new(head.s()).s(), res.s()));
        }
    }
    for nhead in it {
        unsafe {
            if nhead.named() {
                let y = Shield::new(nhead.s());
                let x = Shield::new(Rf_cons(y.s(), res.s()));
                SET_TAG(x.s(), nhead.name().s());
                res = x;
            } else {
                res = Shield::new(Rf_cons(Shield::new(nhead.s()).s(), res.s()));
            }
        }
    }
    Ok(unsafe { res.s() })

}

pub fn language(xs: &[&Args]) -> RResult<SEXP> {
    let call = Shield::new(try!(pairlist(xs)));
    unsafe {
        SET_TAG(call.s(), R_NilValue);
        SET_TYPEOF(call.s(), LANGSXP as ::std::os::raw::c_int);
        Ok(call.s())
    }
}

pub fn pairlist1(hd: &Args, xs: &[&Args]) -> RResult<SEXP> {
    let mut res: Shield = Shield::new(unsafe { Rf_cons(hd.s(), R_NilValue) });
    let mut it = xs.iter().rev();
    let head: &Args = match it.next() {
        None => return Ok(unsafe { res.s() }),
        Some(x) => *x,
    };
    unsafe {
        if head.named() {
            let y = Shield::new(head.s());
            let x = Shield::new(Rf_cons(y.s(), res.s()));
            SET_TAG(x.s(), head.name().s());
            res = x;
            rp(res.s());
        } else {
            res = Shield::new(Rf_cons(Shield::new(head.s()).s(), res.s()));
            rp(res.s());
        }
    }
    for nhead in it {
        unsafe {
            if nhead.named() {
                let y = Shield::new(nhead.s());
                let x = Shield::new(Rf_cons(y.s(), res.s()));
                SET_TAG(x.s(), nhead.name().s());
                res = x;
                rp(res.s());
            } else {
                rp(nhead.s());
                res = Shield::new(Rf_cons(Shield::new(nhead.s()).s(), res.s()));
                rp(res.s());
            }
        }
    }
    Ok(unsafe { res.s() })

}

pub fn language1(hd: &Args, xs: &[&Args]) -> RResult<SEXP> {
    let call = Shield::new(unsafe { Rf_lcons(hd.s(), try!(pairlist(xs))) });
    unsafe {
        SET_TAG(call.s(), R_NilValue);
        SET_TYPEOF(call.s(), LANGSXP as ::std::os::raw::c_int);
        Ok(call.s())
    }
}
