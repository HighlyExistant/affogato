use core::num::FpCategory;

use super::{FromPrimitive, IntoPrimitive};

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
pub fn clamp<T: PartialOrd>(input: T, min: T, max: T) -> T {
    debug_assert!(min <= max, "min must be less than or equal to max");
    if input < min {
        min
    } else if input > max {
        max
    } else {
        input
    }
}

macro_rules! impl_has_negatives {
    ($one:tt, $($structure:tt),*) => {
        $(
            impl Signed for $structure {
                fn flip_sign(self) -> Self {
                    self*-$one
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
pub trait FloatingPoint {
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
pub trait UsesArithmetic: core::ops::Add<Output = Self> + core::ops::Sub<Output = Self> + core::ops::Mul<Output = Self> + core::ops::Div<Output = Self> 
    where Self: Sized {}
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
    core::ops::RemAssign<T> +
    {}
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
impl<T: core::ops::Add<Output = Self> + core::ops::Sub<Output = Self> + core::ops::Mul<Output = Self> + core::ops::Div<Output = Self>> UsesArithmetic for T {}
pub trait Number 
    where Self: Sized + 
    core::ops::Add<Output = Self> +
    core::ops::Sub<Output = Self> +
    core::ops::Mul<Output = Self> +
    core::ops::Div<Output = Self> +
    core::ops::Rem<Output = Self> +
    core::ops::AddAssign +
    core::ops::SubAssign +
    core::ops::MulAssign +
    core::ops::DivAssign + 
    core::ops::RemAssign + 
    Clone + 
    Copy + 
    One + 
    Zero + 
    PartialEq + 
    PartialOrd + 
    Bounds + 
    FromPrimitive +
    IntoPrimitive
    {}
impl<T> Number for T 
    where T: Sized + 
    core::ops::Add<Output = Self> +
    core::ops::Sub<Output = Self> +
    core::ops::Mul<Output = Self> +
    core::ops::Div<Output = Self> +
    core::ops::Rem<Output = Self> +
    core::ops::AddAssign +
    core::ops::SubAssign +
    core::ops::MulAssign +
    core::ops::DivAssign + 
    core::ops::RemAssign + 
    Clone + 
    Copy + 
    One + 
    Zero + 
    PartialEq + 
    PartialOrd + 
    Bounds +
    FromPrimitive +
    IntoPrimitive {
    
}
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
pub trait Bounds {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    
    const MIN: Self;
    const MAX: Self;
}
impl_has_negatives!(1, i8, i16, i32, i64, i128, isize);
impl FloatingPoint for f32 {
    fn acos(self) -> Self {
        affogato_core::cmath::acosf(self)
    }
    fn acosh(self) -> Self {
        affogato_core::cmath::acoshf(self)
    }
    fn asin(self) -> Self {
        affogato_core::cmath::asinf(self)
    }
    fn asinh(self) -> Self {
        affogato_core::cmath::asinhf(self)
    }
    fn atan(self) -> Self {
        affogato_core::cmath::atanf(self)
    }
    fn atan2(self, other: Self) -> Self {
        affogato_core::cmath::atan2f(self, other)
    }
    fn atanh(self) -> Self {
        affogato_core::cmath::atanhf(self)
    }
    fn cbrt(self) -> Self {
        affogato_core::cmath::cbrtf(self)
    }
    fn ceil(self) -> Self {
        affogato_core::cmath::ceilf(self)
    }
    fn cos(self) -> Self {
        affogato_core::cmath::cosf(self)
    }
    fn exp(self) -> Self {
        affogato_core::cmath::expf(self)
    }
    fn exp2(self) -> Self {
        affogato_core::cmath::exp2f(self)
    }
    fn exp_m1(self) -> Self {
        affogato_core::cmath::expm1f(self)
    }
    fn floor(self) -> Self {
        affogato_core::cmath::floorf(self)
    }
    fn fract(self) -> Self {
        self - self.trunc()
    }
    fn hypot(self, other: Self) -> Self {
        affogato_core::cmath::hypotf(self, other)
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
        matches!(self.classify(), FpCategory::Normal)
    }
    fn is_subnormal(self) -> bool {
        matches!(self.classify(), FpCategory::Subnormal)
    }
    fn ln(self) -> Self {
        affogato_core::cmath::logf(self)
    }
    fn ln_1p(self) -> Self {
        affogato_core::cmath::log1pf(self)
    }
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }
    fn log10(self) -> Self {
        affogato_core::cmath::log10f(self)
    }
    fn log2(self) -> Self {
        affogato_core::cmath::log2f(self)
    }
    fn powf(self, n: Self) -> Self {
        affogato_core::cmath::powf(self, n)
    }
    fn powi(self, n: i32) -> Self {
        affogato_core::cmath::powf(self, n as f32)
    }
    fn recip(self) -> Self {
        f32::ONE/self
    }
    fn round(self) -> Self {
        affogato_core::cmath::roundf(self)
    }
    fn signum(self) -> Self {
        if self.is_nan() { Self::NAN } else { 1.0_f32.copysign(self) }
    }
    fn sin(self) -> Self {
        affogato_core::cmath::sinf(self)
    }
    fn sin_cos(self) -> (Self, Self)
            where Self: Sized {
        (self.sin(), self.cos())
    }
    fn sinh(self) -> Self {
        affogato_core::cmath::sinhf(self)
    }
    fn sqrt(self) -> Self {
        affogato_core::cmath::sqrtf(self)
    }
    fn tan(self) -> Self {
        affogato_core::cmath::tanf(self)
    }
    fn tanh(self) -> Self {
        affogato_core::cmath::tanhf(self)
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
        affogato_core::cmath::truncf(self)
    }
}
impl FloatingPoint for f64 {
    fn acos(self) -> Self {
        affogato_core::cmath::acos(self)
    }
    fn acosh(self) -> Self {
        affogato_core::cmath::acosh(self)
    }
    fn asin(self) -> Self {
        affogato_core::cmath::asin(self)
    }
    fn asinh(self) -> Self {
        affogato_core::cmath::asinh(self)
    }
    fn atan(self) -> Self {
        affogato_core::cmath::atan(self)
    }
    fn atan2(self, other: Self) -> Self {
        affogato_core::cmath::atan2(self, other)
    }
    fn atanh(self) -> Self {
        affogato_core::cmath::atanh(self)
    }
    fn cbrt(self) -> Self {
        affogato_core::cmath::cbrt(self)
    }
    fn ceil(self) -> Self {
        affogato_core::cmath::ceil(self)
    }
    fn cos(self) -> Self {
        affogato_core::cmath::cos(self)
    }
    fn exp(self) -> Self {
        affogato_core::cmath::exp(self)
    }
    fn exp2(self) -> Self {
        affogato_core::cmath::exp2(self)
    }
    fn exp_m1(self) -> Self {
        affogato_core::cmath::expm1(self)
    }
    fn floor(self) -> Self {
        affogato_core::cmath::floor(self)
    }
    fn fract(self) -> Self {
        self - self.trunc()
    }
    fn hypot(self, other: Self) -> Self {
        affogato_core::cmath::hypot(self, other)
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
        matches!(self.classify(), FpCategory::Normal)
    }
    fn is_subnormal(self) -> bool {
        matches!(self.classify(), FpCategory::Subnormal)
    }
    fn ln(self) -> Self {
        affogato_core::cmath::log(self)
    }
    fn ln_1p(self) -> Self {
        affogato_core::cmath::log1p(self)
    }
    fn log(self, base: Self) -> Self {
        self.ln() / base.ln()
    }
    fn log10(self) -> Self {
        affogato_core::cmath::log10(self)
    }
    fn log2(self) -> Self {
        affogato_core::cmath::log2(self)
    }
    fn powf(self, n: Self) -> Self {
        affogato_core::cmath::pow(self, n)
    }
    fn powi(self, n: i32) -> Self {
        affogato_core::cmath::pow(self, n as f64)
    }
    fn recip(self) -> Self {
        f64::ONE/self
    }
    fn round(self) -> Self {
        affogato_core::cmath::round(self)
    }
    fn signum(self) -> Self {
        if self.is_nan() { Self::NAN } else { 1.0_f64.copysign(self) }
    }
    fn sin(self) -> Self {
        affogato_core::cmath::sin(self)
    }
    fn sin_cos(self) -> (Self, Self)
            where Self: Sized {
        (self.sin(), self.cos())
    }
    fn sinh(self) -> Self {
        affogato_core::cmath::sinh(self)
    }
    fn sqrt(self) -> Self {
        affogato_core::cmath::sqrt(self)
    }
    fn tan(self) -> Self {
        affogato_core::cmath::tan(self)
    }
    fn tanh(self) -> Self {
        affogato_core::cmath::tanh(self)
    }
    fn to_degrees(self) -> Self {
        self * (180.0f64 / core::f64::consts::PI)
    }
    fn to_radians(self) -> Self {
        const RADS_PER_DEG: f64 = core::f64::consts::PI / 180.0;
        self * RADS_PER_DEG
    }
    fn trunc(self) -> Self {
        affogato_core::cmath::trunc(self)
    }
}
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

impl_bounds!(f32, f64);
impl_bounds_ord!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_properties!(0, 1, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_properties!(0.0, 1.0, f32, f64);