context("R Object")

l1 = list("sd", "Sd")
l2 = structure(list(sd = "sd", Sd = "Sd"), .Names = c("sd", "Sd"))

expect_equal(l1, rtest:::uclist())
expect_equal(l1, rtest:::clist())
expect_equal(l2, rtest:::unlist())
expect_equal(l2, rtest:::nlist())
