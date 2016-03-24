use rdll::*;
use environment::*;
use std::ptr::null_mut;
use std::ffi::*;
use std::os::raw::*;
use error::*;
use std::env;

pub static mut REng: *mut REngine = null_mut();

pub struct REngine {
    verbose_m: bool, // constructor, or setter
    interactive_m: bool, // constructor only
    global_env: EnvirN,
    code: String,
}

impl REngine {
    pub fn init() -> RResult<Self> {
        let argv: Vec<CString> = vec![CString::new("REmbeddedPostgres").unwrap(),
                                      CString::new("--no-save").unwrap(),
                                      CString::new("--gui=none").unwrap(),
                                      CString::new("--vanilla").unwrap(),
                                      CString::new("--slave").unwrap(),
                                      CString::new("--no-readline").unwrap(),
                                      CString::new("--silent").unwrap()];
        REngine::initialize(argv, false, false)

    }
    pub fn initialize(args: Vec<CString>, verbose: bool, interactive: bool) -> RResult<Self> {
        let dd = args.len();
        unsafe {
            if !REng.is_null() {
                return rerror(REKind::BindingIsLocked("there is a runing R engine".into()));
            }
        }
        let args: Vec<*mut c_char> = args.into_iter()
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
            unsafe {
                let xs: String = try!(CStr::from_ptr(get_R_HOME()).to_owned().into_string());
                env::set_var("R_HOME", &xs)
            }

        }
        let d = unsafe { Rf_initEmbeddedR(dd as c_int, args.as_ptr() as *mut *mut c_char) };

        if cfg!(not(target_os = "windows")) {
            unsafe {
                R_CStackLimit = ::std::os::raw::c_ulong::max_value();
            }
        }

        if d != 0 {
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
        let mut res = REngine {
            verbose_m: verbose,
            interactive_m: interactive, // constructor only
            global_env: envir,
            code: "".into(),
        };

        unsafe {
            REng = &mut res;
        }
        Ok(res)
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
            REng = null_mut();
        }
    }
}
