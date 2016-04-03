
say_hi = function(){ .Call('rtest_say_hi',PACKAGE = 'rtest')}

message_warning = function(){ invisible(.Call('rtest_message_warning',PACKAGE = 'rtest'))}

message_message = function(){ invisible(.Call('rtest_message_message',PACKAGE = 'rtest'))}

message_r_printf = function(){ invisible(.Call('rtest_message_r_printf',PACKAGE = 'rtest'))}

dll_r_finite = function(x){ .Call('rtest_dll_r_finite',PACKAGE = 'rtest', x)}

dll_is_na = function(x){ .Call('rtest_dll_is_na',PACKAGE = 'rtest', x)}

dll_is_nan = function(x){ .Call('rtest_dll_is_nan',PACKAGE = 'rtest', x)}

dll_option = function(x){ .Call('rtest_dll_option',PACKAGE = 'rtest', x)}

dvec = function(x){ .Call('rtest_dvec',PACKAGE = 'rtest', x)}

dmat = function(d){ .Call('rtest_dmat',PACKAGE = 'rtest', d)}

mat3 = function(d){ .Call('rtest_mat3',PACKAGE = 'rtest', d)}

mat4 = function(d){ .Call('rtest_mat4',PACKAGE = 'rtest', d)}

charvec_at = function(x,y){ .Call('rtest_charvec_at',PACKAGE = 'rtest', x,y)}

numvec_at = function(x,y){ .Call('rtest_numvec_at',PACKAGE = 'rtest', x,y)}

clist = function(){ .Call('rtest_clist',PACKAGE = 'rtest')}

nlist = function(){ .Call('rtest_nlist',PACKAGE = 'rtest')}

uclist = function(){ .Call('rtest_uclist',PACKAGE = 'rtest')}

unlist = function(){ .Call('rtest_unlist',PACKAGE = 'rtest')}

list_data_frame = function(x){ .Call('rtest_list_data_frame',PACKAGE = 'rtest', x)}

charvec = function(){ .Call('rtest_charvec',PACKAGE = 'rtest')}

ncharvec = function(){ .Call('rtest_ncharvec',PACKAGE = 'rtest')}

ucharvec = function(){ .Call('rtest_ucharvec',PACKAGE = 'rtest')}

uncharvec = function(){ .Call('rtest_uncharvec',PACKAGE = 'rtest')}

charvec_setc = function(){ .Call('rtest_charvec_setc',PACKAGE = 'rtest')}

boolvec = function(){ .Call('rtest_boolvec',PACKAGE = 'rtest')}

nboolvec = function(){ .Call('rtest_nboolvec',PACKAGE = 'rtest')}

numvec = function(){ .Call('rtest_numvec',PACKAGE = 'rtest')}

nnumvec = function(){ .Call('rtest_nnumvec',PACKAGE = 'rtest')}

intvec = function(){ .Call('rtest_intvec',PACKAGE = 'rtest')}

nintvec = function(){ .Call('rtest_nintvec',PACKAGE = 'rtest')}
