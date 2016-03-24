use rdll::*;
use environment::*;

use std::ffi::*;
use std::os::raw::*;
use error::*;
use std::env;
use rtype::*;
use Enum_Unnamed37::*;
use util::*;
use dll::*;
use traits::*;
use error::REKind::*;

pub static mut REng:bool = false;

pub struct REngine {
    verbose_m: bool, // constructor, or setter
    global_env: EnvirN,
    code: String,
}

impl REngine {
    pub fn init() -> RResult<Self> {
        let argv: Vec<CString> = vec![CString::new("REmbeddedPostgres").unwrap(),
                                      CString::new("--gui=none").unwrap(),
                                      CString::new("--no-save").unwrap(),
									  CString::new("--no-readline").unwrap(),
                                      CString::new("--silent").unwrap(),
                                      CString::new("--vanilla").unwrap(),
                                      CString::new("--slave").unwrap(),
                                      ];
        REngine::initialize(argv, false, false)

    }
    pub fn initialize(args: Vec<CString>, verbose: bool, interactive: bool) -> RResult<Self> {
        let dd = args.len();
        unsafe {
            if REng {
                return rerror(REKind::BindingIsLocked("there is a runing R engine".into()));
            }
        }
        let args: Vec<*mut c_char> = args.iter()
                                         .map(|arg| arg.as_ptr() as *mut c_char)
                                         .collect();
        // let rr: Vec<(String,String)> = std::env::vars().collect();
        if cfg!(not(target_os = "windows")) {
            unsafe {
                R_SignalHandlers = 0;
            }
        }

        // init_tempdir?

        if let Err(_) = env::var("R_HOME") {

            if cfg!(target_os= "windows"){

            unsafe {

                let xs: String = try!(CStr::from_ptr(get_R_HOME()).to_owned().into_string());

                env::set_var("R_HOME", &xs)

            }

        	} else{
        		panic!("can not find R_HOME")
        	}
        }
        let d = unsafe { Rf_initEmbeddedR(dd as c_int, args.as_ptr() as *mut *mut c_char) };

//        if cfg!(unix) {
//            unsafe {
//                R_CStackLimit = ::std::os::raw::c_ulong::max_value();
//            }
//        }

        if d == 0 {
            return rraise("can not create REngine.");
        }

        unsafe {
            R_ReplDLLinit();
        } // populate the repl console buffers



        let mut rst = structRstart::default();
        let startr: *mut structRstart = &mut rst;
        unsafe {
            R_DefParams(startr);
        }

        rst.R_Interactive = if interactive {
            Rboolean::TRUE
        } else {
            Rboolean::FALSE
        };       // sets interactive() to eval to false
        rst.R_Verbose = if verbose {
            Rboolean::TRUE
        } else {
            Rboolean::FALSE
        };

        unsafe {
            R_SetParams(startr);
        }
        let envir = EnvirN::global();

        unsafe {
            REng = true;
        }
        Ok(REngine {
            verbose_m: verbose,
            global_env: envir,
            code: "".into(),
        })
    }
    pub fn eval<D: RNew>(&mut self, code: &str) -> RResult<D> {
        let mut ans: SEXP = unsafe { R_NilValue };
        let mut status: Enum_Unnamed37 = Enum_Unnamed37::PARSE_OK;
        let mut error_occurred: c_int = 0;
        unsafe {
            self.code = code.into();
            let cmd_sexp = Rf_allocVector(STRSXP, 1);
            Rf_protect(cmd_sexp);

            SET_STRING_ELT(cmd_sexp, 0, Rf_mkChar(c_str(code).as_ptr()));
            let s_ptr: *mut ParseStatus = &mut status;
            let cmdexpr = Rf_protect(R_ParseVector(cmd_sexp, -1, s_ptr, R_NilValue));

            match status {

                PARSE_OK => {
                    // Loop is needed here as EXPSEXP might be of length > 1
                    for i in 0..Rf_xlength(cmdexpr) {
                        ans = R_tryEval(VECTOR_ELT(cmdexpr, i),
                                        self.global_env.s(),
                                        &mut error_occurred);
                        if error_occurred == 1 {
                            if self.verbose_m {
                                r_warn(&format!("Error in evaluating R code {:?}\n", status));
                            }
                            Rf_unprotect(2);
                            self.code = "".into();
                            return rerror(EvalError("EVAL_ERROR".into()));
                        }
                        if self.verbose_m {
                            Rf_PrintValue(ans);
                        }
                    }
                    self.code = "".into();
                }
                PARSE_INCOMPLETE => {}
                PARSE_NULL => {
                    if self.verbose_m {
                        r_warn(&format!("%s: ParseStatus is null ({:?})\n", status));
                    }
                    Rf_unprotect(2);
                    self.code = "".into();
                    return rerror(EvalError("PARSE_NULL".into()));

                }
                PARSE_ERROR => {
                    if self.verbose_m {
                        r_warn(&format!("Parse Error: \"{:?}\"\n", code));
                    }
                    Rf_unprotect(2);
                    self.code = "".into();
                    return rerror(EvalError("PARSE_ERROR".into()));

                }
                PARSE_EOF => {
                    if self.verbose_m {
                        r_warn(&format!("ParseStatus is eof ({:?})\n", status));
                    }
                }
            }
            Rf_unprotect(2);
            D::rnew(ans)
        }
    }
}


impl Drop for REngine {
    fn drop(&mut self) {
        unsafe {
            R_dot_Last();
            R_RunExitFinalizers();
            R_CleanTempDir();
            // Rf_KillAllDevices();
            // #ifndef WIN32
            // fpu_setup(FALSE);
            // #endif
            Rf_endEmbeddedR(0);
            REng = false;
        }
    }
}
