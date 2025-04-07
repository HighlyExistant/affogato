use std::fmt::Debug;

use crate::Real;

use super::{Vector, Vector2, Vector3, Vector4};

pub trait RadialCoordinate {
    type Vector: Vector;
    type Angles;
    fn length(&self) -> <Self::Vector as Vector>::Scalar;
    fn angles(&self) -> Self::Angles;
    fn to_cartesian(&self) -> Self::Vector
        where for<'a> Self::Vector: From<&'a Self> {
        Self::Vector::from(self)
    }
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
}
impl<T: Real> RadialCoordinate for PolarCoordinate<T> {
    type Angles = T;
    type Vector = Vector2<T>;
    fn angles(&self) -> Self::Angles {
        self.angle
    }
    fn length(&self) -> T {
        self.length
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
#[derive(Clone, Copy)]
pub struct SphericalCoordinate<T> {
    pub length: T,
    pub polar: T,
    pub azimuth: T,
}
impl<T: Real> RadialCoordinate for SphericalCoordinate<T> {
    type Vector = Vector3<T>;
    type Angles = (T, T);
    fn angles(&self) -> (T, T) {
        (self.polar, self.azimuth)
    }
    fn length(&self) -> T {
        self.length
    }
}
impl<T: Debug> Debug for SphericalCoordinate<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SphericalCoordinate")
            .field("length", &self.length)
            .field("polar", &self.polar)
            .field("azimuth", &self.azimuth)
            .finish()
    }
}
impl<T: Real> SphericalCoordinate<T> {
    pub fn new(length: T,  polar: T, azimuth: T) -> Self {
        Self { length,  polar, azimuth }
    }
}
impl<T: Real> From<SphericalCoordinate<T>> for Vector3<T> {
    fn from(value: SphericalCoordinate<T>) -> Self {
        Vector3::new(
            value.length*value.polar.sin()*value.azimuth.cos(), 
            value.length*value.polar.sin()*value.azimuth.sin(), 
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
        Self { length: value.length(), azimuth: Vector2::new(value.x, value.y).angle(), polar: acos }
    }
}
impl<T: Real> From<&Vector3<T>> for SphericalCoordinate<T> {
    fn from(value: &Vector3<T>) -> Self {
        let acos = value.z.div(value.length()).acos();
        Self { length: value.length(), azimuth: Vector2::new(value.x, value.y).angle(), polar: acos }
    }
}

#[derive(Clone, Copy)]
pub struct HyperSphereD4Coordinates<T: Real> {
    pub length: T,
    pub polar: T,
    pub azimuth: T,
    pub phi: T,
}
impl<T: Real> RadialCoordinate for HyperSphereD4Coordinates<T> {
    type Vector = Vector4<T>;
    type Angles = (T, T, T);
    fn angles(&self) -> (T, T, T) {
        (self.polar, self.azimuth, self.phi)
    }
    fn length(&self) -> T {
        self.length
    }
}

impl<T: Real> HyperSphereD4Coordinates<T> {
    pub fn new(length: T,  polar: T, azimuth: T, phi: T) -> Self {
        Self { length,  polar, azimuth, phi }
    }
}

impl<T: Real + Debug> Debug for HyperSphereD4Coordinates<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SphericalCoordinate")
            .field("length", &self.length)
            .field("polar", &self.polar)
            .field("azimuth", &self.azimuth)
            .field("phi", &self.phi)
            .finish()
    }
}

impl<T: Real> From<HyperSphereD4Coordinates<T>> for Vector4<T> {
    fn from(value: HyperSphereD4Coordinates<T>) -> Self {
        Vector4::new(
            value.length*value.phi.sin()*value.polar.sin()*value.azimuth.cos(), 
            value.length*value.phi.sin()*value.polar.sin()*value.azimuth.sin(), 
            value.length*value.phi.sin()*value.polar.cos(), 
            value.length*value.phi.cos()
        )
    }
}
impl<T: Real> From<Vector4<T>> for HyperSphereD4Coordinates<T> {
    fn from(value: Vector4<T>) -> Self {
        let acos = value.z.div(Vector3::from(value).length()).acos();
        let acos2 = value.w.div(value.length()).acos();
        Self { length: value.length(), azimuth: Vector2::new(value.x, value.y).angle(), polar: acos, phi: acos2 }
    }
}