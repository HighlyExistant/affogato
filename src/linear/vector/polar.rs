use std::fmt::Debug;

use num_traits::Float;

use crate::{linear::FVec2, FloatingPoint};

use super::{Vector, Vector2, Vector3};

pub trait IntoVector<'a> 
    where Self: 'a {
    type Vector: From<&'a Self> + Vector;
    fn into_vector(&'a self) -> Self::Vector {
        Self::Vector::from(self)
    }
}

#[derive(Clone, Copy)]
pub struct PolarCoordinate<T> {
    pub length: T,
    pub angle: T,
}
impl<T: FloatingPoint> PolarCoordinate<T> {
    pub fn new(length: T, angle: T) -> Self {
        Self { length, angle }
    }
    pub fn angle(&self) -> T {
        self.angle
    }
    pub fn length(&self) -> T {
        self.angle
    }
}
impl<T: FloatingPoint> From<PolarCoordinate<T>> for Vector2<T> {
    fn from(value: PolarCoordinate<T>) -> Self {
        Vector2::new(value.angle.cos()*value.length, value.angle.sin()*value.length)
    }
}
impl<T: FloatingPoint> From<&PolarCoordinate<T>> for Vector2<T> {
    fn from(value: &PolarCoordinate<T>) -> Self {
        Vector2::new(value.angle.cos()*value.length, value.angle.sin()*value.length)
    }
}
impl<T: FloatingPoint> From<Vector2<T>> for PolarCoordinate<T> {
    fn from(value: Vector2<T>) -> Self {
        Self { length: value.length(), angle: value.angle() }
    }
}
impl<T: FloatingPoint> From<&Vector2<T>> for PolarCoordinate<T> {
    fn from(value: &Vector2<T>) -> Self {
        value.into()
    }
}
impl<'a, T: FloatingPoint> IntoVector<'a> for PolarCoordinate<T> {
    type Vector = Vector2<T>;
}
#[derive(Clone, Copy)]
pub struct SphericalCoordinate<T> {
    pub length: T,
    pub polar: T,
    pub alpha: T,
}
impl<T: Debug> Debug for SphericalCoordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SphericalCoordinate")
            .field("length", &self.length)
            .field("polar", &self.polar)
            .field("alpha", &self.alpha)
            .finish()
    }
}
impl<T: FloatingPoint> SphericalCoordinate<T> {
    pub fn new(length: T,  polar: T, alpha: T) -> Self {
        Self { length,  polar, alpha }
    }
    pub fn angles(&self) -> (T, T) {
        (self.polar, self.alpha)
    }
    pub fn length(&self) -> T {
        self.length
    }
}
impl<T: FloatingPoint> From<SphericalCoordinate<T>> for Vector3<T> {
    fn from(value: SphericalCoordinate<T>) -> Self {
        Vector3::new(
            value.length*value.polar.sin()*value.alpha.cos(), 
            value.length*value.polar.sin()*value.alpha.sin(), 
            value.length*value.polar.cos()
        )
    }
}
impl<T: FloatingPoint> From<&SphericalCoordinate<T>> for Vector3<T> {
    fn from(value: &SphericalCoordinate<T>) -> Self {
        value.into()
    }
}
impl<T: FloatingPoint> From<Vector3<T>> for SphericalCoordinate<T> {
    fn from(value: Vector3<T>) -> Self {
        let acos = value.z.div(value.length()).acos();
        Self { length: value.length(), alpha: Vector2::new(value.x, value.y).angle(), polar: acos }
    }
}
impl<T: FloatingPoint> From<&Vector3<T>> for SphericalCoordinate<T> {
    fn from(value: &Vector3<T>) -> Self {
        let acos = value.z.div(value.length()).acos();
        Self { length: value.length(), alpha: Vector2::new(value.x, value.y).angle(), polar: acos }
    }
}
impl<'a, T: FloatingPoint> IntoVector<'a> for SphericalCoordinate<T> {
    type Vector = Vector3<T>;
}