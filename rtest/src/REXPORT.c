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
