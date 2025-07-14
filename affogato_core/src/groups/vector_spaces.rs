use crate::{num::{FloatingPoint, Number, UniversalOperationsOn, Zero}, sets::Real};

pub trait CoordinateSpace
    where Self: UniversalOperationsOn<Self::Element> + UniversalOperationsOn<Self> {
    type Element: Number;
    // retrieves a point inside the vector, checking whether it is out of bounds
    fn get(&self, index: usize) -> Option<Self::Element>;
    // retrieves a point inside the vector
    unsafe fn get_unchecked(&self, index: usize) -> Self::Element;
    /// the amount of scalar values this vector has.
    fn len(&self) -> usize;
    fn binary_operation<F: Fn(Self::Element, Self::Element) -> Self::Element>(&self, rhs: Self, f: F) -> Self;
    fn unary_operation<F: Fn(Self::Element) -> Self::Element>(&self, f: F) -> Self;
}
pub trait InnerProduct: VectorSpace {
    fn inner_product(&self, other: &Self) -> Self::Scalar;
}
pub trait OuterProduct {
    type Output;
    fn outer_product(&self, other: &Self) -> Self::Output;
}

pub trait MetricSpace {
    type Distance: Number;
    fn distance(&self, other: &Self) -> Self::Distance;
}

pub trait VectorSpace
    where Self: Zero +
    UniversalOperationsOn<Self> + UniversalOperationsOn<Self::Scalar> +
    Clone + Copy {
    type Scalar: Number;
    type CrossProduct;
    /// The dot product is a common linear algebra function which is defined as
    /// the sum of the products of each respective scalar value in the vector.
    /// # Properties of the Dot Product
    /// * The dot product is commutative
    /// * The angle between the two vectors is greater than 90 degrees if the dot product is negative
    /// * The vectors are perpendicular if the dot product equals 0
    /// * The dot product of two normalized vectors, returns the cosine of the angle between those vectors.
    #[inline]
    fn dot(&self, other: &Self) -> Self::Scalar 
        where Self: InnerProduct {
        self.inner_product(other)
    }
    #[inline]
    fn cross(&self, other: &Self) -> Self::CrossProduct 
        where Self: OuterProduct<Output = Self::CrossProduct> {
        self.outer_product(other)
    }
}

pub trait NormedVectorSpace
    where Self: VectorSpace + MetricSpace<Distance = Self::Scalar> {
    fn normalize(&self) -> Self;
    fn length_squared(&self) -> Self::Scalar;
    fn length(&self) -> Self::Scalar 
        where Self::Scalar: Real {
        self.length_squared().sqrt()
    }
    fn direction_to(&self, point: &Self) -> Self 
        where Self::Scalar: Real,
        Self: core::ops::Sub<Output = Self> + Sized { 
            point.clone().sub(self.clone()).normalize()
    }
    fn point_at(&self, point: &Self, distance: Self::Scalar) -> Self 
    where Self::Scalar: Real,
    Self: core::ops::Sub<Output = Self> + Sized {
        self.direction_to(point).mul(distance)+self.clone()
    }
}