use crate::linear::{FVec3, Vector};
mod polygon;
pub use polygon::*;
#[repr(C, align(16))]
#[derive(Default, Debug, Clone, Copy)]
pub struct Triangle3D {
    pub v: [FVec3; 3],
}

impl Triangle3D {
    pub fn new(v0: FVec3, v1: FVec3, v2: FVec3) -> Self {
        Self { v: [v0,v1,v2] }
    }
}
impl polygon::CalculateCentroid for Triangle3D {
    type VectorType = FVec3;
    fn centroid(&self) -> FVec3 {
        FVec3::new(
            (self.v[0].x + self.v[1].x + self.v[2].x)*0.33333,
            (self.v[0].y + self.v[1].y + self.v[2].y)*0.33333, 
            (self.v[0].z + self.v[1].z + self.v[2].z)*0.33333
        )
    }
}

#[repr(C, align(16))]
#[derive(Default, Clone, Copy)]
pub struct Cube {
    pub min: FVec3,
    pub max: FVec3,
}

impl Cube {
    pub fn new(min: FVec3, max: FVec3) -> Self {
        Self { min: min, max: max }
    }
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle3D) -> Self {
        let mut t = *self;
        t.min = t.min.min(&triangle.v[0]);
        t.min = t.min.min(&triangle.v[1]);
        t.min = t.min.min(&triangle.v[2]);
        t.max = t.max.max(&triangle.v[0]);
        t.max = t.max.max(&triangle.v[1]);
        t.max = t.max.max(&triangle.v[2]);
        t
    }
    pub fn aabb_adjust_bounds(&self, aabb: &Self) -> Self {
        let mut t = *self;
        t.min = t.min.min(&aabb.min);
        t.max = t.min.min(&aabb.max);
        t
    }
    pub fn vector_adjust_bounds(&self, v: FVec3) -> Self {
        let mut t = *self;
        t.min = t.min.min(&v);
        t.max = t.min.max(&v);
        t
    }
}

impl polygon::CalculateCentroid for Cube {
    type VectorType = FVec3;
    fn centroid(&self) -> FVec3 {
        FVec3::new(
            (self.min.x + self.max.x)*0.5, 
            (self.min.y + self.max.y)*0.5, 
            (self.min.z + self.max.z)*0.5
        )
    }
}

impl From<FVec3> for Cube {
    fn from(value: FVec3) -> Self {
        Self { min: value, max: value }
    }
}