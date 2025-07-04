use crate::{sdf::SignedDistance, vector::{Vector2, Vector3}};

use affogato_core::{groups::vector_spaces::{MetricSpace, VectorSpace, NormedVectorSpace}, num::Number, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

macro_rules! impl_ops_hsphere {
    ($structure:tt, $vector:tt) => {
        impl<T: Number> core::ops::Add<$vector<T>> for $structure<T> {
            type Output = Self;
            fn add(self, rhs: $vector<T>) -> Self::Output {
                Self { center: self.center+rhs, radius: self.radius }
            }
        }
        impl<T: Number> core::ops::Sub<$vector<T>> for $structure<T> {
            type Output = Self;
            fn sub(self, rhs: $vector<T>) -> Self::Output {
                Self { center: self.center-rhs, radius: self.radius }
            }
        }
        impl<T: Number> core::ops::Mul<T> for $structure<T> {
            type Output = Self;
            fn mul(self, rhs: T) -> Self::Output {
                Self { center: self.center, radius: self.radius*rhs }
            }
        }
        impl<T: Number> core::ops::Div<T> for $structure<T> {
            type Output = Self;
            fn div(self, rhs: T) -> Self::Output {
                Self { center: self.center, radius: self.radius/rhs }
            }
        }
        impl<T: Number> core::cmp::PartialEq for $structure<T> {
            fn eq(&self, other: &Self) -> bool {
                self.center == other.center && self.radius == other.radius
            }
        }
    };
}
pub trait HyperSphere<V: VectorSpace> {
    fn center(&self) -> V;
    fn radius(&self) -> V::Scalar;
}

#[repr(C, align(16))]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
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
impl_ops_hsphere!(Circle, Vector2);
#[repr(C, align(16))]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
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

impl<T: Real> SignedDistance<Vector2<T>> for Circle<T> {
    type Distance = T;
    fn sdf(&self, object: &Vector2<T>) -> Self::Distance {
        self.center.distance(&object)-self.radius
    }
}

impl<T: Real> SignedDistance<Self> for Circle<T> {
    type Distance = T;
    fn sdf(&self, object: &Self) -> Self::Distance {
        self.center.distance(&object.center)-self.radius-object.radius
    }
}

impl_ops_hsphere!(Sphere, Vector3);