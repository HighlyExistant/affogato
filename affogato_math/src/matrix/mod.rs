use core::{fmt::Display, ops::{Index, IndexMut}};

use affogato_core::{groups::vector_spaces::VectorSpace, num::{Number, One, Signed, Zero}, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use bytemuck::{Pod, Zeroable};
use crate::{algebra::Quaternion, vector::{Vector2, Vector3, Vector4}};
pub trait SquareMatrix: Sized {
    type Column: VectorSpace;
    type LowerDimension;
    fn set_identity(&mut self) { *self = Self::identity(); }
    /// The identity of the matrix is one that when multiplied does nothing to a matrix. 
    /// The components of this matrix look like:
    /// ```no_run, ignore
    /// ┌1     0┐
    /// │  .    │
    /// │    .  │
    /// └0     1┘
    /// ```
    fn identity() -> Self;
    fn transpose(&self) -> Self;
    /// The determinant can be understood as the area of the parallelogram formed by
    /// the vectors represented in a matrix. It can help distinguish if the matrix has
    /// been scaled in any way. A matrix whos scale has not changed will have a determinant
    /// of 1.
    fn determinant(&self) -> <Self::Column as VectorSpace>::Scalar;
    fn cofactor(&self, column: usize, row: usize) -> Self::LowerDimension;
    fn cofactor_matrix(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Signed;
    fn adjoint(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Signed {
        self.cofactor_matrix().transpose()
    }
    // Doesn't check whether the determinant is zero
    unsafe fn inverse_unchecked(&self) -> Self 
        where <Self::Column as VectorSpace>::Scalar: Real, 
            Self: core::ops::Mul<<Self::Column as VectorSpace>::Scalar, Output = Self> {
        self.cofactor_matrix().transpose()*(<Self::Column as VectorSpace>::Scalar::ONE/self.determinant())
    }
    // Returns None if the determinant is zero
    fn inverse(&self) -> Option<Self> 
        where <Self::Column as VectorSpace>::Scalar: Real, 
            Self: core::ops::Mul<<Self::Column as VectorSpace>::Scalar, Output = Self> {
        let det = self.determinant();
        if det == <Self::Column as VectorSpace>::Scalar::ZERO {
            None
        } else {
            Some(self.cofactor_matrix().transpose()*(<Self::Column as VectorSpace>::Scalar::ONE/self.determinant()))
        }
    }
    /// Returns a matrix where the diagonal of the matrix is given by a vector.
    fn diagonal(diagonal: Self::Column) -> Self;
}

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
            rotation.cos()*scaling.x(), -(rotation.sin()), T::ZERO, 
            rotation.sin(), rotation.cos()*scaling.y(), T::ZERO, 
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
/// column major matrix
#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Matrix4<T: Number> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>,
}
impl<T: Number> Zero for Matrix4<T> {
    const ZERO: Self = Matrix4::empty();
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }
}
impl<T: Number> Index<usize> for Matrix4<T> {
    type Output = Vector4<T>;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[Vector4<T>; 4]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Matrix4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [Vector4<T>; 4]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Matrix4<T>  {
    pub const fn empty() -> Self {
        Self::new(
            T::ZERO, T::ZERO, T::ZERO, T::ZERO, 
            T::ZERO, T::ZERO, T::ZERO, T::ZERO, 
            T::ZERO, T::ZERO, T::ZERO, T::ZERO, 
            T::ZERO, T::ZERO, T::ZERO, T::ZERO
        )
    }
    pub const fn new(xx: T, xy: T, xz: T, xw: T, yx: T, yy: T, yz: T, yw: T, zx: T, zy: T, zz: T, zw: T, wx: T, wy: T, wz: T, ww: T) -> Self {
        Self::from_vec(
            Vector4::new(xx, xy, xz, xw), 
            Vector4::new(yx, yy, yz, yw), 
            Vector4::new(zx, zy, zz, zw), 
            Vector4::new(wx, wy, wz, ww) 
        )
    }
    pub const fn from_vec(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y, z, w }
    }
    #[inline(always)]
    pub const fn x(&self) -> Vector4<T> {
        self.x
    }
    #[inline(always)]
    pub const fn y(&self) -> Vector4<T> {
        self.y
    }
    #[inline(always)]
    pub const fn z(&self) -> Vector4<T> {
        self.z
    }
    #[inline(always)]
    pub const fn w(&self) -> Vector4<T> {
        self.w
    }
    pub const fn from_translation(v: Vector3<T>) -> Self {
        Matrix4::new(
            T::ONE, T::ZERO, T::ZERO, T::ZERO,
            T::ZERO, T::ONE, T::ZERO, T::ZERO,
            T::ZERO, T::ZERO, T::ONE, T::ZERO,
            v.x(), v.y(), v.z(), T::ONE,
        )
    }
    pub const fn from_scale(v: Vector3<T>) -> Self {
        Matrix4::new(
            v.x(), T::ZERO, T::ZERO, T::ZERO,
            T::ZERO, v.y(), T::ZERO, T::ZERO,
            T::ZERO, T::ZERO, v.z(), T::ZERO,
            T::ZERO, T::ZERO, T::ZERO, T::ONE,
        )
    }
    pub fn scale(&self, v: Vector3<T>) -> Self {
        let mut this = self.clone();
        this.x *= v.x();
        this.y *= v.y();
        this.z *= v.z();
        this
    }
    pub fn translate(&self, v: Vector3<T>) -> Self {
        Self::from_vec( 
            self.x, 
            self.y, 
            self.z, 
            self.w + Vector4::<T>::from(v) 
        )
    }
    pub fn from_transform(pos: Vector3<T>, rot: Quaternion<T>, scale: Vector3<T>) -> Self 
        where T: Real + Display{
        let mut mat = Matrix4::from(Matrix3::from(rot)).scale(scale);
        mat.w.set_x(pos.x());
        mat.w.set_y(pos.y());
        mat.w.set_z(pos.z());
        mat
    }
    
    pub fn epsilon_eq(&self, other: &Self, epsilon: T) -> bool 
        where T: Real {
        for (a, b) in Into::<[Vector4<T>; 4]>::into(*self).into_iter().zip(Into::<[Vector4<T>; 4]>::into(*other)) {
            if !a.epsilon_eq(b, epsilon){
                return false;
            }
        }
        true
    }
}

impl<T: Number> SquareMatrix for Matrix4<T> {
    type Column = Vector4<T>;
    type LowerDimension = Matrix3<T>;
    fn identity() -> Self {
        Self { 
            x: Vector4::new( 
                T::ONE,  
                T::ZERO, 
                T::ZERO, 
                T::ZERO 
            ), 
            y: Vector4::new(
                T::ZERO, 
                T::ONE, 
                T::ZERO, 
                T::ZERO 
            ), 
            z: Vector4::new( 
                T::ZERO, 
                T::ZERO, 
                T::ONE, 
                T::ZERO 
            ), 
            w: Vector4::new( 
                T::ZERO, 
                T::ZERO, 
                T::ZERO, 
                T::ONE 
            )
        }
    }
    fn transpose(&self) -> Self {
        Self { 
            x: Vector4::new(
                self.x.x(), 
                self.y.x(), 
                self.z.x(), 
                self.w.x(),
            ), 
            y: Vector4::new( 
                self.x.y(), 
                self.y.y(), 
                self.z.y(), 
                self.w.y(), 
            ), 
            z: Vector4::new( 
                self.x.z(), 
                self.y.z(), 
                self.z.z(), 
                self.w.z(), 
            ), 
            w: Vector4::new( 
                self.x.w(), 
                self.y.w(), 
                self.z.w(), 
                self.w.w(), 
            )
        }
    }
    fn determinant(&self) -> <Self::Column as VectorSpace>::Scalar {
        let m1 = Matrix3::new(
            self.y.y(), self.y.z(), self.y.w(), 
            self.z.y(), self.z.z(), self.z.w(), 
            self.w.y(), self.w.z(), self.w.w());
        let m2 = Matrix3::new(
            self.x.y(), self.x.z(), self.x.w(), 
            self.z.y(), self.z.z(), self.z.w(), 
            self.w.y(), self.w.z(), self.w.w());
        let m3 = Matrix3::new(
            self.x.y(), self.x.z(), self.x.w(), 
            self.y.y(), self.y.z(), self.y.w(), 
            self.w.y(), self.w.z(), self.w.w());
        let m4 = Matrix3::new(
            self.x.y(), self.x.z(), self.x.w(), 
            self.y.y(), self.y.z(), self.y.w(), 
            self.z.y(), self.z.z(), self.z.w());
        m1.determinant()*self.x.x() -
        m2.determinant()*self.y.x() +
        m3.determinant()*self.z.x() -
        m4.determinant()*self.w.x()
    }
    fn cofactor(&self, column: usize, row: usize) -> Matrix3<T> {
        let mut mat3 = Matrix3::empty();
        let mut idx_y = 0;
        for i in 0..3 {
            if idx_y == row {
                idx_y += 1;
            }
            let mut idx_x = 0;
            for j in 0..3 {
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
            let xx = self.cofactor(0, 0).determinant();
            let xy = self.cofactor(1, 0).determinant();
            let xz = self.cofactor(2, 0).determinant();
            let xw = self.cofactor(3, 0).determinant();
            let yx = self.cofactor(0, 1).determinant();
            let yy = self.cofactor(1, 1).determinant();
            let yz = self.cofactor(2, 1).determinant();
            let yw = self.cofactor(3, 1).determinant();
            let zx = self.cofactor(0, 2).determinant();
            let zy = self.cofactor(1, 2).determinant();
            let zz = self.cofactor(2, 2).determinant();
            let zw = self.cofactor(3, 2).determinant();
            let wx = self.cofactor(0, 3).determinant();
            let wy = self.cofactor(1, 3).determinant();
            let wz = self.cofactor(2, 3).determinant();
            let ww = self.cofactor(3, 3).determinant();

            Self::new(
                xx, -yx, zx, -wx, 
                -xy, yy, -zy, wy, 
                xz, -yz, zz, -wz, 
                -xw, yw, -zw, ww)
    }
    fn diagonal(diagonal: Self::Column) -> Self {
        Self::new(diagonal.x(), T::ZERO, T::ZERO, T::ZERO, 
            T::ZERO, diagonal.y(), T::ZERO, T::ZERO, 
            T::ZERO, T::ZERO, diagonal.z(), T::ZERO, 
            T::ZERO, T::ZERO, T::ZERO, diagonal.w())
    }
}
impl<T: Number> From<T> for Matrix4<T> {
    ///
    /// Makes the identity element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self { x: Vector4::new(value, T::ZERO, T::ZERO, T::ZERO), y: Vector4::new(T::ZERO, value, T::ZERO, T::ZERO), z: Vector4::new(T::ZERO, T::ZERO, value, T::ZERO), w: Vector4::new(T::ZERO, T::ZERO, T::ZERO, value) }
    }
}
impl<T: Number> core::ops::Add for Matrix4<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x()), y: (self.y + rhs.y()), z: (self.z + rhs.z()), w: (self.w + rhs.w()) }
    }
    type Output = Self;
}
impl<T: Number> core::ops::Sub for Matrix4<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x()), y: (self.y - rhs.y()), z: (self.z - rhs.z()), w: (self.w - rhs.w()) }
    }
    type Output = Self;
}
impl<T: Number> core::ops::Mul for Matrix4<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector4::new(
                rhs.x().x() * self.x.x() + rhs.x().y() * self.y.x() + rhs.x().z() * self.z.x() + rhs.x().w() * self.w.x(),
                rhs.x().x() * self.x.y() + rhs.x().y() * self.y.y() + rhs.x().z() * self.z.y() + rhs.x().w() * self.w.y(),
                rhs.x().x() * self.x.z() + rhs.x().y() * self.y.z() + rhs.x().z() * self.z.z() + rhs.x().w() * self.w.z(),
                rhs.x().x() * self.x.w() + rhs.x().y() * self.y.w() + rhs.x().z() * self.z.w() + rhs.x().w() * self.w.w() 
            ), 
            y: Vector4::new(
                rhs.y().x() * self.x.x() + rhs.y().y() * self.y.x() + rhs.y().z() * self.z.x() + rhs.y().w() * self.w.x(),
                rhs.y().x() * self.x.y() + rhs.y().y() * self.y.y() + rhs.y().z() * self.z.y() + rhs.y().w() * self.w.y(),
                rhs.y().x() * self.x.z() + rhs.y().y() * self.y.z() + rhs.y().z() * self.z.z() + rhs.y().w() * self.w.z(),
                rhs.y().x() * self.x.w() + rhs.y().y() * self.y.w() + rhs.y().z() * self.z.w() + rhs.y().w() * self.w.w() 
            ),
            z: Vector4::new( 
                rhs.z().x() * self.x.x() + rhs.z().y() * self.y.x() + rhs.z().z() * self.z.x() + rhs.z().w() * self.w.x(),
                rhs.z().x() * self.x.y() + rhs.z().y() * self.y.y() + rhs.z().z() * self.z.y() + rhs.z().w() * self.w.y(),
                rhs.z().x() * self.x.z() + rhs.z().y() * self.y.z() + rhs.z().z() * self.z.z() + rhs.z().w() * self.w.z(),
                rhs.z().x() * self.x.w() + rhs.z().y() * self.y.w() + rhs.z().z() * self.z.w() + rhs.z().w() * self.w.w() 
            ),
            w: Vector4::new( 
                rhs.w().x() * self.x.x() + rhs.w().y() * self.y.x() + rhs.w().z() * self.z.x() + rhs.w().w() * self.w.x(), 
                rhs.w().x() * self.x.y() + rhs.w().y() * self.y.y() + rhs.w().z() * self.z.y() + rhs.w().w() * self.w.y(), 
                rhs.w().x() * self.x.z() + rhs.w().y() * self.y.z() + rhs.w().z() * self.z.z() + rhs.w().w() * self.w.z(), 
                rhs.w().x() * self.x.w() + rhs.w().y() * self.y.w() + rhs.w().z() * self.z.w() + rhs.w().w() * self.w.w() 
            )
        }
    }
    type Output = Self;
}
impl<T: Number> core::ops::Mul<T> for Matrix4<T>  {
    type Output = Self;
    fn mul(self, rhs: T) -> Self::Output {
        Matrix4::from_vec(self.x*rhs, self.y*rhs, self.z*rhs, self.w*rhs)
    }
}

impl<T: Number> core::ops::Mul<Vector4<T>> for Matrix4<T>  {
    /// # Multiplying Matrix4 with Vector4
    /// 
    /// when you multiply a Matrix4 with a Vector4 we treat the vector
    /// as a 4x4 matrix * 4x1 matrix since it is impossible to multiply
    /// a 4x1 matrix * 4x4 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Vector4::<T>::new(
            self.x.x() * rhs.x() + self.y.x() * rhs.y() + self.z.x() * rhs.z() + self.w.x() * rhs.w(),
            self.x.y() * rhs.x() + self.y.y() * rhs.y() + self.z.y() * rhs.z() + self.w.y() * rhs.w(),
            self.x.z() * rhs.x() + self.y.z() * rhs.y() + self.z.z() * rhs.z() + self.w.z() * rhs.w(),
            self.x.w() * rhs.x() + self.y.w() * rhs.y() + self.z.w() * rhs.z() + self.w.w() * rhs.w()
        )
    }
    type Output = Vector4<T>;
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
impl<T: Number> From<Matrix3<T>> for Matrix4<T> {
    fn from(value: Matrix3<T>) -> Self {
        Self::from_vec(
            Vector4::new(value.x().x(), value.x().y(), value.x().z(), T::ZERO ), 
            Vector4::new(value.y().x(), value.y().y(), value.y().z(), T::ZERO ), 
            Vector4::new(value.z().x(), value.z().y(), value.z().z(), T::ZERO ),
            Vector4::new(T::ZERO, T::ZERO, T::ZERO, T::ONE ),
        )
    }
}

unsafe impl<T: Number> Zeroable for Matrix4<T> {
    fn zeroed() -> Self {
        Self::ZERO
    }
}
unsafe impl<T: Number + Pod> Pod for Matrix4<T> {}

#[cfg(feature="alloc")]
mod alloc_feature {
    extern crate alloc;
    use core::fmt::Display;
    use affogato_core::num::Number;
    use alloc::{string::String};

    use crate::{matrix::{Matrix2, Matrix3, Matrix4}};
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

    impl<T: Number + Display> Display for Matrix3<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut row1 = String::from('┌');
            let mut row2 = String::from('│');
            let mut row3 = String::from('└');

            for i in 0..2 {
                let mut str_row1 = alloc::format!("{}, ", self[i].x());
                let mut str_row2 = alloc::format!("{}, ", self[i].y());
                let mut str_row3 = alloc::format!("{}, ", self[i].z());
                let max = str_row1.len().max(str_row2.len().max(str_row3.len()));
                str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
                row1.push_str(str_row1.as_str());
                row2.push_str(str_row2.as_str());
                row3.push_str(str_row3.as_str());
            }
            let mut str_row1 = alloc::format!("{}", self.z.x());
            let mut str_row2 = alloc::format!("{}", self.z.y());
            let mut str_row3 = alloc::format!("{}", self.z.z());
            let max = str_row1.len().max(str_row2.len().max(str_row3.len()));
            str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
            row1.push_str(str_row1.as_str());
            row2.push_str(str_row2.as_str());
            row3.push_str(str_row3.as_str());
            
            row1.push_str("┐\n");
            row2.push_str("│\n");
            row3.push_str("┘\n");
            f.write_str(row1.as_str())?;
            f.write_str(row2.as_str())?;
            f.write_str(row3.as_str())
        }
    }

    impl<T: Number + Display> Display for Matrix4<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let mut row1 = String::from('┌');
            let mut row2 = String::from('│');
            let mut row3 = String::from('│');
            let mut row4 = String::from('└');
            for i in 0..3 {
                let mut str_row1 = alloc::format!("{}, ", self[i].x());
                let mut str_row2 = alloc::format!("{}, ", self[i].y());
                let mut str_row3 = alloc::format!("{}, ", self[i].z());
                let mut str_row4 = alloc::format!("{}, ", self[i].w());
                let max = str_row1.len().max(str_row2.len().max(str_row3.len().max(str_row4.len())));
                str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
                str_row4.push_str((0..(max-str_row4.len())).map(|_|{' '}).collect::<String>().as_str());
                row1.push_str(str_row1.as_str());
                row2.push_str(str_row2.as_str());
                row3.push_str(str_row3.as_str());
                row4.push_str(str_row4.as_str());
            }
            let mut str_row1 = alloc::format!("{}", self.w.x());
            let mut str_row2 = alloc::format!("{}", self.w.y());
            let mut str_row3 = alloc::format!("{}", self.w.z());
            let mut str_row4 = alloc::format!("{}", self.w.w());
            let max = str_row1.len().max(str_row2.len().max(str_row3.len().max(str_row4.len())));
            str_row1.push_str((0..(max-str_row1.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row2.push_str((0..(max-str_row2.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row3.push_str((0..(max-str_row3.len())).map(|_|{' '}).collect::<String>().as_str());
            str_row4.push_str((0..(max-str_row4.len())).map(|_|{' '}).collect::<String>().as_str());
            row1.push_str(str_row1.as_str());
            row2.push_str(str_row2.as_str());
            row3.push_str(str_row3.as_str());
            row4.push_str(str_row4.as_str());
            
            row1.push_str("┐\n");
            row2.push_str("│\n");
            row3.push_str("│\n");
            row4.push_str("┘\n");
            f.write_str(row1.as_str())?;
            f.write_str(row2.as_str())?;
            f.write_str(row3.as_str())?;
            f.write_str(row4.as_str())
        }
    }

    impl<T: Number> From<Matrix4<T>> for alloc::vec::Vec<T> {
        fn from(value: Matrix4<T>) -> Self {
            alloc::vec![
                value.x().x(), value.x().y(), value.x().z(), value.x().w(), 
                value.y().x(), value.y().y(), value.y().z(), value.y().w(), 
                value.z().x(), value.z().y(), value.z().z(), value.z().w(),
                value.w().x(), value.w().y(), value.w().z(), value.w().w(),
            ]
        }
    }
    impl<T: Number> From<Matrix3<T>> for alloc::vec::Vec<T> {
        fn from(value: Matrix3<T>) -> Self {
            alloc::vec![
                value.x().x(), value.x().y(), value.x().z(), 
                value.y().x(), value.y().y(), value.y().z(), 
                value.z().x(), value.z().y(), value.z().z(),
            ]
        }
    }
    impl<T: Number> From<Matrix2<T>> for alloc::vec::Vec<T> {
        fn from(value: Matrix2<T>) -> Self {
            alloc::vec![
                value.x().x(), value.x().y(), 
                value.y().x(), value.y().y(), 
            ]
        }
    }
}

impl<T: Number> From<Matrix4<T>> for [T; 4*4] {
    fn from(value: Matrix4<T>) -> Self {
        [
            value.x().x(), value.x().y(), value.x().z(), value.x().w(), 
            value.y().x(), value.y().y(), value.y().z(), value.y().w(), 
            value.z().x(), value.z().y(), value.z().z(), value.z().w(),
            value.w().x(), value.w().y(), value.w().z(), value.w().w(),
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
impl<T: Number> From<Matrix2<T>> for [T; 2*2] {
    fn from(value: Matrix2<T>) -> Self {
        [
            value.x().x(), value.x().y(), 
            value.y().x(), value.y().y(), 
        ]
    }
}
impl<T: Number> From<Matrix4<T>> for [Vector4<T>; 4] {
    fn from(value: Matrix4<T>) -> Self {
        [
            value.x(), 
            value.y(), 
            value.z(),
            value.w(),
        ]
    }
}
impl<T: Number> From<Matrix3<T>> for [Vector3<T>; 3] {
    fn from(value: Matrix3<T>) -> Self {
        [
            value.x(), 
            value.y(), 
            value.z(),
        ]
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
#[cfg(feature = "alloc")]
pub use alloc_feature::*;

#[cfg(test)]
mod tests {
    use core::ops::Mul;

    use crate::{matrix::SquareMatrix, vector::{FMat2, FMat3, FMat4, FVec2, FVec3, FVec4, VectorSpace}, One};
    #[test]
    fn determinant_test() {
        fn inner_test<M: SquareMatrix>(matrix: M, expected: <M::Column as VectorSpace>::Scalar) {
            let determinant = matrix.determinant();
            assert!(determinant == expected, "determinant function is not implemented correctly");
        }
        inner_test(FMat2::new(1.0, 2.0, 3.0, 4.0), -2.0);
        inner_test(FMat3::new(1.0, 4.0, 5.0, 4.0, 5.0, 4.0, 1.0, 8.0, 6.0), 53.0);
        inner_test(FMat4::new(
            1.0, 4.0, 6.0, 3.0, 
            12.0, 3.0, 5.0, 6.0,
            0.0, 0.0, 45.0, 1.0,
            3.0, 2.0, 0.0, 2.0 
        ), 535.0);
    }
    #[test]
    fn transpose_test() {
        assert!(
            FMat2::new(1.0, 2.0, 3.0, 4.0).transpose().epsilon_eq(&FMat2::new(1.0, 3.0, 2.0, 4.0), 0.00001), 
            "The transpose of a matrix was implemented incorrectly"
        );
        assert!(
            FMat3::new(1.0, 4.0, 5.0, 4.0, 5.0, 4.0, 1.0, 8.0, 6.0).transpose().epsilon_eq( 
                &FMat3::new(1.0, 4.0, 1.0, 4.0, 5.0, 8.0, 5.0, 4.0, 6.0),
                0.00001
            ), "The transpose of a matrix was implemented incorrectly"
        );
        assert!(
            FMat4::new(
                1.0, 4.0, 6.0, 3.0, 
                12.0, 3.0, 5.0, 6.0,
                0.0, 0.0, 45.0, 1.0,
                3.0, 2.0, 0.0, 2.0 
            ).transpose().epsilon_eq(
                &FMat4::new(
                    1.0, 12.0, 0.0, 3.0,
                    4.0, 3.0, 0.0, 2.0, 
                    6.0, 5.0, 45.0, 0.0, 
                    3.0, 6.0, 1.0, 2.0
                ),
                0.00001
            ), "The transpose of a matrix was implemented incorrectly"
        );
    }
    #[test]
    fn inverse_test() {
        let matrix = FMat2::new(1.0, 2.0, 3.0, 4.0);
        assert!(
            matrix*matrix.inverse().unwrap() == FMat2::identity(), "The inverse of a matrix was implemented incorrectly"
        );
        let matrix = FMat3::new(1.0, 3.0, 5.0, 4.0, 7.0, 8.0, 1.0, 1.0, 1.0);
        assert!(
            matrix*matrix.inverse().unwrap() == FMat3::identity(), "The inverse of a matrix was implemented incorrectly"
        );
        let matrix = FMat4::new(
            1.0, 1.0, 1.0, 1.0,
            2.0, 4.0, 6.0, 4.0,
            6.0, 4.0, 7.0, 1.0,
            1.0, 1.0, 1.0, 2.0
        );
        assert!(
            (matrix*matrix.inverse().unwrap()).epsilon_eq(&FMat4::identity(), 0.00001), "The inverse of a matrix was implemented incorrectly"
        );
    }
    #[test]
    fn mul_test() {
        // Matrix2 mul test
        let a = FMat2::new(1.0, 2.0, 3.0, 4.0);
        let b = FMat2::new(2.0, 4.0, 6.0, 8.0);
        assert!(a.mul(b) == FMat2::new(14.0, 20.0, 30.0, 44.0), "Implemented mul incorrectly");
        assert!(b.mul(a) == FMat2::new(14.0, 20.0, 30.0, 44.0), "Implemented mul incorrectly");
        // Matrix3 mul test
        let a = FMat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let b = FMat3::new(2.0, 4.0, 6.0, 2.0, 4.0, 6.0, 2.0, 4.0, 6.0);
        assert!(a.mul(b) == FMat3::new(60.0, 72.0, 84.0, 60.0, 72.0, 84.0, 60.0, 72.0, 84.0), "Implemented mul incorrectly");
        assert!(b.mul(a) == FMat3::new(12.0, 24.0, 36.0, 30.0, 60.0, 90.0, 48.0, 96.0, 144.0), "Implemented mul incorrectly");
        // Matrix4 mul test
        let a = FMat4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let b = FMat4::new(2.0, 4.0, 6.0, 8.0, 2.0, 4.0, 6.0, 8.0,  2.0, 4.0, 6.0, 8.0,  2.0, 4.0, 6.0, 8.0);
        assert!(a.mul(b) == FMat4::new(180.0, 200.0, 220.0, 240.0, 180.0, 200.0, 220.0, 240.0, 180.0, 200.0, 220.0, 240.0, 180.0, 200.0, 220.0, 240.0), "Implemented mul incorrectly");
        assert!(b.mul(a) == FMat4::new(20.0, 40.0, 60.0, 80.0, 52.0, 104.0, 156.0, 208.0, 84.0, 168.0, 252.0, 336.0, 116.0, 232.0, 348.0, 464.0), "Implemented mul incorrectly");
    }
    #[test]
    fn mul_vector_test() {
        let a = FMat2::new(1.0, 2.0, 3.0, 4.0);
        let vb = FVec2::new(1.0, 2.0);
        assert!(a.mul(vb) == FVec2::new(7.0, 10.0), "Implemented mul for vectors incorrectly");
        let a = FMat3::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0);
        let vb = FVec3::new(1.0, 2.0, 3.0);
        assert!(a.mul(vb) == FVec3::new(30.0, 36.0, 42.0), "Implemented mul for vectors incorrectly");
        let a = FMat4::new(1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0);
        let vb = FVec4::new(1.0, 2.0, 3.0, 4.0);
        assert!(a.mul(vb) == FVec4::new(90.0, 100.0, 110.0, 120.0), "Implemented mul for vectors incorrectly");
    }
}