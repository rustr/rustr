context("message")
test_that("message",{
expect_warning(rtest:::message_warning(),"this is warning from rust")

expect_message(rtest:::message_message(),"this is message from rust")

expect_output(rtest:::message_r_printf(),"this is printf from rust")
})