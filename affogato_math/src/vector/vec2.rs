
use core::ops::{Add, Index, IndexMut, Neg};
use crate::{epsilon_eq, vector::{impl_macros::{self, impl_all_from, impl_all_from_vec, impl_all_scalar_ops, impl_fromvec2, impl_ops, impl_scalar_ops, vector_permutations}, vec3::Vector3, vec4::Vector4}};
use bytemuck::{Pod, Zeroable};
use paste::paste;

use affogato_core::{groups::vector_spaces::{InnerProduct, MetricSpace, NormedVectorSpace, OuterProduct, VectorSpace}, num::{Bounds, Number, One, Signed, Zero}, sets::Real};
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
    pub fn cos(&self)-> T 
        where T: Real {
        self.normalize().dot(&Self::right())
    }
    pub fn sin(&self)-> T 
        where T: Real {
        T::from_f64(core::f64::consts::FRAC_PI_2) - self.cos()
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