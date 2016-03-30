
say_hi = function(){ .Call('rtest_say_hi',PACKAGE = 'rtest')}

message_warning = function(){ invisible(.Call('rtest_message_warning',PACKAGE = 'rtest'))}

message_message = function(){ invisible(.Call('rtest_message_message',PACKAGE = 'rtest'))}

message_r_printf = function(){ invisible(.Call('rtest_message_r_printf',PACKAGE = 'rtest'))}

dll_r_finite = function(x){ .Call('rtest_dll_r_finite',PACKAGE = 'rtest', x)}
