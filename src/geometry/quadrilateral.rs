use std::fmt::Debug;

use crate::{linear::{DVec2, DVec3, FVec2, FVec3, Vector, Vector2, Vector3}, Number};

use super::{CalculateCentroid, Triangle2D, Triangle3D};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Cube<T: Number> {
    pub min: Vector3<T>,
    pub max: Vector3<T>,
}
impl<T: Number> Default for Cube<T> {
    fn default() -> Self {
        Self { min: Vector3::from(T::min_value()), max: Vector3::from(T::max_value()) }
    }
}
impl<T: Number> Cube<T> {
    pub fn new(min: Vector3<T>, max: Vector3<T>) -> Self {
        Self { min: min, max: max }
    }
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle3D<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&triangle.v[0]);
        t.min = t.min.min(&triangle.v[1]);
        t.min = t.min.min(&triangle.v[2]);
        t.max = t.max.max(&triangle.v[0]);
        t.max = t.max.max(&triangle.v[1]);
        t.max = t.max.max(&triangle.v[2]);
        t
    }
    pub fn aabb_adjust_bounds(&self, aabb: &Self) -> Self 
        where T: Debug {
        let mut t = *self;
        // println!("aabb.min {:?} t.min {:?} min = {:?}", aabb.min, t.min, t.min.min(&aabb.min));
        t.min = t.min.min(&aabb.min);
        t.max = t.max.max(&aabb.max);
        t
    }
    pub fn vector_adjust_bounds(&self, v: Vector3<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&v);
        t.max = t.min.max(&v);
        t
    }
    pub fn fix_bounds(&self) -> Self {
        let mut t = *self;
        t.min = self.min.min(&self.max);
        t.max = self.max.max(&self.min);
        t
    } 
    pub fn inverted_bounds_default() -> Self {
        Self { min: Vector3::from(T::max_value()), max: Vector3::from(T::min_value()) }
    }
}

impl CalculateCentroid for Cube<f32> {
    type VectorType = FVec3;
    fn centroid(&self) -> FVec3 {
        FVec3::new(
            (self.min.x + self.max.x)*0.5, 
            (self.min.y + self.max.y)*0.5, 
            (self.min.z + self.max.z)*0.5
        )
    }
}
impl CalculateCentroid for Cube<f64> {
    type VectorType = DVec3;
    fn centroid(&self) -> DVec3 {
        DVec3::new(
            (self.min.x + self.max.x)*0.5, 
            (self.min.y + self.max.y)*0.5, 
            (self.min.z + self.max.z)*0.5
        )
    }
}

impl From<FVec3> for Cube<f32> {
    fn from(value: FVec3) -> Self {
        Self { min: value, max: value }
    }
}
impl From<DVec3> for Cube<f64> {
    fn from(value: DVec3) -> Self {
        Self { min: value, max: value }
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy)]
pub struct Square<T: Number> {
    pub min: Vector2<T>,
    pub max: Vector2<T>,
}
impl<T: Number> Default for Square<T> {
    fn default() -> Self {
        Self { min: Vector2::from(T::min_value()), max: Vector2::from(T::max_value()) }
    }
}
impl<T: Number> Square<T> {
    pub fn new(min: Vector2<T>, max: Vector2<T>) -> Self {
        Self { min: min, max: max }
    }
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle2D<T>) -> Self {
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
    pub fn vector_adjust_bounds(&self, v: Vector2<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&v);
        t.max = t.min.max(&v);
        t
    }
}
impl<T: Number> From<Vector2<T>> for Square<T> {
    fn from(value: Vector2<T>) -> Self {
        let square = Square::default();
        square.vector_adjust_bounds(value)
    }
}
impl<T: Number> From<Triangle2D<T>> for Square<T> {
    fn from(value: Triangle2D<T>) -> Self {
        let square = Square::default();
        square.triangle_adjust_bounds(&value)
    }
}

impl CalculateCentroid for Square<f32> {
    type VectorType = FVec2;
    fn centroid(&self) -> FVec2 {
        FVec2::new(
            (self.min.x + self.max.x)*0.5, 
            (self.min.y + self.max.y)*0.5, 
        )
    }
}
impl CalculateCentroid for Square<f64> {
    type VectorType = DVec2;
    fn centroid(&self) -> DVec2 {
        DVec2::new(
            (self.min.x + self.max.x)*0.5, 
            (self.min.y + self.max.y)*0.5, 
        )
    }
}

impl From<Triangle3D<f32>> for Cube<f32> {
    fn from(value: Triangle3D<f32>) -> Self {
        Self { min: value.v[0].min(&value.v[1].min(&value.v[2])), max: value.v[0].max(&value.v[1].max(&value.v[2])) }
    }
}