use crate::{algebra::Quaternion, matrix::{Matrix2, Matrix3, Matrix4}, vector::{FMat3, Vector2, Vector3}, Real};

pub trait Rotation<Rot> {
    fn rotate(&mut self, rotate_by: &Rot);
}

impl<T: Real> Rotation<T> for Vector2<T> {
    fn rotate(&mut self, rotate_by: &T) {
        let (sina, cosa) = rotate_by.sin_cos();
        *self = Matrix2::new(cosa, sina, -sina, cosa)**self;
    }
}

impl<T: Real> Rotation<Quaternion<T>> for Vector3<T> {
    fn rotate(&mut self, rotate_by: &Quaternion<T>) {
        *self = *rotate_by*(*self)
    }
}

impl<T: Real> Rotation<Quaternion<T>> for Quaternion<T> {
    fn rotate(&mut self, rotate_by: &Quaternion<T>) {
        *self = *rotate_by**self*rotate_by.conjugate()
    }
}

impl<T: Real> Rotation<T> for Matrix2<T> {
    fn rotate(&mut self, rotate_by: &T) {
        let (sina, cosa) = rotate_by.sin_cos();
        *self = *self*Matrix2::new(cosa, sina, -sina, cosa)
    }
}
impl<T: Real> Rotation<Quaternion<T>> for Matrix3<T> {
    fn rotate(&mut self, rotate_by: &Quaternion<T>) {
        *self = *self * Matrix3::from(*rotate_by)
    }
}
impl<T: Real> Rotation<Quaternion<T>> for Matrix4<T> {
    fn rotate(&mut self, rotate_by: &Quaternion<T>) {
        *self = *self * Matrix4::from(Matrix3::from(*rotate_by))
    }
}