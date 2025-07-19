use core::ops::{Index, IndexMut};

use affogato_core::{groups::vector_spaces::{CoordinateSpace, InnerProduct, MetricSpace, NormedVectorSpace, VectorSpace}, num::{Bounds, Number, One, Signed, Zero}, sets::Real};

use bytemuck::{Pod, Zeroable};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use crate::vector::impl_macros::{impl_all_from, impl_all_from_vec, impl_all_scalar_ops, impl_fromvec5, impl_ops, impl_scalar_ops};

#[repr(C)]
#[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
#[derive(Default, Clone, Copy, Debug, Hash)]
pub struct Vector5<T: Number> {
    x: T,
    y: T,
    z: T,
    w: T,
    a: T,
}

impl_ops!(Vector5, x, y, z, w, a);
impl_all_scalar_ops!(Vector5, x, y, z, w, a);
impl_all_from_vec!(impl_fromvec5);

impl<T: Number> Vector5<T> {
    pub const fn new(x: T, y: T, z: T, w: T, a: T) -> Self {
        Self { x, y, z, w, a }
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
    pub const fn a(&self) -> T {
        self.a
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

    #[inline]
    pub fn set_a(&mut self, a: T) {
        self.a = a;
    }
    pub fn as_slice(&self) -> &[T] {
        unsafe { core::slice::from_raw_parts(self as *const _ as _, self.len()) }
    }
    
    pub fn as_mut_slice(&mut self) -> &mut [T] {
        unsafe { core::slice::from_raw_parts_mut(self as *mut _ as _, self.len()) }
    }

    #[cfg(feature="rand")]
    pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
        where T: rand::distr::uniform::SampleUniform {
        Vector5::new(generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()))
    }
}

impl<T: Signed + Number> Signed for Vector5<T> {
    fn abs(self) -> Self {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs(), self.a.abs())
    }
    fn flip_sign(self) -> Self {
        Self::new(-self.x, -self.y, -self.z, -self.w, -self.a)
    }
    fn is_negative(self) -> bool {
        self.x.is_negative() &&
        self.y.is_negative() &&
        self.z.is_negative() &&
        self.w.is_negative() &&
        self.a.is_negative()
    }
    fn is_positive(self) -> bool {
        self.x.is_positive() &&
        self.y.is_positive() &&
        self.z.is_positive() &&
        self.w.is_positive() &&
        self.a.is_positive() 
    }
}

impl<T: Number> CoordinateSpace for Vector5<T> {
    type Element = T;
    fn get(&self, index: usize) -> Option<Self::Element> {
        self.as_slice().get(index).copied()
    }
    unsafe fn get_unchecked(&self, index: usize) -> Self::Element {
        unsafe { *self.as_slice().get_unchecked(index) }
    }
    fn len(&self) -> usize {
        5
    }
    fn binary_operation<F: Fn(Self::Element, Self::Element) -> Self::Element>(&self, rhs: Self, f: F) -> Self {
        Self::new(f(self.x, rhs.x), f(self.y, rhs.y), f(self.z, rhs.z), f(self.w, rhs.w), f(self.a, rhs.a))
    }
    fn unary_operation<F: Fn(Self::Element) -> Self::Element>(&self, f: F) -> Self {
        Self::new(f(self.x), f(self.y), f(self.z), f(self.w), f(self.a))
    }
}

impl<T: Number> Zero for Vector5<T> {
    const ZERO: Self = Self::new(T::ZERO, T::ZERO, T::ZERO, T::ZERO, T::ZERO);
    fn is_zero(&self) -> bool {
        self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero() && self.a.is_zero()
    }
}
impl<T: Number> One for Vector5<T> {
    const ONE: Self = Self::new(T::ONE, T::ONE, T::ONE, T::ONE, T::ONE);
    fn is_one(&self) -> bool {
        self.x.is_one() && self.y.is_one() && self.z.is_one() && self.w.is_one() && self.a.is_one()
    }
}

impl<T: Number> Index<usize> for Vector5<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { core::mem::transmute::<&Self, &[T; 5]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector5<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 5]>(self) };
        &mut val[index]
    }
}