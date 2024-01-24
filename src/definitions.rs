use std::ops::{AddAssign, SubAssign, MulAssign, RemAssign, DivAssign};

use num_traits::{Bounded, NumCast, Num, AsPrimitive, Float, Zero, One, Signed};

use crate::{complex::Quaternion, linear::Vector3};
pub trait Number:
    Copy
    + Clone
    + Num
    + NumCast
    + PartialOrd
    + AddAssign
    + SubAssign
    + MulAssign
    + DivAssign
    + RemAssign
    + PartialEq
    + PartialOrd
    + Bounded
    + Zero
    + One
{
}
impl<T> Number for T where
    T: Copy
        + Clone
        + Num
        + NumCast
        + PartialOrd
        + AddAssign
        + SubAssign
        + MulAssign
        + DivAssign
        + RemAssign
        + PartialEq
        + PartialOrd
        + Bounded
        {
}


pub trait FloatingPoint:
    Float
    + Number
    + Signed
    + AsPrimitive<f32>
    + AsPrimitive<f64>
{
}

impl<T> FloatingPoint for T where
    T:  Float
        + Number
        + Signed
        + AsPrimitive<T>
        + AsPrimitive<f32>
        + AsPrimitive<f64>
{
}
pub trait SignedNumber: 
    Signed 
    + Number {}
impl<T> SignedNumber for T where
    T: Signed
    + Number {}

pub trait Rotation<T: FloatingPoint> {
    fn quaternion(&self) -> Quaternion<T>;
    fn euler(&self) -> Vector3<T>;
}