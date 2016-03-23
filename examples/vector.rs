extern crate rustr;
use rustr::*;

#[no_mangle]
pub extern fn test_real1(a:SEXP) -> SEXP {
    unsafe {
    	let re = REAL(a);
    	*re = 2 as ::std::os::raw::c_double ;
    	*(re.offset(1)) = 3 as ::std::os::raw::c_double;
        a
    }
}

#[no_mangle]
pub extern fn hello_rust1(a:SEXP) -> ::std::os::raw::c_int {
    unsafe{
        IS_S4_OBJECT(a)
    }
}

fn main(){}