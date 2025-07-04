use crate::{vector::Vector3, IsNormalized};

use affogato_core::{groups::vector_spaces::{NormedVectorSpace, VectorSpace}, num::Number, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

/// Represents a 3 dimensional hyperplane
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug)]
pub struct Plane<T: Number> {
    normal: Vector3<T>, 
    /// it's possible for this distance to be signed
    distance: T,
}

impl<T: Number> Plane<T> {
    /// # Safety
    /// Ensure that the `normal` provided is normalized beforehand.
    pub unsafe fn from_normal_distance_unchecked(normal: Vector3<T>, distance: T) -> Self {
        Self { normal, distance }
    }
    /// Checks whether the normal provided is normalized, returns `None` if the vector is not normalized
    /// and [`Plane`] if it is normalized. Checks have a performance cost, so if you're already certain that
    /// a vector is already normalized then use `from_normal_distance_unchecked`, or if you want the function
    /// to succeed even when the vector is not normalized use `from_normal_distance`.
    pub fn from_normal_distance_checked(normal: Vector3<T>, distance: T) -> Option<Self> 
        where Vector3<T>: IsNormalized {
        if normal.normalized() {
            Some(Self { normal: normal, distance })
        } else {
            None
        }
    }
    /// Create a [`Plane`] from a normal and a distance. This is similar to `from_normal_distance_unchecked`, but ensures
    /// that `normal` is normalized. If normal is already normalized, it's reccomended you just use `from_normal_distance_unchecked`.
    pub fn from_normal_distance(normal: Vector3<T>, distance: T) -> Self 
        where T: Real {
        Self { normal: normal.normalize(), distance }
    }
    /// # Safety
    /// Ensure that the `normal` provided is normalized beforehand.
    pub unsafe fn from_normal_point(normal: Vector3<T>, point: Vector3<T>) -> Self {
        Self { normal, distance: normal.dot(&point) }
    }
    pub fn is_point_left(&self, point: Vector3<T>) -> bool {
        self.normal.dot(&point) - self.distance > T::ZERO
    }
    pub fn is_point_right(&self, point: Vector3<T>) -> bool {
        self.normal.dot(&point) - self.distance < T::ZERO
    }
    pub fn is_point_on_positive_side(&self, point: Vector3<T>) -> bool {
        self.normal.dot(&point) - self.distance >= T::ZERO
    }
    pub fn is_point_contained(&self, point: Vector3<T>) -> bool {
        self.normal.dot(&point) - self.distance == T::ZERO
    }
    pub fn normal(&self) -> Vector3<T> {
        self.normal
    }
    pub fn distance(&self) -> T {
        self.distance
    }
}