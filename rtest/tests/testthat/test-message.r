context("message")
test_that("message",{
expect_warning(message_warning(),"this is warning from rust")

expect_message(message_message(),"this is message from rust")

expect_output(message_r_printf(),"this is printf from rust")
})