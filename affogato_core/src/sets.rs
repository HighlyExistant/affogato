use crate::num::{FloatingPoint, Number, Signed};


pub trait Real: Signed + FloatingPoint {
    
}
impl Real for f32 {}
impl Real for f64 {}

pub trait Integer: Signed + Number  {

}
impl Integer for i8 {}
impl Integer for i16 {}
impl Integer for i32 {}
impl Integer for i64 {}
impl Integer for i128 {}
impl Integer for isize {}

pub trait Cardinal: Number  {

}
impl Cardinal for u8 {}
impl Cardinal for u16 {}
impl Cardinal for u32 {}
impl Cardinal for u64 {}
impl Cardinal for u128 {}
impl Cardinal for usize {}