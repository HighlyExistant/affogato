mod quadrilateral;
mod triangle;
mod sphere;
mod segment;

pub use quadrilateral::*;
pub use triangle::*;
pub use sphere::*;
pub use segment::*;

use crate::vector::Vector;

pub trait CalculateCentroid {
    type VectorType: Vector;
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