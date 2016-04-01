## Rust and R Integration

[![Build Status](https://travis-ci.org/rustr/rustr.svg?branch=master)](https://travis-ci.org/rustr/rustr)
[![Current Version](http://meritbadge.herokuapp.com/rustr)](https://crates.io/crates/rustr)
[![License: Apache-2.0](https://img.shields.io/crates/l/rustr.svg)](#License)

`rustr` is a Rust library that provides a Rust API to work with R.

Write pure Rust code with `rustr`, and then use `rustinr` R package to generate Rust interfaces to R.

More info: https://rustr.org

This project is now under construction. **Issues** and **contributions** are welcome!

### Todo for v0.2

#### rustr

- [x] random numbers from R
- [ ] check bindgen code
- [x] more methods for `R Vector Obejct` types
- [x] stablize R in Rust with `feature::engine` module
- [x] more `RNew` `IntoR` `From` `Into` for types
- [x] DataFrame type for RList
- [x] [`nalgebra`][1] crate types
- [x] [`num`][2] crate types
- [x] [`log`][3] crate `log` trait

#### rustinr

- [ ] find and fix bugs
- [ ] CRAN
- [ ] knitr code engine

#### book

- [ ] more about `rdll` module
- [ ] about concurrency
- [ ] about `unsafe`
- [ ] about memory safety and garbage collection
- [ ] R in Rust with `feature::engine` module


#### gallery

- [ ] more examples with vectors [rustr/gallery #1](https://github.com/rustr/gallery/issues/1)
- [ ] more `RNew`, `IntoR` `From` `Into` for types
- [ ] how to use `rustr::rdll` module
- [ ] R Object - DataFrame type
- [ ] [`nalgebra`][1] crates types
- [ ] [`num`][2] crates types
- [ ] [`rusty-machine`][rm] crate `rusty-machine` examples
- [ ] R in Rust with `feature::engine` module

### Todo for future

- [ ] date type with [chrono][4]
- [ ] sugar function
- [ ] R6 & Rust type

Miss a feature? Talk on [forum](https://groups.google.com/forum/#!forum/rustr) or create an issue. 


[1]: https://github.com/sebcrozet/nalgebra
[2]: https://github.com/rust-num/num
[3]: https://github.com/rust-lang-nursery/log
[4]: https://github.com/lifthrasiir/rust-chrono
[rm]: https://github.com/AtheMathmo/rusty-machine
[nd]: https://github.com/bluss/rust-ndarray