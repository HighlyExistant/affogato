use core::fmt::Debug;

use affogato_core::{num::Zero, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::{vector::{OuterProduct, Vector, Vector3}};

use super::{Sphere, Triangle3D};

#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug)]
pub struct RayHitInfo<V: Vector> {
    pub distance: V::Scalar,
    pub normal: V,
    pub point: V,
}

pub trait Ray {
    type Vector: Vector;
    fn set_origin(&mut self, at: Self::Vector);
    fn look(&mut self, at: Self::Vector);
    fn at(&self, distance: <Self::Vector as Vector>::Scalar) -> Self::Vector;
    fn origin(&self) -> &Self::Vector;
    fn direction(&self) -> &Self::Vector;
}
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Clone, Copy)]
pub struct Ray3D<T: Real> {
    origin: Vector3<T>,
    direction: Vector3<T>,
}
impl<T: Real> Ray3D<T>  {
    pub fn new(orig: Vector3<T>, look_at: Vector3<T>) -> Self {
        Self { origin: orig, direction: (look_at-orig).normalize() }
    }
    /// Returns a ray pointing to the left of the graph <-1, 0, 0>
    pub fn left() -> Self {
        Self { origin: Vector3::ZERO, direction: Vector3::left() }
    }
    /// Returns a ray pointing to the right of the graph <1, 0, 0>
    pub fn right() -> Self {
        Self { origin: Vector3::ZERO, direction: Vector3::right() }
    }
    /// Returns a ray pointing to the top of the graph <0, 1, 0>
    pub fn top() -> Self {
        Self { origin: Vector3::ZERO, direction: Vector3::top() }
    }
    /// Returns a ray pointing to the top of the graph <0, -1, 0>
    pub fn bottom() -> Self {
        Self { origin: Vector3::ZERO, direction: Vector3::bottom() }
    }
    /// Returns a ray pointing forward to the graph <0, 0, 1>
    pub fn forward() -> Self {
        Self { origin: Vector3::ZERO, direction: Vector3::forward() }
    }
    /// Returns a ray pointing backward to the graph <0, 0, -1>
    pub fn backward() -> Self {
        Self { origin: Vector3::ZERO, direction: Vector3::backward() }
    }
    pub fn intersect_sphere(&self, sphere: &Sphere<T>) -> Option<RayHitInfo<Vector3<T>>> {
        let mut hit_info = None;
        let oc = sphere.center-self.origin;
        let a = self.direction.dot(&self.direction);
        let b = self.direction.dot(&oc)*T::from_f64(-2.0);
        let c = oc.dot(&oc) - sphere.radius*sphere.radius;
        let discriminant = b*b - T::from_f64(4.0)*a*c;
        if discriminant >= T::ZERO  {
            let dst = (-b - discriminant.sqrt()) / (T::from_f64(2.0)*a);

            if dst >= T::ZERO {
                
                let point = self.origin + self.direction*dst;
                let normal = (point-sphere.center).normalize();
                hit_info = Some(RayHitInfo { distance: dst, normal, point });
            }
        }
        hit_info
    }
    pub fn intersect_triangle(&self, triangle: &Triangle3D<T>) -> Option<RayHitInfo<Vector3<T>>> {
        let edge_ab = triangle[1]-triangle[0];
        let edge_ac = triangle[2]-triangle[0];
        let normal = edge_ab.cross(&edge_ac);
        let ao = self.origin-triangle[0];
        let dao = ao.cross(&self.direction);

        let determinant = -self.direction.dot(&normal);
        let inv_determinant = T::ONE/determinant;

        let dst = ao.dot(&normal)*inv_determinant;
        let u = edge_ac.dot(&dao)*inv_determinant;
        let v = edge_ab.dot(&dao)*inv_determinant;
        let w = T::ONE-u-v;

        if determinant >= T::from_f64(1e-6) && dst >= T::ZERO && u >= T::ZERO && v >= T::ZERO && w >= T::ZERO {
            Some(RayHitInfo { 
                distance: dst, 
                normal: normal, 
                point: self.origin+self.direction*dst, 
            })
        } else {
            None
        }
    }
    // This is still in production
    // pub fn intersect_cube(&self, rect: &Rect3D<T>) -> bool 
    //     where T: Debug {
    //     let mut tmin = (rect.min.x - self.origin.x) / self.direction.x; 
    //     let mut tmax = (rect.max.x - self.origin.x) / self.direction.x; 

    //     if tmin > tmax {
    //         core::mem::swap(&mut tmin, &mut tmax);
    //     } 

    //     let mut tymin = (rect.min.y - self.origin.y) / self.direction.y; 
    //     let mut tymax = (rect.max.y - self.origin.y) / self.direction.y; 

    //     if tymin > tymax {
    //         core::mem::swap(&mut tymin, &mut tymax);
    //     } 

    //     if tmin > tymax || tymin > tmax {
    //         return false; 
    //     }

    //     if tymin > tmin {
    //         tmin = tymin
    //     } 
    //     if tymax < tmax {
    //         tmax = tymax
    //     } 

    //     let mut tzmin = (rect.min.z - self.origin.z) / self.direction.z; 
    //     let mut tzmax = (rect.max.z - self.origin.z) / self.direction.z; 

    //     if tzmin > tzmax {
    //         core::mem::swap(&mut tzmin, &mut tzmax);
    //     } 

    //     if tmin > tzmax || tzmin > tmax {
    //         return false;
    //     }

    //     if tzmin > tmin {
    //         tmin = tzmin
    //     }
    //     if tzmax < tmax {
    //         tmax = tzmax
    //     } 
    //     true
    // }
}

impl<T: Real> Ray for Ray3D<T>  {
    type Vector = Vector3<T>;
    fn direction(&self) -> &Self::Vector {
        &self.direction
    }
    fn origin(&self) -> &Self::Vector {
        &self.origin
    }
    fn look(&mut self, at: Self::Vector) {
        self.direction = (at-self.origin).normalize();
    }
    fn set_origin(&mut self, origin: Self::Vector) {
        self.origin = origin;
    }
    fn at(&self, distance: <Self::Vector as Vector>::Scalar) -> Self::Vector {
        return self.origin + self.direction*distance;
    }
}