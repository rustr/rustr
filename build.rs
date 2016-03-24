// build.rs

// Bring in a dependency on an externally maintained `gcc` package which manages
// invoking the C compiler.
use std::env;

fn main() {
	match env::var("RLIBPATH") {
    	Ok(val) =>     println!("cargo:rustc-link-search=native={}", val) , 
    	Err(_) =>      (),
	};
}
