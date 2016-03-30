
say_hi = function(){ .Call('rtest_say_hi',PACKAGE = 'rtest')}

message_warning = function(){ invisible(.Call('rtest_message_warning',PACKAGE = 'rtest'))}
