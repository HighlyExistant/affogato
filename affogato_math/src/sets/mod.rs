mod natural;
use core::fmt::Debug;

pub use natural::*;
mod properties;
pub use properties::*;

macro_rules! impl_structures {
    ($trait_impl:tt, $($structure:tt),*) => {
        $(
            impl $trait_impl for $structure {}
        )*
    };
}
macro_rules! impl_from_primitive {
    ($($structure:tt),*) => {
        $(
            impl FromPrimitive for $structure {
                fn from_u8(val: u8) -> Self {
                    val as Self
                }
                fn from_u16(val: u16) -> Self {
                    val as Self
                }
                fn from_u32(val: u32) -> Self {
                    val as Self
                }
                fn from_u64(val: u64) -> Self {
                    val as Self
                }
                fn from_u128(val: u128) -> Self {
                    val as Self
                }
                fn from_usize(val: usize) -> Self {
                    val as Self
                }
                fn from_i8(val: i8) -> Self {
                    val as Self
                }
                fn from_i16(val: i16) -> Self {
                    val as Self
                }
                fn from_i32(val: i32) -> Self {
                    val as Self
                }
                fn from_i64(val: i64) -> Self {
                    val as Self
                }
                fn from_i128(val: i128) -> Self {
                    val as Self
                }
                fn from_isize(val: isize) -> Self {
                    val as Self
                }
                fn from_f32(val: f32) -> Self {
                    val as Self
                }
                fn from_f64(val: f64) -> Self {
                    val as Self
                }
            }

            impl IntoPrimitive for $structure {
                fn to_u8(self) -> u8 {
                    self as u8
                }
                fn to_u16(self) -> u16 {
                    self as u16
                }
                fn to_u32(self) -> u32 {
                    self as u32
                }
                fn to_u64(self) -> u64 {
                    self as u64
                }
                fn to_u128(self) -> u128 {
                    self as u128
                }
                fn to_usize(self) -> usize {
                    self as usize
                }
                fn to_i8(self) -> i8 {
                    self as i8
                }
                fn to_i16(self) -> i16 {
                    self as i16
                }
                fn to_i32(self) -> i32 {
                    self as i32
                }
                fn to_i64(self) -> i64 {
                    self as i64
                }
                fn to_i128(self) -> i128 {
                    self as i128
                }
                fn to_isize(self) -> isize {
                    self as isize
                }
                fn to_f32(self) -> f32 {
                    self as f32
                }
                fn to_f64(self) -> f64 {
                    self as f64
                }
            }
        )*
    };
}

pub trait Cardinal: UsesArithmetic + Number {}
pub trait Integer: HasNegatives + UsesArithmetic + Number {}
pub trait FromPrimitive {
    fn from_u8(val: u8) -> Self;
    fn from_u16(val: u16) -> Self;
    fn from_u32(val: u32) -> Self;
    fn from_u64(val: u64) -> Self;
    fn from_u128(val: u128) -> Self;
    fn from_usize(val: usize) -> Self;
    fn from_i8(val: i8) -> Self;
    fn from_i16(val: i16) -> Self;
    fn from_i32(val: i32) -> Self;
    fn from_i64(val: i64) -> Self;
    fn from_i128(val: i128) -> Self;
    fn from_isize(val: isize) -> Self;
    fn from_f32(val: f32) -> Self;
    fn from_f64(val: f64) -> Self;
}
pub trait IntoPrimitive {
    fn to_u8(self) -> u8;
    fn to_u16(self) -> u16;
    fn to_u32(self) -> u32;
    fn to_u64(self) -> u64;
    fn to_u128(self) -> u128;
    fn to_usize(self) -> usize;
    fn to_i8(self) -> i8;
    fn to_i16(self) -> i16;
    fn to_i32(self) -> i32;
    fn to_i64(self) -> i64;
    fn to_i128(self) -> i128;
    fn to_isize(self) -> isize;
    fn to_f32(self) -> f32;
    fn to_f64(self) -> f64;
}
pub trait Real: HasNegatives + UsesArithmetic + FloatingPoint + Number + Debug {
    
    /// Archimedes' constant (π)
    const PI: Self;

    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    const TAU: Self;

    /// The golden ratio (φ)
    const PHI: Self;

    /// The Euler-Mascheroni constant (γ)
    const EGAMMA: Self;

    /// π/2
    const FRAC_PI_2: Self;

    /// π/3
    const FRAC_PI_3: Self;

    /// π/4
    const FRAC_PI_4: Self;

    /// π/6
    const FRAC_PI_6: Self;

    /// π/8
    const FRAC_PI_8: Self;

    /// 1/π
    const FRAC_1_PI: Self;

    /// 1/sqrt(π)
    const FRAC_1_SQRT_PI: Self;

    /// 2/π
    const FRAC_2_PI: Self;

    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: Self;

    /// sqrt(2)
    const SQRT_2: Self;

    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: Self;

    /// sqrt(3)
    const SQRT_3: Self;

    /// 1/sqrt(3)
    const FRAC_1_SQRT_3: Self;

    /// Euler's number (e)
    const E: Self;

    /// log<sub>2</sub>(e)
    const LOG2_E: Self;

    /// log<sub>2</sub>(10)
    const LOG2_10: Self;

    /// log<sub>10</sub>(e)
    const LOG10_E: Self;

    /// log<sub>10</sub>(2)
    const LOG10_2: Self;

    /// ln(2)
    const LN_2: Self;

    /// ln(10)
    const LN_10: Self;

    /// epsilone
    const EPSILON: Self;
}
/// A helper trait to distinguish all numbers that are not Real numbers. A Real number
pub trait HasRealProduct<T: Real, Out>: core::ops::Mul<T, Output = Out> + core::ops::Div<T, Output = Out>
    where Self: Sized {}
impl<T: Real, Out, V> HasRealProduct<T, Out> for V
    where V: core::ops::Mul<T, Output = Out> + core::ops::Div<T, Output = Out> {

}
mod sealed {
    #![allow(unused)]
    use super::{NaturalU8, NaturalU16, NaturalU32, NaturalU64, NaturalU128, NaturalUsize};
    pub trait Sealed {}
    impl_structures!(Sealed, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, NaturalU8, NaturalU16, NaturalU32, NaturalU64, NaturalU128, NaturalUsize);

    /// Helper trait for all non rational numbers. A member should not implement Reals and NotRational.
    pub trait NotRational: Sealed {}

}
pub use sealed::NotRational;
pub trait Set {}
pub struct Naturals;
pub struct Cardinals;
pub struct Integers;
pub struct Reals;
impl_from_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
impl_structures!(NotRational, u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, NaturalU8, NaturalU16, NaturalU32, NaturalU64, NaturalU128, NaturalUsize);
impl_structures!(Set, Naturals, Cardinals, Integers, Reals);
impl_structures!(Cardinal, u8, u16, u32, u64, u128, usize);
impl_structures!(Integer, i8, i16, i32, i64, i128, isize);
impl Real for f32 {
    
    /// Archimedes' constant (π)
    const PI: f32 = 3.14159265358979323846264338327950288_f32;

    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    const TAU: f32 = 6.28318530717958647692528676655900577_f32;

    /// The golden ratio (φ)
    const PHI: f32 = 1.618033988749894848204586834365638118_f32;

    /// The Euler-Mascheroni constant (γ)
    const EGAMMA: f32 = 0.577215664901532860606512090082402431_f32;

    /// π/2
    const FRAC_PI_2: f32 = 1.57079632679489661923132169163975144_f32;

    /// π/3
    const FRAC_PI_3: f32 = 1.04719755119659774615421446109316763_f32;

    /// π/4
    const FRAC_PI_4: f32 = 0.785398163397448309615660845819875721_f32;

    /// π/6
    const FRAC_PI_6: f32 = 0.52359877559829887307710723054658381_f32;

    /// π/8
    const FRAC_PI_8: f32 = 0.39269908169872415480783042290993786_f32;

    /// 1/π
    const FRAC_1_PI: f32 = 0.318309886183790671537767526745028724_f32;

    /// 1/sqrt(π)
    const FRAC_1_SQRT_PI: f32 = 0.564189583547756286948079451560772586_f32;

    /// 2/π
    const FRAC_2_PI: f32 = 0.636619772367581343075535053490057448_f32;

    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: f32 = 1.12837916709551257389615890312154517_f32;

    /// sqrt(2)
    const SQRT_2: f32 = 1.41421356237309504880168872420969808_f32;

    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: f32 = 0.707106781186547524400844362104849039_f32;

    /// sqrt(3)
    const SQRT_3: f32 = 1.732050807568877293527446341505872367_f32;

    /// 1/sqrt(3)
    const FRAC_1_SQRT_3: f32 = 0.577350269189625764509148780501957456_f32;

    /// Euler's number (e)
    const E: f32 = 2.71828182845904523536028747135266250_f32;

    /// log<sub>2</sub>(e)
    const LOG2_E: f32 = 1.44269504088896340735992468100189214_f32;

    /// log<sub>2</sub>(10)
    const LOG2_10: f32 = 3.32192809488736234787031942948939018_f32;

    /// log<sub>10</sub>(e)
    const LOG10_E: f32 = 0.434294481903251827651128918916605082_f32;

    /// log<sub>10</sub>(2)
    const LOG10_2: f32 = 0.301029995663981195213738894724493027_f32;

    /// ln(2)
    const LN_2: f32 = 0.693147180559945309417232121458176568_f32;

    /// ln(10)
    const LN_10: f32 = 2.30258509299404568401799145468436421_f32;
    
    /// epsilon
    const EPSILON: f32 = 1.19209290e-07_f32;
}
impl Real for f64 {
    
    /// Archimedes' constant (π)
    const PI: f64 = 3.14159265358979323846264338327950288_f64;

    /// The full circle constant (τ)
    ///
    /// Equal to 2π.
    const TAU: f64 = 6.28318530717958647692528676655900577_f64;

    /// The golden ratio (φ)
    const PHI: f64 = 1.618033988749894848204586834365638118_f64;

    /// The Euler-Mascheroni constant (γ)
    const EGAMMA: f64 = 0.577215664901532860606512090082402431_f64;

    /// π/2
    const FRAC_PI_2: f64 = 1.57079632679489661923132169163975144_f64;

    /// π/3
    const FRAC_PI_3: f64 = 1.04719755119659774615421446109316763_f64;

    /// π/4
    const FRAC_PI_4: f64 = 0.785398163397448309615660845819875721_f64;

    /// π/6
    const FRAC_PI_6: f64 = 0.52359877559829887307710723054658381_f64;

    /// π/8
    const FRAC_PI_8: f64 = 0.39269908169872415480783042290993786_f64;

    /// 1/π
    const FRAC_1_PI: f64 = 0.318309886183790671537767526745028724_f64;

    /// 1/sqrt(π)
    const FRAC_1_SQRT_PI: f64 = 0.564189583547756286948079451560772586_f64;

    /// 2/π
    const FRAC_2_PI: f64 = 0.636619772367581343075535053490057448_f64;

    /// 2/sqrt(π)
    const FRAC_2_SQRT_PI: f64 = 1.12837916709551257389615890312154517_f64;

    /// sqrt(2)
    const SQRT_2: f64 = 1.41421356237309504880168872420969808_f64;

    /// 1/sqrt(2)
    const FRAC_1_SQRT_2: f64 = 0.707106781186547524400844362104849039_f64;

    /// sqrt(3)
    const SQRT_3: f64 = 1.732050807568877293527446341505872367_f64;

    /// 1/sqrt(3)
    const FRAC_1_SQRT_3: f64 = 0.577350269189625764509148780501957456_f64;

    /// Euler's number (e)
    const E: f64 = 2.71828182845904523536028747135266250_f64;

    /// log<sub>2</sub>(e)
    const LOG2_E: f64 = 1.44269504088896340735992468100189214_f64;

    /// log<sub>2</sub>(10)
    const LOG2_10: f64 = 3.32192809488736234787031942948939018_f64;

    /// log<sub>10</sub>(e)
    const LOG10_E: f64 = 0.434294481903251827651128918916605082_f64;

    /// log<sub>10</sub>(2)
    const LOG10_2: f64 = 0.301029995663981195213738894724493027_f64;

    /// ln(2)
    const LN_2: f64 = 0.693147180559945309417232121458176568_f64;

    /// ln(10)
    const LN_10: f64 = 2.30258509299404568401799145468436421_f64;

    /// ln(10)
    const EPSILON: f64 = 2.2204460492503131e-16_f64;
}


pub struct RationalNumber;
pub struct NotRationalNumber;
pub trait NumberType {}
impl NumberType for RationalNumber {}
impl NumberType for NotRationalNumber {}