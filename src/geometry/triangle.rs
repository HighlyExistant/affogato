use crate::{vector::{Vector2, Vector3}, Number, Real};

use super::CalculateCentroid;

#[repr(C, align(16))]
#[derive(Default, Debug, Clone, Copy)]
pub struct Triangle3D<T: Number> {
    pub v: [Vector3<T>; 3],
}

impl<T: Number> Triangle3D<T> {
    pub fn new(v0: Vector3<T>, v1: Vector3<T>, v2: Vector3<T>) -> Self {
        Self { v: [v0,v1,v2] }
    }
}
impl<T: Real> CalculateCentroid for Triangle3D<T> {
    type VectorType = Vector3<T>;
    fn centroid(&self) -> Vector3<T> {
        Vector3::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*T::from_f64(0.33333),
            (self.v[0].y + self.v[1].y + self.v[2].y)*T::from_f64(0.33333), 
            (self.v[0].z + self.v[1].z + self.v[2].z)*T::from_f64(0.33333)
        )
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Triangle2D<T: Number> {
    pub v: [Vector2<T>; 3],
}

impl<T: Number> Triangle2D<T> {
    pub fn new(v0: Vector2<T>, v1: Vector2<T>, v2: Vector2<T>) -> Self {
        Self { v: [v0,v1,v2] }
    }
}
impl<T: Real> CalculateCentroid for Triangle2D<T> {
    type VectorType = Vector2<T>;
    fn centroid(&self) -> Vector2<T> {
        Vector2::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*T::from_f64(0.33333),
            (self.v[0].y + self.v[1].y + self.v[2].y)*T::from_f64(0.33333), 
        )
    }
}