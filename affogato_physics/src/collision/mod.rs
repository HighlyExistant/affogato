#![allow(unused)]
mod rect;
mod sphere;
pub mod gjk;
pub mod gjk2d;
use affogato_core::groups::vector_spaces::VectorSpace;
pub use gjk::*;
pub use rect::*;
pub use sphere::*;
/// Used for when we can analytically solve an exact point of contact, or the general mean point of contact
#[derive(Debug, Clone)]
pub struct HitCollisionInfo<V: VectorSpace> {
    pub distance: V::Scalar,
    pub normal: V,
    pub point: V,
}
#[derive(Debug, Clone, Default)]
pub struct CollisionInfo<V: VectorSpace> {
    pub distance: V::Scalar,
    pub normal: V,
}

pub trait Collision<T> {
    type CollisionInfo;
    fn collides(&self, object: &T) -> Option<Self::CollisionInfo>;
}

