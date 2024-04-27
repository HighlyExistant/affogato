#![allow(unused)]
use std::fmt::Debug;

use num_traits::AsPrimitive;

use crate::{linear::{Matrix3, Vector3}, FloatingPoint, Number, Rotation};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Quaternion<T: FloatingPoint> {
    pub vector: Vector3<T>,
    pub scalar: T,
}

/// ===========================================================
/// 
/// Implementation for Quaternion
/// 
/// ===========================================================

///
/// These functions are available for all vectors.
/// 
pub fn normalize_radians(a: f32) -> f32 {
    const PI2: f32 = 2.0*std::f32::consts::PI;
    a - PI2 * f32::floor((a + std::f32::consts::PI)/PI2)
}
impl<T: FloatingPoint> Quaternion<T>  {
    pub fn new(v: Vector3<T>, s: T) -> Self {
        Self { vector: v, scalar: s }
    }
    /// # from_euler
    /// 
    /// converts euler angles into quaternion form.
    /// 
    /// this function is heavily inspired by [this stackexchange post](https://math.stackexchange.com/questions/2975109/how-to-convert-euler-angles-to-quaternions-and-get-the-same-euler-angles-back-fr)
    pub fn from_euler(v: Vector3<T>) -> Self 
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        let (yaw, pitch, roll) = (v.x, v.y, v.z);
        let x = T::sin(roll*0.5.as_()) * T::cos(pitch*0.5.as_()) * T::cos(yaw*0.5.as_()) - T::cos(roll*0.5.as_()) * T::sin(pitch*0.5.as_()) * T::sin(yaw*0.5.as_());
        let y = T::cos(roll*0.5.as_()) * T::sin(pitch*0.5.as_()) * T::cos(yaw*0.5.as_()) + T::sin(roll*0.5.as_()) * T::cos(pitch*0.5.as_()) * T::sin(yaw*0.5.as_());
        let z = T::cos(roll*0.5.as_()) * T::cos(pitch*0.5.as_()) * T::sin(yaw*0.5.as_()) - T::sin(roll*0.5.as_()) * T::sin(pitch*0.5.as_()) * T::cos(yaw*0.5.as_());
        let w = T::cos(roll*0.5.as_()) * T::cos(pitch*0.5.as_()) * T::cos(yaw*0.5.as_()) + T::sin(roll*0.5.as_()) * T::sin(pitch*0.5.as_()) * T::sin(yaw*0.5.as_());
        Self { vector: Vector3::new(x, y, z), scalar: w }
        
    }
    pub fn angle_axis(angle: T, vector: Vector3<T>) -> Self 
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        let half_angle = angle * 0.5.as_();
        let s = half_angle.sin();
        Self { vector: vector * s, scalar: half_angle.cos() }
    }
    pub fn to_euler(&self) -> Vector3<T>
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        let (x, y, z, w) = (self.vector.x, self.vector.y, self.vector.z, self.scalar);
        let t0 = 2.0.as_() * (w * x + y * z);
        let t1 = 1.0.as_() - 2.0.as_() * (x * x + y * y);
        let roll = T::atan2(t0, t1);
        let t2 = 2.0.as_() * (w * y - z * x);
        let t2 = if t2 > 1.0.as_() {1.0.as_()} else {t2};
        let t2 = if t2 < (-1.0).as_() {(-1.0).as_()} else {t2};
        let pitch = T::asin(t2);
        let t3 = 2.0.as_() * (w * z + x * y);
        let t4 = 1.0.as_() - 2.0.as_() * (y * y + z * z);
        let yaw = T::atan2(t3, t4);
        return Vector3::new(normalize_radians(yaw.as_()).as_(), normalize_radians(pitch.as_()).as_(), normalize_radians(roll.as_()).as_());

    }
    pub fn euler_rotate(&mut self, euler: Vector3<T>)
        where T: FloatingPoint,
        f32: AsPrimitive<T>, 
        f64: AsPrimitive<T> {
        let euler_ = Quaternion::from_euler(euler);
        *self = *self * euler_;
    }
    pub fn right(self) -> Vector3<T>
        where T: Number {
        self * Vector3::new(T::one(), T::zero(),T::zero())
    }
    pub fn up(self) -> Vector3<T>
        where T: Number {
        self * Vector3::new(T::zero(),T::one(), T::zero())
    }
    pub fn forward(self) -> Vector3<T>
        where T: Number {
        self * Vector3::new(T::zero(), T::zero(), T::one())
    }
}

impl<T: FloatingPoint> From<Quaternion<T>> for Matrix3<T> {
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
            x: Vector3::new( 
                 T::one() - yy2 - zz2, 
                 xy2 + sz2, 
                 xz2 - sy2, 
            ), 
            y: Vector3::new( 
                xy2 - sz2, 
                T::one() - xx2 - zz2, 
                yz2 + sx2, 
            ), 
            z: Vector3::new( 
                xz2 + sy2, 
                yz2 - sx2, 
                T::one() - xx2 - yy2, 
            ), 
        }
    }
}
// traits for bitwise operations

impl<T: FloatingPoint> std::ops::Mul<Vector3<T>> for Quaternion<T> {
    fn mul(self, rhs: Vector3<T>) -> Self::Output {
        let x2 = self.vector.x + self.vector.x;
        let y2 = self.vector.y + self.vector.y;
        let z2 = self.vector.z + self.vector.z;
        
        let xx2 = x2 * self.vector.x;
        let xy2 = x2 * self.vector.y;
        let xz2 = x2 * self.vector.z;

        let yy2 = y2 * self.vector.y;
        let yz2 = y2 * self.vector.z;
        let zz2 = z2 * self.vector.z;

        let sy2 = y2 * self.scalar;
        let sz2 = z2 * self.scalar;
        let sx2 = x2 * self.scalar;

        Vector3::new(
            (T::one() - (yy2 + zz2)) * rhs.x + (xy2 - sz2) * rhs.y + (xz2 + sy2) * rhs.z,
            (xy2 + sz2) * rhs.x + (T::one() - (xx2 + zz2)) * rhs.y + (yz2 - sx2) * rhs.z,
            (xz2 - sy2) * rhs.x + (yz2 + sx2) * rhs.y + (T::one() - (xx2 + yy2)) * rhs.z,
        )
    }
    type Output = Vector3<T>;
}
impl<T: FloatingPoint> std::ops::Mul for Quaternion<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            vector: Vector3::new(
                self.scalar * rhs.vector.x + self.vector.x * rhs.scalar + self.vector.y * rhs.vector.z - self.vector.z * rhs.vector.y, 
                self.scalar * rhs.vector.y + self.vector.y * rhs.scalar + self.vector.z * rhs.vector.x - self.vector.x * rhs.vector.z, 
                self.scalar * rhs.vector.z + self.vector.z * rhs.scalar + self.vector.x * rhs.vector.y - self.vector.y * rhs.vector.x
            ), 
            scalar: self.scalar * rhs.scalar - self.vector.x * rhs.vector.x - self.vector.y * rhs.vector.y - self.vector.z * rhs.vector.z
        }
    }
    type Output = Self;
}

impl<T: FloatingPoint> Rotation<T> for Quaternion<T> 
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T> {
    fn quaternion(&self) -> Quaternion<T> { *self }
    fn euler(&self) -> Vector3<T> { self.to_euler() }
}