
say_hi = function(){ .Call('rtest_say_hi',PACKAGE = 'rtest')}

message_warning = function(){ invisible(.Call('rtest_message_warning',PACKAGE = 'rtest'))}

message_message = function(){ invisible(.Call('rtest_message_message',PACKAGE = 'rtest'))}

message_r_printf = function(){ invisible(.Call('rtest_message_r_printf',PACKAGE = 'rtest'))}

dll_r_finite = function(x){ .Call('rtest_dll_r_finite',PACKAGE = 'rtest', x)}

dll_is_na = function(x){ .Call('rtest_dll_is_na',PACKAGE = 'rtest', x)}

dll_is_nan = function(x){ .Call('rtest_dll_is_nan',PACKAGE = 'rtest', x)}

dll_option = function(x){ .Call('rtest_dll_option',PACKAGE = 'rtest', x)}

clist = function(){ .Call('rtest_clist',PACKAGE = 'rtest')}

nlist = function(){ .Call('rtest_nlist',PACKAGE = 'rtest')}

uclist = function(){ .Call('rtest_uclist',PACKAGE = 'rtest')}

unlist = function(){ .Call('rtest_unlist',PACKAGE = 'rtest')}

dvec = function(x){ .Call('rtest_dvec',PACKAGE = 'rtest', x)}

dmat = function(d){ .Call('rtest_dmat',PACKAGE = 'rtest', d)}

mat3 = function(d){ .Call('rtest_mat3',PACKAGE = 'rtest', d)}

mat4 = function(d){ .Call('rtest_mat4',PACKAGE = 'rtest', d)}
