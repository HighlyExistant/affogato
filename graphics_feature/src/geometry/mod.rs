#![cfg(feature="alloc")]
extern crate alloc;
use affogato_core::{num::Number, sets::Real};
use alloc::vec::Vec;
use affogato_math::{geometry::{Rect3D, CubicSegment2D, LinearSegment2D, QuadraticSegment2D, Rect, Segment2D, Triangle2D, Triangle3D}, vector::{Vector, Vector2, Vector3}};
use affogato_physics::kinematics::KinematicSegmentList;
pub enum VertexTopology {
    Point,
    Line,
    Triangle,
}
pub trait Geometry<V: Vector> {
    fn vertices(&self) -> Vec<V>;
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>>;
}

impl<T: Number> Geometry<Vector3<T>> for Rect3D<T> {
    fn vertices(&self) -> Vec<Vector3<T>> {
        self.get_vertices()
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(self.get_edge_indices()),
            VertexTopology::Point => Some(alloc::vec![0, 1, 2, 3, 4, 5, 6, 7]),
            VertexTopology::Triangle => Some(self.get_tri_indices()),
            _ => None,
        }
    }
}

impl<T: Number> Geometry<Vector2<T>> for Rect<T> {
    fn vertices(&self) -> Vec<Vector2<T>> {
        self.get_vertices()
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(self.get_edge_indices()),
            VertexTopology::Point => Some(alloc::vec![0, 1, 2, 3]),
            VertexTopology::Triangle => Some(self.get_tri_indices()),
            _ => None,
        }
    }
}

impl<T: Number> Geometry<Vector2<T>> for Triangle2D<T> {
    fn vertices(&self) -> Vec<Vector2<T>> {
        alloc::vec![self[0], self[1], self[2]]
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(alloc::vec![0, 1, 1, 2, 2, 3]),
            VertexTopology::Point => Some(alloc::vec![0, 1, 2]),
            VertexTopology::Triangle => Some(alloc::vec![0, 1, 2]),
            _ => None,
        }
    }
}

impl<T: Number> Geometry<Vector3<T>> for Triangle3D<T> {
    fn vertices(&self) -> Vec<Vector3<T>> {
        alloc::vec![self[0], self[1], self[2]]
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(alloc::vec![0, 1, 1, 2, 2, 3]),
            VertexTopology::Point => Some(alloc::vec![0, 1, 2]),
            VertexTopology::Triangle => Some(alloc::vec![0, 1, 2]),
            _ => None,
        }
    }
}
impl<T: Number> Geometry<Vector2<T>> for LinearSegment2D<T> {
    fn vertices(&self) -> Vec<Vector2<T>> {
        alloc::vec![self.start, self.end]
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(alloc::vec![0, 1]),
            VertexTopology::Point => Some(alloc::vec![0, 1]),
            _ => None,
        }
    }
}

impl<T: Number> Geometry<Vector2<T>> for QuadraticSegment2D<T> {
    fn vertices(&self) -> Vec<Vector2<T>> {
        alloc::vec![self.start, self.control, self.end]
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(alloc::vec![0, 1, 1, 2]),
            VertexTopology::Point => Some(alloc::vec![0, 1, 2]),
            _ => None,
        }
    }
}

impl<T: Number> Geometry<Vector2<T>> for CubicSegment2D<T> {
    fn vertices(&self) -> Vec<Vector2<T>> {
        alloc::vec![self.start, self.control1, self.control2, self.end]
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => Some(alloc::vec![0, 1, 1, 2, 2, 3]),
            VertexTopology::Point => Some(alloc::vec![0, 1, 2, 3]),
            _ => None,
        }
    }
}

impl<T: Number> Geometry<Vector2<T>> for Segment2D<T> {
    fn vertices(&self) -> Vec<Vector2<T>> {
        match self {
            Segment2D::Linear(linear) => linear.vertices(),
            Segment2D::Quadratic(quadratic) => quadratic.vertices(),
            Segment2D::Cubic(cubic) => cubic.vertices(),
        }
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match self {
            Segment2D::Linear(linear) => linear.indices(topology),
            Segment2D::Quadratic(quadratic) => quadratic.indices(topology),
            Segment2D::Cubic(cubic) => cubic.indices(topology),
        }
    }
}

impl<V: Vector> Geometry<V> for KinematicSegmentList<V> 
    where V::Scalar: Real {
    fn vertices(&self) -> Vec<V> {
        self.iter().cloned().collect::<Vec<_>>()
    }
    fn indices(&self, topology: VertexTopology) -> Option<Vec<u32>> {
        match topology {
            VertexTopology::Line => {
                let mut vector: Vec<u32> = Vec::with_capacity(self.len()*2);
                let mut prev = 0;
                for i in 1..self.len() as u32 {
                    vector.push(prev);
                    vector.push(i);
                    prev = i;
                }
                Some(vector)
            },
            VertexTopology::Point => Some((0..self.len() as u32).collect::<Vec<_>>()),
            _ => None,
        }
    }
}