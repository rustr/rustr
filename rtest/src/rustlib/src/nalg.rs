use super::*;

// #[rustr_export]
pub fn dvec(x: DVec<f64>)->DVec<f64>{
    x
}

// #[rustr_export]
pub fn dint_mat(d:DMat<f64>)->DMat<f64>{
   r_printf(&format!("{:?},{:?}",d.ncols(), d.nrows()));
   d
}

fn pr<D: ::std::fmt::Debug>(d:&D){r_printf(&format!("{:?}\n",d))}

// #[rustr_export]
pub fn mat3(d:Mat3<f64>)->Mat3<f64>{{pr(&d)};d}

// #[rustr_export]
pub fn mat2(d:Mat2<f64>)->Mat2<f64>{{pr(&d)};d}

// #[rustr_export]
pub fn mat1(d:Mat1<f64>)->Mat1<f64>{{pr(&d)};d}

// #[rustr_export]
pub fn mat4(d:Mat4<f64>)->Mat4<f64>{{pr(&d)};d}

// #[rustr_export]
pub fn mat5(d:Mat2<f64>)->Mat2<f64>{{pr(&d)};d}

// #[rustr_export]
pub fn mat6(d:Mat1<f64>)->Mat1<f64>{{pr(&d)};d}


// int

// #[rustr_export]
pub fn int_mat3(d:Mat3<u32>)->Mat3<u32>{{pr(&d)};d}

// #[rustr_export]
pub fn int_mat2(d:Mat2<u32>)->Mat2<u32>{{pr(&d)};d}

// #[rustr_export]
pub fn int_mat1(d:Mat1<u32>)->Mat1<u32>{{pr(&d)};d}

// #[rustr_export]
pub fn int_mat4(d:Mat4<u32>)->Mat4<u32>{{pr(&d)};d}

// #[rustr_export]
pub fn int_mat5(d:Mat2<u32>)->Mat2<u32>{{pr(&d)};d}

// #[rustr_export]
pub fn int_mat6(d:Mat1<u32>)->Mat1<u32>{{pr(&d)};d}
