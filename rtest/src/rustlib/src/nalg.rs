use super::*;

// #[rustr_export]
pub fn dvec(x: DVec<f64>)->DVec<f64>{
    x
}

// #[rustr_export]
pub fn dmat(d:DMat<f64>)->DMat<f64>{
   r_printf(&format!("{:?},{:?}",d.ncols(), d.nrows()));
   d
}

// #[rustr_export]
pub fn mat3(d:Mat3<f64>)->Mat3<f64>{
   d
}

// #[rustr_export]
pub fn mat4(d:Mat4<f64>)->Mat4<f64>{
   d
}
