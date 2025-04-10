use crate::{vector::{CrossProduct, Vector, Vector3}, Real, Zero};

use super::{Sphere, Triangle3D};
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
#[derive(Clone)]
pub struct Ray3D<T: Real> {
    orig: Vector3<T>,
    dir: Vector3<T>,
}
impl<T: Real> Ray3D<T>  {
    pub fn new(orig: Vector3<T>, look_at: Vector3<T>) -> Self {
        Self { orig, dir: (look_at-orig).normalize() }
    }
    pub fn left() -> Self {
        Self { orig: Vector3::ZERO, dir: Vector3::left() }
    }
    pub fn right() -> Self {
        Self { orig: Vector3::ZERO, dir: Vector3::right() }
    }
    pub fn top() -> Self {
        Self { orig: Vector3::ZERO, dir: Vector3::top() }
    }
    pub fn bottom() -> Self {
        Self { orig: Vector3::ZERO, dir: Vector3::bottom() }
    }
    pub fn forward() -> Self {
        Self { orig: Vector3::ZERO, dir: Vector3::forward() }
    }
    pub fn backward() -> Self {
        Self { orig: Vector3::ZERO, dir: Vector3::backward() }
    }
    pub fn intersect_sphere(&self, sphere: &Sphere<T>) -> Option<RayHitInfo<Vector3<T>>> {
        let mut hit_info = None;
        let oc = sphere.center-self.orig;
        let a = self.dir.dot(&self.dir);
        let b = self.dir.dot(&oc)*T::from_f64(-2.0);
        let c = oc.dot(&oc) - sphere.radius*sphere.radius;
        let discriminant = b*b - T::from_f64(4.0)*a*c;
        if discriminant >= T::ZERO  {
            let dst = (-b - discriminant.sqrt()) / (T::from_f64(2.0)*a);

            if dst >= T::ZERO {
                
                let point = self.orig + self.dir*dst;
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
        let ao = self.orig-triangle[0];
        let dao = ao.cross(&self.dir);

        let determinant = -self.dir.dot(&normal);
        let inv_determinant = T::ONE/determinant;

        let dst = ao.dot(&normal)*inv_determinant;
        let u = edge_ac.dot(&dao)*inv_determinant;
        let v = edge_ab.dot(&dao)*inv_determinant;
        let w = T::ONE-u-v;

        if determinant >= T::from_f64(1e-6) && dst >= T::ZERO && u >= T::ZERO && v >= T::ZERO && w >= T::ZERO {
            Some(RayHitInfo { 
                distance: dst, 
                normal: normal, 
                point: self.orig+self.dir*dst, 
            })
        } else {
            None
        }
    }
}
impl<T: Real> Ray for Ray3D<T>  {
    type Vector = Vector3<T>;
    fn direction(&self) -> &Self::Vector {
        &self.dir
    }
    fn origin(&self) -> &Self::Vector {
        &self.orig
    }
    fn look(&mut self, at: Self::Vector) {
        self.dir = (at-self.orig).normalize();
    }
    fn set_origin(&mut self, origin: Self::Vector) {
        self.orig = origin;
    }
    fn at(&self, distance: <Self::Vector as Vector>::Scalar) -> Self::Vector {
        return self.orig + self.dir*distance;
    }
}