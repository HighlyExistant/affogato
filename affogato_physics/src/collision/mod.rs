pub trait Collision<T> {
    type CollisionInfo;
    fn collides(&self, object: &T) -> Option<Self::CollisionInfo>;
}