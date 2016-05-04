extern crate rustr;

use rustr::*;

#[cfg(all(feature = "engine"))]
use rustr::feature::engine::*;

#[cfg(all(feature = "engine"))]
#[test]
fn init_eng() {
   let mut re = REngine::init().unwrap();
   let res: f64 = re.eval("1+1").unwrap();
   println!("{}", res);
   assert_eq!(2.0, res);
}
