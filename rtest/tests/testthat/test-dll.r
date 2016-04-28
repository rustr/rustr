context("dll module")
test_that("dll module",{
expect_true(dll_r_finite(10))
expect_true(!dll_r_finite(NaN))
expect_true(!dll_r_finite(Inf))
expect_true(!dll_r_finite(-Inf))

expect_true(!dll_is_na(10))
expect_true(!dll_is_na(NaN))
expect_true(!dll_is_na(Inf))
expect_true(!dll_is_na(-Inf))

expect_true(!dll_is_nan(10))
expect_true(dll_is_nan(NaN))
expect_true(!dll_is_nan(Inf))
expect_true(!dll_is_nan(-Inf))

options(happy=T)
expect_true(dll_option("happy"))
})

