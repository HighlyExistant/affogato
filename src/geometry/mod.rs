mod quadrilateral;
mod triangle;

pub use quadrilateral::*;
pub use triangle::*;

use crate::linear::Vector;

pub trait CalculateCentroid {
    type VectorType: Vector;
    fn centroid(&self) -> Self::VectorType;
}