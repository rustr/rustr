context("R Object")

l1 = list("sd", "Sd")
l2 = structure(list(sd = "sd", Sd = "Sd"), .Names = c("sd", "Sd"))

expect_equal(l1, rtest:::uclist())
expect_equal(l1, rtest:::clist())
expect_equal(l2, rtest:::unlist())
expect_equal(l2, rtest:::nlist())

c1 = c("sd", "Sd")
c2 = structure(c("sd", "Sd"), .Names = c("sd", "Sd"))

expect_equal(c1, rtest:::ucharvec())
expect_equal(c1, rtest:::charvec())
expect_equal(c2, rtest:::uncharvec())
expect_equal(c2, rtest:::ncharvec())

expect_equal( rtest:::charvec_at(c("sd","sS","sa","as","sdsd","sdsss"),0L),"sd")
expect_error(rtest:::charvec_at(c("sd","sS","sa","as","sdsd","sdsss"),11L))

expect_equal( rtest:::numvec_at(c(0,1,2,3),3L),3)
expect_equal(0, rtest:::numvec_at(c(5,1,2,3),4L))
