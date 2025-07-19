use core::ops::Sub;

use affogato_core::{clamp, groups::vector_spaces::{NormedVectorSpace, VectorSpace}, num::Number, sets::Real};

use crate::{geometry::LinearSegment2D, vector::Vector2};

pub trait SignedDistance<T> {
    type Distance;
    fn sdf(&self, object: &T) -> Self::Distance;
}

pub trait RoundSignedDistance<T>: SignedDistance<T> {
    type Radius;
    fn round_sdf(&self, object: &T, r: Self::Radius) -> Self::Distance;
}

impl<T: Number> SignedDistance<Vector2<T>> for LinearSegment2D<T> {
    type Distance = T;
    fn sdf(&self, object: &Vector2<T>) -> Self::Distance {
        self.round_sdf(object, T::ONE)
    }
}

impl<T: Number> RoundSignedDistance<Vector2<T>> for LinearSegment2D<T> {
    type Radius = T;
    fn round_sdf(&self, object: &Vector2<T>, r: Self::Radius) -> Self::Distance {
        let dist_obj_start = object.sub(self.start);
        let dist_line = self.end-self.start;
        let h = clamp(dist_obj_start.dot(&dist_line).div(dist_line.length_squared()), T::ZERO, T::ONE);

        return (dist_obj_start - dist_line*h).length_squared() - r;
    }
}