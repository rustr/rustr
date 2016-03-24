extern crate rustr;

use rustr::*;
#[cfg(feature = "engine")]
use rustr::feature::engine::*;

fn main() {
    let mut re = REngine::init().unwrap();
    let res: f64 = re.eval("1+1").unwrap();
    println!("{}", res);
    assert_eq!(2.0, res);
}