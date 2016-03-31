use super::*;

// #[rustr_export]
pub fn charvec_at(x:CharVec,y:usize)->RResult<String>{
    x.at(y)
}

// #[rustr_export]
pub fn numvec_at(x:NumVec,y:usize)->f64{
    x.at(y).unwrap_or(0.0)
}

