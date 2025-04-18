use std::fmt::Display;

use crate::{algebra::Quaternion, matrix::{Matrix2, Matrix3, Matrix4}, vector::{Vector2, Vector3, Vector4}, Real};
mod rotate;
pub use rotate::*;
pub trait Translation<Pos> {
    fn translate(&mut self, translation: Pos);
}

impl<T: Real> Translation<Vector3<T>> for Vector3<T> {
    fn translate(&mut self, translation: Vector3<T>) {
        *self = translation+(*self)
    }
}

impl<T: Real> Translation<Vector4<T>> for Vector4<T> {
    fn translate(&mut self, translation: Vector4<T>) {
        *self = translation+(*self)
    }
}

impl<T: Real> Translation<Vector2<T>> for Matrix3<T> {
    fn translate(&mut self, translation: Vector2<T>) {
        self.z.x = translation.x;
        self.z.y = translation.y;
    }
}

impl<T: Real> Translation<Vector3<T>> for Matrix4<T> {
    fn translate(&mut self, translation: Vector3<T>) {
        self.w.x = translation.x;
        self.w.y = translation.y;
        self.w.z = translation.z;
    }
}