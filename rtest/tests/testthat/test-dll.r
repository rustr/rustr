context("dll module")

expect_true(rtest:::dll_r_finite(10))
expect_true(!rtest:::dll_r_finite(NaN))
expect_true(!rtest:::dll_r_finite(Inf))
expect_true(!rtest:::dll_r_finite(-Inf))
