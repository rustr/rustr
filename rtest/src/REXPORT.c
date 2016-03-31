#include <Rinternals.h>
#include <R.h>

extern SEXP rustr_say_hi();
SEXP rtest_say_hi(){ return(rustr_say_hi());}
extern SEXP rustr_message_warning();
SEXP rtest_message_warning(){ rustr_message_warning();return(R_NilValue);}
extern SEXP rustr_message_message();
SEXP rtest_message_message(){ rustr_message_message();return(R_NilValue);}
extern SEXP rustr_message_r_printf();
SEXP rtest_message_r_printf(){ rustr_message_r_printf();return(R_NilValue);}
extern SEXP rustr_dll_r_finite(SEXP x);
SEXP rtest_dll_r_finite(SEXP x){ return(rustr_dll_r_finite(x));}
extern SEXP rustr_dll_is_na(SEXP x);
SEXP rtest_dll_is_na(SEXP x){ return(rustr_dll_is_na(x));}
extern SEXP rustr_dll_is_nan(SEXP x);
SEXP rtest_dll_is_nan(SEXP x){ return(rustr_dll_is_nan(x));}
extern SEXP rustr_dll_option(SEXP x);
SEXP rtest_dll_option(SEXP x){ return(rustr_dll_option(x));}
extern SEXP rustr_clist();
SEXP rtest_clist(){ return(rustr_clist());}
extern SEXP rustr_nlist();
SEXP rtest_nlist(){ return(rustr_nlist());}
extern SEXP rustr_uclist();
SEXP rtest_uclist(){ return(rustr_uclist());}
extern SEXP rustr_unlist();
SEXP rtest_unlist(){ return(rustr_unlist());}
extern SEXP rustr_charvec();
SEXP rtest_charvec(){ return(rustr_charvec());}
extern SEXP rustr_ncharvec();
SEXP rtest_ncharvec(){ return(rustr_ncharvec());}
extern SEXP rustr_ucharvec();
SEXP rtest_ucharvec(){ return(rustr_ucharvec());}
extern SEXP rustr_uncharvec();
SEXP rtest_uncharvec(){ return(rustr_uncharvec());}
extern SEXP rustr_dvec(SEXP x);
SEXP rtest_dvec(SEXP x){ return(rustr_dvec(x));}
extern SEXP rustr_dmat(SEXP d);
SEXP rtest_dmat(SEXP d){ return(rustr_dmat(d));}
extern SEXP rustr_mat3(SEXP d);
SEXP rtest_mat3(SEXP d){ return(rustr_mat3(d));}
extern SEXP rustr_mat4(SEXP d);
SEXP rtest_mat4(SEXP d){ return(rustr_mat4(d));}
