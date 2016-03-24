## Test Windows

```bash
env RLIBPATH="C:\Program Files\R\R-devel\bin\x64" cargo test --features "engine"

env RUST_BACKTRACE=1 RLIBPATH="C:\Program Files\R\R-devel\bin\x64" cargo test --features "engine"

export R_HOME=$(Rscript -e "cat(R.home())")

```
