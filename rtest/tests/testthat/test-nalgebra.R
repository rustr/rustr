context("nalgebra")

test_that("realmat",{
    tmat3 = matrix(as.integer(c(1,2,3,4,5,6,7,8,9)),nrow = 3)
    expect_output(int_mat3(tmat3), "m11: 1, m21: 4, m31: 7, m12: 2, m22: 5, m32: 8, m13: 3, m23: 6, m33: 9")
    capture.output(expect_identical(int_mat3(tmat3),tmat3))
})
