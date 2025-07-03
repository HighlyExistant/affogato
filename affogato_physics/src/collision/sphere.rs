use affogato_core::{num::Zero, sets::Real};
use affogato_math::{geometry::{Circle, HyperSphere, Sphere}, vector::{Vector, Vector2, Vector3}};

use super::{Collision, HitCollisionInfo};

impl<T: Real> Collision<Vector3<T>> for Sphere<T> {
    type CollisionInfo = HitCollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Vector3<T>) -> Option<Self::CollisionInfo> {
        let distance = object.distance(&self.center);
        if distance < self.radius {
            let normal = self.center.direction_to(object);
            Some(HitCollisionInfo { 
                distance, 
                normal, 
                point: normal*distance+self.center
            })
        } else {
            None
        }
    }
}
fn hsphere_collision<V: Vector + PartialEq, S: HyperSphere<V>>(s0: &S, s1: &S) -> Option<HitCollisionInfo<V>> 
    where V::Scalar: Real {
    // Case 1: Both spheres have the same center
    if s0.center() == s1.center() {
        return Some(HitCollisionInfo { 
            distance: V::Scalar::ZERO, 
            normal: <V as Zero>::ZERO, 
            point: s0.center()
        })
    }
    // Case 2: Both spheres have different center
    let distance = s1.center().distance(&s0.center());
    if distance < s0.radius() + s1.radius() {
        let normal = s0.center().direction_to(&s1.center());
        Some(HitCollisionInfo { 
            distance: distance-s1.radius(), 
            normal: normal.clone(), 
            point: normal*(distance-s1.radius())+s0.center()
        })
    } else {
        None
    }
}
impl<T: Real> Collision<Sphere<T>> for Sphere<T> {
    type CollisionInfo = HitCollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Sphere<T>) -> Option<Self::CollisionInfo> {
        hsphere_collision(self, object)
    }
}
impl<T: Real> Collision<Circle<T>> for Circle<T> {
    type CollisionInfo = HitCollisionInfo<Vector2<T>>;
    fn collides(&self, object: &Circle<T>) -> Option<Self::CollisionInfo> {
        hsphere_collision(self, object)
    }
}