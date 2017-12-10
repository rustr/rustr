extern crate rustr;

pub use rustr::*;

#[cfg(all(feature = "engine"))]
pub use rustr::feature::engine::*;

#[cfg(all(feature = "engine"))]
mod rengine {
    use super::*;

    #[test]
    fn init_eng() {
        let mut re = unsafe { REngine::init().unwrap() };
        let res: f64 = re.eval("1+1").unwrap();
        assert_eq!(2.0, res);
    }

    #[test]
    fn init_second_eng() {
        let mut re = unsafe { REngine::init().unwrap() };
        let res: f64 = re.eval("1+1").unwrap();
        assert_eq!(2.0, res);
    }

    #[test]
    fn z_eng_leave() {
        let re = unsafe { REngine::init().unwrap() };
        unsafe { re.leave() }
    }
}
