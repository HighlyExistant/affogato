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
macro_rules! impl_float {
    ($($structure:tt),*) => {
        $(
            impl FloatingPoint for $structure {
                fn acos(self) -> Self {
                    $structure::acos(self)
                }
                fn acosh(self) -> Self {
                    $structure::acosh(self)
                }
                fn asin(self) -> Self {
                    $structure::asin(self)
                }
                fn asinh(self) -> Self {
                    $structure::asinh(self)
                }
                fn atan(self) -> Self {
                    $structure::atan(self)
                }
                fn atan2(self, other: Self) -> Self {
                    $structure::atan2(self, other)
                }
                fn atanh(self) -> Self {
                    $structure::atanh(self)
                }
                fn cbrt(self) -> Self {
                    $structure::cbrt(self)
                }
                fn ceil(self) -> Self {
                    $structure::ceil(self)
                }
                fn clamp(self, min: Self, max: Self) -> Self {
                    $structure::clamp(self, min, max)
                }
                fn copysign(self, sign: Self) -> Self {
                    $structure::copysign(self, sign)
                }
                fn cos(self) -> Self {
                    $structure::cos(self)
                }
                fn exp(self) -> Self {
                    $structure::exp(self)
                }
                fn exp2(self) -> Self {
                    $structure::exp2(self)
                }
                fn exp_m1(self) -> Self {
                    $structure::exp_m1(self)
                }
                fn floor(self) -> Self {
                    $structure::floor(self)
                }
                fn fract(self) -> Self {
                    $structure::fract(self)
                }
                fn hypot(self, other: Self) -> Self {
                    $structure::hypot(self, other)
                }
                fn ln(self) -> Self {
                    $structure::ln(self)
                }
                fn ln_1p(self) -> Self {
                    $structure::ln_1p(self)
                }
                fn log(self, base: Self) -> Self {
                    $structure::log(self, base)
                }
                fn log10(self) -> Self {
                    $structure::log10(self)
                }
                fn log2(self) -> Self {
                    $structure::log2(self)
                }
                fn powf(self, n: Self) -> Self {
                    $structure::powf(self, n)
                }
                fn powi(self, n: i32) -> Self {
                    $structure::powi(self, n)
                }
                fn recip(self) -> Self {
                    $structure::recip(self)
                }
                fn round(self) -> Self {
                    $structure::round(self)
                }
                fn signum(self) -> Self {
                    $structure::signum(self)
                }
                fn sin(self) -> Self {
                    $structure::sin(self)
                }
                fn sinh(self) -> Self {
                    $structure::sinh(self)
                }
                fn sqrt(self) -> Self {
                    $structure::sqrt(self)
                }
                fn tan(self) -> Self {
                    $structure::tan(self)
                }
                fn tanh(self) -> Self {
                    $structure::tanh(self)
                }
                fn to_degrees(self) -> Self {
                    $structure::to_degrees(self)
                }
                fn to_radians(self) -> Self {
                    $structure::to_radians(self)
                }
                fn trunc(self) -> Self {
                    $structure::trunc(self)
                }
                fn sin_cos(self) -> (Self, Self) {
                    $structure::sin_cos(self)
                }
                fn is_finite(self) -> bool {
                    $structure::is_finite(self)
                }
                fn is_infinite(self) -> bool {
                    $structure::is_infinite(self)
                }
                fn is_nan(self) -> bool {
                    $structure::is_nan(self)
                }
                fn is_normal(self) -> bool {
                    $structure::is_normal(self)
                }
                fn is_subnormal(self) -> bool {
                    $structure::is_subnormal(self)
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
                const MIN: Self = std::$structure::MIN;
                const MAX: Self = std::$structure::MAX;
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
                const MIN: Self = std::$structure::MIN;
                const MAX: Self = std::$structure::MAX;
            }
        )*
    };

}
macro_rules! impl_has_negatives {
    ($one:tt, $($structure:tt),*) => {
        $(
            impl HasNegatives for $structure {
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
    fn clamp(self, min: Self, max: Self) -> Self;
    fn copysign(self, sign: Self) -> Self;
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
pub trait UsesArithmetic: std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self> 
    where Self: Sized {}
pub trait UniversalOperationsOn<T>
    where Self: Sized + 
    std::ops::Add<T, Output = Self> + 
    std::ops::Sub<T, Output = Self> + 
    std::ops::Mul<T, Output = Self> + 
    std::ops::Div<T, Output = Self> +
    std::ops::Rem<T, Output = Self> +
    std::ops::AddAssign<T> +
    std::ops::SubAssign<T> +
    std::ops::MulAssign<T> +
    std::ops::DivAssign<T> +
    std::ops::RemAssign<T> +
    {}
impl<T, A> UniversalOperationsOn<T> for A 
    where A: Sized + 
    std::ops::Add<T, Output = Self> + 
    std::ops::Sub<T, Output = Self> + 
    std::ops::Mul<T, Output = Self> + 
    std::ops::Div<T, Output = Self> +
    std::ops::Rem<T, Output = Self> +
    std::ops::AddAssign<T> +
    std::ops::SubAssign<T> +
    std::ops::MulAssign<T> +
    std::ops::DivAssign<T> +
    std::ops::RemAssign<T> {

}
impl<T: std::ops::Add<Output = Self> + std::ops::Sub<Output = Self> + std::ops::Mul<Output = Self> + std::ops::Div<Output = Self>> UsesArithmetic for T {}

pub trait Number 
    where Self: Sized + 
    std::ops::Add<Output = Self> +
    std::ops::Sub<Output = Self> +
    std::ops::Mul<Output = Self> +
    std::ops::Div<Output = Self> +
    std::ops::Rem<Output = Self> +
    std::ops::AddAssign +
    std::ops::SubAssign +
    std::ops::MulAssign +
    std::ops::DivAssign + 
    std::ops::RemAssign + 
    Clone + 
    Copy + 
    One + 
    Zero + 
    PartialOrd + 
    Bounds
    {
}
impl<T> Number for T 
    where T: Sized + 
    std::ops::Add<Output = Self> +
    std::ops::Sub<Output = Self> +
    std::ops::Mul<Output = Self> +
    std::ops::Div<Output = Self> +
    std::ops::Rem<Output = Self> +
    std::ops::AddAssign +
    std::ops::SubAssign +
    std::ops::MulAssign +
    std::ops::DivAssign + 
    std::ops::RemAssign + 
    Clone + 
    Copy + 
    One + 
    Zero + 
    PartialOrd + 
    Bounds {
    
}

pub trait HasNegatives: std::ops::Neg<Output = Self> {
    fn is_negative(self) -> bool;
    fn is_positive(self) -> bool;
    fn abs(self) -> Self;
    fn flip_sign(self) -> Self;
}
pub trait Bounds {
    fn min(self, other: Self) -> Self;
    fn max(self, other: Self) -> Self;
    const MIN: Self;
    const MAX: Self;
}
impl_bounds!(f32, f64);
impl_bounds_ord!(u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_properties!(0, 1, u8, i8, u16, i16, u32, i32, u64, i64, u128, i128, usize, isize);
impl_properties!(0.0, 1.0, f32, f64);
impl_float!(f32, f64);
impl_has_negatives!(1, i8, i16, i32, i64, i128, isize);
impl HasNegatives for f32 {
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
impl HasNegatives for f64 {
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