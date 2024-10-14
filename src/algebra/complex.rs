use std::{fmt::{Display, Write}, ops::{Add, Div, Mul}};

use crate::{linear::{Matrix2, Vector2}, FloatingPoint};

pub struct ComplexNumber<T: FloatingPoint> {
    real: T,
    imaginary: T,
}
impl<T: FloatingPoint> Display for ComplexNumber<T> 
    where T: Display {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let sign = self.imaginary.is_sign_negative();
        if sign {
            f.write_str(&format!("{} - {}i", self.real, self.imaginary.abs()))
        } else {
            f.write_str(&format!("{} + {}i", self.real, self.imaginary))
        }
    }
}
impl<T: FloatingPoint> ComplexNumber<T> {
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
impl<T: FloatingPoint> Add for ComplexNumber<T> {
    type Output = Self;
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.real+rhs.real, self.imaginary+rhs.imaginary)
    }
}
impl<T: FloatingPoint> Mul for ComplexNumber<T> {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self::Output {
        let mat = self.matrix()*rhs.matrix();
        Self::new(mat.x.x, mat.y.x)
    }
}