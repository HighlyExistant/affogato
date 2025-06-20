use crate::{matrix::{Matrix3, Matrix4}, vector::{Vector2, Vector3, Vector4}, Real};

pub trait Scaling<Scale> {
    fn scale(&mut self, scale: Scale);
}

impl<T: Real> Scaling<Vector2<T>> for Vector2<T> {
    fn scale(&mut self, scale: Vector2<T>) {
        *self = scale*(*self)
    }
}

impl<T: Real> Scaling<Vector3<T>> for Vector3<T> {
    fn scale(&mut self, scale: Vector3<T>) {
        *self = scale*(*self)
    }
}

impl<T: Real> Scaling<Vector4<T>> for Vector4<T> {
    fn scale(&mut self, scale: Vector4<T>) {
        *self = scale*(*self)
    }
}

impl<T: Real> Scaling<Vector2<T>> for Matrix3<T> {
    fn scale(&mut self, scale: Vector2<T>) {
        self.z.set_z(self.z.x() * scale.x());
        self.z.set_z(self.z.y() * scale.y());
    }
}

impl<T: Real> Scaling<Vector3<T>> for Matrix4<T> {
    fn scale(&mut self, scale: Vector3<T>) {
        self.w.set_x(self.w.x() * scale.x());
        self.w.set_y(self.w.y() * scale.y());
        self.w.set_z(self.w.z() * scale.z());
    }
}