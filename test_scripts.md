## Test Windows

```bash
env RUST_TEST_THREADS=1  RLIBPATH="C:\Program Files\R\R-devel\bin\x64" cargo test --features "engine"

env RUST_BACKTRACE=1 RLIBPATH="C:\Program Files\R\R-devel\bin\x64" cargo test --features "engine"

export R_HOME=$(Rscript -e "cat(R.home())")

export PATH=$PATH:"/c/Program Files/R/R-devel/bin/x64"

```
