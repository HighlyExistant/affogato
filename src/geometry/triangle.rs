use crate::{linear::{DVec2, DVec3, FVec2, FVec3, Vector2, Vector3}, Number};

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
impl CalculateCentroid for Triangle3D<f32> {
    type VectorType = FVec3;
    fn centroid(&self) -> FVec3 {
        FVec3::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*0.33333,
            (self.v[0].y + self.v[1].y + self.v[2].y)*0.33333, 
            (self.v[0].z + self.v[1].z + self.v[2].z)*0.33333
        )
    }
}
impl CalculateCentroid for Triangle3D<f64> {
    type VectorType = DVec3;
    fn centroid(&self) -> DVec3 {
        DVec3::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*0.33333,
            (self.v[0].y + self.v[1].y + self.v[2].y)*0.33333, 
            (self.v[0].z + self.v[1].z + self.v[2].z)*0.33333
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
impl CalculateCentroid for Triangle2D<f32> {
    type VectorType = FVec2;
    fn centroid(&self) -> FVec2 {
        FVec2::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*0.33333,
            (self.v[0].y + self.v[1].y + self.v[2].y)*0.33333, 
        )
    }
}
impl CalculateCentroid for Triangle2D<f64> {
    type VectorType = DVec2;
    fn centroid(&self) -> DVec2 {
        DVec2::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*0.33333,
            (self.v[0].y + self.v[1].y + self.v[2].y)*0.33333, 
        )
    }
}
