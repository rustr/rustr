context("R Object")
test_that("R Object",{
l1 = list("sd", "Sd")
l2 = structure(list(sd = "sd", Sd = "Sd"), .Names = c("sd", "Sd"))

expect_equal(l1, uclist())
expect_equal(l1, clist())
expect_equal(l2, unlist())
expect_equal(l2, nlist())

c1 = c("sd", "Sd")
c2 = structure(c("sd", "Sd"), .Names = c("sd", "Sd"))

expect_equal(c1, ucharvec())
expect_equal(c1, charvec())
expect_equal(c2, uncharvec())
expect_equal(c2, ncharvec())

expect_equal( charvec_at(c("sd","sS","sa","as","sdsd","sdsss"),0L),"sd")
expect_error(charvec_at(c("sd","sS","sa","as","sdsd","sdsss"),11L))

expect_equal( numvec_at(c(0,1,2,3),3L),3)
expect_equal(1, numvec_at(c(5,1,2,3),1L))

expect_equal(c(1,2), numvec())
expect_equal(structure(c(1, 3), .Names = c("sd", "Sd")), nnumvec())

expect_equal(c(1L,2L), intvec())
expect_equal(structure(c(1L, 3L), .Names = c("sd", "Sd")), nintvec())

expect_equal(list_data_frame(list(sd=1,2,ss=3)),structure(list(sd = 1, 2, ss = 3), .Names = c("sd", "", "ss"), row.names = 1L, class = "data.frame"))

expect_equal(list_data_frame(list()),data.frame())
expect_error( list_data_frame(list(sd = c(1,2),1)),"not all colunm are the same length")
})
