## Test Windows

```bash
env RUST_TEST_THREADS=1  RLIBPATH="C:\Program Files\R\R-devel\bin\x64" cargo test --features "engine"

env RUST_BACKTRACE=1 RLIBPATH="C:\Program Files\R\R-devel\bin\x64" cargo test --features "engine"

export R_HOME=$(Rscript -e "cat(R.home())")

export PATH=$PATH:"/c/Program Files/R/R-devel/bin/x64"

```

## Test Mac

If you installed R from `brew cask install r-app` or direct install of application package:

```bash
env R_HOME=/Library/Frameworks/R.framework/Resources RUST_TEST_THREADS=1 RLIBPATH=/Library/Frameworks/R.framework/Libraries cargo test --features "engine"
env R_HOME=/Library/Frameworks/R.framework/Resources RUST_BACKTRACE=1 RLIBPATH=/Library/Frameworks/R.framework/Libraries cargo test --features "engine"
```

If you installed R from `brew install r`, change the version number in the calls below:

```bash
env RUST_TEST_THREADS=1 R_HOME=/usr/local/Cellar/r/3.4.3/lib/R RLIBPATH=/usr/local/Cellar/r/3.4.3/lib/ cargo test --features "engine"
env RUST_BACKTRACE=1 R_HOME=/usr/local/Cellar/r/3.4.3/lib/R RLIBPATH=/usr/local/Cellar/r/3.4.3/lib/ cargo test --features "engine"
```