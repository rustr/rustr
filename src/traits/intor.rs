use ::rdll::*;
use ::rtype::*;
use error::{RResult, REKind, rerror};
use traits::*;
use ::protect::stackp::*;
use std::ffi::{CString, CStr};
use std::collections::{HashMap, VecDeque, BTreeMap, BTreeSet, BinaryHeap, HashSet, LinkedList};
use REKind::*;
use util::*;

impl IntoR for () {
    fn intor(&self) -> RResult<SEXP> {

        unsafe { Ok(Self::uintor(self)) }
    }
}

impl UIntoR for () {
    unsafe fn uintor(&self) -> SEXP {
        R_NilValue
    }
}

impl IntoR for bool {
    fn intor(&self) -> RResult<SEXP> {
        unsafe { Ok(Self::uintor(self)) }
    }
}

impl UIntoR for bool {
    unsafe fn uintor(&self) -> SEXP {
        let rvec = Shield::new(Rf_allocVector(LGLSXP, 1));
        let ptr = LOGICAL(rvec.s());
        *ptr.offset(0) = *self as ::std::os::raw::c_int;
        rvec.s()
    }
}

impl RNew for bool {
    fn rnew(x: SEXP) -> RResult<bool> {
        if RTYPEOF(x) != LGLSXP && unsafe { Rf_xlength(x) == 1 } {
            return rerror(NotCompatible("expecting a boolean".into()));
        }
        unsafe { Ok(Self::urnew(x)) }
    }
}

impl URNew for bool {
    unsafe fn urnew(x: SEXP) -> bool {
        let ptr = LOGICAL(x);
        *ptr.offset(0) == 1
    }
}

// fromr integer /////////////////////////////

macro_rules! gen_fromr_int {
    ($sexp:ident; $sexpget:ident; $err:expr; $into:ty; $($x:ty),*) => (
		$(
// main block
impl RNew for $x {
	fn rnew(x:SEXP) -> RResult<$x> {
		unsafe {
			if RTYPEOF(x) != $sexp  || Rf_xlength(x) != 1{
    			return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
			}
			Ok(Self::urnew(x))
		}
	}
}

impl URNew for $x {
	unsafe fn urnew(x:SEXP) -> $x {
			let rptr = $sexpget(x);
			*rptr.offset(0) as $x
	}
}

impl UIntoR for $x {
    unsafe fn uintor(&self) -> SEXP {
            let rvec = Shield::new(Rf_allocVector($sexp, 1));
            let rptr = $sexpget(rvec.s());
            *rptr.offset(0) = (*self) as $into;
            rvec.s()
    }
}

impl IntoR for $x {
    fn intor(&self) -> RResult<SEXP> {
		unsafe{Ok(Self::uintor(self))}
    }
}
			// end main block
		)*
    )
}

// expanded
//
// impl FromR<u64> for SEXP {
// 	fn fromr(&self) -> RResult<u64> {
// 		...
// 		...
// 	}
// }

gen_fromr_int!(INTSXP; INTEGER; "integer"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_int!(REALSXP; REAL; "value"; ::std::os::raw::c_double; f64,f32);

// for u8

impl RNew for u8 {
    fn rnew(x: SEXP) -> RResult<u8> {
        unsafe {
            if (RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP) || Rf_xlength(x) != 1 {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for u8 {
    unsafe fn urnew(x: SEXP) -> u8 {
        if RTYPEOF(x) == INTSXP {
            let rptr = INTEGER(x);
            let res = *rptr.offset(0) as u8;
            return res;
        }
        let rptr = RAW(x);
        *rptr.offset(0) as u8
    }
}

impl UIntoR for u8 {
    unsafe fn uintor(&self) -> SEXP {
        let rvec = Shield::new(Rf_allocVector(INTSXP, 1));
        let rptr = INTEGER(rvec.s());
        *rptr.offset(0) = (*self) as ::std::os::raw::c_int;
        rvec.s()
    }
}

impl IntoR for u8 {
    fn intor(&self) -> RResult<SEXP> {
        unsafe { Ok(Self::uintor(self)) }
    }
}



// intor fromr vec number /////////////////////////////

macro_rules! gen_fromr_vec {
    ($collection:ident; $push:ident; $sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::with_capacity(lens as usize);
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push(*rptr.offset(ii as isize) as $x);
            }
            vecs
        }
    }


impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector($sexp, size_x as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as $into ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_vec!(Vec; push; INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec!(Vec; push; REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_vec!(BinaryHeap; push;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec!(HashSet; insert;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec!(VecDeque; push_back; INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_vec!(VecDeque; push_back; REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);

// bool

macro_rules! gen_fromr_vec_bool {
    ($collection:ident; $push:ident; $sexp:ident; $sexpget:ident; $err:expr;  $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::with_capacity(lens as usize);
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push(*rptr.offset(ii as isize) ==1 );
            }
            vecs
        }
    }


impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector($sexp, size_x as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as ::std::os::raw::c_int  ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

// end main block
		)*
    )
}

gen_fromr_vec_bool!(Vec; push; LGLSXP; LOGICAL; "boolean vector"; bool);
gen_fromr_vec_bool!(BinaryHeap; push;LGLSXP; LOGICAL; "boolean vector"; bool);
gen_fromr_vec_bool!(HashSet; insert;LGLSXP; LOGICAL; "boolean vector"; bool);
gen_fromr_vec_bool!(VecDeque; push_back; LGLSXP; LOGICAL; "boolean vector"; bool);


// u8
macro_rules! gen_u8_collection{
	($collection:ident; $push:ident)=>(

impl RNew for $collection<u8> {
    fn rnew(x:SEXP) -> RResult<$collection<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<u8> {
    unsafe fn urnew(x:SEXP) -> $collection<u8> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<u8> = $collection::with_capacity(lens as usize);
            if RTYPEOF(x) ==INTSXP{
	            let rptr = INTEGER(x);
	            for ii in 0..lens {
	                vecs.$push(*rptr.offset(ii as isize) as u8);
	            }
	             return vecs;
            }
            let rptr = RAW(x);
            for ii in 0..lens {
                vecs.$push(*rptr.offset(ii as isize) as u8);
            }
            vecs
        }
    }


impl UIntoR for $collection<u8> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();
            let rvec = Shield::new(Rf_allocVector(INTSXP, size_x as R_xlen_t));
            let rptr = INTEGER(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as ::std::os::raw::c_int ;
				index = index + 1;
            }
            rvec.s()

    }
}

impl IntoR for $collection<u8> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}
	)}

gen_u8_collection!(Vec; push);
gen_u8_collection!(BinaryHeap; push);
gen_u8_collection!(HashSet; insert);
gen_u8_collection!(VecDeque; push_back);

// end u8
macro_rules! gen_ref_int{
	($INTEGER:ident; $INTSXP:ident; $tyy:ty; $($usize:ident),*) =>(
$(impl<'a> UIntoR for &'a [$usize]{
	unsafe fn uintor(&self)->SEXP{
		let size_x = self.len();
            let rvec = Shield::new(Rf_allocVector($INTSXP, size_x as R_xlen_t));
            let rptr = $INTEGER(rvec.s());
            let mut index = 0;
            for ii in *self {
                *rptr.offset(index) = ii.clone() as $tyy ;
				index = index + 1;
            }
            rvec.s()
	}
}

impl<'a> IntoR for &'a [$usize]{
	fn intor(&self)->RResult<SEXP>{
		unsafe{Ok(Self::uintor(self))}
}
}
)*
	)
}

gen_ref_int!(INTEGER; INTSXP; ::std::os::raw::c_int; u64,u32,u16,u8,i64,i32,i16,i8,usize,isize);
gen_ref_int!(REAL; REALSXP; ::std::os::raw::c_double; f64,f32);



macro_rules! gen_fromr_linklist {
    ($collection:ident; $push:ident;$sexp:ident; $sexpget:ident; $err:expr; $into:ty ; $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::new();
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push(*rptr.offset(ii as isize) as $x);
            }
            vecs
    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector($sexp, size_x as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as $into ;
				index = index + 1;
            }
            rvec.s()

    }
}


// end main block
		)*
    )
}

gen_fromr_linklist!(LinkedList;push_front;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);
gen_fromr_linklist!(LinkedList;push_front; REALSXP; REAL; "numeric vector"; ::std::os::raw::c_double; f64,f32);
gen_fromr_linklist!(BTreeSet;insert;INTSXP; INTEGER; "integer vector"; ::std::os::raw::c_int; u64,u32,u16,i64,i32,i16,i8,usize,isize);

// bool

macro_rules! gen_fromr_linklist_bool {
    ($collection:ident; $push:ident;$sexp:ident; $sexpget:ident; $err:expr;  $($x:ty),*) => (
		$(
// main block
impl RNew for $collection<$x> {
    fn rnew(x:SEXP) -> RResult<$collection<$x>> {
        unsafe {
            if RTYPEOF(x) != $sexp {
                return rerror(REKind::NotCompatible(concat!("expecting a ",$err).into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<$x> {
    unsafe fn urnew(x:SEXP) -> $collection<$x> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<$x> = $collection::new();
            let rptr = $sexpget(x);
            for ii in 0..lens {
                vecs.$push(*rptr.offset(ii as isize) ==1);
            }
            vecs
    }
}

impl IntoR for $collection<$x> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

impl UIntoR for $collection<$x> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector($sexp, size_x as R_xlen_t));
            let rptr = $sexpget(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as ::std::os::raw::c_int ;
				index = index + 1;
            }
            rvec.s()

    }
}


// end main block
		)*
    )
}

gen_fromr_linklist_bool!(LinkedList;push_front;INTSXP; INTEGER; "integer vector"; bool);
gen_fromr_linklist_bool!(BTreeSet;insert;INTSXP; INTEGER; "integer vector"; bool);


// u8
macro_rules! gen_u8_link{
	($collection:ident; $push:ident)=>(
impl RNew for $collection<u8> {
    fn rnew(x:SEXP) -> RResult<$collection<u8>> {
        unsafe {
            if RTYPEOF(x) != INTSXP && RTYPEOF(x) != RAWSXP {
                return rerror(REKind::NotCompatible("expecting a INTSXP or RAWSXP".into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl URNew for $collection<u8> {
    unsafe fn urnew(x:SEXP) -> $collection<u8> {
            let lens = Rf_xlength(x);
            let mut vecs: $collection<u8> = $collection::new();
            if RTYPEOF(x) ==INTSXP{
				let rptr = INTEGER(x);
	            for ii in 0..lens {
	                vecs.$push(*rptr.offset(ii as isize) as u8);
	            }
	            return vecs;
            }
            let rptr = RAW(x);
            for ii in 0..lens {
                vecs.$push(*rptr.offset(ii as isize) as u8);
            }
            vecs
    }
}

impl IntoR for $collection<u8> {
    fn intor(&self) -> RResult<SEXP> {
        unsafe{Ok(Self::uintor(self))}
    }
}

impl UIntoR for $collection<u8> {
    unsafe fn uintor(&self) -> SEXP {
        let size_x = self.len();

            let rvec = Shield::new(Rf_allocVector(INTSXP, size_x as R_xlen_t));
            let rptr = INTEGER(rvec.s());
            let mut index = 0;
            for ii in self {
                *rptr.offset(index) = ii.clone() as ::std::os::raw::c_int ;
				index = index + 1;
            }
            rvec.s()

    }
}
)}
// end u8

gen_u8_link!(LinkedList;push_front);
gen_u8_link!(BTreeSet;insert);

// when specialize is merge
// impl<T:Iterator<Item=f64>+ExactSizeIterator> IntoR for T{
//    fn intor(&self) -> RResult<SEXP> {
//        let size_x = self.len();
//        unsafe {
//            let rvec = Shield::new(Rf_allocVector(REALSXP, size_x as R_xlen_t));
//            let rptr = REAL(rvec.s());
//            let mut index = 0;
//
//            for ii in self {
//                *rptr.offset(index) = ii.clone() as ::std::os::raw::c_double ;
// 				index = index + 1;
//            }
//            return Ok(rvec.s());
//        }
//
//    }
// }

// hashmap ////////////////////////
impl<D: IntoR> IntoR for HashMap<String, D> {
    fn intor(&self) -> RResult<SEXP> {
        let size_x = self.len();
        unsafe {
            let name = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let names = name.s();
            let rvec = Shield::new(Rf_allocVector(VECSXP, size_x as R_xlen_t));
            let rvecs = rvec.s();
            let mut index = 0;
            for (mname, mitem) in self {
                let strs: &str = mname.as_ref();
                SET_STRING_ELT(names, index, Rf_mkChar(try!(CString::new(strs)).as_ptr()));
                SET_VECTOR_ELT(rvecs, index, try!(mitem.intor()));
                index = index + 1;
            }
            Rf_setAttrib(rvecs, R_NamesSymbol, names);
            Ok(rvecs)
        }
    }
}

impl<D: RNew> RNew for HashMap<String, D> {
    fn rnew(x: SEXP) -> RResult<HashMap<String, D>> {

        unsafe {
            let names = Rf_getAttrib(x, R_NamesSymbol);
            if Rf_isString(names) != Rboolean::TRUE || RTYPEOF(x) != VECSXP {
                return rerror(REKind::NotCompatible("expecting a named list".into()));
            }
            let lens = Rf_xlength(x);
            let mut res = HashMap::new();

            let selfs = x.s();

            for ii in 0..lens {
                res.insert(try!(CStr::from_ptr(R_CHAR(STRING_ELT(names, ii as R_xlen_t)))
                                    .to_owned()
                                    .into_string()),
                           try!(D::rnew(VECTOR_ELT(selfs, ii as R_xlen_t))));
            }
            Ok(res)
        }
    }
}

// hashmap cstring
impl<D: IntoR> IntoR for HashMap<CString, D> {
    fn intor(&self) -> RResult<SEXP> {
        let size_x = self.len();
        unsafe {
            let name = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let names = name.s();
            let rvec = Shield::new(Rf_allocVector(VECSXP, size_x as R_xlen_t));
            let rvecs = rvec.s();
            let mut index = 0;
            for (mname, mitem) in self {
                SET_STRING_ELT(names, index, Rf_mkChar(mname.as_ptr()));
                SET_VECTOR_ELT(rvecs, index, try!(mitem.intor()));
                index = index + 1;
            }
            Rf_setAttrib(rvecs, R_NamesSymbol, names);
            Ok(rvecs)
        }
    }
}

impl<D: RNew> RNew for HashMap<CString, D> {
    fn rnew(x: SEXP) -> RResult<HashMap<CString, D>> {

        unsafe {
            let names = Rf_getAttrib(x, R_NamesSymbol);
            if Rf_isString(names) != Rboolean::TRUE || RTYPEOF(x) != VECSXP {
                return rerror(REKind::NotCompatible("expecting a named list".into()));
            }
            let lens = Rf_xlength(x);
            let mut res = HashMap::new();

            let selfs = x.s();

            for ii in 0..lens {
                res.insert(CStr::from_ptr(R_CHAR(STRING_ELT(names, ii as R_xlen_t))).to_owned(),
                           try!(D::rnew(VECTOR_ELT(selfs, ii as R_xlen_t))));
            }
            Ok(res)
        }
    }
}

// btreemap /////////////////////////

impl<D: IntoR> IntoR for BTreeMap<String, D> {
    fn intor(&self) -> RResult<SEXP> {
        let size_x = self.len();
        unsafe {
            let name = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let names = name.s();
            let rvec = Shield::new(Rf_allocVector(VECSXP, size_x as R_xlen_t));
            let rvecs = rvec.s();
            let mut index = 0;
            for (mname, mitem) in self {
                let strs: &str = mname.as_ref();
                SET_STRING_ELT(names, index, Rf_mkChar(try!(CString::new(strs)).as_ptr()));
                SET_VECTOR_ELT(rvecs, index, try!(mitem.intor()));
                index = index + 1;
            }
            Rf_setAttrib(rvecs, R_NamesSymbol, names);
            Ok(rvecs)
        }
    }
}

impl<D: RNew> RNew for BTreeMap<String, D> {
    fn rnew(x: SEXP) -> RResult<BTreeMap<String, D>> {

        unsafe {
            let names = Rf_getAttrib(x, R_NamesSymbol);
            if Rf_isString(names) != Rboolean::TRUE || RTYPEOF(x) != VECSXP {
                return rerror(REKind::NotCompatible("expecting a named list".into()));
            }
            let lens = Rf_xlength(x);
            let mut res = BTreeMap::new();

            let selfs = x.s();

            for ii in 0..lens {
                res.insert(try!(CStr::from_ptr(R_CHAR(STRING_ELT(names, ii as R_xlen_t)))
                                    .to_owned()
                                    .into_string()),
                           try!(D::rnew(VECTOR_ELT(selfs, ii as R_xlen_t))));
            }
            Ok(res)
        }
    }
}

// BTreeMap cstring
impl<D: IntoR> IntoR for BTreeMap<CString, D> {
    fn intor(&self) -> RResult<SEXP> {
        let size_x = self.len();
        unsafe {
            let name = Shield::new(Rf_allocVector(STRSXP, size_x as R_xlen_t));
            let names = name.s();
            let rvec = Shield::new(Rf_allocVector(VECSXP, size_x as R_xlen_t));
            let rvecs = rvec.s();
            let mut index = 0;
            for (mname, mitem) in self {
                SET_STRING_ELT(names, index, Rf_mkChar(mname.as_ptr()));
                SET_VECTOR_ELT(rvecs, index, try!(mitem.intor()));
                index = index + 1;
            }
            Rf_setAttrib(rvecs, R_NamesSymbol, names);
            Ok(rvecs)
        }
    }
}

impl<D: RNew> RNew for BTreeMap<CString, D> {
    fn rnew(x: SEXP) -> RResult<BTreeMap<CString, D>> {

        unsafe {
            let names = Rf_getAttrib(x, R_NamesSymbol);
            if Rf_isString(names) != Rboolean::TRUE || RTYPEOF(x) != VECSXP {
                return rerror(REKind::NotCompatible("expecting a named list".into()));
            }
            let lens = Rf_xlength(x);
            let mut res = BTreeMap::new();

            let selfs = x.s();

            for ii in 0..lens {
                res.insert(CStr::from_ptr(R_CHAR(STRING_ELT(names, ii as R_xlen_t))).to_owned(),
                           try!(D::rnew(VECTOR_ELT(selfs, ii as R_xlen_t))));
            }
            Ok(res)
        }
    }
}




// ask your self fromr or rnew
// string ////////////////////////////////
impl URNew for CString {
    unsafe fn urnew(x: SEXP) -> CString {

        let rptr: SEXP = STRING_ELT(x, 0);
        let res = CStr::from_ptr(R_CHAR(rptr));
        res.to_owned()

    }
}

impl RNew for CString {
    fn rnew(x: SEXP) -> RResult<CString> {
        unsafe {
            if RTYPEOF(x) != STRSXP || Rf_xlength(x) != 1 {
                return rerror(REKind::NotCompatible("expecting a string".into()));
            }
            Ok(Self::urnew(x))
        }
    }
}

impl RNew for String {
    fn rnew(x: SEXP) -> RResult<String> {
        let res = try!(CString::rnew(x));
        let res_string: String = try!(res.into_string());
        Ok(res_string)
    }
}


impl IntoR for CString {
    fn intor(&self) -> RResult<SEXP> {
        unsafe { Ok(Self::uintor(self)) }
    }
}

impl UIntoR for CString {
    unsafe fn uintor(&self) -> SEXP {

        let rvec = Shield::new(Rf_allocVector(STRSXP, 1));
        SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(self.as_ptr()));
        rvec.s()

    }
}

impl IntoR for String {
    fn intor(&self) -> RResult<SEXP> {
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, 1));
            let strs: &str = self.as_ref();
            SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(try!(CString::new(strs)).as_ptr()));
            Ok(rvec.s())
        }
    }
}

impl<'a> IntoR for &'a str {
    fn intor(&self) -> RResult<SEXP> {
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, 1));
            SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(try!(CString::new(*self)).as_ptr()));
            Ok(rvec.s())
        }
    }
}

impl<'a> UIntoR for &'a str {
    unsafe fn uintor(&self) -> SEXP {
        let rvec = Shield::new(Rf_allocVector(STRSXP, 1));
        SET_STRING_ELT(rvec.s(), 0, Rf_mkChar(c_str(*self).as_ptr()));
        rvec.s()
    }
}

// string collections macro

macro_rules! gen_string_collections{
	( $(($x:ident;$push:ident)),*) =>
	(
$(
	
impl UIntoR for $x<CString> {
    unsafe fn uintor(&self) -> SEXP {
    	let lens = self.len();
            let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
            let mut index = 0;
            for xs in self{
            	SET_STRING_ELT( rvec.s(), index, Rf_mkChar(xs.as_ptr()));
            	index = index + 1;
            } 
            rvec.s()
    }
}

impl IntoR for $x<CString> {
    fn intor(&self) -> RResult<SEXP> {
    	unsafe{Ok(Self::uintor(self))}
    }
}

impl IntoR for $x<String> {
    fn intor(&self) -> RResult<SEXP> {
    	let lens = self.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
            let mut index = 0;
            for xs in self{
            	let strs :&str = xs.as_ref();
            	SET_STRING_ELT( rvec.s(), index, Rf_mkChar(try!(CString::new(strs)).as_ptr()));
            	index = index + 1;
            } 
           Ok(rvec.s())
        }
    }
}

impl URNew for $x<CString> {
	unsafe fn urnew(x: SEXP) -> $x<CString> {
			let lens = Rf_xlength(x);
			let mut vecs = $x::with_capacity(lens as usize);
			for ii in 0..lens{
				let rptr : SEXP = STRING_ELT(x,ii);
				let res = CStr::from_ptr(R_CHAR(rptr));
				vecs.$push(res.to_owned());
			}
			vecs
	}
}


impl RNew for $x<CString> {
	fn rnew(x: SEXP) -> RResult<$x<CString>> {
		unsafe {
			if  RTYPEOF(x) != STRSXP {
    			return rerror(REKind::NotCompatible(
    					concat!(stringify!($x),"<CString>::rnew expecting a string").into()
    			));
			}
			Ok(Self::urnew(x))
		}
	}
}

impl RNew for $x<String> {
	fn rnew(x:SEXP) -> RResult<$x<String>> {
		unsafe {
			if RTYPEOF(x) != STRSXP  {
    			return rerror(REKind::NotCompatible(
    					concat!(stringify!($x),"<String>::rnew expecting a string").into()));
			}
			
			let lens = Rf_xlength(x);
			let mut vecs = $x::with_capacity(lens as usize);
			for ii in 0..lens{
				let rptr : SEXP = STRING_ELT(x,ii);
				let res = CStr::from_ptr(R_CHAR(rptr));
				vecs.$push(try!(res.to_owned().into_string()));
			}
			Ok(vecs)
		}
	}
})*)
}

gen_string_collections!((Vec;push),(VecDeque;push_back),(BinaryHeap;push),(HashSet;insert));


// borrow

impl<'a> UIntoR for &'a [CString] {
    unsafe fn uintor(&self) -> SEXP {
        let lens = self.len();
        let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
        let mut index = 0;
        for xs in *self {
            SET_STRING_ELT(rvec.s(), index, Rf_mkChar(xs.as_ptr()));
            index = index + 1;
        }
        rvec.s()
    }
}

impl<'a> IntoR for &'a [CString] {
    fn intor(&self) -> RResult<SEXP> {
        unsafe { Ok(Self::uintor(self)) }
    }
}

impl<'a> IntoR for &'a [String] {
    fn intor(&self) -> RResult<SEXP> {
        let lens = self.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
            let mut index = 0;
            for xs in *self {
                let strs: &str = xs.as_ref();
                SET_STRING_ELT(rvec.s(),
                               index,
                               Rf_mkChar(try!(CString::new(strs)).as_ptr()));
                index = index + 1;
            }
            Ok(rvec.s())
        }
    }
}

impl<'a, 'b> IntoR for &'a [&'b str] {
    fn intor(&self) -> RResult<SEXP> {
        let lens = self.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
            let mut index = 0;
            for xs in *self {
                SET_STRING_ELT(rvec.s(), index, Rf_mkChar(try!(CString::new(*xs)).as_ptr()));
                index = index + 1;
            }
            Ok(rvec.s())
        }
    }
}

macro_rules! gen_string_collections_linkedlist{
	( $(($x:ident;$push:ident)),*) =>
	(
$(

impl UIntoR for $x<CString> {
    unsafe fn uintor(&self) -> SEXP {
    	let lens = self.len();
            let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
            let mut index = 0;
            for xs in self{
            	SET_STRING_ELT( rvec.s(), index, Rf_mkChar(xs.as_ptr()));
            	index = index + 1;
            } 
            rvec.s()
    }
}


impl IntoR for $x<CString> {
    fn intor(&self) -> RResult<SEXP> {
    	
            unsafe{ Ok(Self::uintor(self))}
    }
}

impl IntoR for $x<String> {
    fn intor(&self) -> RResult<SEXP> {
    	let lens = self.len();
        unsafe {
            let rvec = Shield::new(Rf_allocVector(STRSXP, lens as R_xlen_t));
            let mut index = 0;
            for xs in self{
            	let strs :&str = xs.as_ref();
            	SET_STRING_ELT( rvec.s(), index, Rf_mkChar(try!(CString::new(strs)).as_ptr()));
            	index = index + 1;
            } 
            Ok(rvec.s())
        }
    }
}

impl URNew for $x<CString> {
	unsafe fn urnew(x: SEXP) -> $x<CString> {

			let lens = Rf_xlength(x);
			let mut vecs = $x::new();
			for ii in 0..lens{
				let rptr : SEXP = STRING_ELT(x,ii);
				let res = CStr::from_ptr(R_CHAR(rptr));
				vecs.$push(res.to_owned());
			}
			vecs
		}
}

impl RNew for $x<CString> {
	fn rnew(x: SEXP) -> RResult<$x<CString>> {
		unsafe {
			if  RTYPEOF(x) != STRSXP  {
    			return rerror(REKind::NotCompatible(
    					concat!(stringify!($x),"<CString>::rnew expecting a string").into()
    			));
			}

			Ok(Self::urnew(x))
		}
	}
}

impl RNew for $x<String> {
	fn rnew(x:SEXP) -> RResult<$x<String>> {
		unsafe {
			if RTYPEOF(x) != STRSXP {
    			return rerror(REKind::NotCompatible(
    					concat!(stringify!($x),"<String>::rnew expecting a string").into()));
			}
			
			let lens = Rf_xlength(x);
			let mut vecs = $x::new();
			for ii in 0..lens{
				let rptr : SEXP = STRING_ELT(x,ii);
				let res = CStr::from_ptr(R_CHAR(rptr));
				vecs.$push(try!(res.to_owned().into_string()));
			}
			Ok(vecs)
		}
	}
})*)
}

gen_string_collections_linkedlist!((BTreeSet;insert),(LinkedList;push_front));
