use super::*;

// #[rustr_export]
pub fn charvec_at(x:CharVec,y:usize)->RResult<String>{
    x.at(y)
}

// #[rustr_export]
pub fn numvec_at(x:NumVec,y:usize)->f64{
    x.at(y).unwrap_or(0.0)
}

// #[rustr_export]
pub fn ref_numvec_at(x:&NumVec,y:usize)->f64{
    x.at(y).unwrap_or(0.0)
}

// #[rustr_export]
pub fn ref_mut_numvec_at(x:&NumVec,y:& mut usize)->f64{
    x.at(*y).unwrap_or(0.0)
}

// vector ----------------------------

// #[rustr_export]
pub fn clist()-> RResult<RList>{

    Ok(rlist!("sd","Sd"))
}

// #[rustr_export]
pub fn nlist()-> RResult<RList>{

    Ok(rlist!(sd ~ "sd", Sd ~ "Sd"))
}

// #[rustr_export]
pub fn uclist()-> RList{

    unsafe{urlist!("sd","Sd")}
}

// #[rustr_export]
pub fn unlist()-> RList{

    unsafe{urlist!(sd ~ "sd", Sd ~ "Sd")}
}

// #[rustr_export]
pub fn list_data_frame(x:RList)-> RResult<RList>{
   let mut y = x;
   try!(y.as_data_frame());
   Ok(y)
}

// charvec

// #[rustr_export]
pub fn charvec()-> RResult<CharVec>{

    Ok(charvec!("sd","Sd"))
}

// #[rustr_export]
pub fn ncharvec()-> RResult<CharVec>{

    Ok(charvec!(sd ~ "sd", Sd ~ "Sd"))
}

// #[rustr_export]
pub fn ucharvec()-> CharVec{

    unsafe{ucharvec!("sd","Sd".into())}
}

// #[rustr_export]
pub fn uncharvec()-> CharVec{

    unsafe{ucharvec!(sd ~ "sd", Sd ~ "Sd")}
}

// #[rustr_export]
pub fn charvec_setc()-> RResult<CharVec>{
    let mut res = CharVec::alloc(4);
    try!(res.setc(0*2+0,RChar::na()));
    try!(res.setc(0*2+1,RChar::na()));
    try!(res.setc(1*2+0,RChar::na()));
    try!(res.setc(1*2+1,RChar::na()));
    try!(res.set_dim(&[2,2]));
    Ok(res)
}

// boolvec

// #[rustr_export]
pub fn boolvec()-> BoolVec{

    {boolvec!(true,false)}
}

// #[rustr_export]
pub fn nboolvec()-> BoolVec{

    {boolvec!(sd ~ true, Sd ~ false)}
}

// numvec

// #[rustr_export]
pub fn numvec()-> NumVec{

    {numvec!(1.0,2.0)}
}

// #[rustr_export]
pub fn nnumvec()-> NumVec{

    {numvec!(sd ~ 1.0, Sd ~ 3.0)}
}

// #[rustr_export]
pub fn intvec()-> IntVec{

    {intvec!(1,2)}
}

// #[rustr_export]
pub fn nintvec()-> IntVec{

    {intvec!(sd ~ 1, Sd ~ 3)}
}
