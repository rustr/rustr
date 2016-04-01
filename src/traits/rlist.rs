use super::{IntoR,UIntoR};
use error::*;
use protect::stackp::*;
use rdll::*;
use rtype::*;
use super::ToSEXP;
use std::collections::{VecDeque, BTreeSet, BinaryHeap, HashSet, LinkedList};
use std::ffi::CString;

macro_rules! gen_vec_vec_intor{
	($typ1:ident, $typ2:ident, $($real:ty),*) => {
		$(
impl IntoR for $typ1<$typ2<$real>>{
	fn intor(&self)->RResult<SEXP>{
		unsafe{
		let res = Shield::new(Rf_allocVector(VECSXP, self.len() as R_xlen_t));
		for (ind,xx) in self.iter().enumerate(){
			SET_VECTOR_ELT(res.s(), ind as R_xlen_t, try!(xx.intor()));
		}
		Ok(res.s())
	}
	}
} )*
	}
}

// Do somebody really need more? Pull request please.

gen_vec_vec_intor!(Vec,Vec, String,CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(Vec,VecDeque,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(VecDeque,VecDeque,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(BinaryHeap,Vec,String, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(BinaryHeap,VecDeque,String, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(HashSet,Vec,String, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(HashSet,VecDeque,String, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(LinkedList,Vec,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(Vec,LinkedList,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(LinkedList,VecDeque,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(LinkedList,LinkedList,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(VecDeque,LinkedList,String, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(BTreeSet,Vec,String, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_intor!(BTreeSet,VecDeque,String, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);

macro_rules! gen_vec_vec_intor_str{
	($typ1:ident, $typ2:ident) => {
		
impl<'a> IntoR for $typ1<$typ2<&'a str>>{
	fn intor(&self)->RResult<SEXP>{
		unsafe{
		let res = Shield::new(Rf_allocVector(VECSXP, self.len() as R_xlen_t));
		for (ind,xx) in self.iter().enumerate(){
			SET_VECTOR_ELT(res.s(), ind as R_xlen_t, try!(xx.intor()));
		}
		Ok(res.s())
	}
	}
} 
	}
}

gen_vec_vec_intor_str!(Vec,Vec);
gen_vec_vec_intor_str!(Vec,VecDeque);
gen_vec_vec_intor_str!(VecDeque,VecDeque);
gen_vec_vec_intor_str!(BinaryHeap,Vec);
gen_vec_vec_intor_str!(BinaryHeap,VecDeque);
gen_vec_vec_intor_str!(HashSet,Vec);
gen_vec_vec_intor_str!(HashSet,VecDeque);
gen_vec_vec_intor_str!(LinkedList,Vec);
gen_vec_vec_intor_str!(Vec,LinkedList);
gen_vec_vec_intor_str!(LinkedList,VecDeque);
gen_vec_vec_intor_str!(LinkedList,LinkedList);
gen_vec_vec_intor_str!(VecDeque,LinkedList);
gen_vec_vec_intor_str!(BTreeSet,Vec);
gen_vec_vec_intor_str!(BTreeSet,VecDeque);


macro_rules! gen_vec_vec_uintor{
	($typ1:ident, $typ2:ident, $($real:ty),*) => {
		$(
impl UIntoR for $typ1<$typ2<$real>>{
	unsafe fn uintor(&self)-> SEXP{
		let res = Shield::new(Rf_allocVector(VECSXP, self.len() as R_xlen_t));
		for (ind,xx) in self.iter().enumerate(){
			SET_VECTOR_ELT(res.s(), ind as R_xlen_t, xx.uintor());
		}
		res.s()
	}
}
// end main block
 )*
	}
}

gen_vec_vec_uintor!(Vec,Vec, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(Vec,VecDeque, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(VecDeque,VecDeque,CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(BinaryHeap,Vec,CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(BinaryHeap,VecDeque,CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(HashSet,Vec,CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(HashSet,VecDeque, CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(LinkedList,Vec, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(Vec,LinkedList,CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(LinkedList,VecDeque, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(LinkedList,LinkedList,CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(VecDeque,LinkedList, CString,f64,f32,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(BTreeSet,Vec,CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
gen_vec_vec_uintor!(BTreeSet,VecDeque,CString,u16,i16,u8,u32,u64,i8,i32,i64,usize,isize,bool);
