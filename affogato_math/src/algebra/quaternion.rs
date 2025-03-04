#![allow(unused)]
use std::fmt::{Debug, Display};

use crate::{matrix::Matrix3, vector::Vector3, Number, Real};

#[repr(C)]
#[derive(Clone, Copy, Debug, Default)]
pub struct Quaternion<T: Real> {
    pub vector: Vector3<T>,
    pub scalar: T,
}
impl<T: Real + Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i = format!("{}i", self.vector.x.abs());
        let j = if self.vector.y.is_negative() {
            format!("- {}j", self.vector.y.abs())
        } else {
            format!("+ {}j", self.vector.y.abs())
        };
        let k = if self.vector.z.is_negative() {
            format!("- {}k", self.vector.z.abs())
        } else {
            format!("+ {}k", self.vector.z.abs())
        };
        let w = if self.scalar.is_negative() {
            format!("- {}", self.scalar.abs())
        } else {
            format!("+ {}", self.scalar.abs())
        };
        f.write_str(format!("{i} {j} {k} {w}").as_str())
    }
}
/// ===========================================================
/// 
/// Implementation for Quaternion
/// 
/// ===========================================================

///
/// These functions are available for all vectors.
/// 
pub fn normalize_radians<T: Real>(a: T) -> T {
    let pi2: T = T::from_f64(2.0)*T::PI;
    a - pi2 * T::floor((a + T::PI)/pi2)
}
impl<T: Real> Quaternion<T>  {
    pub fn new(v: Vector3<T>, s: T) -> Self {
        Self { vector: v, scalar: s }
    }
    /// # from_euler
    /// 
    /// converts euler angles into quaternion form.
    /// 
    /// this function is heavily inspired by [this stackexchange post](https://math.stackexchange.com/questions/2975109/how-to-convert-euler-angles-to-quaternions-and-get-the-same-euler-angles-back-fr)
    pub fn from_euler(v: Vector3<T>) -> Self 
        where T: Real, {
        let (yaw, pitch, roll) = (v.x, v.y, v.z);
        let x = T::sin(roll*T::from_f64(0.5)) * T::cos(pitch*T::from_f64(0.5)) * T::cos(yaw*T::from_f64(0.5)) - T::cos(roll*T::from_f64(0.5)) * T::sin(pitch*T::from_f64(0.5)) * T::sin(yaw*T::from_f64(0.5));
        let y = T::cos(roll*T::from_f64(0.5)) * T::sin(pitch*T::from_f64(0.5)) * T::cos(yaw*T::from_f64(0.5)) + T::sin(roll*T::from_f64(0.5)) * T::cos(pitch*T::from_f64(0.5)) * T::sin(yaw*T::from_f64(0.5));
        let z = T::cos(roll*T::from_f64(0.5)) * T::cos(pitch*T::from_f64(0.5)) * T::sin(yaw*T::from_f64(0.5)) - T::sin(roll*T::from_f64(0.5)) * T::sin(pitch*T::from_f64(0.5)) * T::cos(yaw*T::from_f64(0.5));
        let w = T::cos(roll*T::from_f64(0.5)) * T::cos(pitch*T::from_f64(0.5)) * T::cos(yaw*T::from_f64(0.5)) + T::sin(roll*T::from_f64(0.5)) * T::sin(pitch*T::from_f64(0.5)) * T::sin(yaw*T::from_f64(0.5));
        Self { vector: Vector3::new(x, y, z), scalar: w }
        
    }
    pub fn angle_axis(angle: T, vector: Vector3<T>) -> Self 
        where T: Real, {
        let half_angle = angle * T::from_f64(0.5);
        let s = half_angle.sin();
        Self { vector: vector * s, scalar: half_angle.cos() }
    }
    pub fn to_euler(&self) -> Vector3<T>
        where T: Real, {
        let (x, y, z, w) = (self.vector.x, self.vector.y, self.vector.z, self.scalar);
        let t0 = T::from_f64(2.0) * (w * x + y * z);
        let t1 = T::from_f64(1.0) - T::from_f64(2.0) * (x * x + y * y);
        let roll = T::atan2(t0, t1);
        let t2 = T::from_f64(2.0) * (w * y - z * x);
        let t2 = if t2 > T::from_f64(1.0) {T::from_f64(1.0)} else {t2};
        let t2 = if t2 < (T::from_f64(-1.0)) {(T::from_f64(-1.0))} else {t2};
        let pitch = T::asin(t2);
        let t3 = T::from_f64(2.0) * (w * z + x * y);
        let t4 = T::from_f64(1.0) - T::from_f64(2.0) * (y * y + z * z);
        let yaw = T::atan2(t3, t4);
        return Vector3::new(normalize_radians(yaw), normalize_radians(pitch), normalize_radians(roll));
    }
    pub fn euler_rotate(&mut self, euler: Vector3<T>)
        where T: Real, {
        let euler_ = Quaternion::from_euler(euler);
        *self = *self * euler_;
    }
    pub fn right(self) -> Vector3<T>
        where T: Number {
        self * Vector3::new(T::ONE, T::ZERO,T::ZERO)
    }
    pub fn up(self) -> Vector3<T>
        where T: Number {
        self * Vector3::new(T::ZERO,T::ONE, T::ZERO)
    }
    pub fn forward(self) -> Vector3<T>
        where T: Number {
        self * Vector3::new(T::ZERO, T::ZERO, T::ONE)
    }
}

impl<T: Real> From<Quaternion<T>> for Matrix3<T> {
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
                 T::ONE - yy2 - zz2, 
                 xy2 + sz2, 
                 xz2 - sy2, 
            ), 
            y: Vector3::new( 
                xy2 - sz2, 
                T::ONE - xx2 - zz2, 
                yz2 + sx2, 
            ), 
            z: Vector3::new( 
                xz2 + sy2, 
                yz2 - sx2, 
                T::ONE - xx2 - yy2, 
            ), 
        }
    }
}
// traits for bitwise operations

impl<T: Real> std::ops::Mul<Vector3<T>> for Quaternion<T> {
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
            (T::ONE - (yy2 + zz2)) * rhs.x + (xy2 - sz2) * rhs.y + (xz2 + sy2) * rhs.z,
            (xy2 + sz2) * rhs.x + (T::ONE - (xx2 + zz2)) * rhs.y + (yz2 - sx2) * rhs.z,
            (xz2 - sy2) * rhs.x + (yz2 + sx2) * rhs.y + (T::ONE - (xx2 + yy2)) * rhs.z,
        )
    }
    type Output = Vector3<T>;
}
impl<T: Real> std::ops::Mul for Quaternion<T>  {
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