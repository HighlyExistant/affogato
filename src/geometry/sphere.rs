use crate::{vector::Vector3, Number};

#[repr(C, align(16))]
#[derive(Clone, Copy, Debug)]
pub struct Sphere<T: Number> {
    pub center: Vector3<T>,
    pub radius: f32,
}

impl<T: Number> Sphere<T> {
    pub fn new(center: Vector3<T>, radius: f32) -> Self {
        Self { center, radius }
    }
}