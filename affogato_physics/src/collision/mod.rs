use affogato_math::vector::Vector;
mod rect;
mod sphere;
pub use rect::*;
pub use sphere::*;
#[derive(Debug, Clone)]
pub struct CollisionInfo<V: Vector> {
    pub distance: V::Scalar,
    pub normal: V,
    pub point: V,
}

pub trait Collision<T> {
    type CollisionInfo;
    fn collides(&self, object: &T) -> Option<Self::CollisionInfo>;
}

