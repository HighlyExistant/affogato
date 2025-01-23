use crate::{vector::{Vector, Vector3}, Real};

pub trait Ray {
    type Vector: Vector;
    fn set_origin(&mut self, at: Self::Vector);
    fn look(&mut self, at: Self::Vector);
    fn at(&mut self, distance: Self::Vector) -> Self::Vector;
    fn origin(&self) -> &Self::Vector;
    fn direction(&self) -> &Self::Vector;
}

pub struct Ray3D<T: Real> {
    orig: Vector3<T>,
    dir: Vector3<T>,
}
impl<T: Real> Ray3D<T>  {
    pub fn new(orig: Vector3<T>, look_at: Vector3<T>) -> Self {
        Self { orig, dir: orig-look_at }
    }
    pub fn left() -> Self {
        
    }
    pub fn right() -> Self {
        
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
        self.dir = (self.orig-at).normalize();
    }
    fn set_origin(&mut self, origin: Self::Vector) {
        self.orig = origin;
    }
    fn at(&mut self, distance: Self::Vector) -> Self::Vector {
        return self.orig + distance*self.dir;
    }
}