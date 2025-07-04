use core::ops::{Index, IndexMut, Sub};

use bytemuck::{Pod, Zeroable};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use affogato_core::{groups::vector_spaces::{InnerProduct, MetricSpace, NormedVectorSpace, OuterProduct, VectorSpace}, num::{Bounds, Number, One, Signed, Zero}, sets::Real};

use crate::vector::{impl_macros::{impl_all_from, impl_all_from_vec, impl_all_scalar_ops, impl_fromvec3, impl_ops, impl_scalar_ops, vector_permutations}, vec2::Vector2, vec4::Vector4};

#[repr(C)]
#[cfg(feature="glsl")]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug, Hash)]
pub struct Vector3<T: Number> {
    x: T,
    y: T,
    z: T,
    padding: T,
}

#[repr(C)]
#[cfg(not(feature="glsl"))]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug, Hash)]
pub struct Vector3<T: Number> {
    x: T,
    y: T,
    z: T,
}
impl_ops!(Vector3, x, y, z);
impl_all_scalar_ops!(Vector3, x, y, z);
impl_all_from_vec!(impl_fromvec3);

impl<T: Number> InnerProduct for Vector3<T> {
    fn inner_product(&self, other: &Self) -> Self::Scalar {
        (self.x()*other.x())+(self.y()*other.y())+(self.z()*other.z())
    }
}

impl<T: Number> OuterProduct for Vector3<T> {
    type Output = Self;
    /// The outer product, also known as the cross product, is used to find a vector 
    /// perpendicular to 2 vectors. 
    /// # Properties of the Cross Product
    /// * finds a vector perpendicular to the 2 given vectors.
    /// * If the vectors are collinear it will give you a 0 vector.
    fn outer_product(&self, other: &Self) -> Self::Output {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl<T: Real> MetricSpace for Vector3<T> {
    type Distance = T;
    fn distance(&self, other: &Self) -> Self::Distance {
        (self.clone()-other.clone()).length()
    }
}
impl<T: Number> Zero for Vector3<T> {
    const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO);
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero()
    }
}
impl<T: Number> One for Vector3<T> {
    const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE);
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one() && self.z.is_one()
    }
}

impl<T: Number> VectorSpace for Vector3<T> {
    type Scalar = T;
    type CrossProduct = Self;
}

impl<T: Real> NormedVectorSpace for Vector3<T> {
    fn normalize(&self) -> Self {
        let magnitude = self.length();
        self.clone()/magnitude
    }
    fn length_squared(&self) -> Self::Scalar {
        (self.x()*self.x())+(self.y()*self.y())+(self.z()*self.z())
    }
}

impl<T: Number> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[T; 3]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 3]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Vector3<T> {
    #[cfg(feature="glsl")]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z, padding: T::ZERO }
    }
    #[cfg(not(feature="glsl"))]
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z }
    }
    /// Returns a vector pointing to the right of the graph <1, 0, 0>
    pub const fn right() -> Self {
        Self::new(T::ONE, T::ZERO, T::ZERO)
    }
    /// Returns a vector pointing to the left of the graph <-1, 0, 0>
    pub fn left() -> Self 
        where T: core::ops::Neg<Output = T> {
        Self::new(-T::ONE, T::ZERO, T::ZERO)
    }
    /// Returns a vector pointing to the top of the graph <0, 1, 0>
    pub const fn top() -> Self {
        Self::new(T::ZERO, T::ONE, T::ZERO)
    }
    /// Returns a vector pointing to the top of the graph <0, -1, 0>
    pub fn bottom() -> Self 
        where T: core::ops::Neg<Output = T> {
        Self::new(T::ZERO, -T::ONE, T::ZERO)
    }
    /// Returns a vector pointing forward to the graph <0, 0, 1>
    pub const fn forward() -> Self {
        Self::new(T::ZERO, T::ZERO, T::ONE)
    }
    /// Returns a vector pointing backward to the graph <0, 0, -1>
    pub fn backward() -> Self
        where T: core::ops::Neg<Output = T> {
        Self::new(T::ZERO, T::ZERO, -T::ONE)
    }

    #[inline]
    pub const fn x(&self) -> T {
        self.x
    }

    #[inline]
    pub const fn y(&self) -> T {
        self.y
    }

    #[inline]
    pub const fn z(&self) -> T {
        self.z
    }

    #[inline]
    pub fn set_x(&mut self, x: T) {
        self.x = x;
    }

    #[inline]
    pub fn set_y(&mut self, y: T) {
        self.y = y;
    }

    #[inline]
    pub fn set_z(&mut self, z: T) {
        self.z = z;
    }

    vector_permutations!(Vector2, x, y);
    vector_permutations!(Vector2, y, x);
    vector_permutations!(Vector2, x, z);
    vector_permutations!(Vector2, z, x);
    vector_permutations!(Vector2, y, z);
    vector_permutations!(Vector2, z, y);
    
    /// from: https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
    pub fn line_distance(&self, a: Self, b: Self) -> T 
        where T: Real {
        let dir_ba = b-a;
        let dir_pa = *self-a;
        dir_pa.cross(&dir_ba).length().div(dir_ba.length())
    }
    pub fn signed_plane_distance(&self, a: Self, b: Self, c: Self) -> T 
        where T: Real {
        let normal = b.sub(a).cross(&c.sub(a)).normalize();
        normal.dot(&self.sub(a))
    }
    pub fn epsilon_eq(&self, p: Self, epsilon: T) -> bool 
        where T: Real {
        let p = (self.clone()-p).abs();
        p.x <= epsilon &&
        p.y <= epsilon &&
        p.z <= epsilon 
    }
    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
        where T: rand::distr::uniform::SampleUniform {
        Vector3::new(generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()))
    }
}
impl<T: Signed + Number> Signed for Vector3<T> {
    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    fn flip_sign(self) -> Self {
        Self::new(-self.x, -self.y, -self.z)
    }
    fn is_negative(self) -> bool {
        self.x.is_negative() &&
        self.y.is_negative() &&
        self.z.is_negative() 
    }
    fn is_positive(self) -> bool {
        self.x.is_positive() &&
        self.y.is_positive() &&
        self.z.is_positive() 
    }
}

impl<T: Number> From<T> for Vector3<T>  {
    fn from(value: T) -> Self {
        Self::new(value, value, value)
    }
}

impl<T: Number> From<Vector2<T>> for Vector3<T>  {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x(), value.y(), T::ONE)
    }
}
impl<T: Number> From<Vector4<T>> for Vector3<T> {
    fn from(value: Vector4<T>) -> Self {
        Self::new(value.x(), value.y(), value.z())
    }
}
impl<T: Number> From<(T, T, T)> for Vector3<T> {
    fn from(value: (T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2)
    }
}
impl<T: Number> From<[T; 3]> for Vector3<T> {
    fn from(value: [T; 3]) -> Self {
        Self::new(value[0], value[1], value[2])
    }
}
impl<T: Number> From<Vector3<T>> for (T, T, T)  {
    fn from(value: Vector3<T>) -> Self {
        (value.x, value.y, value.z)
    }
}
impl<T: Number> From<Vector3<T>> for [T; 3]  {
    fn from(value: Vector3<T>) -> Self {
        [value.x, value.y, value.z]
    }
}

impl<T: Number> Bounds for Vector3<T> {
    const MIN: Self = Self::new(T::MIN, T::MIN, T::MIN);
    const MAX: Self = Self::new(T::MAX, T::MAX, T::MAX);
    fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
        )
    }
    fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
        )
    }
}