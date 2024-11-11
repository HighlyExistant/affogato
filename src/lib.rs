#![feature(const_fn_floating_point_arithmetic)]
use std::ops::{Add, DivAssign, Mul, Sub};

use algebra::Quaternion;
use linear::Vector3;
use num_traits::AsPrimitive;
use sets::RationalNumber;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}
pub mod linear;
pub mod algebra;
pub mod geometry;
pub mod polynomial;
pub mod spatial;
pub mod sets;
// pub trait ConstantZero {
//     const ZERO: Self;
// }
// impl ConstantZero for i8 { const ZERO: Self = 0; }
// impl ConstantZero for i16 { const ZERO: Self = 0; }
// impl ConstantZero for i32 { const ZERO: Self = 0; }
// impl ConstantZero for i64 { const ZERO: Self = 0; }
// impl ConstantZero for isize { const ZERO: Self = 0; }
// impl ConstantZero for u8 { const ZERO: Self = 0; }
// impl ConstantZero for u16 { const ZERO: Self = 0; }
// impl ConstantZero for u32 { const ZERO: Self = 0; }
// impl ConstantZero for u64 { const ZERO: Self = 0; }
// impl ConstantZero for usize { const ZERO: Self = 0; }
// impl ConstantZero for f32 { const ZERO: Self = 0.0; }
// impl ConstantZero for f64 { const ZERO: Self = 0.0; }
// pub trait Number:
//     Copy
//     + Clone
//     + num_traits::Num
//     + num_traits::NumCast
//     + std::cmp::PartialOrd
//     + std::ops::AddAssign
//     + std::ops::SubAssign
//     + std::ops::MulAssign
//     + std::ops::DivAssign
//     + std::ops::RemAssign
//     + PartialEq
//     + PartialOrd
//     + num_traits::Bounded
//     + num_traits::Zero
//     + num_traits::One
//     + num_traits::FromPrimitive
//     + ConstantZero
// {
// }
// impl<T> Number for T where
//     T: Copy
//         + Clone
//         + num_traits::Num
//         + num_traits::NumCast
//         + std::cmp::PartialOrd
//         + std::ops::AddAssign
//         + std::ops::SubAssign
//         + std::ops::MulAssign
//         + std::ops::DivAssign
//         + std::ops::RemAssign
//         + PartialEq
//         + PartialOrd
//         + num_traits::Bounded
//         + num_traits::FromPrimitive
//         + ConstantZero
//         {
// }


// pub trait RationalNumber:
//     num_traits::Float
//     + Number
//     + num_traits::Signed
//     + num_traits::AsPrimitive<f32>
//     + num_traits::AsPrimitive<f64>
// {
// }

// impl<T> RationalNumber for T where
//     T:  num_traits::Float
//         + Number
//         + num_traits::Signed
//         + num_traits::AsPrimitive<T>
//         + num_traits::AsPrimitive<f32>
//         + num_traits::AsPrimitive<f64>
// {
// }
// pub trait SignedNumber: 
//     num_traits::Signed 
//     + Number {}
// impl<T> SignedNumber for T where
//     T: num_traits::Signed
//     + Number {}

pub trait Rotation<T: RationalNumber> {
    fn quaternion(&self) -> Quaternion<T>;
    fn euler(&self) -> Vector3<T>;
}
// list of functions that I couldn't think how to categorize

/// classical lerp used to linearly interpolate
/// between two values depending on what variable t
/// says. note that t should be a number between 0.0 - 1.0
pub fn lerp<T, V: Add<T, Output = V> + Add<Output = V> + Sub<V, Output = V> + Copy>(a: V, b: V, t: T) -> V 
    where T: RationalNumber,
    V: Mul<T, Output = V> {
    a + (b - a) * t
}
pub fn smoothstep<T: RationalNumber, V: Add<T, Output = V> + Add<Output = V> + Sub<V, Output = V> +Mul<T, Output = V> + Copy>(a: V, b: V, t: T) -> V {
    (a - b * t) * (t * t)
}
pub fn inverse_lerp<T: AsPrimitive<V>, V: RationalNumber + Sub + DivAssign + AsPrimitive<V>>(from: V, to: V, t: T) -> V {
    (t.as_() - from.as_()) / (to - from)
}
pub fn remap<T: AsPrimitive<V>, V: RationalNumber + Sub + DivAssign + AsPrimitive<V>>(imin: V, imax: V, omin: V, omax: V, t: T) -> V {
    let t2 = inverse_lerp(imin, imax, t);
    lerp(omin, omax, t2)
}
pub fn bilinear_interpolation<T: RationalNumber>(q12: T, q22: T, q11: T, q21: T, x: T, y: T) -> T  {
    let r1 = lerp(q12, q22, x);
    let r2 = lerp(q11, q21, x);
    lerp(r1, r2, y)
}

#[cfg(test)]
mod tests {
    use geometry::LinearSegment2D;
    use linear::{DVec3, FVec2, FVec3, SphericalCoordinate};

    use super::*;

    #[test]
    fn it_works() {
        let segment = LinearSegment2D::new(FVec2::from(0.0), FVec2::new(1.0, 0.0));
        println!("{}", segment.length());
    }
    #[test]
    fn spherical() {
        let vector = DVec3::new(1.5, 1.6, 1.7);
        let spherical = SphericalCoordinate::from(vector);
        let back_to = DVec3::from(spherical);
        println!("vector {vector:?}");
        println!("spherical {spherical:?}");
        println!("back_to {back_to:?}");
    }
}
