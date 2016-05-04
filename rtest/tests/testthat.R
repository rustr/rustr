library(testthat)
library(rtest)
library(devtools)
load_all(find.package("rtest"))

test_check("rtest")
