use num_traits::{Float, AsPrimitive};

use crate::{algebra::Quaternion, FloatingPoint, Number};

use super::{Matrix4, SquareMatrix, Vector3};

pub trait Transformation<T: Number> {
    fn matrix(&self) -> Matrix4<T> { Matrix4::identity() }
    fn translation(&self) -> Vector3<T>;
    fn rotation(&self) -> Quaternion<T>
        where T: FloatingPoint;
    fn scaling(&self) -> Vector3<T>;
    fn identity(&mut self);
}
#[derive(Clone)]
pub struct Transform<T: FloatingPoint> {
    pub translation: Vector3<T>,
    pub scaling: Vector3<T>,
    pub rotation: Quaternion<T>,
}

impl<T: FloatingPoint> Default for Transform<T> 
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T> {
    fn default() -> Self {
        Self { 
            translation: Vector3::new(T::zero(), T::zero(), T::zero()), 
            scaling: Vector3::new(T::one(), T::one(), T::one()), 
            rotation: Quaternion::from_euler(Vector3::new(T::zero(), T::zero(), T::zero()))
        }
    }
}

impl<T: FloatingPoint> Transform<T> {
    pub fn new(translation: Vector3<T>, scaling: Vector3<T>, rotation: Quaternion<T>) -> Self {
        Self { translation, scaling, rotation }
    }
}