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

#[no_mangle]
pub extern "C" fn rustr_charvec_at(x : SEXP, y : SEXP)->SEXP{

 let x_ : CharVec = unwrapr!( CharVec::rnew(x) );

let y_ : usize = unwrapr!( usize::rnew(y) );
 let res  = unwrapr!( charvec_at(x_, y_));

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_numvec_at(x : SEXP, y : SEXP)->SEXP{

 let x_ : NumVec = unwrapr!( NumVec::rnew(x) );

let y_ : usize = unwrapr!( usize::rnew(y) );
 let res  = numvec_at(x_, y_);

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
pub extern "C" fn rustr_list_data_frame(x : SEXP)->SEXP{

 let x_ : RList = unwrapr!( RList::rnew(x) );
 let res  = unwrapr!( list_data_frame(x_));

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_charvec()->SEXP{

  let res  = unwrapr!( charvec());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_ncharvec()->SEXP{

  let res  = unwrapr!( ncharvec());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_ucharvec()->SEXP{

  let res  = ucharvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_uncharvec()->SEXP{

  let res  = uncharvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_charvec_setc()->SEXP{

  let res  = unwrapr!( charvec_setc());

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_boolvec()->SEXP{

  let res  = boolvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_nboolvec()->SEXP{

  let res  = nboolvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_numvec()->SEXP{

  let res  = numvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_nnumvec()->SEXP{

  let res  = nnumvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_intvec()->SEXP{

  let res  = intvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

#[no_mangle]
pub extern "C" fn rustr_nintvec()->SEXP{

  let res  = nintvec();

 let res_sexp : SEXP = unwrapr!(res.intor());

 return res_sexp;
}

