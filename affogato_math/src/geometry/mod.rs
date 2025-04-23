mod rect;
mod triangle;
mod sphere;
mod segment;
mod ray;

pub use rect::*;
pub use triangle::*;
pub use sphere::*;
pub use segment::*;
pub use ray::*;

use crate::{algebra::Quaternion, matrix::{Matrix2, Matrix3, Matrix4}, vector::{Vector, Vector2, Vector3, Vector4}, Number, Real};

pub trait CalculateCentroid {
    type VectorType: Vector;
    /// The centroid is the center of a geometric object.
    fn centroid(&self) -> Self::VectorType;
}
pub type FLinearSegment2D = LinearSegment2D<f32>;
pub type DLinearSegment2D = LinearSegment2D<f64>;
pub type FQuadraticSegment2D = QuadraticSegment2D<f32>;
pub type DQuadraticSegment2D = QuadraticSegment2D<f64>;
pub type FCubicSegment2D = CubicSegment2D<f32>;
pub type DCubicSegment2D = CubicSegment2D<f64>;
pub type FSegment2D = Segment2D<f32>;
pub type DSegment2D = Segment2D<f64>;

pub trait Dimension {
    const DIMENSION: usize;
}

impl<T: Number> Dimension for Vector2<T> {
    const DIMENSION: usize = 2;
}
impl<T: Real> Dimension for Matrix2<T> {
    const DIMENSION: usize = 2;
}
impl<T: Number> Dimension for Circle<T> {
    const DIMENSION: usize = 2;
}
impl<T: Number> Dimension for Rect<T> {
    const DIMENSION: usize = 2;
}
impl<T: Number> Dimension for Triangle2D<T> {
    const DIMENSION: usize = 2;
}
impl<T: Number> Dimension for Vector3<T> {
    const DIMENSION: usize = 3;
}
impl<T: Real> Dimension for Matrix3<T> {
    const DIMENSION: usize = 3;
}
impl<T: Number> Dimension for Sphere<T> {
    const DIMENSION: usize = 3;
}
impl<T: Number> Dimension for Rect3D<T> {
    const DIMENSION: usize = 3;
}
impl<T: Number> Dimension for Triangle3D<T> {
    const DIMENSION: usize = 3;
}
impl<T: Number> Dimension for Vector4<T> {
    const DIMENSION: usize = 4;
}
impl<T: Real> Dimension for Quaternion<T> {
    const DIMENSION: usize = 4;
}
impl<T: Real> Dimension for Matrix4<T> {
    const DIMENSION: usize = 4;
}