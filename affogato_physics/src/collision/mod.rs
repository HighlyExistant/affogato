#![allow(unused)]
use affogato_math::vector::Vector;
mod rect;
mod sphere;
mod gjk;
pub use gjk::*;
pub use rect::*;
pub use sphere::*;
/// Used for when we can analytically solve an exact point of contact, or the general mean point of contact
#[derive(Debug, Clone)]
pub struct HitCollisionInfo<V: Vector> {
    pub distance: V::Scalar,
    pub normal: V,
    pub point: V,
}
#[derive(Debug, Clone)]
pub struct CollisionInfo<V: Vector> {
    pub distance: V::Scalar,
    pub normal: V,
}

pub trait Collision<T> {
    type CollisionInfo;
    fn collides(&self, object: &T) -> Option<Self::CollisionInfo>;
}

