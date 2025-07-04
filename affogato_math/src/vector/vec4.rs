use core::ops::{Index, IndexMut};

use affogato_core::{groups::vector_spaces::{InnerProduct, MetricSpace, NormedVectorSpace, VectorSpace}, num::{Bounds, Number, One, Signed, Zero}, sets::Real};
use bytemuck::{Pod, Zeroable};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::vector::{impl_macros::{impl_all_from, impl_all_from_vec, impl_all_scalar_ops, impl_fromvec4, impl_ops, impl_scalar_ops}, vec2::Vector2, vec3::Vector3};

#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug, Hash)]
pub struct Vector4<T: Number> {
    x: T,
    y: T,
    z: T,
    w: T,
}

impl_ops!(Vector4, x, y, z, w);
impl_all_scalar_ops!(Vector4, x, y, z, w);
impl_all_from_vec!(impl_fromvec4);

impl<T: Number> InnerProduct for Vector4<T> {
    fn inner_product(&self, other: &Self) -> Self::Scalar {
        (self.x()*other.x())+(self.y()*other.y())+(self.z()*other.z())+(self.w()*other.w())
    }
}

impl<T: Real> MetricSpace for Vector4<T> {
    type Distance = T;
    fn distance(&self, other: &Self) -> Self::Distance {
        (self.clone()-other.clone()).length()
    }
}
impl<T: Number> Zero for Vector4<T> {
    const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO);
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero()
    }
}
impl<T: Number> One for Vector4<T> {
    const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE);
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one() && self.z.is_one() && self.w.is_one()
    }
}

impl<T: Number> VectorSpace for Vector4<T> {
    type Scalar = T;
    type CrossProduct = ();
}

impl<T: Real> NormedVectorSpace for Vector4<T> {
    fn normalize(&self) -> Self {
        let magnitude = self.length();
        self.clone()/magnitude
    }
    fn length_squared(&self) -> Self::Scalar {
        (self.x()*self.x())+(self.y()*self.y())+(self.z()*self.z())+(self.w()*self.w())
    }
}

impl<T: Number> Index<usize> for Vector4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[T; 4]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 4]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
    
    pub fn xyz(&self) -> Vector3<T> {
        Vector3::new(self.x, self.y, self.z)
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
    pub const fn w(&self) -> T {
        self.w
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

    #[inline]
    pub fn set_w(&mut self, w: T) {
        self.w = w;
    }
    pub fn epsilon_eq(&self, p: Self, epsilon: T) -> bool 
        where T: Real {
        let p = (self.clone()-p).abs();
        p.x <= epsilon &&
        p.y <= epsilon &&
        p.z <= epsilon &&
        p.w <= epsilon
    }

    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
        where T: rand::distr::uniform::SampleUniform {
        Vector4::new(generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()))
    }
}

impl<T: Signed + Number> Signed for Vector4<T> {
    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }
    fn flip_sign(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w)
    }
    fn is_negative(self) -> bool {
        self.x.is_negative() &&
        self.y.is_negative() &&
        self.z.is_negative() &&
        self.w.is_negative() 
    }
    fn is_positive(self) -> bool {
        self.x.is_positive() &&
        self.y.is_positive() &&
        self.z.is_positive() &&
        self.w.is_positive() 
    }
}

impl<T: Number> From<T> for Vector4<T>  {
    fn from(value: T) -> Self {
        Self::new(value, value, value, value)
    }
}

impl<T: Number> From<Vector2<T>> for Vector4<T>  {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x(), value.y(), T::ZERO, T::ONE)
    }
}
impl<T: Number> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self::new(value.x(), value.y(), value.z(), T::ONE)
    }
}

impl<T: Number> From<(T, T, T, T)> for Vector4<T> {
    fn from(value: (T, T, T, T)) -> Self {
        Self::new(value.0, value.1, value.2, value.3)
    }
}
impl<T: Number> From<[T; 4]> for Vector4<T> {
    fn from(value: [T; 4]) -> Self {
        Self::new(value[0], value[1], value[2], value[3])
    }
}
impl<T: Number> From<Vector4<T>> for (T, T, T, T)  {
    fn from(value: Vector4<T>) -> Self {
        (value.x, value.y, value.z, value.w)
    }
}
impl<T: Number> From<Vector4<T>> for [T; 4]  {
    fn from(value: Vector4<T>) -> Self {
        [value.x, value.y, value.z, value.w]
    }
}

impl<T: Number> Bounds for Vector4<T> {
    const MIN: Self = Self::new(T::MIN, T::MIN, T::MIN, T::MIN);
    const MAX: Self = Self::new(T::MAX, T::MAX, T::MAX, T::MAX);
    fn min(self, other: Self) -> Self {
        Self::new(
            self.x.min(other.x),
            self.y.min(other.y),
            self.z.min(other.z),
            self.w.min(other.w),
        )
    }
    fn max(self, other: Self) -> Self {
        Self::new(
            self.x.max(other.x),
            self.y.max(other.y),
            self.z.max(other.z),
            self.w.max(other.w),
        )
    }
}