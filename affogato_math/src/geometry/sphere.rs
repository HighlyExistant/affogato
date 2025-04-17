use crate::{vector::{Vector, Vector2, Vector3}, Number};

pub trait HyperSphere<V: Vector> {
    fn center(&self) -> V;
    fn radius(&self) -> V::Scalar;
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Circle<T: Number> {
    pub center: Vector2<T>,
    pub radius: T,
}

impl<T: Number> Circle<T> {
    pub fn new(center: Vector2<T>, radius: T) -> Self {
        Self { center, radius }
    }
}
impl<T: Number> HyperSphere<Vector2<T>> for Circle<T> {
    fn center(&self) -> Vector2<T> {
        self.center
    }
    fn radius(&self) -> T {
        self.radius
    }
}

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Sphere<T: Number> {
    pub center: Vector3<T>,
    pub radius: T,
}

impl<T: Number> Sphere<T> {
    pub fn new(center: Vector3<T>, radius: T) -> Self {
        Self { center, radius }
    }
}

impl<T: Number> HyperSphere<Vector3<T>> for Sphere<T> {
    fn center(&self) -> Vector3<T> {
        self.center
    }
    fn radius(&self) -> T {
        self.radius
    }
}