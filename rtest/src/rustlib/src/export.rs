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

#[no_mangle]
pub extern "C" fn rustr_dll_option(x : SEXP)->SEXP{

 let x_ : String = unwrapr!( String::rnew(x) );
 let res  = unwrapr!( dll_option(x_));

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_clist()->SEXP{

  let res  = unwrapr!( clist());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_nlist()->SEXP{

  let res  = unwrapr!( nlist());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_uclist()->SEXP{

  let res  = uclist();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_unlist()->SEXP{

  let res  = unlist();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_dvec(x : SEXP)->SEXP{

 let x_ : DVec<f64> = unwrapr!( DVec::rnew(x) );
 let res  = dvec(x_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_dmat(d : SEXP)->SEXP{

 let d_ : DMat<f64> = unwrapr!( DMat::rnew(d) );
 let res  = dmat(d_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_mat3(d : SEXP)->SEXP{

 let d_ : Mat3<f64> = unwrapr!( Mat3::rnew(d) );
 let res  = mat3(d_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_mat4(d : SEXP)->SEXP{

 let d_ : Mat4<f64> = unwrapr!( Mat4::rnew(d) );
 let res  = mat4(d_);

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

