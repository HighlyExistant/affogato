#![allow(unused)]
use crate::{algebra::Quaternion, sets::Number, RationalNumber};

use super::{Vector2, Vector3, Vector4};

pub trait SquareMatrix: Sized {
    fn set_identity(&mut self) { *self = Self::identity(); }
    fn identity() -> Self;
    fn transpose(&self) -> Self;
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix2<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix2x3<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix2x4<T> {
    pub x: Vector2<T>,
    pub y: Vector2<T>,
    pub z: Vector2<T>,
    pub w: Vector2<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix3<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix3x2<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix3x4<T> {
    pub x: Vector3<T>,
    pub y: Vector3<T>,
    pub z: Vector3<T>,
    pub w: Vector3<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Matrix4<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
    pub z: Vector4<T>,
    pub w: Vector4<T>,
}
#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Matrix4x2<T> {
    pub x: Vector4<T>,
    pub y: Vector4<T>,
}

/// ===========================================================
/// 
/// Implementation for Matrix2
/// 
/// ===========================================================
impl<T: Number> SquareMatrix for Matrix2<T> {
    fn identity() -> Self {
        Self { 
            x: Vector2 { 
                x: T::one(), 
                y: T::zero() }, 
            y: Vector2 { 
                x: T::zero(), 
                y: T::one() 
            } 
        }
    }
    fn transpose(&self) -> Self {
        Self { 
            x: Vector2 { 
                x: self.y.y, 
                y: self.y.x 
            }, 
            y: Vector2 { 
                x: self.x.y, 
                y: self.x.x
            } 
        }
    }
}
///
/// These functions are available for all matrices.
/// 
impl<T: Number> Matrix2<T>  {
    pub fn new(xx: T, xy: T, yx: T, yy: T) -> Self {
        Self { x: Vector2 { x: xx, y: xy }, y: Vector2 { x: yx, y: yy } }
    }
    pub fn from_vec(x: Vector2<T>, y: Vector2<T>) -> Self {
        Self { x, y }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix2<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix2<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Matrix2<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector2 { 
                x: self.x.x * rhs.x.x + self.y.x * rhs.x.y, 
                y: self.x.y * rhs.x.x + self.y.y * rhs.x.y 
            }, 
            y: Vector2 { 
                x: self.x.x * rhs.y.x + self.y.x * rhs.y.y, 
                y: self.x.y * rhs.y.x + self.y.y * rhs.y.y 
            } 
        }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Vector2<T>> for Matrix2<T>  {
    /// # Multiplying Matrix2 with Vector2
    /// 
    /// when you multiply a Matrix2 with a Vector2 we treat the vector
    /// as a 2x2 matrix * 2x1 matrix since it is impossible to multiply
    /// a 2x1 matrix * 2x2 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Vector2<T>) -> Self::Output {
        Vector2::<T> {
            x: self.x.x * rhs.x + self.x.y * rhs.y,
            y: self.y.x * rhs.x + self.y.y * rhs.y
        }
    }
    type Output = Vector2<T>;
}
impl<T: Number> From<T> for Matrix2<T> {
    ///
    /// Makes every element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self { x: Vector2::from(value), y: Vector2::from(value) }
    }
}
/// ===========================================================
/// 
/// Implementation for Matrix2x3
/// 
/// ===========================================================
impl<T: Number> Matrix2x3<T> {
    pub fn new(xx: T, xy: T, yx: T, yy: T, zx: T, zy: T) -> Self {
        Self { x: Vector2 { x: xx, y: xy }, y: Vector2 { x: yx, y: yy }, z: Vector2 { x: zx, y: zy } }
    }
    pub fn from_vec(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>) -> Self {
        Self { x, y, z }
    }
}
// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix2x3<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix2x3<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Matrix3<T>> for Matrix2x3<T>  {
    fn mul(self, rhs: Matrix3<T>) -> Self::Output {
        Self { 
            x: Vector2 { 
                x: self.x.x * rhs.x.x + self.y.x * rhs.x.y + self.z.x * rhs.x.z, 
                y: self.x.y * rhs.x.x + self.y.y * rhs.x.y + self.z.y * rhs.x.z
            }, 
            y: Vector2 { 
                x: self.x.x * rhs.y.x + self.y.x * rhs.y.y + self.z.x * rhs.y.z, 
                y: self.x.y * rhs.y.x + self.y.y * rhs.y.y + self.z.y * rhs.y.z 
            },
            z: Vector2 { 
                x: self.x.x * rhs.z.x + self.y.x * rhs.z.y + self.z.x * rhs.z.z,
                y: self.x.y * rhs.z.x + self.y.y * rhs.z.y + self.z.y * rhs.z.z 
            }
        }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Matrix3x2<T>> for Matrix2x3<T>  {
    fn mul(self, rhs: Matrix3x2<T>) -> Self::Output {
        Matrix2 {
            x: Vector2 { 
                x: self.x.x * rhs.x.x + self.y.x * rhs.x.y + self.z.x * rhs.x.z, 
                y: self.x.y * rhs.x.x + self.y.y * rhs.x.y + self.z.y * rhs.x.z
            },
            y: Vector2 { 
                x: self.x.x * rhs.y.x + self.y.x * rhs.y.y + self.z.x * rhs.y.z, 
                y: self.x.y * rhs.y.x + self.y.y * rhs.y.y + self.z.y * rhs.y.z 
            },
        }
    }
    type Output = Matrix2<T>;
}
/// ===========================================================
/// 
/// Implementation for Matrix2x4
/// 
/// ===========================================================
impl<T: Number> Matrix2x4<T> {
    pub fn new(xx: T, xy: T, yx: T, yy: T, zx: T, zy: T, wx: T, wy: T) -> Self {
        Self { x: Vector2 { x: xx, y: xy }, y: Vector2 { x: yx, y: yy }, z: Vector2 { x: zx, y: zy }, w: Vector2 { x: wx, y: wy }  }
    }
    pub fn from_vec(x: Vector2<T>, y: Vector2<T>, z: Vector2<T>, w: Vector2<T>) -> Self {
        Self { x, y, z, w }
    }
}
// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix2x4<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z), w: (self.w + rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix2x4<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z), w: (self.w - rhs.w)  }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Matrix4<T>> for Matrix2x4<T> {
    fn mul(self, rhs: Matrix4<T>) -> Self::Output {
        Self { 
            x: Vector2 { 
                x: self.x.x * rhs.x.x + self.y.x * rhs.x.y + self.z.x * rhs.x.z + self.w.x * rhs.x.w, 
                y: self.x.y * rhs.x.x + self.y.y * rhs.x.y + self.z.y * rhs.x.z + self.w.y * rhs.x.w 
            },
            y: Vector2 { 
                x: self.x.x * rhs.y.x + self.y.x * rhs.y.y + self.z.x * rhs.y.z + self.w.x * rhs.y.w, 
                y: self.x.y * rhs.y.x + self.y.y * rhs.y.y + self.z.y * rhs.y.z + self.w.y * rhs.y.w 
            },
            z: Vector2 { 
                x: self.x.x * rhs.z.x + self.y.x * rhs.z.y + self.z.x * rhs.z.z + self.w.x * rhs.z.w, 
                y: self.x.y * rhs.z.x + self.y.y * rhs.z.y + self.z.y * rhs.z.z + self.w.y * rhs.z.w 
            },
            w: Vector2 { 
                x: self.x.x * rhs.w.x + self.y.x * rhs.w.y + self.z.x * rhs.w.z + self.w.x * rhs.w.w, 
                y: self.x.y * rhs.w.x + self.y.y * rhs.w.y + self.z.y * rhs.w.z + self.w.y * rhs.w.w 
            },
        }
    }
    type Output = Self;
}
/// ===========================================================
/// 
/// Implementation for Matrix3
/// 
/// ===========================================================

///
/// These functions are available for all matrices.
/// 
impl<T: Number> Matrix3<T>  {
    pub fn new(xx: T, xy: T, xz: T, yx: T, yy: T, yz: T, zx: T, zy: T, zz: T) -> Self {
        Self { x: Vector3::new(xx,xy,xz), y: Vector3::new(yx, yy, yz ), z: Vector3::new(zx, zy, zz) }
    }
    pub fn from_vec(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>) -> Self {
        Self { x, y, z }
    }
    pub fn translate(mut self, translate: Vector3<T>) -> Self {
        self.x.z = translate.x;
        self.y.z = translate.y;
        self.z.z = translate.z;
        self
    }
    pub fn from_scale(v: Vector3<T>) -> Self {
        Matrix3::new(
            v.x, T::zero(), T::zero(), 
            T::zero(), v.y, T::zero(), 
            T::zero(), T::zero(), v.z
        )
    }
    pub fn from_translation(v: Vector3<T>) -> Self {
        Matrix3::new(
            T::one(), T::zero(), T::zero(), 
            T::zero(), T::one(), T::zero(), 
            v.x, v.y, v.z
        )
    }
}
///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: RationalNumber> Matrix3<T>  {
    pub fn from_transform(translation: Vector2<T>, scaling: Vector2<T>, rotation: T) -> Self {
        Matrix3::new(
            rotation.cos()*scaling.x, -(rotation.sin()), T::zero(), 
            rotation.sin(), rotation.cos()*scaling.y, T::zero(), 
            translation.x, translation.y, T::one()
        )
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix3<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix3<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Matrix3<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector3::new( 
                rhs.x.x * self.x.y + rhs.x.y * self.y.y + rhs.x.z * self.z.y,
                rhs.x.x * self.x.x + rhs.x.y * self.y.x + rhs.x.z * self.z.x,
                rhs.x.x * self.x.z + rhs.x.y * self.y.z + rhs.x.z * self.z.z,
            ), 
            y: Vector3::new( 
                rhs.y.x * self.x.x + rhs.y.y * self.y.x + rhs.y.z * self.z.x,
                rhs.y.x * self.x.y + rhs.y.y * self.y.y + rhs.y.z * self.z.y,
                rhs.y.x * self.x.z + rhs.y.y * self.y.z + rhs.y.z * self.z.z,
            ),
            z: Vector3::new(
                rhs.z.x * self.x.x + rhs.z.y * self.y.x + rhs.z.z * self.z.x,
                rhs.z.x * self.x.y + rhs.z.y * self.y.y + rhs.z.z * self.z.y,
                rhs.z.x * self.x.z + rhs.z.y * self.y.z + rhs.z.z * self.z.z,
            )
        }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Vector3<T>> for Matrix3<T>  {
    /// # Multiplying Matrix3 with Vector3
    /// 
    /// when you multiply a Matrix3 with a Vector3 we treat the vector
    /// as a 3x3 matrix * 3x1 matrix since it is impossible to multiply
    /// a 3x1 matrix * 3x3 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        Vector3::new(
           self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z,
           self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z,
           self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z
        )
    }
    type Output = Vector3<T>;
}
impl<T: Number> From<T> for Matrix3<T> {
    ///
    /// Makes every element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self { x: Vector3::from(value), y: Vector3::from(value), z: Vector3::from(value) }
    }
}
impl<T: Number> SquareMatrix for Matrix3<T> {
    fn identity() -> Self {
        Self { 
            x: Vector3::new( 
                T::one(), 
                T::zero(), 
                T::zero() 
            ), 
            y: Vector3::new( 
                T::zero(), 
                T::one(), 
                T::zero() 
            ), 
            z: Vector3::new( 
                T::zero(), 
                T::zero(), 
                T::one() 
            ) 
        }
    }
    fn transpose(&self) -> Self {
        Self { 
            x: Vector3::new( 
                self.x.x, 
                self.y.x, 
                self.z.x
            ), 
            y: Vector3::new( 
                self.x.y,
                self.y.y,
                self.z.y
            ), 
            z: Vector3::new( 
                self.x.z, 
                self.y.z, 
                self.z.z
            )
        }
    }
}

/// ===========================================================
/// 
/// Implementation for Matrix3x2
/// 
/// ===========================================================

impl<T: Number> Matrix3x2<T>  {
    pub fn new(xx: T, xy: T, xz: T, yx: T, yy: T, yz: T) -> Self {
        Self { x: Vector3::new(xx, xy,xz), y: Vector3::new(yx, yy, yz) }
    }
    pub fn from_vec(x: Vector3<T>, y: Vector3<T>) -> Self {
        Self { x, y }
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix3x2<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix3x2<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Matrix2<T>> for Matrix3x2<T>  {
    fn mul(self, rhs: Matrix2<T>) -> Self::Output {
        Self { 
            x: Vector3::new(
                rhs.x.x * self.x.x + rhs.x.y * self.y.x,
                rhs.x.x * self.x.y + rhs.x.y * self.y.y,
                rhs.x.x * self.x.z + rhs.x.y * self.y.z
            ), 
            y: Vector3::new(
                rhs.y.x * self.x.x + rhs.y.y * self.y.x,
                rhs.y.x * self.x.y + rhs.y.y * self.y.y,
                rhs.y.x * self.x.z + rhs.y.y * self.y.z
            ),
        }
    }
    type Output = Self; 
}

/// ===========================================================
/// 
/// Implementation for Matrix3x4
/// 
/// ===========================================================

impl<T: Number> Matrix3x4<T>  {
    pub fn new(xx: T, xy: T, xz: T, yx: T, yy: T, yz: T, zx: T, zy: T, zz: T, wx: T, wy: T, wz: T) -> Self {
        Self { 
            x: Vector3::new(xx, xy, xz), 
            y: Vector3::new(yx, yy, yz), 
            z: Vector3::new(zx, zy, zz), 
            w: Vector3::new(wx, wy, wz),
        }
    }
    pub fn from_vec(x: Vector3<T>, y: Vector3<T>, z: Vector3<T>, w: Vector3<T>) -> Self {
        Self { x, y, z, w }
    }
}
impl<T: Number> std::ops::Add for Matrix3x4<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z), w: (self.w + rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix3x4<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z), w: (self.w - rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul<Matrix4<T>> for Matrix3x4<T>  {
    fn mul(self, rhs: Matrix4<T>) -> Self::Output {
        Self { 
            x: Vector3::new( 
                rhs.x.x * self.x.x + rhs.x.y * self.y.x + rhs.x.z * self.z.x + rhs.x.w * self.w.x,
                rhs.x.x * self.x.y + rhs.x.y * self.y.y + rhs.x.z * self.z.y + rhs.x.w * self.w.y,
                rhs.x.x * self.x.z + rhs.x.y * self.y.z + rhs.x.z * self.z.z + rhs.x.w * self.w.z
            ), 
            y: Vector3::new( 
                rhs.y.x * self.x.x + rhs.y.y * self.y.x + rhs.y.z * self.z.x + rhs.y.w * self.w.x,
                rhs.y.x * self.x.y + rhs.y.y * self.y.y + rhs.y.z * self.z.y + rhs.y.w * self.w.y,
                rhs.y.x * self.x.z + rhs.y.y * self.y.z + rhs.y.z * self.z.z + rhs.y.w * self.w.z
            ),
            z: Vector3::new(
                rhs.z.x * self.x.x + rhs.z.y * self.y.x + rhs.z.z * self.z.x + rhs.z.w * self.w.x,
                rhs.z.x * self.x.y + rhs.z.y * self.y.y + rhs.z.z * self.z.y + rhs.z.w * self.w.y,
                rhs.z.x * self.x.z + rhs.z.y * self.y.z + rhs.z.z * self.z.z + rhs.z.w * self.w.z
            ),
            w: Vector3::new(
                rhs.w.x * self.x.x + rhs.w.y * self.y.x + rhs.w.z * self.z.x + rhs.w.w * self.w.x, 
                rhs.w.x * self.x.y + rhs.w.y * self.y.y + rhs.w.z * self.z.y + rhs.w.w * self.w.y, 
                rhs.w.x * self.x.z + rhs.w.y * self.y.z + rhs.w.z * self.z.z + rhs.w.w * self.w.z
            )
        }
    }
    type Output = Self;
}
/// ===========================================================
/// 
/// Implementation for Matrix4
/// 
/// ===========================================================

///
/// These functions are available for all matrices.
/// 
impl<T: Number> Matrix4<T>  {
    pub fn new(xx: T, xy: T, xz: T, xw: T, yx: T, yy: T, yz: T, yw: T, zx: T, zy: T, zz: T, zw: T, wx: T, wy: T, wz: T, ww: T) -> Self {
        Self { x: Vector4 { x: xx, y: xy, z: xz, w: xw }, y: Vector4 { x: yx, y: yy, z: yz, w: yw }, z: Vector4 { x: zx, y: zy, z: zz, w: zw }, w: Vector4 { x: wx, y: wy, z: wz, w: ww } }
    }
    pub fn from_vec(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y, z, w }
    }
}

///
/// These functions are available for all numbers including
/// floating point.
/// 
impl<T: Number> Matrix4<T>  {
    pub fn from_translation(v: Vector3<T>) -> Self {
        Matrix4::new(
            T::one(), T::zero(), T::zero(), T::zero(),
            T::zero(), T::one(), T::zero(), T::zero(),
            T::zero(), T::zero(), T::one(), T::zero(),
            v.x, v.y, v.z, T::one(),
        )
    }
    pub fn from_scale(v: Vector3<T>) -> Self {
        Matrix4::new(
            v.x, T::zero(), T::zero(), T::zero(),
            T::zero(), v.y, T::zero(), T::zero(),
            T::zero(), T::zero(), v.z, T::zero(),
            T::zero(), T::zero(), T::zero(), T::one(),
        )
    }
    pub fn scale(&self, v: Vector3<T>) -> Self {
        Self { x: self.x * v.x, y: self.y * v.y, z: self.z * v.z, w: self.w }
    }
    pub fn translate(&self, v: Vector3<T>) -> Self {
        Self { x: self.x, y: self.y, z: self.z, w: self.w + Vector4::<T>::from(v) }
        
    }
}
impl<T: RationalNumber> From<Quaternion<T>> for Matrix4<T> {
    fn from(value: Quaternion<T>) -> Self {
        let x2 = value.vector.x + value.vector.x;
        let y2 = value.vector.y + value.vector.y;
        let z2 = value.vector.z + value.vector.z;

        let xx2 = x2 * value.vector.x;
        let xy2 = x2 * value.vector.y;
        let xz2 = x2 * value.vector.z;

        let yy2 = y2 * value.vector.y;
        let yz2 = y2 * value.vector.z;
        let zz2 = z2 * value.vector.z;

        let sy2 = y2 * value.scalar;
        let sz2 = z2 * value.scalar;
        let sx2 = x2 * value.scalar;

        Self { 
            x: Vector4 { 
                x: T::one() - yy2 - zz2, 
                y: xy2 + sz2, 
                z: xz2 - sy2, 
                w: T::zero()
            }, 
            y: Vector4 { 
                x: xy2 - sz2, 
                y: T::one() - xx2 - zz2, 
                z: yz2 + sx2, 
                w: T::zero()
            }, 
            z: Vector4 { 
                x: xz2 + sy2, 
                y: yz2 - sx2, 
                z: T::one() - xx2 - yy2, 
                w: T::zero()
            }, 
            w: Vector4 { 
                x: T::zero(), 
                y: T::zero(), 
                z: T::zero(), 
                w: T::one() 
            } 
        }
    }
}

impl<T: Number> SquareMatrix for Matrix4<T> {
    fn identity() -> Self {
        Self { 
            x: Vector4 { 
                x: T::one(),  
                y: T::zero(), 
                z: T::zero(), 
                w: T::zero() 
            }, 
            y: Vector4 { 
                x: T::zero(), 
                y: T::one(), 
                z: T::zero(), 
                w: T::zero() 
            }, 
            z: Vector4 { 
                x: T::zero(), 
                y: T::zero(), 
                z: T::one(), 
                w: T::zero() 
            }, 
            w: Vector4 { 
                x: T::zero(), 
                y: T::zero(), 
                z: T::zero(), 
                w: T::one() 
            } 
        }
    }
    fn transpose(&self) -> Self {
        
        Self { 
            x: Vector4 { 
                x: self.x.x, 
                y: self.y.x, 
                z: self.z.x, 
                w: self.w.x,
            }, 
            y: Vector4 { 
                x: self.x.y, 
                y: self.y.y, 
                z: self.z.y, 
                w: self.w.y, 
            }, 
            z: Vector4 { 
                x: self.x.z, 
                y: self.y.z, 
                z: self.z.z, 
                w: self.w.z, 
            }, 
            w: Vector4 { 
                x: self.x.w, 
                y: self.y.w, 
                z: self.z.w, 
                w: self.w.w, 
            } 
        }
    }
}
impl<T: Number> From<T> for Matrix4<T> {
    ///
    /// Makes every element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self { x: Vector4::from(value), y: Vector4::from(value), z: Vector4::from(value), w: Vector4::from(value) }
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix4<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y), z: (self.z + rhs.z), w: (self.w + rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix4<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y), z: (self.z - rhs.z), w: (self.w - rhs.w) }
    }
    type Output = Self;
}
impl<T: Number> std::ops::Mul for Matrix4<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            x: Vector4 { 
                x: rhs.x.x * self.x.x + rhs.x.y * self.y.x + rhs.x.z * self.z.x + rhs.x.w * self.w.x,
                y: rhs.x.x * self.x.y + rhs.x.y * self.y.y + rhs.x.z * self.z.y + rhs.x.w * self.w.y,
                z: rhs.x.x * self.x.z + rhs.x.y * self.y.z + rhs.x.z * self.z.z + rhs.x.w * self.w.z,
                w: rhs.x.x * self.x.w + rhs.x.y * self.y.w + rhs.x.z * self.z.w + rhs.x.w * self.w.w 
            }, 
            y: Vector4 { 
                x: rhs.y.x * self.x.x + rhs.y.y * self.y.x + rhs.y.z * self.z.x + rhs.y.w * self.w.x,
                y: rhs.y.x * self.x.y + rhs.y.y * self.y.y + rhs.y.z * self.z.y + rhs.y.w * self.w.y,
                z: rhs.y.x * self.x.z + rhs.y.y * self.y.z + rhs.y.z * self.z.z + rhs.y.w * self.w.z,
                w: rhs.y.x * self.x.w + rhs.y.y * self.y.w + rhs.y.z * self.z.w + rhs.y.w * self.w.w 
            },
            z: Vector4 { 
                x: rhs.z.x * self.x.x + rhs.z.y * self.y.x + rhs.z.z * self.z.x + rhs.z.w * self.w.x,
                y: rhs.z.x * self.x.y + rhs.z.y * self.y.y + rhs.z.z * self.z.y + rhs.z.w * self.w.y,
                z: rhs.z.x * self.x.z + rhs.z.y * self.y.z + rhs.z.z * self.z.z + rhs.z.w * self.w.z,
                w: rhs.z.x * self.x.w + rhs.z.y * self.y.w + rhs.z.z * self.z.w + rhs.z.w * self.w.w 
            },
            w: Vector4 { 
                x: rhs.w.x * self.x.x + rhs.w.y * self.y.x + rhs.w.z * self.z.x + rhs.w.w * self.w.x, 
                y: rhs.w.x * self.x.y + rhs.w.y * self.y.y + rhs.w.z * self.z.y + rhs.w.w * self.w.y, 
                z: rhs.w.x * self.x.z + rhs.w.y * self.y.z + rhs.w.z * self.z.z + rhs.w.w * self.w.z, 
                w: rhs.w.x * self.x.w + rhs.w.y * self.y.w + rhs.w.z * self.z.w + rhs.w.w * self.w.w 
            }
        }
    }
    type Output = Self;
}

impl<T: Number> std::ops::Mul<Vector4<T>> for Matrix4<T>  {
    /// # Multiplying Matrix4 with Vector4
    /// 
    /// when you multiply a Matrix4 with a Vector4 we treat the vector
    /// as a 4x4 matrix * 4x1 matrix since it is impossible to multiply
    /// a 4x1 matrix * 4x4 matrix since matrix multiplication is not commutative.
    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Vector4::<T> {
            x: self.x.x * rhs.x + self.x.y * rhs.y + self.x.z * rhs.z + self.x.w * rhs.w,
            y: self.y.x * rhs.x + self.y.y * rhs.y + self.y.z * rhs.z + self.y.w * rhs.w,
            z: self.z.x * rhs.x + self.z.y * rhs.y + self.z.z * rhs.z + self.z.w * rhs.w,
            w: self.w.x * rhs.x + self.w.y * rhs.y + self.w.z * rhs.z + self.w.w * rhs.w
        }
    }
    type Output = Vector4<T>;
}

impl<T: Number> From<Matrix4<T>> for Matrix3<T> {
    fn from(value: Matrix4<T>) -> Self {
        Self { 
            x: Vector3::new(value.x.x, value.x.y, value.x.z), 
            y: Vector3::new(value.y.x, value.y.y, value.y.z), 
            z: Vector3::new(value.z.x, value.z.y, value.z.z) 
        }
    }
}
impl<T: Number> From<Matrix3<T>> for Matrix4<T> {
    fn from(value: Matrix3<T>) -> Self {
        Self { 
            x: Vector4 { x: value.x.x, y: value.x.y, z: value.x.z, w: T::zero() }, 
            y: Vector4 { x: value.y.x, y: value.y.y, z: value.y.z, w: T::zero() }, 
            z: Vector4 { x: value.z.x, y: value.z.y, z: value.z.z, w: T::zero() },
            w: Vector4 { x: T::zero(), y: T::zero(), z: T::zero(), w: T::one() },
        }
    }
}

/// ===========================================================
/// 
/// Implementation for Matrix4x2
/// 
/// ===========================================================

impl<T: Number> Matrix4x2<T>  {
    pub fn new(xx: T, xy: T, xz: T, xw: T, yx: T, yy: T, yz: T, yw: T) -> Self {
        Self { x: Vector4 { x: xx, y: xy, z: xz, w: xw }, y: Vector4 { x: yx, y: yy, z: yz, w: yw } }
    }
    pub fn from_vec(x: Vector4<T>, y: Vector4<T>, z: Vector4<T>, w: Vector4<T>) -> Self {
        Self { x, y}
    }
}

impl<T: Number> From<T> for Matrix4x2<T> {
    ///
    /// Makes every element in  the matrix the value specified
    /// 
    fn from(value: T) -> Self {
        Self { x: Vector4::from(value), y: Vector4::from(value) }
    }
}

// traits for bitwise operations
impl<T: Number> std::ops::Add for Matrix4x2<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self { x: (self.x + rhs.x), y: (self.y + rhs.y)}
    }
    type Output = Self;
}
impl<T: Number> std::ops::Sub for Matrix4x2<T>  {
    fn sub(self, rhs: Self) -> Self::Output {
        Self { x: (self.x - rhs.x), y: (self.y - rhs.y) }
    }
    type Output = Self;
}

impl <T: Number> std::ops::Mul<Matrix2<T>> for Matrix4x2<T> {
    fn mul(self, rhs: Matrix2<T>) -> Self::Output {
        Self { 
            x: Vector4 { 
                x: rhs.x.x * self.x.x + rhs.x.y * self.y.x,
                y: rhs.x.x * self.x.y + rhs.x.y * self.y.y,
                z: rhs.x.x * self.x.z + rhs.x.y * self.y.z,
                w: rhs.x.x * self.x.w + rhs.x.y * self.y.w 
            }, 
            y: Vector4 { 
                x: rhs.y.x * self.x.x + rhs.y.y * self.y.x,
                y: rhs.y.x * self.x.y + rhs.y.y * self.y.y,
                z: rhs.y.x * self.x.z + rhs.y.y * self.y.z,
                w: rhs.y.x * self.x.w + rhs.y.y * self.y.w 
            },
        }
    }
    type Output = Self;
}

impl<T: Number> Default for Matrix2<T> {
    fn default() -> Self {
        Self::identity()
    }
}
impl<T: Number> Default for Matrix3<T> {
    fn default() -> Self {
        Self::identity()
    }
}

impl<T: Number> Default for Matrix4<T> {
    fn default() -> Self {
        Self::identity()
    }
}


/*
 * TODO LIST:
 * Implement multiplication for the following:
 * Matrix2x3 * Matrix3x4
 * Matrix2x4 * Matrix4x2
 * Matrix3x2 * Matrix2x3
 * Matrix3x2 * Matrix2x4
 * Matrix3x4 * Matrix4x2
 * Matrix3x4 * Matrix4x3
 * Matrix4x2 * Matrix2x3
 * Matrix4x2 * Matrix2x4
*/