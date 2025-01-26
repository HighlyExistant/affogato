use crate::matrix::{Matrix2, Matrix3, Matrix4};

use super::{Vector2, Vector3, Vector4};


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