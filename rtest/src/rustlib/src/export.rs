use super::*;

#[no_mangle]
pub extern "C" fn rustr_say_hi()->SEXP{

  let res  = unwrapr!( say_hi());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_message_warning(){

  message_warning();

 }

#[no_mangle]
pub extern "C" fn rustr_message_message(){

  message_message();

 }

#[no_mangle]
pub extern "C" fn rustr_message_r_printf(){

  message_r_printf();

 }

#[no_mangle]
pub extern "C" fn rustr_dll_r_finite(x : SEXP)->SEXP{

 let x_ : c_double = unwrapr!( c_double::rnew(x) );
 let res  = dll_r_finite(x_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_dll_is_na(x : SEXP)->SEXP{

 let x_ : c_double = unwrapr!( c_double::rnew(x) );
 let res  = dll_is_na(x_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_dll_is_nan(x : SEXP)->SEXP{

 let x_ : c_double = unwrapr!( c_double::rnew(x) );
 let res  = dll_is_nan(x_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

