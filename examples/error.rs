extern crate rustr;

use rustr::error::*;

fn main() {
    println!("Hello, world! {}",RError::stop());

    let d: Result<i32, _> = Err(
    	(RError::new(
    		REKind::Other(::std::io::Error::new(::std::io::ErrorKind::InvalidInput,
                       "data provided contains a nul byte").into())
    	)
    )
    );
    d.unwrap();
}
