use core::ops::{Index, IndexMut};

use affogato_core::{groups::vector_spaces::VectorSpace, num::{Number, Signed, Zero}, sets::Real};
use bytemuck::{Pod, Zeroable};

use crate::{matrix::{Matrix2, Matrix4, SquareMatrix}, vector::{Vector2, Vector3}};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

/// column major matrix
#[cfg(feature="glsl")]
#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3<T: Number> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
    padding: Vector3<T>,
}

/// column major matrix
#[cfg(not(feature="glsl"))]
#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix3<T: Number> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
}
impl<T: Number> Zero for Matrix3<T> {
    const ZERO: Self = Matrix3::empty();
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}
impl<T: Number> Index<usize> for Matrix3<T> {
    type Output = Vector3<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[Vector3<T>; 3]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Matrix3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [Vector3<T>; 3]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Matrix3<T>  {
    pub const fn empty() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO, T::ZERO, T::ZERO, T::ZERO, T::ZERO, T::ZERO)
    }
    pub const fn new(xx: T, xy: T, xz: T, yx: T, yy: T, yz: T, zx: T, zy: T, zz: T) -> Self {
        Self::from_vec(Vector3::new(xx,xy,xz), Vector3::new(yx, yy, yz), Vector3::new(zx, zy, zz))
    }
    #[inline(always)]
    pub const fn x(&self) -> Vector3<T> {
        self.x
    }
    #[inline(always)]
    pub const fn y(&self) -> Vector3<T> {
        self.y
    }
    #[inline(always)]
    pub const fn z(&self) -> Vector3<T> {
        self.z
    }

    #[cfg(not(feature="glsl"))]
    pub const fn from_vec(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self { x, y, z }
    }

    #[cfg(feature="glsl")]
    pub const fn from_vec(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self { x, y, z, padding: Vector3::ZERO }
    }
    pub fn translate(mut self, translate: Vector3<T>) -> Self {
        self.x.set_z(translate.x());
        self.y.set_z(translate.y());
        self.z.set_z(translate.z());
        self
    }
    pub fn from_scale(v: Vector3<T>) -> Self {
        Self::new(
            v.x(), T::ZERO, T::ZERO, 
            T::ZERO, v.y(), T::ZERO, 
            T::ZERO, T::ZERO, v.z()
        )
    }
    pub fn from_translation(v: Vector3<T>) -> Self {
        Self::new(
            T::ONE, T::ZERO, T::ZERO, 
            T::ZERO, T::ONE, T::ZERO, 
            v.x(), v.y(), v.z()
        )
    }
    pub fn epsilon_eq(&self, other: &Self, epsilon: T) -> bool 
        where T: Real {
        for (a, b) in Into::<[Vector3<T>; 3]>::into(*self).into_iter().zip(Into::<[Vector3<T>; 3]>::into(*other)) {
            if !a.epsilon_eq(b, epsilon){
                return false;
            }
        }
        true
    }
}
impl<T: Real> Matrix3<T>  {
    pub fn from_transform(translation: Vector2<T>, scaling: Vector2<T>, rotation: T) -> Self {
        Self::new(
            rotation.cos()*scaling.x(), rotation.sin(), T::ZERO, 
            -rotation.sin(), rotation.cos()*scaling.y(), T::ZERO, 
            translation.x(), translation.y(), T::ONE
        )
    }
}
impl<T: Number> core::ops::Add for Matrix3<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self::from_vec(self.x + rhs.x(), self.y + rhs.y(), self.z + rhs.z())
    }
    type Output = Self;
}
impl<T: Number> core::ops::Sub for Matrix3<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self::from_vec(self.x - rhs.x(), self.y - rhs.y(), self.z - rhs.z())
    }
    type Output = Self;
}
impl<T: Number> core::ops::Mul for Matrix3<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self::from_vec(
            Vector3::new( 
                rhs.x().x() * self.x.x() + rhs.x().y() * self.y.x() + rhs.x().z() * self.z.x(),
                rhs.x().x() * self.x.y() + rhs.x().y() * self.y.y() + rhs.x().z() * self.z.y(),
                rhs.x().x() * self.x.z() + rhs.x().y() * self.y.z() + rhs.x().z() * self.z.z(),
            ), 
            Vector3::new( 
                rhs.y().x() * self.x.x() + rhs.y().y() * self.y.x() + rhs.y().z() * self.z.x(),
                rhs.y().x() * self.x.y() + rhs.y().y() * self.y.y() + rhs.y().z() * self.z.y(),
                rhs.y().x() * self.x.z() + rhs.y().y() * self.y.z() + rhs.y().z() * self.z.z(),
            ),
            Vector3::new(
                rhs.z().x() * self.x.x() + rhs.z().y() * self.y.x() + rhs.z().z() * self.z.x(),
                rhs.z().x() * self.x.y() + rhs.z().y() * self.y.y() + rhs.z().z() * self.z.y(),
                rhs.z().x() * self.x.z() + rhs.z().y() * self.y.z() + rhs.z().z() * self.z.z(),
            )
        )
    }
    type Output = Self;
}
impl<T: Number> core::ops::Mul<Vector3<T>> for Matrix3<T>  {
    /// # Multiplying Matrix3 with Vector3
    /// 
    /// when you multiply a Matrix3 with a Vector3 we treat the vector
    /// as a 3x3 matrix * 3x1 matrix since it is impossible to multiply
    /// a 3x1 matrix * 3x3 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(
           self.x.x() * rhs.x() + self.y.x() * rhs.y() + self.z.x() * rhs.z(),
           self.x.y() * rhs.x() + self.y.y() * rhs.y() + self.z.y() * rhs.z(),
           self.x.z() * rhs.x() + self.y.z() * rhs.y() + self.z.z() * rhs.z()
        )
    }
    type Output = Vector3<T>;
}
impl<T: Number> From<T> for Matrix3<T> {
    ///
    /// Makes the identity element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self::from_vec(
            Vector3::new(value, T::ZERO, T::ZERO), 
            Vector3::new(T::ZERO, value, T::ZERO), 
            Vector3::new(T::ZERO, T::ZERO, value)
        )
    }
}

impl<T: Number> From<Matrix2<T>> for Matrix3<T> {
    fn from(value: Matrix2<T>) -> Self {
        Self::new(
            value.x().x(), value.x().y(), T::ZERO, 
            value.y().x(), value.y().y(), T::ZERO, 
            T::ZERO, T::ZERO, T::ONE, 
        )
    }
}
impl<T: Number> SquareMatrix for Matrix3<T> {
    type Column = Vector3<T>;
    type LowerDimension = Matrix2<T>;
    fn identity() -> Self {
        Self::from_vec( 
            Vector3::new( 
                T::ONE, 
                T::ZERO, 
                T::ZERO 
            ), 
            Vector3::new( 
                T::ZERO, 
                T::ONE, 
                T::ZERO 
            ), 
            Vector3::new( 
                T::ZERO, 
                T::ZERO, 
                T::ONE 
            ) 
        )
    }
    fn transpose(&self) -> Self {
        Self::from_vec(
            Vector3::new( 
                self.x.x(), 
                self.y.x(), 
                self.z.x()
            ), 
            Vector3::new( 
                self.x.y(),
                self.y.y(),
                self.z.y()
            ), 
            Vector3::new( 
                self.x.z(), 
                self.y.z(), 
                self.z.z()
            ) 
        )
    }
    fn determinant(&self) -> <Self::Column as VectorSpace>::Scalar {
        let m1 = Matrix2::from_vec(
            Vector2::new(self.y.y(), self.y.z()), 
            Vector2::new(self.z.y(), self.z.z()));
        let m2 = Matrix2::from_vec(
            Vector2::new(self.x.y(), self.x.z()), 
            Vector2::new(self.z.y(), self.z.z()));
        let m3 = Matrix2::from_vec(
            Vector2::new(self.x.y(), self.x.z()), 
            Vector2::new(self.y.y(), self.y.z()));
        
        let m1_det = m1.determinant()*self.x.x();
        let m2_det = m2.determinant()*self.y.x();
        let m3_det = m3.determinant()*self.z.x();
        
        let det = m1_det - m2_det + m3_det;
        det
    }
    fn cofactor(&self, column: usize, row: usize) -> Matrix2<T> {
        let mut mat3 = Matrix2::empty();
        let mut idx_y = 0;
        for i in 0..2 {
            if idx_y == row {
                idx_y += 1;
            }
            let mut idx_x = 0;
            for j in 0..2 {
                if idx_x == column {
                    idx_x += 1;
                }
                mat3[j][i] = self[idx_x][idx_y];
                idx_x += 1;
            }
            idx_y += 1;
        }
        mat3
    }
    fn cofactor_matrix(&self) -> Self 
        where T: Signed {
        Self::new(
            self.cofactor(0, 0).determinant(), -self.cofactor(0, 1).determinant(), self.cofactor(0, 2).determinant(), 
            -self.cofactor(1, 0).determinant(), self.cofactor(1, 1).determinant(), -self.cofactor(1, 2).determinant(), 
            self.cofactor(2, 0).determinant(), -self.cofactor(2, 1).determinant(), self.cofactor(2, 2).determinant()
        )
    }
    fn diagonal(diagonal: Self::Column) -> Self {
        Self::new(diagonal.x(), T::ZERO, T::ZERO, T::ZERO, diagonal.y(), T::ZERO, T::ZERO, T::ZERO, diagonal.z())
    }
}
impl<T: Number> core::ops::Mul<T> for Matrix3<T>  {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Matrix3::from_vec(self.x*rhs, self.y*rhs, self.z*rhs)
    }
}

unsafe impl<T: Number> Zeroable for Matrix3<T> {
    fn zeroed() -> Self {
        Self::ZERO
    }
}
unsafe impl<T: Number + Pod> Pod for Matrix3<T> {}

impl<T: Number> From<Matrix3<T>> for [Vector3<T>; 3] {
    fn from(value: Matrix3<T>) -> Self {
        [
            value.x(), 
            value.y(), 
            value.z(),
        ]
    }
}

impl<T: Number> From<Matrix3<T>> for [T; 3*3] {
    fn from(value: Matrix3<T>) -> Self {
        [
            value.x().x(), value.x().y(), value.x().z(), 
            value.y().x(), value.y().y(), value.y().z(), 
            value.z().x(), value.z().y(), value.z().z(),
        ]
    }
}

impl<T: Number> From<Matrix4<T>> for Matrix3<T> {
    fn from(value: Matrix4<T>) -> Self {
        Self::from_vec(
            Vector3::new(value.x().x(), value.x().y(), value.x().z()), 
            Vector3::new(value.y().x(), value.y().y(), value.y().z()), 
            Vector3::new(value.z().x(), value.z().y(), value.z().z()) 
        )
    }
}