use affogato_core::sets::Real;

use crate::{matrix::{Matrix3, Matrix4}, vector::{Vector2, Vector3, Vector4}};
pub trait Translation<Pos> {
    fn translate(&mut self, translation: Pos);
}

impl<T: Real> Translation<Vector2<T>> for Vector2<T> {
    fn translate(&mut self, translation: Vector2<T>) {
        *self = translation+(*self)
    }
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
        self.z.set_x(self.z.x() + translation.x());
        self.z.set_y(self.z.y() + translation.y());
    }
}

impl<T: Real> Translation<Vector3<T>> for Matrix4<T> {
    fn translate(&mut self, translation: Vector3<T>) {
        self.w.set_w(self.w.x() + translation.x());
        self.w.set_w(self.w.y() + translation.y());
        self.w.set_w(self.w.z() + translation.z());
    }
}