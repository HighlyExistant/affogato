#![allow(unused)]
use std::fmt::{Debug, Display};

use crate::{matrix::{Matrix3, Matrix4}, vector::{Vector3, Vector4}, Number, Real, Zero};

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Quaternion<T: Real> {
    pub scalar: T,
    pub i: T,
    pub j: T,
    pub k: T,
}
impl<T: Real> Default for Quaternion<T> {
    fn default() -> Self {
        Self::new(Vector3::ZERO, T::ONE)
    }
}
impl<T: Real + Display> Display for Quaternion<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let w = format!("{}", self.scalar);
        let i = if self.i.is_negative() {
            format!("- {}i", self.i.abs())
        } else {
            format!("+ {}i", self.i.abs())
        };
        let j = if self.j.is_negative() {
            format!("- {}j", self.j.abs())
        } else {
            format!("+ {}j", self.j.abs())
        };
        let k = if self.k.is_negative() {
            format!("- {}k", self.k.abs())
        } else {
            format!("+ {}k", self.k.abs())
        };
        f.write_str(format!("{w} {i} {j} {k}").as_str())
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
impl<T: Real> From<Vector4<T>> for Quaternion<T> {
    fn from(value: Vector4<T>) -> Self {
        Self { scalar: value.x, i: value.y, j: value.z, k: value.w }
    }
}
impl<T: Real> Quaternion<T>  {
    pub fn new(v: Vector3<T>, s: T) -> Self {
        Self { i: v.x, j: v.y, k: v.z, scalar: s }
    }
    /// # from_euler
    /// 
    /// converts euler angles into quaternion form.
    pub fn from_euler(v: Vector3<T>) -> Self 
        where T: Real, {
        // this function is heavily inspired by [this wikipedia article](https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles)
        let cy = (v.z*T::from_f64(0.5)).cos();
        let sy = (v.z*T::from_f64(0.5)).sin();
        let cr = (v.x*T::from_f64(0.5)).cos();
        let sr = (v.x*T::from_f64(0.5)).sin();
        let cp = (v.y*T::from_f64(0.5)).cos();
        let sp = (v.y*T::from_f64(0.5)).sin();
        Self { 
            scalar: cy * cr * cp + sy * sr * sp,
            i: cy * sr * cp - sy * cr * sp,
            j: cy * cr * sp + sy * sr * cp, 
            k: sy * cr * cp - cy * sr * sp, 
        }
        
    }
    pub fn conjugate(&self) -> Self {
        Self { i: -self.i, j: -self.j, k: -self.k, scalar: self.scalar }
    }
    pub fn angle_axis(angle: T, vector: Vector3<T>) -> Self 
        where T: Real, {
        let half_angle = angle * T::from_f64(0.5);
        let (s, c) = half_angle.sin_cos();
        Self { i: vector.x * s, j: vector.y * s, k: vector.z * s, scalar: c }
    }
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self { scalar: self.scalar/length, i: self.i/length, j: self.j/length, k: self.k/length }
    }
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> T {
        self.scalar*self.scalar + self.i*self.i + self.j*self.j + self.k*self.k
    }
    pub fn to_euler(&self) -> Vector3<T>
        where T: Real, {
        let sinr_cosp = T::from_f64(2.0) * (self.scalar * self.i + self.j * self.k);
        let cosr_cosp = T::from_f64(1.0) - T::from_f64(2.0) * (self.i * self.i + self.j * self.j);
        let x = T::atan2(sinr_cosp, cosr_cosp);
        let sinp = T::from_f64(2.0) * (self.scalar * self.j - self.k * self.i);
        let y = if (sinp.abs() >= T::ONE) {
            (T::PI/T::from_f64(2.0)).copysign(sinp)
        } else {
            sinp.asin()
        };
        let siny_cosp = T::from_f64(2.0) * (self.scalar * self.k + self.i * self.j);
        let cosy_cosp = T::ONE - T::from_f64(2.0) * (self.j * self.j + self.k * self.k);
        let z = T::atan2(siny_cosp, cosy_cosp);
        Vector3::new(x, y, z)
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
        let x2 = value.i + value.i;
        let y2 = value.j + value.j;
        let z2 = value.k + value.k;

        let xx2 = x2 * value.i;
        let xy2 = x2 * value.j;
        let xz2 = x2 * value.k;

        let yy2 = y2 * value.j;
        let yz2 = y2 * value.k;
        let zz2 = z2 * value.k;

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
        let x2 = self.i + self.i;
        let y2 = self.j + self.j;
        let z2 = self.k + self.k;
        
        let xx2 = x2 * self.i;
        let xy2 = x2 * self.j;
        let xz2 = x2 * self.k;

        let yy2 = y2 * self.j;
        let yz2 = y2 * self.k;
        let zz2 = z2 * self.k;

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
            scalar: self.scalar * rhs.scalar - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.scalar * rhs.i + self.i * rhs.scalar + self.j * rhs.k - self.k * rhs.j,
            j: self.scalar * rhs.j + self.j * rhs.scalar + self.k * rhs.i - self.i * rhs.k,
            k: self.scalar * rhs.k + self.k * rhs.scalar + self.i * rhs.j - self.j * rhs.i,
        }
    }
    type Output = Self;
}

impl<T: Real> From<Quaternion<T>> for Matrix4<T> {
    fn from(value: Quaternion<T>) -> Self {
        Matrix4::new(
            value.scalar, value.i, value.j, value.k, 
            -value.i, value.scalar, value.k, -value.j, 
            -value.j, -value.k, value.scalar, value.i, 
            -value.k, value.j, -value.i, value.scalar
        )
    }
}

impl<T: Real> std::ops::Mul<Vector4<T>> for Quaternion<T> {
    type Output = Vector4<T>;
    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Matrix4::from(self)*rhs
    }
}


mod test {

}