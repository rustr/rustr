//! R Enviroment type
//!
//!


use ::rdll::*;
use ::storage::*;
use ::traits::*;
use ::rtype::*;
use ::error::*;
use std::convert::*;

use ::util::*;

use eval::*;
use ::symbol::*;
use ::protect::stackp::*;

pub type Envir = EnvirM<Preserve>;
pub type EnvirN = EnvirM<NoProtect>;

use std::str::FromStr;

gen_traits_sexp!(EnvirM);

impl<T: SEXPbucket>  FromStr for EnvirM<T> {
	type Err = RError;
    fn from_str(name: &str) -> RResult<EnvirM<T>> {
        unsafe {

            // similar to matchEnvir@envir.c
            if name == ".GlobalEnv" {
                Ok(EnvirM { data: T::new(R_GlobalEnv) })
            } else if name == "package:base" {
                Ok(EnvirM { data: T::new(R_BaseEnv) })
            } else {
                let as_environment_sym = Rf_install(c_str("as.environment").as_ptr());
                let res = rustr_eval(Rf_lang2(as_environment_sym,
                                              Rf_mkString(c_str(name).as_ptr())),
                                     R_GlobalEnv);
                match res {
                    Ok(aa) => Ok(EnvirM { data: T::new(aa) }),
                    Err(_) => {
                        rraise("fail to create enviroment")
                    } 
                }
            }
        }
    }
}

impl<T: SEXPbucket> EnvirM<T> {
    pub fn as_envir<TT: ToSEXP>(s: TT) -> RResult<EnvirM<T>> {
        unsafe {
            if RTYPEOF(s.s()) == ENVSXP {
                return Ok(EnvirM { data: T::new(s.s()) });
            }
            return Ok(EnvirM {
                data: T::new(try!(rustr_eval(Rf_lang2(Rf_install(c_str("as.environment")
                                                                     .as_ptr()),
                                                      s.s()),
                                             R_GlobalEnv))),
            });
        }
    }
    pub unsafe fn from_sexp_envir(x: SEXP) -> EnvirM<T> {
        EnvirM { data: T::new(x) }
    }
    pub fn set<E: ToSEXP>(&mut self, x: E) -> RResult<()> {
        match EnvirM::<T>::as_envir(unsafe { x.s() }) {
            Ok(res) => {
                self.data.set(unsafe { res.s() });
                Ok(())
            }
            Err(e) => rraise(e),
        }
    }

    pub fn new<E: ToSEXP>(&mut self, x: E) -> RResult<EnvirM<T>> {
        match EnvirM::<T>::as_envir(unsafe { x.s() }) {
            Ok(res) => Ok(EnvirM { data: unsafe { T::new(res.s()) } }),
            Err(e) => rraise(e),
        }
    }

    pub fn from_pos(pos: ::std::os::raw::c_int) -> RResult<EnvirM<T>> {
        unsafe {
            let as_environment_sym = Rf_install(c_str("as.environment").as_ptr());
            let res = rustr_eval(Rf_lang2(as_environment_sym, Rf_ScalarInteger(pos)),
                                 R_GlobalEnv);
            match res {
                Ok(aa) => Ok(EnvirM { data: T::new(aa) }),
                Err(e) => {
                    return rraise(e);
                } 
            }
        }
    }
    /** 
         * Indicates if this is a user defined database.
         */
    fn is_user_database(&self) -> bool {
        unsafe {
            return OBJECT(self.data.s()) == 1 &&
                   Rf_inherits(self.data.s(), c_str("UserDefinedDatabase").as_ptr()) ==
                   Rboolean::TRUE;
        }
    }
    pub fn global() -> EnvirM<NoProtect> {
        unsafe {
            return EnvirM { data: NoProtect::new(R_GlobalEnv) };
        }
    }

    pub fn empty() -> EnvirM<T> {
        unsafe {
            EnvirM { data: T::new(R_EmptyEnv) }
        }
    }

    pub fn base() -> EnvirM<T> {
        unsafe {
            EnvirM { data: T::new(R_BaseEnv) }
        }
    }

    pub fn base_namespace() -> EnvirM<T> {
        unsafe {
            EnvirM { data: T::new(R_BaseNamespace) }
        }
    }
    pub fn namespace_env(package: &str) -> RResult<EnvirM<T>> {
        unsafe {
            let get_namespace_sym = Rf_install(c_str("getNamespace").as_ptr());
            let env = rustr_eval(Rf_lang2(get_namespace_sym, Rf_mkString(c_str(package).as_ptr())),
                                 R_GlobalEnv);
            match env {
                Ok(aa) => Ok(EnvirM { data: T::new(aa) }),
                Err(e) => {
                    rraise(e)
                } 
            }
        }

    }
    pub fn parent(&self) -> EnvirM<T> {
        unsafe {
            EnvirM { data: T::new(ENCLOS(self.data.s())) }
        }
    }

    /**
         * creates a new environment whose this is the parent
         */
    pub fn new_child(&self, hashed: ::std::os::raw::c_int) -> RResult<EnvirM<T>> {
        unsafe {
            let new_env_sym = cstr_sym("new.env");
            let sexp = try!(rustr_eval(Rf_lang3(new_env_sym,
                                                Rf_ScalarLogical(hashed),
                                                self.data.s()),
                                       R_GlobalEnv));

            return Ok(EnvirM { data: T::new(sexp) });
        }
    }
    pub fn ls<D: RNew>(&self, all: Rboolean) -> RResult<D> {

        unsafe {
            if self.is_user_database() {
                let tb: *mut R_ObjectTable =
                    ::std::mem::transmute(R_ExternalPtrAddr(HASHTAB(self.data.s())));
                match (*tb).objects {
                    Some(resfn) => return D::rnew(resfn(tb)),
                    None => return rraise("ls() R_ExternalPtrAddr is invalid."),
                }
            } else {

                return D::rnew(R_lsInternal(self.data.s(), all));
            }
        }
    }
    pub fn get<D: RNew>(&self, name: &str) -> RResult<D> {
        unsafe {
            let res = Rf_findVarInFrame(self.data.s(), Symbol::from(name).s());

            if res == R_UnboundValue {
                return D::rnew(R_NilValue);
            }

            // We need to evaluate if it is a promise
            if RTYPEOF(res) == PROMSXP {
                let res2 = Rf_eval(res, self.data.s());
                return D::rnew(res2);
            }
            D::rnew(res)
        }
    }
    pub fn find<D: RNew>(&self, name: &str) -> RResult<D> {
        unsafe {
            let res = Rf_findVar(Symbol::from(name).s(), self.data.s());

            if res == R_UnboundValue {
                return rraise(format!("binding not found: {:?}", name));
            }
            // We need to evaluate if it is a promise
            if RTYPEOF(res) == PROMSXP {
                let res2 = Rf_eval(res, self.data.s());
                return D::rnew(res2);
            }

            return D::rnew(res);
        }

    }

    pub fn exists(&self, name: &str) -> bool {
        unsafe {
            let res = Rf_findVarInFrame(self.data.s(), Symbol::from(name).s());
            return res != R_UnboundValue;
        }
    }
    #[allow(non_snake_case)]
    pub fn bindingIsLocked(&self, name: &str) -> RResult<bool> {
        if !self.exists(name) {
            return rraise(format!("no such binding: {:?}", name));
        }
        unsafe {
            return Ok(R_BindingIsLocked(Symbol::from(name).s(), self.data.s()) == Rboolean::TRUE);
        }

    }

    pub fn assign<E: ToSEXP>(&self, name: &str, x: E) -> RResult<()> {

        match self.bindingIsLocked(name) {
            Ok(xx) => {
                if self.exists(name) && xx {
                    return rraise(format!("binding_is_locked: {:?}", name));
                }
            }
            Err(_) => {
                return rraise(format!("no such binding: {:?}", name));
            }
        }
        unsafe {
            let name_sym = Rf_install(c_str(name).as_ptr());

            Rf_defineVar(name_sym, x.s(), self.data.s());
            return Ok(());
        }

    }
    pub fn is_locked(&self) -> bool {
        unsafe {
            return R_EnvironmentIsLocked(self.data.s()) == Rboolean::TRUE;
        }
    }

    pub fn binding_is_active(&self, name: &str) -> RResult<bool> {
        if !self.exists(name) {
            return rraise(format!("no such binding: {:?}", name));
        }
        unsafe {
            return Ok(R_BindingIsActive(Symbol::from(name).s(), self.data.s()) == Rboolean::TRUE);

        }
    }
    pub fn lock(&self, bindings: Rboolean) {
        unsafe {
            R_LockEnvironment(self.data.s(), bindings);
        }
    }

    pub fn lock_binding(&self, name: &str) -> RResult<()> {
        if !self.exists(name) {
            return rraise(format!("no such binding: {:?}", name));
        }
        unsafe {
            R_LockBinding(Symbol::from(name).s(), self.data.s());
            return Ok(());
        }

    }

    pub fn unlock_binding(&self, name: &str) -> RResult<()> {
        if !self.exists(name) {
            return rraise(format!("no such binding: {:?}", name));
        }
        unsafe {
            R_unLockBinding(Symbol::from(name).s(), self.data.s());
            return Ok(());
        }
    }
    /**
         * remove an object from this environment
         */
    pub fn remove(&self, name: &str) -> RResult<bool> {
        if self.exists(name) {
            let res1 = match self.bindingIsLocked(name) {
                Ok(aa) => aa,
                Err(_) => {
                    return rraise(format!("no such binding: {:?}", name));
                }
            };
            if res1 {
                return rraise(format!("binding is locked: {}", name));
            } else {
                unsafe {
                    // unless we want to copy all of do_remove,
                    // we have to go back to R to do this operation
                    let internal_sym = Rf_install(c_str(".Internal").as_ptr());
                    let remove_sym = Rf_install(c_str("remove").as_ptr());
                    let call = Shield::new(Rf_lang2(internal_sym,
                                                    Rf_lang4(remove_sym,
                                                             Rf_mkString(c_str(name).as_ptr()),
                                                             self.data.s(),
                                                             Rf_ScalarLogical(0))));
                    Rf_eval(call.s(), R_GlobalEnv);
                }

            }
        } else {
            return rraise(format!("no such binding: {:?}", name));
        }
        return Ok(true);

    }
}
