use affogato_math::{geometry::{CalculateCentroid, Rect, Rect3D}, vector::{Vector, Vector2, Vector3}, Real};

use super::{Collision, CollisionInfo};

impl<T: Real> Collision<Vector3<T>> for Rect3D<T> {
    type CollisionInfo = CollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Vector3<T>) -> Option<Self::CollisionInfo> {
        if self.intersect_point(object) {
            let centroid = self.centroid();
            let normal = centroid.direction_to(object);
            let distance = centroid.distance(object);
            Some(CollisionInfo { 
                distance, 
                normal, 
                point: object.clone() 
            })
        } else {
            None
        }
    }
}

impl<T: Real> Collision<Rect3D<T>> for Vector3<T> {
    type CollisionInfo = CollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Rect3D<T>) -> Option<Self::CollisionInfo> {
        if object.intersect_point(self) {
            let centroid = object.centroid();
            let normal = self.direction_to(&centroid);
            let distance = self.distance(&centroid);
            Some(CollisionInfo { 
                distance, 
                normal, 
                point: self.clone() 
            })
        } else {
            None
        }
    }
}

impl<T: Real> Collision<Rect3D<T>> for Rect3D<T> {
    type CollisionInfo = CollisionInfo<Vector3<T>>;
    fn collides(&self, object: &Rect3D<T>) -> Option<Self::CollisionInfo> {
        if self.intersect(object) {
            let obj_centroid = object.centroid();
            let centroid = self.centroid();
            let normal = centroid.direction_to(&obj_centroid);
            let distance = centroid.distance(&obj_centroid);
            Some(CollisionInfo { 
                distance, 
                normal, 
                point: normal*distance+centroid
            })
        } else {
            None
        }
    }
}