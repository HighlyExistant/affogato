use affogato_core::{num::{Number, One, Zero}, sets::Real};

use crate::{algebra::Quaternion, geometry::{CalculateCentroid, CubicSegment2D, LinearSegment2D, QuadraticSegment2D}, matrix::{Matrix2, Matrix3, Matrix4}, vector::{FMat3, FVec2, Vector2, Vector3}};

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
impl<T: Real> Rotation<T> for Matrix3<T> {
    fn rotate(&mut self, rotate_by: &T) {
        *self = *self *Matrix3::from_transform(Vector2::ZERO, Vector2::ONE, *rotate_by);
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

impl<T: Real> Rotation<T> for LinearSegment2D<T> {
    fn rotate(&mut self, rotate_by: &T) {
        let centroid = self.centroid();
        let rotation = Matrix2::from_rotation(*rotate_by);
        self.start = (rotation*(self.start-centroid))+centroid;
        self.end = (rotation*(self.end-centroid))+centroid;
    }
}

impl<T: Real> Rotation<T> for QuadraticSegment2D<T> {
    fn rotate(&mut self, rotate_by: &T) {
        let centroid = self.centroid();
        let rotation = Matrix2::from_rotation(*rotate_by);
        self.start = (rotation*(self.start-centroid))+centroid;
        self.control = (rotation*(self.control-centroid))+centroid;
        self.end = (rotation*(self.end-centroid))+centroid;
    }
}

impl<T: Real> Rotation<T> for CubicSegment2D<T> {
    fn rotate(&mut self, rotate_by: &T) {
        let centroid = self.centroid();
        let rotation = Matrix2::from_rotation(*rotate_by);
        self.start = (rotation*(self.start-centroid))+centroid;
        self.control1 = (rotation*(self.control1-centroid))+centroid;
        self.control2 = (rotation*(self.control2-centroid))+centroid;
        self.end = (rotation*(self.end-centroid))+centroid;
    }
}