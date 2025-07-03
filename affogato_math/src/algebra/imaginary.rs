#![allow(unused)]

use affogato_core::sets::Real;
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

macro_rules! impl_ops_float {
    ($trait:tt, $func:tt, $($primitive:tt),*) => {
        $(
            impl core::ops::$trait<Imaginary<$primitive>> for $primitive {
                type Output = Imaginary<$primitive>;
                fn $func(self, rhs: Imaginary<$primitive>) -> Self::Output {
                    Imaginary::from(rhs.0.$func(self))
                }
            }
        )*
    };
}
/// A lonely imaginary number. It has the property that `i*i == -1`.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Default, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Imaginary<T: Real>(T);

impl<T: Real> From<T> for Imaginary<T> {
    fn from(value: T) -> Self {
        Self(value)
    }
}
// i * i
impl<T: Real> core::ops::Mul<Self> for Imaginary<T> {
    type Output = T;
    fn mul(self, rhs: Self) -> Self::Output {
        -(self.0*rhs.0)
    }
}
// i * r
impl<T: Real> core::ops::Mul<T> for Imaginary<T> {
    type Output = Imaginary<T>;
    fn mul(self, rhs: T) -> Self::Output {
        Imaginary::from(self.0*rhs)
    }
}
// i / r
impl<T: Real> core::ops::Div<T> for Imaginary<T> {
    type Output = Imaginary<T>;
    fn div(self, rhs: T) -> Self::Output {
        Imaginary::from(self.0/rhs)
    }
}

impl_ops_float!(Mul, mul, f32, f64);
impl_ops_float!(Div, div, f32, f64);
