#![allow(unused)]
use core::fmt::{Debug, Display};

#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{matrix::{Matrix3, Matrix4}, vector::{Vector, Vector3, Vector4}, Number, Real, Zero};
/// Represents a number with 1 real scalar component `w` and 1 vector component 
/// that contains 3 imaginary values `i`, `j` and `k`. This quaternion assures that
/// the equation `i*i` = `j*j` = `k*k` = `i*j*k` = `-1`. It's commonly used to 
/// represent rotations in 3d space.
#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy, Debug)]
pub struct Quaternion<T: Real> {
    pub w: T,
    pub i: T,
    pub j: T,
    pub k: T,
}
impl<T: Real> Default for Quaternion<T> {
    fn default() -> Self {
        Self::from_scalar_vector(Vector3::ZERO, T::ONE)
    }
}
pub fn normalize_radians<T: Real>(a: T) -> T {
    let pi2: T = T::from_f64(2.0)*T::PI;
    a - pi2 * T::floor((a + T::PI)/pi2)
}
impl<T: Real> From<Vector4<T>> for Quaternion<T> {
    fn from(value: Vector4<T>) -> Self {
        Self { w: value.x(), i: value.y(), j: value.z(), k: value.w() }
    }
}
impl<T: Real> Quaternion<T>  {
    pub const fn new(w: T, i: T, j: T, k: T) -> Self {
        Self { w, i, j, k }
    }
    pub const fn from_scalar_vector(v: Vector3<T>, s: T) -> Self {
        Self::new(s, v.x(), v.y(), v.z())
    }
    pub const fn identity() -> Self {
        Self::from_scalar_vector(Vector3::ZERO, T::ONE)
    }
    /// # from_euler
    /// 
    /// converts euler angles into quaternion form.
    pub fn from_euler(v: Vector3<T>) -> Self 
        where T: Real, {
        // this function is heavily inspired by [this wikipedia article](https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles)
        let cy = (v.z()*T::from_f64(0.5)).cos();
        let sy = (v.z()*T::from_f64(0.5)).sin();
        let cr = (v.x()*T::from_f64(0.5)).cos();
        let sr = (v.x()*T::from_f64(0.5)).sin();
        let cp = (v.y()*T::from_f64(0.5)).cos();
        let sp = (v.y()*T::from_f64(0.5)).sin();
        Self::new(
            cy * cr * cp + sy * sr * sp,
            cy * sr * cp - sy * cr * sp,
            cy * cr * sp + sy * sr * cp, 
            sy * cr * cp - cy * sr * sp, 
        )
    }
    pub fn conjugate(self) -> Self {
        Self::new(
            self.w,
            -self.i,
            -self.j,
            -self.k,
        )
    }
    pub fn angle_axis(angle: T, vector: Vector3<T>) -> Self 
        where T: Real, {
        let half_angle = angle * T::from_f64(0.5);
        let (s, c) = half_angle.sin_cos();
        Self::new(
            c,
            vector.x() * s,
            vector.y() * s,
            vector.z() * s
        )
    }
    pub fn normalize(&self) -> Self {
        let length = self.length();
        Self { w: self.w/length, i: self.i/length, j: self.j/length, k: self.k/length }
    }
    pub fn dot(&self, other: &Self) -> T {
        Vector4::new(self.i, self.j, self.k, self.w).dot(&Vector4::new(other.i, other.j, other.k, other.w))
    }
    pub fn nlerp(self, mut other: Self, t: T) -> Self {
        if self.dot(&other) < T::ZERO {
            other = -other;
        }

        (self * (T::ONE - t) + other * t).normalize()
    }
    pub fn slerp(self, mut other: Self, t: T) -> Self {
        let mut dot = self.dot(&other);
        let dot_threshold = T::from_f64(0.9995f64);

        if dot < T::ZERO {
            other = -other;
            dot = -dot;
        }

        if dot > dot_threshold {
            self.nlerp(other, t)
        } else {
            let robust_dot = dot.min(T::ONE).max(-T::ONE);

            let theta = robust_dot.acos();

            let scale1 = (theta * (T::ONE - t)).sin();
            let scale2 = (theta * t).sin();

            (self * scale1 + other * scale2).normalize()
        }
    }
    pub fn length(&self) -> T {
        self.length_squared().sqrt()
    }
    pub fn length_squared(&self) -> T {
        self.w*self.w + self.i*self.i + self.j*self.j + self.k*self.k
    }
    pub fn to_euler(&self) -> Vector3<T>
        where T: Real, {
        let sinr_cosp = T::from_f64(2.0) * (self.w * self.i + self.j * self.k);
        let cosr_cosp = T::from_f64(1.0) - T::from_f64(2.0) * (self.i * self.i + self.j * self.j);
        let x = T::atan2(sinr_cosp, cosr_cosp);
        let sinp = T::from_f64(2.0) * (self.w * self.j - self.k * self.i);
        let y = if (sinp.abs() >= T::ONE) {
            (T::PI/T::from_f64(2.0)).copysign(sinp)
        } else {
            sinp.asin()
        };
        let siny_cosp = T::from_f64(2.0) * (self.w * self.k + self.i * self.j);
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

        let sy2 = y2 * value.w;
        let sz2 = z2 * value.w;
        let sx2 = x2 * value.w;

        Self::from_vec(
            Vector3::new( 
                 T::ONE - yy2 - zz2, 
                 xy2 + sz2, 
                 xz2 - sy2, 
            ), 
            Vector3::new( 
                xy2 - sz2, 
                T::ONE - xx2 - zz2, 
                yz2 + sx2, 
            ), 
            Vector3::new( 
                xz2 + sy2, 
                yz2 - sx2, 
                T::ONE - xx2 - yy2, 
            ), 
        )
    }
}
// traits for bitwise operations

impl<T: Real> core::ops::Mul<Vector3<T>> for Quaternion<T> {
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

        let sy2 = y2 * self.w;
        let sz2 = z2 * self.w;
        let sx2 = x2 * self.w;

        Vector3::new(
            (T::ONE - (yy2 + zz2)) * rhs.x() + (xy2 - sz2) * rhs.y() + (xz2 + sy2) * rhs.z(),
            (xy2 + sz2) * rhs.x() + (T::ONE - (xx2 + zz2)) * rhs.y() + (yz2 - sx2) * rhs.z(),
            (xz2 - sy2) * rhs.x() + (yz2 + sx2) * rhs.y() + (T::ONE - (xx2 + yy2)) * rhs.z(),
        )
    }
    type Output = Vector3<T>;
}
impl<T: Real> core::ops::Neg for Quaternion<T>  {
    fn neg(self) -> Self::Output {
        Self::new(
            -self.w,
            -self.i,
            -self.j,
            -self.k,
        )
    }
    type Output = Self;
}
impl<T: Real> core::ops::Mul<T> for Quaternion<T>  {
    fn mul(self, rhs: T) -> Self::Output {
        Self::new(
            self.w*rhs, 
            self.i*rhs, 
            self.j*rhs, 
            self.k*rhs
        )
    }
    type Output = Self;
}
impl<T: Real> core::ops::Add for Quaternion<T>  {
    fn add(self, rhs: Self) -> Self::Output {
        Self::new(
            self.w*rhs.w, 
            self.i*rhs.i, 
            self.j*rhs.j, 
            self.k*rhs.k
        )
    }
    type Output = Self;
}
impl<T: Real> core::ops::Mul for Quaternion<T>  {
    fn mul(self, rhs: Self) -> Self::Output {
        Self { 
            w: self.w * rhs.w - self.i * rhs.i - self.j * rhs.j - self.k * rhs.k,
            i: self.w * rhs.i + self.i * rhs.w + self.j * rhs.k - self.k * rhs.j,
            j: self.w * rhs.j + self.j * rhs.w + self.k * rhs.i - self.i * rhs.k,
            k: self.w * rhs.k + self.k * rhs.w + self.i * rhs.j - self.j * rhs.i,
        }
    }
    type Output = Self;
}

impl<T: Real> From<Quaternion<T>> for Matrix4<T> {
    fn from(value: Quaternion<T>) -> Self {
        Matrix4::new(
            value.w, value.i, value.j, value.k, 
            -value.i, value.w, value.k, -value.j, 
            -value.j, -value.k, value.w, value.i, 
            -value.k, value.j, -value.i, value.w
        )
    }
}

impl<T: Real> core::ops::Mul<Vector4<T>> for Quaternion<T> {
    type Output = Vector4<T>;
    fn mul(self, rhs: Vector4<T>) -> Self::Output {
        Matrix4::from(self)*rhs
    }
}

#[cfg(feature="alloc")]
mod alloc_feature {
    use core::fmt::Display;

    use crate::{algebra::Quaternion, Real};

    extern crate alloc;

    impl<T: Real + Display> Display for Quaternion<T> {
        fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            let w = alloc::format!("{}", self.w);
            let i = if self.i.is_negative() {
                alloc::format!("- {}i", self.i.abs())
            } else {
                alloc::format!("+ {}i", self.i.abs())
            };
            let j = if self.j.is_negative() {
                alloc::format!("- {}j", self.j.abs())
            } else {
                alloc::format!("+ {}j", self.j.abs())
            };
            let k = if self.k.is_negative() {
                alloc::format!("- {}k", self.k.abs())
            } else {
                alloc::format!("+ {}k", self.k.abs())
            };
            f.write_str(alloc::format!("{w} {i} {j} {k}").as_str())
        }
    }
}