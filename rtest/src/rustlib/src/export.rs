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

