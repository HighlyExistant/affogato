use core::num;
use std::{num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8}, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Rem, RemAssign, Sub, SubAssign}};

use num_traits::{AsPrimitive, Bounded, Float, FromPrimitive, One, Zero};

use crate::{algebra::Quaternion, linear::Vector3};

pub trait NaturalNumber 
    where Self: Sized + {}
impl NaturalNumber for NonZeroU8 {}
impl NaturalNumber for NonZeroU16 {}
impl NaturalNumber for NonZeroU32 {}
impl NaturalNumber for NonZeroU64 {}
impl NaturalNumber for NonZeroU128 {}

pub trait WholeNumber
    where Self: Sized + 
    Number {}
    
impl WholeNumber for u8 {}
impl WholeNumber for u16 {}
impl WholeNumber for u32 {}
impl WholeNumber for u64 {}
impl WholeNumber for u128 {}

pub trait IntegerNumber
    where Self: Sized + 
    Number {}
impl IntegerNumber for i8 {}
impl IntegerNumber for i16 {}
impl IntegerNumber for i32 {}
impl IntegerNumber for i64 {}
impl IntegerNumber for i128 {}

pub trait RationalNumber
    where Self: Sized + 
    Float + 
    Number {
    const INF: Self;
    const NEG_INF: Self;
    const NAN: Self;
    const NEG_ZERO: Self;
    const PI: Self;
    const TAU: Self;
    const PHI: Self;
    const EGAMMA: Self;
    const FRAC_PI_2: Self;
    const FRAC_PI_3: Self;
    const FRAC_PI_4: Self;
    const FRAC_PI_6: Self;
    const FRAC_PI_8: Self;
    const FRAC_1_PI: Self;
    const FRAC_1_SQRT_PI: Self;
    const FRAC_2_PI: Self;
    const FRAC_2_SQRT_PI: Self;
    const SQRT_2: Self;
    const FRAC_1_SQRT_2: Self;
    const SQRT_3: Self;
    const FRAC_1_SQRT_3: Self;
    const E: Self;
    const LOG2_E: Self;
    const LOG2_10: Self;
    const LOG10_E: Self;
    const LOG10_2: Self;
    const LN_2: Self;
    const LN_10: Self;
}
impl RationalNumber for f32 {
    const INF: Self = 1.0/0.0;
    const NEG_INF: Self = -1.0/0.0;
    const NAN: Self = 0.0/0.0;
    const NEG_ZERO: Self = -0.0;
    // FIXME: replace with mathematical constants from cmath.
    const PI: Self = 3.14159265358979323846264338327950288_f32;
    const TAU: Self = 6.28318530717958647692528676655900577_f32;
    const PHI: Self = 1.618033988749894848204586834365638118_f32;
    const EGAMMA: Self = 0.577215664901532860606512090082402431_f32;
    const FRAC_PI_2: Self = 1.57079632679489661923132169163975144_f32;
    const FRAC_PI_3: Self = 1.04719755119659774615421446109316763_f32;
    const FRAC_PI_4: Self = 0.785398163397448309615660845819875721_f32;
    const FRAC_PI_6: Self = 0.52359877559829887307710723054658381_f32;
    const FRAC_PI_8: Self = 0.39269908169872415480783042290993786_f32;
    const FRAC_1_PI: Self = 0.318309886183790671537767526745028724_f32;
    const FRAC_1_SQRT_PI: Self = 0.564189583547756286948079451560772586_f32;
    const FRAC_2_PI: Self = 0.636619772367581343075535053490057448_f32;
    const FRAC_2_SQRT_PI: Self = 1.12837916709551257389615890312154517_f32;
    const SQRT_2: Self = 1.41421356237309504880168872420969808_f32;
    const FRAC_1_SQRT_2: Self = 0.707106781186547524400844362104849039_f32;
    const SQRT_3: Self = 1.732050807568877293527446341505872367_f32;
    const FRAC_1_SQRT_3: Self = 0.577350269189625764509148780501957456_f32;
    const E: Self = 2.71828182845904523536028747135266250_f32;
    const LOG2_E: Self = 1.44269504088896340735992468100189214_f32;
    const LOG2_10: Self = 3.32192809488736234787031942948939018_f32;
    const LOG10_E: Self = 0.434294481903251827651128918916605082_f32;
    const LOG10_2: Self = 0.301029995663981195213738894724493027_f32;
    const LN_2: Self = 0.693147180559945309417232121458176568_f32;
    const LN_10: Self = 2.30258509299404568401799145468436421_f32;
}
impl RationalNumber for f64 {
    const INF: Self = 1.0/0.0;
    const NEG_INF: Self = -1.0/0.0;
    const NAN: Self = 0.0/0.0;
    const NEG_ZERO: Self = -0.0;
    const PI: Self = 3.14159265358979323846264338327950288_f64;
    const TAU: Self = 6.28318530717958647692528676655900577_f64;
    const PHI: Self = 1.618033988749894848204586834365638118_f64;
    const EGAMMA: Self = 0.577215664901532860606512090082402431_f64;
    const FRAC_PI_2: Self = 1.57079632679489661923132169163975144_f64;
    const FRAC_PI_3: Self = 1.04719755119659774615421446109316763_f64;
    const FRAC_PI_4: Self = 0.785398163397448309615660845819875721_f64;
    const FRAC_PI_6: Self = 0.52359877559829887307710723054658381_f64;
    const FRAC_PI_8: Self = 0.39269908169872415480783042290993786_f64;
    const FRAC_1_PI: Self = 0.318309886183790671537767526745028724_f64;
    const FRAC_1_SQRT_PI: Self = 0.564189583547756286948079451560772586_f64;
    const FRAC_2_PI: Self = 0.636619772367581343075535053490057448_f64;
    const FRAC_2_SQRT_PI: Self = 1.12837916709551257389615890312154517_f64;
    const SQRT_2: Self = 1.41421356237309504880168872420969808_f64;
    const FRAC_1_SQRT_2: Self = 0.707106781186547524400844362104849039_f64;
    const SQRT_3: Self = 1.732050807568877293527446341505872367_f64;
    const FRAC_1_SQRT_3: Self = 0.577350269189625764509148780501957456_f64;
    const E: Self = 2.71828182845904523536028747135266250_f64;
    const LOG2_E: Self = 1.44269504088896340735992468100189214_f64;
    const LOG2_10: Self = 3.32192809488736234787031942948939018_f64;
    const LOG10_E: Self = 0.434294481903251827651128918916605082_f64;
    const LOG10_2: Self = 0.301029995663981195213738894724493027_f64;
    const LN_2: Self = 0.693147180559945309417232121458176568_f64;
    const LN_10: Self = 2.30258509299404568401799145468436421_f64;
}

pub trait Number 
    where Self: Sized + 
    Add<Output = Self> +
    Sub<Output = Self> +
    Mul<Output = Self> +
    Div<Output = Self> +
    Rem<Output = Self> +
    AddAssign +
    SubAssign +
    MulAssign +
    DivAssign + 
    RemAssign + 
    FromPrimitive + 
    Clone + 
    Copy + 
    One + 
    Zero + 
    Bounded + 
    PartialOrd + 
    {
        const MAX: Self;
        const MIN: Self;
        const ZERO: Self;
        const ONE: Self;
}
pub trait Rotation<T: RationalNumber> {
    fn quaternion(&self) -> Quaternion<T>;
    fn euler(&self) -> Vector3<T>;
}
macro_rules! impl_number {
    ($type_:ident, $zero:expr, $one:expr) => {
        impl Number for $type_ {
            const MAX: Self = std::$type_::MAX;
            const MIN: Self = std::$type_::MIN;
            const ZERO: Self = $zero;
            const ONE: Self = $one;
        }
    };
} 
impl_number!(u8, 0, 1);
impl_number!(u16, 0, 1);
impl_number!(u32, 0, 1);
impl_number!(u64, 0, 1);
impl_number!(u128, 0, 1);
impl_number!(i8, 0, 1);
impl_number!(i16, 0, 1);
impl_number!(i32, 0, 1);
impl_number!(i64, 0, 1);
impl_number!(i128, 0, 1);
impl_number!(f32, 0.0, 1.0);
impl_number!(f64, 0.0, 1.0);