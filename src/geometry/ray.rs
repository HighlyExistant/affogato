use crate::{vector::{Vector, Vector3}, Real, Zero};

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