extern crate rustr;
extern crate libc;

#[crate_type = "staticlib"]

use rustr::*;


#[no_mangle]
pub extern fn test_real1(a:SEXP) -> SEXP {
    unsafe {
    	let re = REAL(a);
    	*re = 2 as libc::c_double ;
    	*(re.offset(1)) = 3 as libc::c_double;
        a
    }
}

#[no_mangle]
pub extern fn hello_rust1(a:SEXP) -> ::libc::c_int {
    
    unsafe{
        IS_S4_OBJECT(a)
    }
}

#[no_mangle]
pub extern fn test_pre1() -> SEXP {
    	let r = Preserve::empty();
    	r.s()
}
