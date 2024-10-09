mod quadrilateral;
mod triangle;
mod sphere;
mod segment;

pub use quadrilateral::*;
pub use triangle::*;
pub use sphere::*;
pub use segment::*;

use crate::linear::Vector;

pub trait CalculateCentroid {
    type VectorType: Vector;
    fn centroid(&self) -> Self::VectorType;
}