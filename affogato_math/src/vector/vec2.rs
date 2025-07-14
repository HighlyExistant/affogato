
use core::ops::{Add, Index, IndexMut, Neg};
use crate::{epsilon_eq, vector::{impl_macros::{self, impl_all_from, impl_all_from_vec, impl_all_scalar_ops, impl_fromvec2, impl_ops, impl_scalar_ops, vector_permutations}, vec3::Vector3, vec4::Vector4}};
use bytemuck::{Pod, Zeroable};
use paste::paste;

use affogato_core::{groups::vector_spaces::{CoordinateSpace, InnerProduct, MetricSpace, NormedVectorSpace, OuterProduct, VectorSpace}, num::{Bounds, Number, One, Signed, Zero}, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};
#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug, Hash)]
pub struct Vector2<T: Number> {
    x: T,
    y: T,
}

impl_ops!(Vector2,x,y);
impl_all_scalar_ops!(Vector2, x ,y);
impl_all_from_vec!(impl_fromvec2);

impl<T: Number> InnerProduct for Vector2<T> {
    fn inner_product(&self, other: &Self) -> Self::Scalar {
        (self.x()*other.x())+(self.y()*other.y())
    }
}

impl<T: Number> OuterProduct for Vector2<T> {
    type Output = T;
    /// In 2 dimensions there is no cross product as we understand it in 3d. Instead of returning
    /// a vector, it returns a scalar value. The absolute value of this scalar represents the area 
    /// of the parallelogram formed by the 2 vectors.
    /// 
    /// If you want to get the sin of 2 vectors, you can normalize the result of the outer product,
    /// similar to how you can in the dot product.
    fn outer_product(&self, other: &Self) -> Self::Output {
        (self.x * other.y) - (self.y * other.x)
    }
}

impl<T: Real> MetricSpace for Vector2<T> {
    type Distance = T;
    fn distance(&self, other: &Self) -> Self::Distance {
        (self.clone()-other.clone()).length()
    }
}
impl<T: Number> Zero for Vector2<T> {
    const ZERO: Self = Self::new(T::ZERO, T::ZERO);
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero()
    }
}
impl<T: Number> One for Vector2<T> {
    const ONE: Self = Self::new(T::ONE, T::ONE);
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one()
    }
}

impl<T: Number> VectorSpace for Vector2<T> {
    type Scalar = T;
    type CrossProduct = T;
}

impl<T: Real> NormedVectorSpace for Vector2<T> {
    fn normalize(&self) -> Self {
        let magnitude = self.length();
        self.clone()/magnitude
    }
    fn length_squared(&self) -> Self::Scalar {
        (self.x()*self.x())+(self.y()*self.y())
    }
}


impl<T: Number> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[T; 2]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 2]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
    pub fn rotate_90(&self) -> Self 
        where T: Neg<Output = T> {
        Self::new(-self.y, self.x)
    }
    pub fn rotate_180(&self) -> Self 
        where T: Neg<Output = T> {
        Self::new(self.x, -self.y)
    }
    pub fn rotate_270(&self) -> Self 
        where T: Neg<Output = T> {
        Self::new(self.y, -self.x)
    }
    /// Returns a vector pointing to the right of the graph <1, 0>
    pub fn right() -> Self {
        Self::new(T::ONE, T::ZERO)
    }
    /// Returns a vector pointing to the left of the graph <-1, 0>
    pub fn left() -> Self 
        where T: core::ops::Neg<Output = T> {
        Self::new(-T::ONE, T::ZERO)
    }
    /// Returns a vector pointing to the left of the graph <0, 1>
    pub fn top() -> Self {
        Self::new(T::ZERO, T::ONE)
    }
    /// Returns a vector pointing to the left of the graph <0, -1>
    pub fn bottom() -> Self 
        where T: core::ops::Neg<Output = T> {
        Self::new(T::ZERO, -T::ONE)
    }
    pub fn abs(&self) -> Self 
        where T: Signed {
        Self::new(self.x.abs(), self.y.abs())
    }
    pub fn cos2(&self, other: &Self) -> T 
        where T: Real {
        self.dot(&other)/self.length()
    }
    pub fn cos(&self)-> T 
        where T: Real {
        self.cos2(&Self::right())
    }
    pub fn sin2(&self, other: &Self)-> T 
        where T: Real {
        other.cross(&self)/self.length()
    }
    pub fn sin(&self)-> T 
        where T: Real {
        self.sin2(&Self::right())
    }
    pub fn tan(&self)-> T 
        where T: Real {
        let normalize = self.normalize();
        normalize.y.div(normalize.x)
    }
    pub fn angle(&self) -> T 
        where T: Real {
        self.cos().acos()
    }
    pub fn from_angle(angle: T) -> Self
        where T: Real {
        Self::new(angle.cos(), angle.sin())
    }

    #[inline]
    pub fn x(&self) -> T {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> T {
        self.y
    }

    #[inline]
    pub const fn set_x(&mut self, x: T) {
        self.x = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, self.len()) }
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, self.len()) }
    }
    /// The vector triple product is only defined for 3d, this is just a projection
    /// of that 3d version down to 2d.
    pub fn vector_triple_product(&self, b: &Self, c: &Self) -> Self {
        Self::new(
            self.y()*(b.x()*c.y() - b.y()*c.x()), 
            self.x()*(b.y()*c.x() - b.x()*c.y()), 
        )
    }

    vector_permutations!(Vector2, x, y);
    vector_permutations!(Vector2, y, x);
    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
        where T: rand::distr::uniform::SampleUniform {
        Vector2::new(generator.random_range(range.clone()), generator.random_range(range.clone()))
    }
    pub fn epsilon_eq(&self, other: Self, epsilon: T) -> bool 
        where T: Real {
        epsilon_eq(self.x, other.x, epsilon) &&
        epsilon_eq(self.y, other.y, epsilon)
    }
}

impl<T: Signed + Number> Signed for Vector2<T> {
    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs())
    }
    fn flip_sign(self) -> Self {
        Self::new(-self.x, -self.y)
    }
    fn is_negative(self) -> bool {
        self.x.is_negative() &&
        self.y.is_negative() 
    }
    fn is_positive(self) -> bool {
        self.x.is_positive() &&
        self.y.is_positive() 
    }
}

impl<T: Number> From<T> for Vector2<T>  {
    fn from(value: T) -> Self {
        Self::new(value, value)
    }
}

impl<T: Number> From<Vector3<T>> for Vector2<T>  {
    fn from(value: Vector3<T>) -> Self {
        Self::new(value.x(), value.y())
    }
}
impl<T: Number> From<Vector4<T>> for Vector2<T> {
    fn from(value: Vector4<T>) -> Self {
        Self::new(value.x(), value.y())
    }
}
impl<T: Number> From<(T, T)> for Vector2<T> {
    fn from(value: (T, T)) -> Self {
        Self::new(value.0, value.1)
    }
}
impl<T: Number> From<[T; 2]> for Vector2<T> {
    fn from(value: [T; 2]) -> Self {
        Self::new(value[0], value[1])
    }
}
impl<T: Number> From<Vector2<T>> for (T, T)  {
    fn from(value: Vector2<T>) -> Self {
        (value.x, value.y)
    }
}
impl<T: Number> From<Vector2<T>> for [T; 2]  {
    fn from(value: Vector2<T>) -> Self {
        [value.x, value.y]
    }
}

impl<T: Number> Bounds for Vector2<T> {
    const MIN: Self = Self::new(T::MIN, T::MIN);
    const MAX: Self = Self::new(T::MAX, T::MAX);
    fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
        )
    }
    fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
        )
    }
}

impl<T: Number> CoordinateSpace for Vector2<T> {
    type Element = T;
    fn get(&self, index: usize) -> Option<Self::Element> {
        self.as_slice().get(index).copied()
    }
    unsafe fn get_unchecked(&self, index: usize) -> Self::Element {
        unsafe { *self.as_slice().get_unchecked(index) }
    }
    fn len(&self) -> usize {
        2
    }
    fn binary_operation<F: Fn(Self::Element, Self::Element) -> Self::Element>(&self, rhs: Self, f: F) -> Self {
        Self::new(f(self.x, rhs.x), f(self.y, rhs.y))
    }
    fn unary_operation<F: Fn(Self::Element) -> Self::Element>(&self, f: F) -> Self {
        Self::new(f(self.x), f(self.y))
    }
}

#[cfg(test)]
mod tests {
    use crate::vector::FVec2;

    #[test]
    fn trigonometry() {
        assert!(
            FVec2::right().cos() == 1.0,
            "Cosine of a <1, 0> vector is 1, got: {}", FVec2::right().cos()
        );
        assert!(
            FVec2::top().cos() == 0.0,
            "Cosine of a <0, 1> vector is 0, got: {}", FVec2::top().cos() 
        );
        assert!(
            FVec2::right().sin() == 0.0,
            "Sine of a <1, 0> vector is 0, got: {}", FVec2::right().sin()
        );
        assert!(
            FVec2::top().sin() == 1.0,
            "Sine of a <0, 1> vector is 1, got: {}", FVec2::top().sin() 
        )
    }
}