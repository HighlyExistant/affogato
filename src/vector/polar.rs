use std::fmt::Debug;

use crate::Real;

use super::{Vector, Vector2, Vector3};

pub trait IntoVector<'a> 
    where Self: 'a {
    type Vector: From<&'a Self> + Vector;
    fn into_vector(&'a self) -> Self::Vector {
        Self::Vector::from(self)
    }
}
pub trait IntoRadialCoordinate<'a>: Vector 
    where Self: 'a {
    type Radial: From<&'a Self>;
    fn into_radial(&'a self) -> Self::Radial {
        Self::Radial::from(self)
    }
}
impl<'a, T: 'a> IntoRadialCoordinate<'a> for Vector2<T> 
    where T: Real {
    type Radial = PolarCoordinate<T>;
}
impl<'a, T: 'a> IntoRadialCoordinate<'a> for Vector3<T> 
    where T: Real {
    type Radial = SphericalCoordinate<T>;
}

#[derive(Clone, Copy)]
pub struct PolarCoordinate<T> {
    pub length: T,
    pub angle: T,
}
impl<T: Real> PolarCoordinate<T> {
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
impl<T: Real> From<PolarCoordinate<T>> for Vector2<T> {
    fn from(value: PolarCoordinate<T>) -> Self {
        Vector2::new(value.angle.cos()*value.length, value.angle.sin()*value.length)
    }
}
impl<T: Real> From<&PolarCoordinate<T>> for Vector2<T> {
    fn from(value: &PolarCoordinate<T>) -> Self {
        Vector2::new(value.angle.cos()*value.length, value.angle.sin()*value.length)
    }
}
impl<T: Real> From<Vector2<T>> for PolarCoordinate<T> {
    fn from(value: Vector2<T>) -> Self {
        Self { length: value.length(), angle: value.angle() }
    }
}
impl<T: Real> From<&Vector2<T>> for PolarCoordinate<T> {
    fn from(value: &Vector2<T>) -> Self {
        value.into()
    }
}
impl<'a, T: Real + 'a> IntoVector<'a> for PolarCoordinate<T> {
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
impl<T: Real> SphericalCoordinate<T> {
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
impl<T: Real> From<SphericalCoordinate<T>> for Vector3<T> {
    fn from(value: SphericalCoordinate<T>) -> Self {
        Vector3::new(
            value.length*value.polar.sin()*value.alpha.cos(), 
            value.length*value.polar.sin()*value.alpha.sin(), 
            value.length*value.polar.cos()
        )
    }
}
impl<T: Real> From<&SphericalCoordinate<T>> for Vector3<T> {
    fn from(value: &SphericalCoordinate<T>) -> Self {
        value.into()
    }
}
impl<T: Real> From<Vector3<T>> for SphericalCoordinate<T> {
    fn from(value: Vector3<T>) -> Self {
        let acos = value.z.div(value.length()).acos();
        Self { length: value.length(), alpha: Vector2::new(value.x, value.y).angle(), polar: acos }
    }
}
impl<T: Real> From<&Vector3<T>> for SphericalCoordinate<T> {
    fn from(value: &Vector3<T>) -> Self {
        let acos = value.z.div(value.length()).acos();
        Self { length: value.length(), alpha: Vector2::new(value.x, value.y).angle(), polar: acos }
    }
}
impl<'a, T: Real + 'a> IntoVector<'a> for SphericalCoordinate<T> {
    type Vector = Vector3<T>;
}