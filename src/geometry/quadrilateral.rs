use std::fmt::Debug;

use crate::{linalg::{DVec3, FVec3, Vector, Vector2, Vector3}, Number, Real};

use super::{CalculateCentroid, Triangle2D, Triangle3D};

/// Represents an abstract N-Dimensional cube
pub trait HyperCube<T: Number> {
    const DIMENSION: usize;
}
#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Cube<T: Number> {
    min: Vector3<T>,
    max: Vector3<T>,
}
impl<T: Number> HyperCube<T> for Cube<T> {
    const DIMENSION: usize = 3;
}
impl<T: Number> std::cmp::PartialEq for Cube<T> {
    fn eq(&self, other: &Self) -> bool {
        self.min == other.min && self.max == other.max
    }
}
impl<T: Number> Default for Cube<T> {
    fn default() -> Self {
        Self { min: Vector3::from(T::MIN), max: Vector3::from(T::MAX) }
    }
}
impl<T: Number> Cube<T> {
    pub fn new(min: Vector3<T>, max: Vector3<T>) -> Self {
        Self { min: min, max: max }
    }
    /// Using a [`Triangle3D`], adjust the bounds of the [`Cube`] to fit at least the triangle.
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle3D<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&triangle[0]);
        t.min = t.min.min(&triangle[1]);
        t.min = t.min.min(&triangle[2]);
        t.max = t.max.max(&triangle[0]);
        t.max = t.max.max(&triangle[1]);
        t.max = t.max.max(&triangle[2]);
        t
    }
    /// merge 2 [`Cube`] objects together, so that both can fit within eachother.
    pub fn merge(&self, aabb: &Self) -> Self 
        where T: Debug {
        let mut t = *self;
        t.min = t.min.min(&aabb.min);
        t.max = t.max.max(&aabb.max);
        t
    }
    /// Using a [`Vector3`], adjust the bounds of the [`Cube`] to fit at least the vector.
    pub fn vector_adjust_bounds(&self, v: Vector3<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&v);
        t.max = t.min.max(&v);
        t
    }
    fn fix_bounds(&self) -> Self {
        let mut t = *self;
        t.min = self.min.min(&self.max);
        t.max = self.max.max(&self.min);
        t
    }
    pub fn minimum(&self) -> &Vector3<T> {
        &self.min
    }
    pub fn maximum(&self) -> &Vector3<T> {
        &self.max
    }
    /// Initializes a [`Cube`] to have the minimum be the largest value and the
    /// maximum to be the smallest value, Useful in scenarios when garunteeing 
    /// that the first call to adjust bounds will adjust it correctly.
    pub unsafe fn inverted_bounds_default() -> Self {
        Self { min: Vector3::from(T::MAX), max: Vector3::from(T::MIN) }
    }
    pub fn min(&self, other: &Self) -> Self {
        Self { min: self.min.min(&other.min), max: self.max.min(&other.max) }
    }
    pub fn max(&self, other: &Self) -> Self {
        Self { min: self.min.max(&other.min), max: self.max.max(&other.max) }
    }
    /// gets the vertices of the cube in the following order
    /// ```
    ///         7───────────────────5
    ///        ╱                   ╱│
    ///       ╱                   ╱ │
    ///      ╱                   ╱  │
    ///     3───────────────────1   │
    ///     │                   │   │
    ///     │   4               │   6
    ///     │                   │  ╱
    ///     │                   │ ╱
    ///     │                   │╱
    ///     0───────────────────2
    /// ```
    pub fn get_vertices(&self) -> Vec<Vector3<T>> {
        vec![
            Vector3::new(self.min.x, self.min.y, self.min.z),
            Vector3::new(self.max.x, self.max.y, self.min.z),
            Vector3::new(self.max.x, self.min.y, self.min.z),
            Vector3::new(self.min.x, self.max.y, self.min.z),
            Vector3::new(self.min.x, self.min.y, self.max.z),
            Vector3::new(self.max.x, self.max.y, self.max.z),
            Vector3::new(self.max.x, self.min.y, self.max.z),
            Vector3::new(self.min.x, self.max.y, self.max.z)
        ]
    }
    pub fn get_tri_indices(&self) -> Vec<u32> {
        vec![
            0, 2, 1,
            1, 3, 0,
            2, 6, 5,
            5, 1, 2,
            6, 4, 7,
            7, 5, 6,
            4, 0, 3,
            3, 7, 4,
            0, 2, 6,
            6, 4, 0,
            3, 1, 5,
            5, 7, 3
        ]
    }
    pub fn get_edge_indices(&self) -> Vec<u32> {
        vec![
            0, 2,
            2, 1,
            1, 3,
            3, 0,
            0, 4,
            4, 7,
            7, 3,
            7, 5,
            5, 6,
            6, 4,
            5, 1,
            6, 2,
        ]
    }
    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform {
            use rand::Rng;
            Self::new(Vector3::random(generator, range.clone()), Vector3::random(generator, range)).fix_bounds()
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
#[derive(Clone, Copy, Debug)]
pub struct Rect<T: Number> {
    min: Vector2<T>,
    max: Vector2<T>,
}
impl<T: Number> HyperCube<T> for Rect<T> {
    const DIMENSION: usize = 2;
}
impl<T: Number> Default for Rect<T> {
    fn default() -> Self {
        Self { min: Vector2::from(T::MIN), max: Vector2::from(T::MAX) }
    }
}
impl<T: Number> Rect<T> {
    pub fn edge_indices() -> [u32; 8] {
        [
            0, 1,
            1, 2,
            2, 3,
            3, 0
        ]
    }
    pub fn tri_indices() -> [u32; 6] {
        [
            0, 1, 2,
            2, 3, 0
        ]
    }
}
impl<T: Number> Rect<T> {
    pub fn new(min: Vector2<T>, max: Vector2<T>) -> Self {
        Self { min: min, max: max }
    }
    pub fn from_lengths(width: T, height: T) -> Self {
        Self::new(Vector2::new(T::ZERO, T::ZERO), Vector2::new(width, height))
    }
    pub fn width(&self) -> T {
        self.max.x - self.min.x
    }
    pub fn height(&self) -> T {
        self.max.y - self.min.y
    }
    pub fn triangle_adjust_bounds(&self, triangle: &Triangle2D<T>) -> Self {
        let mut t = *self;
        t.min = t.min.min(&triangle[0]);
        t.min = t.min.min(&triangle[1]);
        t.min = t.min.min(&triangle[2]);
        t.max = t.max.max(&triangle[0]);
        t.max = t.max.max(&triangle[1]);
        t.max = t.max.max(&triangle[2]);
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
    fn fix_bounds(&self) -> Self {
        let mut t = *self;
        t.min = self.min.min(&self.max);
        t.max = self.max.max(&self.min);
        t
    }
    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform {
            use rand::Rng;
            Self::new(Vector2::random(generator, range.clone()), Vector2::random(generator, range)).fix_bounds()
    }
    pub fn minimum(&self) -> &Vector2<T> {
        &self.min
    }
    pub fn maximum(&self) -> &Vector2<T> {
        &self.max
    }
    pub fn normalized(&self) -> Self 
        where T: Real {
        let rect = Vector2::new(self.width(), self.height()).normalize();
        Rect::from_lengths(rect.x, rect.y)
    }
    pub fn to_origin(&self) -> Self 
        where T: Real {
        Self::from_lengths(self.width(), self.height())
    }
    /// Initializes a [`Cube`] to have the minimum be the largest value and the
    /// maximum to be the smallest value, Useful in scenarios when garunteeing 
    /// that the first call to adjust bounds will adjust it correctly.
    pub unsafe fn inverted_bounds_default() -> Self {
        Self { min: Vector2::from(T::MAX), max: Vector2::from(T::MIN) }
    }
    pub fn min(&self, other: &Self) -> Self {
        Self { min: self.min.min(&other.min), max: self.max.min(&other.max) }
    }
    pub fn max(&self, other: &Self) -> Self {
        Self { min: self.min.max(&other.min), max: self.max.max(&other.max) }
    }
    /// ```
    /// 3─────────2
    /// │         │
    /// │         │
    /// │         │
    /// 0─────────1
    /// ```
    pub fn get_vertices(&self) -> Vec<Vector2<T>> {
        vec![
            Vector2::new(self.min.x, self.min.y),
            Vector2::new(self.max.x, self.min.y),
            Vector2::new(self.max.x, self.max.y),
            Vector2::new(self.min.x, self.max.y),
        ]
    }
    pub fn get_edge_indices(&self) -> Vec<u32> {
        vec![
            0, 1,
            1, 2,
            2, 3,
            3, 0
        ]
    }
    pub fn get_tri_indices(&self) -> Vec<u32> {
        vec![
            0, 1, 2,
            2, 3, 0
        ]
    }
    pub fn move_horizontal(&self, x: T) -> Self {
        Self { min: Vector2::new(self.min.x+x, self.min.y), max: Vector2::new(self.max.x+x, self.max.y) }
    }
    pub fn move_horizontal_vec2(&self, translate: Vector2<T>) -> Self {
        Self { min: Vector2::new(self.min.x+translate.x, self.min.y+translate.y), max: Vector2::new(self.max.x+translate.x, self.max.y+translate.y) }
    }
    pub fn scale(&self, scale: T) -> Self {
        Self { min: self.min*scale, max: self.max*scale }
    }
    pub fn scale_vec2(&self, scale: Vector2<T>) -> Self {
        Self { min: self.min*scale, max: self.max*scale }
    }
    pub fn invert(&self) -> Self {
        Self { min: self.max, max: self.min }
    }
}
impl<T: Number> From<Vector2<T>> for Rect<T> {
    fn from(value: Vector2<T>) -> Self {
        let rect = Rect::default();
        rect.vector_adjust_bounds(value)
    }
}
impl<T: Number> From<Triangle2D<T>> for Rect<T> {
    fn from(value: Triangle2D<T>) -> Self {
        let rect = Rect::default();
        rect.triangle_adjust_bounds(&value)
    }
}

impl<T: Real> CalculateCentroid for Rect<T> {
    type VectorType = Vector2<T>;
    fn centroid(&self) -> Vector2<T> {
        Vector2::new(
            (self.min.x + self.max.x)*T::from_f64(0.5), 
            (self.min.y + self.max.y)*T::from_f64(0.5), 
        )
    }
}

impl From<Triangle3D<f32>> for Cube<f32> {
    fn from(value: Triangle3D<f32>) -> Self {
        
        Self { min: value[0].min(&value[1].min(&value[2])), max: value[0].max(&value[1].max(&value[2])) }
    }
}