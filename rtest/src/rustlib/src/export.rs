use super::*;

#[no_mangle]
pub extern "C" fn rustr_say_hi()->SEXP{

  let res  = unwrapr!( say_hi());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

