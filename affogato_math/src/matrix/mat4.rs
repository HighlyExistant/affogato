use core::{fmt::Display, ops::{Index, IndexMut}};

use affogato_core::{groups::vector_spaces::VectorSpace, num::{Number, Signed, Zero}, sets::Real};
use bytemuck::{Pod, Zeroable};

use crate::{algebra::Quaternion, matrix::{Matrix2, Matrix3, SquareMatrix}, vector::{Vector2, Vector3, Vector4}};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};


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
        where T: Real + Display {
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