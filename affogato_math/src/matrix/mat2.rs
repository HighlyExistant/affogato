use core::ops::{Index, IndexMut};

use affogato_core::{groups::vector_spaces::VectorSpace, num::{Number, Signed, Zero}, sets::Real};
use bytemuck::{Pod, Zeroable};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{matrix::SquareMatrix, vector::Vector2};

/// column major 2x2 matrix
#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix2<T: Number> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
}
impl<T: Number> Index<usize> for Matrix2<T> {
    type Output = Vector2<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[Vector2<T>; 2]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Matrix2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [Vector2<T>; 2]>(self) };
        &mut val[index]
    }
}
impl<T: Number> SquareMatrix for Matrix2<T> {
    type Column = Vector2<T>;
    type LowerDimension = T;
    fn identity() -> Self {
        Self::from_vec(
            Vector2::new( 
                T::ONE, 
                T::ZERO 
            ), 
            Vector2::new( 
                T::ZERO, 
                T::ONE 
            )
        )
    }
    fn transpose(&self) -> Self {
        Self::from_vec(
            Vector2::new(
                self.x.x(), 
                self.y.x() 
            ), 
            Vector2::new(
                self.x.y(), 
                self.y.y()
            )
        )
    }
    fn determinant(&self) -> <Self::Column as VectorSpace>::Scalar {
        self.x.x()*self.y.y()-self.x.y()*self.y.x()
    }
    fn cofactor(&self, column: usize, row: usize) -> T {
        let x = if column == 0 { 1 } else { 0 };
        let y = if row == 0 { 1 } else { 0};
        self[x][y]
    }
    fn cofactor_matrix(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Signed {
        Self::new(
            self.y.y(), -self.y.x(), 
            -self.x.y(), self.x.x()
        )
    }
    fn diagonal(diagonal: Self::Column) -> Self {
        Self::new(diagonal.x(), T::ZERO, T::ZERO, diagonal.y())
    }
}
impl<T: Number> Zero for Matrix2<T> {
    const ZERO: Self = Matrix2::empty();
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}
impl<T: Number> Matrix2<T>  {
    pub const fn empty() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
    pub const fn new(xx: T, xy: T, yx: T, yy: T) -> Self {
        Self::from_vec(
            Vector2::new(xx, xy), 
            Vector2::new(yx, yy) 
        )
    }
    pub const fn from_vec(x: Vector2<T>, y: Vector2<T>) -> Self {
        Self { x, y }
    }
    pub fn from_scale(scale: Vector2<T>) -> Self {
        Self::new(T::ONE*scale.x(), T::ZERO, T::ZERO, T::ONE*scale.y())
    }
    #[inline(always)]
    pub const fn x(&self) -> Vector2<T> {
        self.x
    }
    #[inline(always)]
    pub const fn y(&self) -> Vector2<T> {
        self.y
    }
    pub fn epsilon_eq(&self, other: &Self, epsilon: T) -> bool 
        where T: Real {
        for (a, b) in Into::<[Vector2<T>; 2]>::into(*self).into_iter().zip(Into::<[Vector2<T>; 2]>::into(*other)) {
            if !a.epsilon_eq(b, epsilon){
                return false;
            }
        }
        true
    }
}

impl<T: Real> Matrix2<T> {
    pub fn from_rotation(angle: T) -> Self {
        Matrix2::new(
            angle.cos(), angle.sin(), 
            -angle.sin(), angle.cos()
        )
    }
}

impl<T: Number> core::ops::Add for Matrix2<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x()), y: (self.y + rhs.y()) }
    }
    type Output = Self;
}
impl<T: Number> core::ops::Sub for Matrix2<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x()), y: (self.y - rhs.y()) }
    }
    type Output = Self;
}
impl<T: Number> core::ops::Mul for Matrix2<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_vec(
            Vector2::new(
                self.x.x() * rhs.x().x() + self.y.x() * rhs.x().y(), 
                self.x.y() * rhs.x().x() + self.y.y() * rhs.x().y() 
            ), 
            Vector2::new( 
                self.x.x() * rhs.y().x() + self.y.x() * rhs.y().y(), 
                self.x.y() * rhs.y().x() + self.y.y() * rhs.y().y() 
            )
        )
    }
    type Output = Self;
}
impl<T: Number> core::ops::Mul<Vector2<T>> for Matrix2<T>  {
    /// # Multiplying Matrix2 with Vector2
    /// 
    /// when you multiply a Matrix2 with a Vector2 we treat the vector
    /// as a 2x2 matrix * 2x1 matrix since it is impossible to multiply
    /// a 2x1 matrix * 2x2 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::<T>::new(
            self.x.x() * rhs.x() + self.y.x() * rhs.y(),
            self.x.y() * rhs.x() + self.y.y() * rhs.y()
        )
    }
    type Output = Vector2<T>;
}
impl<T: Number> core::ops::Mul<T> for Matrix2<T>  {
    /// # Multiplying Matrix2 with Vector2
    /// 
    /// when you multiply a Matrix2 with a Vector2 we treat the vector
    /// as a 2x2 matrix * 2x1 matrix since it is impossible to multiply
    /// a 2x1 matrix * 2x2 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: T) -> Self::Output {
        Matrix2::from_vec(self.x*rhs, self.y*rhs)
    }
    type Output = Matrix2<T>;
}
impl<T: Number> From<T> for Matrix2<T> {
    ///
    /// Makes the identity element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self::from_vec(
            Vector2::new(value, T::ZERO), 
            Vector2::new(T::ZERO, value)
        )
    }
}

unsafe impl<T: Number> Zeroable for Matrix2<T> {
    fn zeroed() -> Self {
        Self::ZERO
    }
}
unsafe impl<T: Number + Pod> Pod for Matrix2<T> {}


#[cfg(feature="alloc")]
mod alloc_feature {
    extern crate alloc;
    use core::fmt::Display;
    use affogato_core::num::Number;
    use alloc::{string::String};

    use crate::matrix::{Matrix2, Matrix2x3, Matrix3, Matrix4};
    impl<T: Number + Display> Display for Matrix2<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut row1 = alloc::string::String::from('┌');
            let mut row2 = String::from('└');
            {
                let mut str_row1 = alloc::format!("{}, ", self.x.x());
                let mut str_row2 = alloc::format!("{}, ", self.x.y());
                let max = str_row1.len().max(str_row2.len());
                str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
                row1.push_str(str_row1.as_str());
                row2.push_str(str_row2.as_str());
            }
            let mut str_row1 = alloc::format!("{}", self.y.x());
            let mut str_row2 = alloc::format!("{}", self.y.y());
            let max = str_row1.len().max(str_row2.len());
            str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
            row1.push_str(str_row1.as_str());
            row2.push_str(str_row2.as_str());
            row1.push_str("┐\n");
            row2.push_str("┘\n");
            f.write_str(row1.as_str())?;
            f.write_str(row2.as_str())
        }
    }
}

impl<T: Number> From<Matrix2<T>> for [Vector2<T>; 2] {
    fn from(value: Matrix2<T>) -> Self {
        [
            value.x(), 
            value.y(), 
        ]
    }
}

impl<T: Number> From<Matrix2<T>> for [T; 2*2] {
    fn from(value: Matrix2<T>) -> Self {
        [
            value.x().x(), value.x().y(), 
            value.y().x(), value.y().y(), 
        ]
    }
}