use core::{fmt::Display, ops::{Add, Div, Mul}};

use affogato_core::sets::Real;
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{matrix::Matrix2, vector::Vector2};
/// Represents a number with 1 real component and 1 imaginary component `i`, where `i^*i == -1.0`.
/// This is useful for when you want to represent rotations in 2 dimensions algebraically.
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, Default)]
pub struct ComplexNumber<T: Real> {
    real: T,
    imaginary: T,
}
impl<T: Real> ComplexNumber<T> {
    pub fn new(real: T, imaginary: T) -> Self {
        Self { real, imaginary }
    }
    pub fn matrix(&self) -> Matrix2<T> {
        Matrix2 { 
            x: Vector2::new(self.real, -self.imaginary), 
            y: Vector2::new(self.imaginary, self.real) 
        }
    }
    pub fn conjugate(&self) -> Self {
        Self::new(self.real, -self.imaginary)
    }
}
impl<T: Real> Add for ComplexNumber<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real+rhs.real, self.imaginary+rhs.imaginary)
    }
}
impl<T: Real> Mul for ComplexNumber<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mat = self.matrix()*rhs.matrix();
        Self::new(mat.x.x(), mat.y.x())
    }
}
impl<T: Real> Div for ComplexNumber<T> 
    where Matrix2<T>: core::ops::Mul<T, Output = Matrix2<T>> +
        core::ops::Mul<Output = Matrix2<T>> {
    type Output = Self;
    fn div(self, rhs: Self) -> Self::Output {
        let denom = rhs.imaginary*rhs.imaginary + rhs.real*rhs.real;
        let real = (self.real * rhs.real + self.imaginary * rhs.imaginary) /denom;
        let imaginary = (rhs.real * self.imaginary - self.real * rhs.imaginary) /denom;
        Self::new(real, imaginary)
    }
}

#[cfg(feature="alloc")]
mod alloc_feature {
    use core::fmt::Display;

    use affogato_core::sets::Real;

    use crate::{algebra::ComplexNumber};

    extern crate alloc;
    impl<T: Real> Display for ComplexNumber<T> 
        where T: Display {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let sign = self.imaginary.is_negative();
            if sign {
                f.write_str(&alloc::format!("{} - {}i", self.real, self.imaginary.abs()))
            } else {
                f.write_str(&alloc::format!("{} + {}i", self.real, self.imaginary))
            }
        }
    }
}