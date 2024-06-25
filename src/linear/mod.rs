#![allow(unused)]
mod vector;
mod matrix;
mod transform;
pub use vector::*;
pub use matrix::*;
pub use transform::*;

use num_traits::AsPrimitive;
use crate::{FloatingPoint, Number};

// Vector types
pub type FVec2 = Vector2<f32>;
pub type DVec2 = Vector2<f64>;

pub type I8Vec2 = Vector2<i8>;
pub type I16Vec2 = Vector2<i16>;
pub type IVec2 = Vector2<i32>;
pub type I64Vec2 = Vector2<i64>;

pub type UI8Vec2 = Vector2<u8>;
pub type UI16Vec2 = Vector2<u16>;
pub type UIVec2 = Vector2<u32>;
pub type UI64Vec2 = Vector2<u64>;
pub type USizeVec2 = Vector2<usize>;
pub type ISizeVec2 = Vector2<isize>;

pub type FVec3 = Vector3<f32>;
pub type DVec3 = Vector3<f64>;

pub type I8Vec3 = Vector3<i8>;
pub type I16Vec3 = Vector3<i16>;
pub type IVec3 = Vector3<i32>;
pub type I64Vec3 = Vector3<i64>;

pub type UI8Vec3 = Vector3<u8>;
pub type UI16Vec3 = Vector3<u16>;
pub type UIVec3 = Vector3<u32>;
pub type UI64Vec3 = Vector3<u64>;
pub type USizeVec3 = Vector3<usize>;
pub type ISizeVec3 = Vector3<isize>;

pub type FVec4 = Vector4<f32>;
pub type DVec4 = Vector4<f64>;

pub type I8Vec4 = Vector4<i8>;
pub type I16Vec4 = Vector4<i16>;
pub type IVec4 = Vector4<i32>;
pub type I64Vec4 = Vector4<i64>;

pub type UI8Vec4 = Vector4<u8>;
pub type UI16Vec4 = Vector4<u16>;
pub type UIVec4 = Vector4<u32>;
pub type UI64Vec4 = Vector4<u64>;
pub type USizeVec4 = Vector4<usize>;
pub type ISizeVec4 = Vector4<isize>;

// Matrix Types

// Matrix 2
pub type FMat2 = Matrix2<f32>;
pub type DMat2 = Matrix2<f64>;

pub type CMat2 = Matrix2<i8>;
pub type SMat2 = Matrix2<i16>;
pub type IMat2 = Matrix2<i32>;
pub type LMat2 = Matrix2<i64>;

pub type UCMat2 = Matrix2<u8>;
pub type USMat2 = Matrix2<u16>;
pub type UIMat2 = Matrix2<u32>;
pub type ULMat2 = Matrix2<u64>;
pub type USizeMat2 = Matrix2<usize>;
pub type ISizeMat2 = Matrix2<isize>;

pub type FMat2x3 = Matrix2x3<f32>;
pub type DMat2x3 = Matrix2x3<f64>;

pub type CMat2x3 = Matrix2x3<i8>;
pub type SMat2x3 = Matrix2x3<i16>;
pub type IMat2x3 = Matrix2x3<i32>;
pub type LMat2x3 = Matrix2x3<i64>;

pub type UCMat2x3 = Matrix2x3<u8>;
pub type USMat2x3 = Matrix2x3<u16>;
pub type UIMat2x3 = Matrix2x3<u32>;
pub type ULMat2x3 = Matrix2x3<u64>;
pub type USizeMat2x3 = Matrix2x3<usize>;
pub type ISizeMat2x3 = Matrix2x3<isize>;

pub type FMat2x4 = Matrix2x4<f32>;
pub type DMat2x4 = Matrix2x4<f64>;

pub type CMat2x4 = Matrix2x4<i8>;
pub type SMat2x4 = Matrix2x4<i16>;
pub type IMat2x4 = Matrix2x4<i32>;
pub type LMat2x4 = Matrix2x4<i64>;

pub type UCMat2x4 = Matrix2x4<u8>;
pub type USMat2x4 = Matrix2x4<u16>;
pub type UIMat2x4 = Matrix2x4<u32>;
pub type ULMat2x4 = Matrix2x4<u64>;
pub type USizeMat2x4 = Matrix2x4<usize>;
pub type ISizeMat2x4 = Matrix2x4<isize>;
// Matrix 3
pub type FMat3 = Matrix3<f32>;
pub type DMat3 = Matrix3<f64>;

pub type CMat3 = Matrix3<i8>;
pub type SMat3 = Matrix3<i16>;
pub type IMat3 = Matrix3<i32>;
pub type LMat3 = Matrix3<i64>;

pub type UCMat3 = Matrix3<u8>;
pub type USMat3 = Matrix3<u16>;
pub type UIMat3 = Matrix3<u32>;
pub type ULMat3 = Matrix3<u64>;
pub type USizeMat3 = Matrix3<usize>;
pub type ISizeMat3 = Matrix3<isize>;

pub type FMat3x2 = Matrix3x2<f32>;
pub type DMat3x2 = Matrix3x2<f64>;

pub type CMat3x2 = Matrix3x2<i8>;
pub type SMat3x2 = Matrix3x2<i16>;
pub type IMat3x2 = Matrix3x2<i32>;
pub type LMat3x2 = Matrix3x2<i64>;

pub type UCMat3x2 = Matrix3x2<u8>;
pub type USMat3x2 = Matrix3x2<u16>;
pub type UIMat3x2 = Matrix3x2<u32>;
pub type ULMat3x2 = Matrix3x2<u64>;
pub type USizeMat3x2 = Matrix3x2<usize>;
pub type ISizeMat3x2 = Matrix3x2<isize>;

pub type FMat3x4 = Matrix3x4<f32>;
pub type DMat3x4 = Matrix3x4<f64>;

pub type CMat3x4 = Matrix3x4<i8>;
pub type SMat3x4 = Matrix3x4<i16>;
pub type IMat3x4 = Matrix3x4<i32>;
pub type LMat3x4 = Matrix3x4<i64>;

pub type UCMat3x4 = Matrix3x4<u8>;
pub type USMat3x4 = Matrix3x4<u16>;
pub type UIMat3x4 = Matrix3x4<u32>;
pub type ULMat3x4 = Matrix3x4<u64>;
pub type USizeMat3x4 = Matrix3x4<usize>;
pub type ISizeMat3x4 = Matrix3x4<isize>;
// Matrix 4
pub type FMat4 = Matrix4<f32>;
pub type DMat4 = Matrix4<f64>;

pub type CMat4 = Matrix4<i8>;
pub type SMat4 = Matrix4<i16>;
pub type IMat4 = Matrix4<i32>;
pub type LMat4 = Matrix4<i64>;

pub type UCMat4 = Matrix4<u8>;
pub type USMat4 = Matrix4<u16>;
pub type UIMat4 = Matrix4<u32>;
pub type ULMat4 = Matrix4<u64>;
pub type USizeMat4 = Matrix4<usize>;
pub type ISizeMat4 = Matrix4<isize>;

pub type FMat4x2 = Matrix4x2<f32>;
pub type DMat4x2 = Matrix4x2<f64>;

pub type CMat4x2 = Matrix4x2<i8>;
pub type SMat4x2 = Matrix4x2<i16>;
pub type IMat4x2 = Matrix4x2<i32>;
pub type LMat4x2 = Matrix4x2<i64>;

pub type UCMat4x2 = Matrix4x2<u8>;
pub type USMat4x2 = Matrix4x2<u16>;
pub type UIMat4x2 = Matrix4x2<u32>;
pub type ULMat4x2 = Matrix4x2<u64>;
pub type USizeMat4x2 = Matrix4x2<usize>;
pub type ISizeMat4x2 = Matrix4x2<isize>;


pub fn slope<T: Number>(a: Vector2<T>, b: Vector2<T>) -> T {
    (b.y - a.y) / (b.x - a.x)
}
/// from [Christer Ericson's Real-Time Collision Detection](https://realtimecollisiondetection.net/)
pub fn barycentric_coordinates<T: Number>(start: Vector2<T>, end: Vector2<T>, control: Vector2<T>, p: Vector2<T>) -> Vector3<T> {
    let v0 = end - start;
    let v1 = control - start;
    let v2 = p - start;
    let d00 = v0.dot(&v0);
    let d01 = v0.dot(&v1);
    let d11 = v1.dot(&v1);
    let d20 = v2.dot(&v0);
    let d21 = v2.dot(&v1);
    let denom = d00 * d11 - d01 * d01;
    let v = (d11 * d20 - d01 * d21) / denom;
    let w = (d00 * d21 - d01 * d20) / denom;
    let u = T::one() - v - w;
    Vector3::new(w, v, u)
}

pub fn line_sdf<T: FloatingPoint>(a: Vector2<T>, b: Vector2<T>, p: Vector2<T>) -> T {
    let pa = p - a;
    let negba = -b + a;
    let ba = b - a;
    (pa.x * negba.y + pa.y * ba.x) / ((negba.y * negba.y) + (ba.x * ba.x)).sqrt()
}

pub fn line_pseudo_sdf<T: FloatingPoint>(a: Vector2<T>, b: Vector2<T>, p: Vector2<T>) -> T {
    let pa = p - a;
    let negba = -b + a;
    let ba = b - a;
    pa.x * negba.y + pa.y * ba.x
}

pub fn quadratic_bezier_curve_sdf<T: FloatingPoint>(start: Vector2<T>, end: Vector2<T>, control: Vector2<T>, barycentric_coordinates: Vector3<T>) -> T
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T>, {
    let control_to_start = start - control;
    let control_to_end = end - control;
    let cross_z = control_to_start.cross(&control_to_end);
    let uv_p = Vector2::<T>::new(0.5.as_() * barycentric_coordinates.x + barycentric_coordinates.z, barycentric_coordinates.z);
    if cross_z < T::zero() {
        uv_p.y - uv_p.x * uv_p.x
    } else {
        uv_p.x * uv_p.x - uv_p.y
    }
}
pub fn non_zero_sign<T: FloatingPoint>(n: T) -> T
    where f32: AsPrimitive<T> {
    2.0.as_()*((n > T::zero()) as i32 as f32).as_()-T::one()
}