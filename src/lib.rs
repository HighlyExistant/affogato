use std::ops::{Add, DivAssign, Mul, Sub};

use algebra::Quaternion;
use linear::Vector3;
use num_traits::AsPrimitive;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod linear;
pub mod algebra;
pub mod geometry;
pub mod polynomial;

pub trait Number:
    Copy
    + Clone
    + num_traits::Num
    + num_traits::NumCast
    + std::cmp::PartialOrd
    + std::ops::AddAssign
    + std::ops::SubAssign
    + std::ops::MulAssign
    + std::ops::DivAssign
    + std::ops::RemAssign
    + PartialEq
    + PartialOrd
    + num_traits::Bounded
    + num_traits::Zero
    + num_traits::One
{
}
impl<T> Number for T where
    T: Copy
        + Clone
        + num_traits::Num
        + num_traits::NumCast
        + std::cmp::PartialOrd
        + std::ops::AddAssign
        + std::ops::SubAssign
        + std::ops::MulAssign
        + std::ops::DivAssign
        + std::ops::RemAssign
        + PartialEq
        + PartialOrd
        + num_traits::Bounded
        {
}


pub trait FloatingPoint:
    num_traits::Float
    + Number
    + num_traits::Signed
    + num_traits::AsPrimitive<f32>
    + num_traits::AsPrimitive<f64>
{
}

impl<T> FloatingPoint for T where
    T:  num_traits::Float
        + Number
        + num_traits::Signed
        + num_traits::AsPrimitive<T>
        + num_traits::AsPrimitive<f32>
        + num_traits::AsPrimitive<f64>
{
}
pub trait SignedNumber: 
    num_traits::Signed 
    + Number {}
impl<T> SignedNumber for T where
    T: num_traits::Signed
    + Number {}

pub trait Rotation<T: FloatingPoint> {
    fn quaternion(&self) -> Quaternion<T>;
    fn euler(&self) -> Vector3<T>;
}
// list of functions that I couldn't think how to categorize

/// classical lerp used to linearly interpolate
/// between two values depending on what variable t
/// says. note that t should be a number between 0.0 - 1.0
pub fn lerp<T: FloatingPoint, V: Add<T, Output = V> + Add<Output = V> + Sub<V, Output = V> +Mul<T, Output = V> + Copy>(a: V, b: V, t: T) -> V {
    a + (b - a) * t
}
pub fn smoothstep<T: FloatingPoint, V: Add<T, Output = V> + Add<Output = V> + Sub<V, Output = V> +Mul<T, Output = V> + Copy>(a: V, b: V, t: T) -> V {
    (a - b * t) * (t * t)
}
pub fn inverse_lerp<T: AsPrimitive<V>, V: FloatingPoint + Sub + DivAssign + AsPrimitive<V>>(from: V, to: V, t: T) -> V {
    (t.as_() - from.as_()) / (to - from)
}
pub fn remap<T: AsPrimitive<V>, V: FloatingPoint + Sub + DivAssign + AsPrimitive<V>>(imin: V, imax: V, omin: V, omax: V, t: T) -> V {
    let t2 = inverse_lerp(imin, imax, t);
    lerp(omin, omax, t2)
}
pub fn bilinear_interpolation<T: FloatingPoint>(q12: T, q22: T, q11: T, q21: T, x: T, y: T) -> T  {
    let r1 = lerp(q12, q22, x);
    let r2 = lerp(q11, q21, x);
    lerp(r1, r2, y)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
