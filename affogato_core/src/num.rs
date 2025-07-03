macro_rules! impl_properties {
    ($zero:tt, $one:tt, $($structure:tt),*) => {
        $(
            impl Zero for $structure {
                const ZERO: Self = $zero;
                fn is_zero(&self) -> bool {
                    *self == $zero
                }
            }
            impl One for $structure {
                const ONE: Self = $one;
                fn is_one(&self) -> bool {
                    *self == $one
                }
            }
        )*
    };
}

macro_rules! impl_signed {
    ($($structure:tt),*) => {
        $(
            impl Signed for $structure {
                fn flip_sign(self) -> Self {
                    self*-1
                }
                fn is_negative(self) -> bool {
                    $structure::is_negative(self)
                }
                fn is_positive(self) -> bool {
                    $structure::is_positive(self)
                }
                fn abs(self) -> Self {
                    $structure::abs(self)
                }
            }
        )*
    };
}
macro_rules! impl_bounds {
    ($($structure:tt),*) => {
        $(
            impl Bounds for $structure {
                fn min(self, other: Self) -> Self {
                    $structure::min(self, other)
                }
                fn max(self, other: Self) -> Self {
                    $structure::max(self, other)
                }
                const MIN: Self = core::$structure::MIN;
                const MAX: Self = core::$structure::MAX;
            }
        )*
    };

}
macro_rules! impl_bounds_ord {
    ($($structure:tt),*) => {
        $(
            impl Bounds for $structure {
                fn min(self, other: Self) -> Self {
                    <$structure as Ord>::min(self, other)
                }
                fn max(self, other: Self) -> Self {
                    <$structure as Ord>::max(self, other)
                }
                const MIN: Self = core::$structure::MIN;
                const MAX: Self = core::$structure::MAX;
            }
        )*
    };
}
pub trait Zero {
    const ZERO: Self;
    fn set_zero(&mut self) 
        where Self: Sized {
        *self = Self::ZERO;
    }
    fn is_zero(&self) -> bool;
}
pub trait One {
    const ONE: Self;
    fn set_one(&mut self) 
        where Self: Sized {
        *self = Self::ONE;
    }
    fn is_one(&self) -> bool;
}
impl_properties!(0, 1, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_properties!(0.0, 1.0, f32, f64);

/// Implemented for every type A that implements the operations
/// Add, Sub, Mul, Div, Rem and their assign variants for type T.
pub trait UniversalOperationsOn<T> 
    where Self: Sized + 
    core::ops::Add<T, Output = Self> + 
    core::ops::Sub<T, Output = Self> + 
    core::ops::Mul<T, Output = Self> + 
    core::ops::Div<T, Output = Self> +
    core::ops::Rem<T, Output = Self> +
    core::ops::AddAssign<T> +
    core::ops::SubAssign<T> +
    core::ops::MulAssign<T> +
    core::ops::DivAssign<T> +
    core::ops::RemAssign<T> + {}
impl<T, A> UniversalOperationsOn<T> for A 
    where A: Sized + 
    core::ops::Add<T, Output = Self> + 
    core::ops::Sub<T, Output = Self> + 
    core::ops::Mul<T, Output = Self> + 
    core::ops::Div<T, Output = Self> +
    core::ops::Rem<T, Output = Self> +
    core::ops::AddAssign<T> +
    core::ops::SubAssign<T> +
    core::ops::MulAssign<T> +
    core::ops::DivAssign<T> +
    core::ops::RemAssign<T> {
}
/// Implemented for all types that are signed.
pub trait Signed: core::ops::Neg<Output = Self> + Sized + Copy {
    fn is_negative(self) -> bool;
    fn is_positive(self) -> bool;
    fn abs(self) -> Self;
    fn flip_sign(self) -> Self;
    fn copysign(self, sign: Self) -> Self {
        if self.is_negative() == sign.is_negative() {
            self
        } else {
            self.neg()
        }
    }
}
impl_signed!(i8, i16, i32, i64, i128, isize);
impl Signed for f32 {
    fn flip_sign(self) -> Self {
        self*-1.0
    }
    fn is_negative(self) -> bool {
        f32::is_sign_negative(self)
    }
    fn is_positive(self) -> bool {
        f32::is_sign_positive(self)
    }
    fn abs(self) -> Self {
        f32::abs(self)
    }
}
impl Signed for f64 {
    fn flip_sign(self) -> Self {
        self*-1.0
    }
    fn is_negative(self) -> bool {
        f64::is_sign_negative(self)
    }
    fn is_positive(self) -> bool {
        f64::is_sign_positive(self)
    }
    fn abs(self) -> Self {
        f64::abs(self)
    }
}
pub trait Number 
    where Self: UniversalOperationsOn<Self> + 
    PartialEq + PartialOrd + 
    Clone + Copy +
    One + Zero + 
    Bounds + 
    FromPrimitive + IntoPrimitive {}
impl<T> Number for T
    where Self: UniversalOperationsOn<Self> + 
    PartialEq + PartialOrd + 
    Clone + Copy +
    One + Zero + 
    Bounds + 
    FromPrimitive + IntoPrimitive {}

/// This trait is implemented for all floating point types:
/// [`f32`], [`f64`]
pub trait FloatingPoint: Number + FloatConsts {
    fn acos(self) -> Self;
    fn acosh(self) -> Self;
    fn asin(self) -> Self;
    fn asinh(self) -> Self;
    fn atan(self) -> Self;
    fn atan2(self, other: Self) -> Self;
    fn atanh(self) -> Self;
    fn cbrt(self) -> Self;
    fn ceil(self) -> Self;
    fn cos(self) -> Self;
    fn exp(self) -> Self;
    fn exp2(self) -> Self;
    fn exp_m1(self) -> Self;
    fn floor(self) -> Self;
    fn fract(self) -> Self;
    fn hypot(self, other: Self) -> Self;
    fn ln(self) -> Self;
    fn ln_1p(self) -> Self;
    fn log(self, base: Self) -> Self;
    fn log10(self) -> Self;
    fn log2(self) -> Self;
    fn powf(self, n: Self) -> Self;
    fn powi(self, n: i32) -> Self;
    fn recip(self) -> Self;
    fn round(self) -> Self;
    fn signum(self) -> Self;
    fn sin(self) -> Self;
    fn sinh(self) -> Self;
    fn sqrt(self) -> Self;
    fn tan(self) -> Self;
    fn tanh(self) -> Self;
    fn to_degrees(self) -> Self;
    fn to_radians(self) -> Self;
    fn trunc(self) -> Self;
    fn sin_cos(self) -> (Self, Self)
        where Self: Sized;
    fn is_finite(self) -> bool;
    fn is_infinite(self) -> bool;
    fn is_nan(self) -> bool;
    fn is_normal(self) -> bool;
    fn is_subnormal(self) -> bool;
}
impl FloatingPoint for f32 {
    fn acos(self) -> Self {
        crate::cmath::acosf(self)
    }
    fn acosh(self) -> Self {
        crate::cmath::acoshf(self)
    }
    fn asin(self) -> Self {
        crate::cmath::asinf(self)
    }
    fn asinh(self) -> Self {
        crate::cmath::asinhf(self)
    }
    fn atan(self) -> Self {
        crate::cmath::atanf(self)
    }
    fn atan2(self, other: Self) -> Self {
        crate::cmath::atan2f(self, other)
    }
    fn atanh(self) -> Self {
        crate::cmath::atanhf(self)
    }
    fn cbrt(self) -> Self {
        crate::cmath::cbrtf(self)
    }
    fn ceil(self) -> Self {
        crate::cmath::ceilf(self)
    }
    fn cos(self) -> Self {
        crate::cmath::cosf(self)
    }
    fn exp(self) -> Self {
        crate::cmath::expf(self)
    }
    fn exp2(self) -> Self {
        crate::cmath::exp2f(self)
    }
    fn exp_m1(self) -> Self {
        crate::cmath::expm1f(self)
    }
    fn floor(self) -> Self {
        crate::cmath::floorf(self)
    }
    fn fract(self) -> Self {
        self - self.trunc()
    }
    fn hypot(self, other: Self) -> Self {
        crate::cmath::hypotf(self, other)
    }
    fn is_finite(self) -> bool {
        self.abs() < Self::INFINITY
    }
    fn is_infinite(self) -> bool {
        (self == f32::INFINITY) | (self == f32::NEG_INFINITY)
    }
    fn is_nan(self) -> bool {
        self != self
    }
    fn is_normal(self) -> bool {
        matches!(self.classify(), core::num::FpCategory::Normal)
    }
    fn is_subnormal(self) -> bool {
        matches!(self.classify(), core::num::FpCategory::Subnormal)
    }
    fn ln(self) -> Self {
        crate::cmath::logf(self)
    }
    fn ln_1p(self) -> Self {
        crate::cmath::log1pf(self)
    }
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }
    fn log10(self) -> Self {
        crate::cmath::log10f(self)
    }
    fn log2(self) -> Self {
        crate::cmath::log2f(self)
    }
    fn powf(self, n: Self) -> Self {
        crate::cmath::powf(self, n)
    }
    fn powi(self, n: i32) -> Self {
        crate::cmath::powf(self, n as f32)
    }
    fn recip(self) -> Self {
        f32::ONE/self
    }
    fn round(self) -> Self {
        crate::cmath::roundf(self)
    }
    fn signum(self) -> Self {
        if self.is_nan() { Self::NAN } else { 1.0_f32.copysign(self) }
    }
    fn sin(self) -> Self {
        crate::cmath::sinf(self)
    }
    fn sin_cos(self) -> (Self, Self)
            where Self: Sized {
        (self.sin(), self.cos())
    }
    fn sinh(self) -> Self {
        crate::cmath::sinhf(self)
    }
    fn sqrt(self) -> Self {
        crate::cmath::sqrtf(self)
    }
    fn tan(self) -> Self {
        crate::cmath::tanf(self)
    }
    fn tanh(self) -> Self {
        crate::cmath::tanhf(self)
    }
    fn to_degrees(self) -> Self {
        const PIS_IN_180: f32 = 57.2957795130823208767981548141051703_f32;
        self * PIS_IN_180
    }
    fn to_radians(self) -> Self {
        const RADS_PER_DEG: f32 = core::f32::consts::PI / 180.0;
        self * RADS_PER_DEG
    }
    fn trunc(self) -> Self {
        crate::cmath::truncf(self)
    }
}
impl FloatingPoint for f64 {
    fn acos(self) -> Self {
        crate::cmath::acos(self)
    }
    fn acosh(self) -> Self {
        crate::cmath::acosh(self)
    }
    fn asin(self) -> Self {
        crate::cmath::asin(self)
    }
    fn asinh(self) -> Self {
        crate::cmath::asinh(self)
    }
    fn atan(self) -> Self {
        crate::cmath::atan(self)
    }
    fn atan2(self, other: Self) -> Self {
        crate::cmath::atan2(self, other)
    }
    fn atanh(self) -> Self {
        crate::cmath::atanh(self)
    }
    fn cbrt(self) -> Self {
        crate::cmath::cbrt(self)
    }
    fn ceil(self) -> Self {
        crate::cmath::ceil(self)
    }
    fn cos(self) -> Self {
        crate::cmath::cos(self)
    }
    fn exp(self) -> Self {
        crate::cmath::exp(self)
    }
    fn exp2(self) -> Self {
        crate::cmath::exp2(self)
    }
    fn exp_m1(self) -> Self {
        crate::cmath::expm1(self)
    }
    fn floor(self) -> Self {
        crate::cmath::floor(self)
    }
    fn fract(self) -> Self {
        self - self.trunc()
    }
    fn hypot(self, other: Self) -> Self {
        crate::cmath::hypot(self, other)
    }
    fn is_finite(self) -> bool {
        self.abs() < Self::INFINITY
    }
    fn is_infinite(self) -> bool {
        (self == Self::INFINITY) | (self == Self::NEG_INFINITY)
    }
    fn is_nan(self) -> bool {
        self != self
    }
    fn is_normal(self) -> bool {
        matches!(self.classify(), core::num::FpCategory::Normal)
    }
    fn is_subnormal(self) -> bool {
        matches!(self.classify(), core::num::FpCategory::Subnormal)
    }
    fn ln(self) -> Self {
        crate::cmath::log(self)
    }
    fn ln_1p(self) -> Self {
        crate::cmath::log1p(self)
    }
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }
    fn log10(self) -> Self {
        crate::cmath::log10(self)
    }
    fn log2(self) -> Self {
        crate::cmath::log2(self)
    }
    fn powf(self, n: Self) -> Self {
        crate::cmath::pow(self, n)
    }
    fn powi(self, n: i32) -> Self {
        crate::cmath::pow(self, n as f64)
    }
    fn recip(self) -> Self {
        f64::ONE/self
    }
    fn round(self) -> Self {
        crate::cmath::round(self)
    }
    fn signum(self) -> Self {
        if self.is_nan() { Self::NAN } else { 1.0_f64.copysign(self) }
    }
    fn sin(self) -> Self {
        crate::cmath::sin(self)
    }
    fn sin_cos(self) -> (Self, Self)
            where Self: Sized {
        (self.sin(), self.cos())
    }
    fn sinh(self) -> Self {
        crate::cmath::sinh(self)
    }
    fn sqrt(self) -> Self {
        crate::cmath::sqrt(self)
    }
    fn tan(self) -> Self {
        crate::cmath::tan(self)
    }
    fn tanh(self) -> Self {
        crate::cmath::tanh(self)
    }
    fn to_degrees(self) -> Self {
        self * (180.0f64 / core::f64::consts::PI)
    }
    fn to_radians(self) -> Self {
        const RADS_PER_DEG: f64 = core::f64::consts::PI / 180.0;
        self * RADS_PER_DEG
    }
    fn trunc(self) -> Self {
        crate::cmath::trunc(self)
    }
}

pub trait Bounds {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    
    const MIN: Self;
    const MAX: Self;
}
impl_bounds!(f32, f64);
impl_bounds_ord!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);

pub trait FloatConsts {
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

impl FloatConsts for f32 {
    
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
impl FloatConsts for f64 {
    
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
impl_from_primitive!(u8, u16, u32, u64, u128, usize, i8, i16, i32, i64, i128, isize, f32, f64);
