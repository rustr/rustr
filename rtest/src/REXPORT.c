#include <Rinternals.h>
#include <R.h>

extern SEXP rustr_say_hi();
SEXP rtest_say_hi(){ return(rustr_say_hi());}
extern SEXP rustr_message_warning();
SEXP rtest_message_warning(){ rustr_message_warning();return(R_NilValue);}
