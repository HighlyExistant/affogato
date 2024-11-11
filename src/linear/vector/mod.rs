#![allow(unused)]
use core::fmt;
use std::{fmt::Debug, ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Rem, RemAssign, Sub, SubAssign}};

use num_traits::{AsPrimitive, Bounded, Float, Num, One, Signed, Zero};

use crate::{algebra::Quaternion, RationalNumber, Rotation, sets::{Number}};
mod atomic;
mod polar;
pub use polar::*;
pub trait CrossProduct {
    fn cross(&self, other: &Self) -> Self::Product;
    type Product;
}
pub trait Vector: num_traits::NumOps + Sized 
    where Self: Add<Self::Scalar, Output = Self> +
        Sub<Self::Scalar, Output = Self> +
        Mul<Self::Scalar, Output = Self> +
        Div<Self::Scalar, Output = Self> +
        Rem<Self::Scalar, Output = Self> +
        AddAssign<Self::Scalar> +
        SubAssign<Self::Scalar> +
        MulAssign<Self::Scalar> +
        DivAssign<Self::Scalar> +
        RemAssign<Self::Scalar> {
    fn length(&self) -> Self::Scalar where Self::Scalar: Float { self.dot(self).sqrt() }
    fn distance(&self, other: &Self) -> Self::Scalar where Self::Scalar: Float { self.dot(other).sqrt() }
    /// Direction gives a normalized vector
    /// that points to the given point.
    fn direction_to(&self, point: &Self) -> Self 
        where Self::Scalar: Float,
        Self: std::ops::Sub<Output = Self> + Sized + Copy { 
            point.sub(*self).normalize()
    }
    fn dot(&self, other: &Self) -> Self::Scalar;
    fn project(&self, other: &Self) -> Self where Self::Scalar: Float;
    fn len(&self) -> usize;
    fn normalize(&self) -> Self where Self::Scalar: Float;
    fn get(&self, idx: usize) -> Option<Self::Scalar>;
    fn get_unchecked(&self, idx: usize) -> Self::Scalar;
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
    type Scalar: Number;
}
pub trait IntoRadialCoordinate<'a>: Vector 
    where Self: 'a {
    type Radial: From<&'a Self>;
    fn into_radial(&'a self) -> Self::Radial {
        Self::Radial::from(self)
    }
}
pub trait Orthogonality: Vector {
    /// Gets a vector whos dot dot product with the current vector equals 0.
    /// The polarity means where the sign of the vector will be. true will make y negative
    /// and false will make x negative
    fn get_orthogonal(&self, polarity: bool) -> Self;
}
pub trait Orthonormality: Vector 
    where Self::Scalar: RationalNumber {
    fn get_orthonormal(&self, polarity: bool, allow_zero: bool) -> Self;

}
macro_rules! impl_ops {
    ($vector:ident, $($element:tt),+) => {
        impl<T: Number> std::ops::Add for $vector <T>  {
            fn add(self, rhs: Self) -> Self::Output {
                Self::new($(self.$element + rhs.$element),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Rem for $vector<T>  {
            fn rem(self, rhs: Self) -> Self::Output {
                Self::new($(self.$element % rhs.$element),+)
            }
            type Output = Self;
        }
        
        impl<T: Number + std::ops::Neg<Output = T>> std::ops::Neg for $vector<T> {
            fn neg(self) -> Self::Output {
                Self::new($(-self.$element),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Sub for $vector<T>  {
            fn sub(self, rhs: Self) -> Self::Output {
                Self::new($(self.$element - rhs.$element),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Mul for $vector<T>  {
            fn mul(self, rhs: Self) -> Self::Output {
                Self::new($(self.$element * rhs.$element),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Div for $vector<T>  {
            fn div(self, rhs: Self) -> Self::Output {
                Self::new($(self.$element / rhs.$element),+)
            }
            type Output = Self;
        }

        // Operations on scalar values
        impl<T: Number> std::ops::Add<T> for $vector<T>  {
            fn add(self, rhs: T) -> Self::Output {
                Self::new($(self.$element + rhs),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Sub<T> for $vector<T>  {
            fn sub(self, rhs: T) -> Self::Output {
                Self::new($(self.$element - rhs),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Mul<T> for $vector<T>  {
            fn mul(self, rhs: T) -> Self::Output {
                Self::new($(self.$element * rhs),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Div<T> for $vector<T>  {
            fn div(self, rhs: T) -> Self::Output {
                Self::new($(self.$element / rhs),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::Rem<T> for $vector<T>  {
            fn rem(self, rhs: T) -> Self::Output {
                Self::new($(self.$element % rhs),+)
            }
            type Output = Self;
        }
        impl<T: Number> std::ops::AddAssign for $vector<T>  {
            fn add_assign(&mut self, rhs: Self) {
                $(self.$element += rhs.$element);+
            }
        }
        impl<T: Number> std::ops::SubAssign for $vector<T>  {
            fn sub_assign(&mut self, rhs: Self) {
                $(self.$element -= rhs.$element);+
            }
        }
        impl<T: Number> std::ops::MulAssign for $vector<T>  {
            fn mul_assign(&mut self, rhs: Self) {
                $(self.$element *= rhs.$element);+
            }
        }
        impl<T: Number> std::ops::DivAssign for $vector<T>  {
            fn div_assign(&mut self, rhs: Self) {
                $(self.$element /= rhs.$element);+
            }
        }
        impl<T: Number> std::ops::RemAssign for $vector<T>  {
            fn rem_assign(&mut self, rhs: Self) {
                $(self.$element %= rhs.$element);+
            }
        }
        impl<T: Number> std::ops::AddAssign<T> for $vector<T>  {
            fn add_assign(&mut self, rhs: T) {
                $(self.$element += rhs);+
            }
        }
        impl<T: Number> std::ops::SubAssign<T> for $vector<T>  {
            fn sub_assign(&mut self, rhs: T) {
                $(self.$element -= rhs);+
            }
        }
        impl<T: Number> std::ops::MulAssign<T> for $vector<T>  {
            fn mul_assign(&mut self, rhs: T) {
                $(self.$element *= rhs);+
            }
        }
        impl<T: Number> std::ops::DivAssign<T> for $vector<T>  {
            fn div_assign(&mut self, rhs: T) {
                $(self.$element /= rhs);+
            }
        }
        impl<T: Number> std::ops::RemAssign<T> for $vector<T>  {
            fn rem_assign(&mut self, rhs: T) {
                $(self.$element %= rhs);+
            }
        }
        impl<T: Number> std::cmp::PartialEq<T>  for $vector<T>  {
            fn eq(&self, other: &T) -> bool {
                true $(&& self.$element == *other)+
            }
            fn ne(&self, other: &T) -> bool {
                true $(&& self.$element == *other)+
            }
        }
        impl<T: Number> std::cmp::PartialEq<Self>  for $vector<T>  {
            fn eq(&self, other: &Self) -> bool {
                true $(&& self.$element == other.$element)+
            }
            fn ne(&self, other: &Self) -> bool {
                true $(&& self.$element == other.$element)+
            }
        }
    };
}
macro_rules! impl_scalar_ops {
    ($structure: ident, $typea: ident, $($element:tt),+) => {
        impl std::ops::Add<$structure<$typea>> for $typea {
            type Output = $structure<$typea>;
            fn add(self, rhs: $structure<$typea>) -> Self::Output {
                $structure::<$typea>::new($(rhs.$element + self),+)

            }
        }
        impl std::ops::Sub<$structure<$typea>> for $typea {
            type Output = $structure<$typea>;
            fn sub(self, rhs: $structure<$typea>) -> Self::Output {
                $structure::<$typea>::new($(rhs.$element - self),+)
            }
        }
        impl std::ops::Mul<$structure<$typea>> for $typea {
            type Output = $structure<$typea>;
            fn mul(self, rhs: $structure<$typea>) -> Self::Output {
                $structure::<$typea>::new($(rhs.$element * self),+)
            }
        }
        impl std::ops::Div<$structure<$typea>> for $typea {
            type Output = $structure<$typea>;
            fn div(self, rhs: $structure<$typea>) -> Self::Output {
                $structure::<$typea>::new($(rhs.$element / self),+)
            }
        }
    };
}
macro_rules! impl_all_scalar_ops {
    ($structure: ident, $($element:tt),+) => {
        impl_scalar_ops!($structure, u8, $($element),+);
        impl_scalar_ops!($structure, u16, $($element),+);
        impl_scalar_ops!($structure, u32, $($element),+);
        impl_scalar_ops!($structure, u64, $($element),+);
        impl_scalar_ops!($structure, i8, $($element),+);
        impl_scalar_ops!($structure, i16, $($element),+);
        impl_scalar_ops!($structure, i32, $($element),+);
        impl_scalar_ops!($structure, i64, $($element),+);
        impl_scalar_ops!($structure, f32, $($element),+);
        impl_scalar_ops!($structure, f64, $($element),+);
    };
}
impl_all_scalar_ops!(Vector2, x, y);
impl_all_scalar_ops!(Vector3, x, y, z);
impl_all_scalar_ops!(Vector4, x, y, z, w);
macro_rules! impl_vec {
    ($len_:expr, $($element:tt),+) => {
        fn dot(&self, other: &Self) -> Self::Scalar {
            T::zero() $(+ self.$element*other.$element)+
        }
        fn project(&self, other: &Self) -> Self where Self::Scalar: Float {
            let vector = self.normalize();
            let t = vector.dot(other);
            Self::new($(vector.$element * t),+)
        }
        fn normalize(&self) -> Self where Self::Scalar: Float {
            let magnitude = self.length();
            Self::new($(self.$element / magnitude),+)
        }
        fn get_unchecked(&self, idx: usize) -> Self::Scalar { [$(self.$element),+][idx] }
        fn get(&self, idx: usize) -> Option<Self::Scalar> { 
            let arr = [$(self.$element),+];
            if idx > arr.len() {
                    None 
                } else { 
                    Some(arr[idx]) 
                }
        }
        fn len(&self) -> usize { $len_ }
        fn min(&self, other: &Self) -> Self {
            Self::new(
                $(
                    if self.$element < other.$element {
                        self.$element
                    } else {
                        other.$element
                    }
                ),+
            )
        }
        fn max(&self, other: &Self) -> Self {
            Self::new(
                $(
                    if self.$element > other.$element {
                        self.$element
                    } else {
                        other.$element
                    }
                ),+
            )
        }
    }
}

impl<T: Number> Bounded for Vector2<T> {
    fn min_value() -> Self {
        Self::from(<Self as Vector>::Scalar::min_value())
    }
    fn max_value() -> Self {
        Self::from(<Self as Vector>::Scalar::max_value())
    }
}
impl<T: Number> Bounded for Vector3<T> {
    fn min_value() -> Self {
        Self::from(<Self as Vector>::Scalar::min_value())
    }
    fn max_value() -> Self {
        Self::from(<Self as Vector>::Scalar::max_value())
    }
}
impl<T: Number> Bounded for Vector4<T> {
    fn min_value() -> Self {
        Self::from(<Self as Vector>::Scalar::min_value())
    }
    fn max_value() -> Self {
        Self::from(<Self as Vector>::Scalar::max_value())
    }
}

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector2<T> {
    pub x: T,
    pub y: T,
}
impl<T> Vector2<T> {
    pub const fn new(x: T, y: T) -> Self { Self { x, y } }
}
impl<T: Sized + Number> Vector2<T> {
    #[cfg(feature="rand")]
    pub fn random(range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform {
        use rand::Rng;
        Self::new(rand::thread_rng().gen_range(range.clone()), rand::thread_rng().gen_range(range.clone()))
    }
    #[cfg(feature="rand_pcg")]
    pub fn pseudo_random<P>(pcg: &mut P, range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform,
        P: rand::RngCore {
        use rand::Rng;
        Self::new(pcg.gen_range(range.clone()), pcg.gen_range(range.clone()))
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
    pub fn right() -> Self {
        Self::new(T::one(), T::zero())
    }
    pub fn left() -> Self 
        where T: std::ops::Neg<Output = T> {
        Self::new(-T::one(), T::zero())
    }
    pub fn top() -> Self {
        Self::new(T::zero(), T::one())
    }
    pub fn bottom() -> Self 
        where T: std::ops::Neg<Output = T> {
        Self::new(T::zero(), -T::one())
    }
    pub fn abs(&self) -> Self 
        where T: Signed {
        Self::new(self.x.abs(), self.y.abs())
    }
    pub fn cos(&self)-> T 
        where T: RationalNumber {
        self.normalize().dot(&Self::right())
    }
    pub fn sin(&self)-> T 
        where T: RationalNumber {
        T::from_f64(std::f64::consts::PI.div(2.0)).unwrap() - self.cos()
    }
    pub fn tan(&self)-> T 
        where T: RationalNumber {
        let normalize = self.normalize();
        normalize.y.div(normalize.x)
    }
    pub fn angle(&self) -> T 
        where T: RationalNumber {
        self.cos().acos()
    }
}

impl<T: Sized + Number> Vector for Vector2<T> { impl_vec!(2, x, y); type Scalar = T; }
impl_ops!(Vector2, x, y);

impl<T: Number> CrossProduct for Vector2<T> {
    fn cross(&self, other: &Self) -> Self::Product {
        (self.x * other.y) - (self.y * other.x)
    }
    type Product = T;
}

impl<'a, T: 'a> IntoRadialCoordinate<'a> for Vector2<T> 
    where T: RationalNumber {
    type Radial = PolarCoordinate<T>;
}

#[cfg(not(feature = "glsl"))]
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
}
#[cfg(feature = "glsl")]
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector3<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    padding: T,
}
impl<T> Vector3<T> {
    #[cfg(not(feature = "glsl"))]
    pub const fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
}
impl<T: Number> Vector3<T> {
    #[cfg(feature = "glsl")]
    pub const fn new(x: T, y: T, z: T) -> Self { Self { x, y, z, padding: T::ZERO } }
    pub fn abs(&self) -> Self 
        where T: Signed {
            
        Self::new(self.x.abs(), self.y.abs(), self.z.abs())
    }
    pub fn xy(&self) -> Vector2<T> {
        Vector2::new(self.x, self.y)
    }
    pub fn yx(&self) -> Vector2<T> {
        Vector2::new(self.y, self.x)
    }
    #[cfg(feature="rand")]
    pub fn random(range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform {
        use rand::Rng;
        Self::new(rand::thread_rng().gen_range(range.clone()), rand::thread_rng().gen_range(range.clone()), rand::thread_rng().gen_range(range))
    }
    #[cfg(feature="rand_pcg")]
    pub fn pseudo_random<P>(pcg: &mut P, range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform,
        P: rand::RngCore {
        use rand::Rng;
        Self::new(pcg.gen_range(range.clone()), pcg.gen_range(range.clone()), pcg.gen_range(range.clone()))
    }
}
impl<T: Number> CrossProduct for Vector3<T> {
    type Product = Self;
    
    fn cross(&self, other: &Self) -> Self::Product {
        Self::new(
            (self.y * other.z) - (self.z * other.y),
            (self.z * other.x) - (self.x * other.z),
            (self.x * other.y) - (self.y * other.x),
        )
    }
}

impl<T: Sized + Number> Vector for Vector3<T> { 
    impl_vec!(3, x, y, z);
    type Scalar = T;
}
impl_ops!(Vector3, x, y, z);

impl<'a, T: 'a> IntoRadialCoordinate<'a> for Vector3<T> 
    where T: RationalNumber {
    type Radial = SphericalCoordinate<T>;
}
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
impl<T> Vector4<T>  {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w } }
}
impl<T: Sized + Number> Vector4<T> {
    pub fn abs(&self) -> Self 
        where T: Signed {
        Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
    }
    #[cfg(feature="rand")]
    pub fn random(range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform {
        use rand::Rng;
        Self::new(rand::thread_rng().gen_range(range.clone()), rand::thread_rng().gen_range(range.clone()), rand::thread_rng().gen_range(range.clone()), rand::thread_rng().gen_range(range.clone()))
    }
    #[cfg(feature="rand_pcg")]
    pub fn pseudo_random<P>(pcg: &mut P, range: std::ops::Range<T>) -> Self 
        where T: rand::distributions::uniform::SampleUniform,
        P: rand::RngCore {
        use rand::Rng;
        Self::new(pcg.gen_range(range.clone()), pcg.gen_range(range.clone()), pcg.gen_range(range.clone()), pcg.gen_range(range.clone()))
    }
}

impl<T: Sized + Number> Vector for Vector4<T> { impl_vec!(4, x, y, z, w); type Scalar = T; }
impl_ops!(Vector4, x, y, z, w);


impl<T: Number> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value }
    }
}
impl<T: Number> From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        #[cfg(not(feature = "glsl"))]
        return Self { x: value, y: value, z: value };
        #[cfg(feature = "glsl")]
        return Self {x: value, y: value, z: value, padding: T::zero() };
    }
}
impl<T: Number> From<T> for Vector4<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value, z: value, w: value }
    }
}
impl<T: Number> From<Vector2<T>> for Vector3<T> {
    fn from(value: Vector2<T>) -> Self {
        #[cfg(not(feature = "glsl"))]
        return Self { x: value.x, y: value.y, z: T::one() };
        #[cfg(feature = "glsl")]
        return Self {x: value.x, y: value.y, z: T::zero(), padding: T::zero() };
    }
}
impl<T: Number> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: T::zero() }
    }
}
impl<T: Number> From<Vector4<T>> for Vector3<T> {
    fn from(value: Vector4<T>) -> Self {
        #[cfg(not(feature = "glsl"))]
        return Self { x: value.x, y: value.y, z: value.z };
        #[cfg(feature = "glsl")]
        return Self { x: value.x, y: value.y, z: value.z, padding: T::zero() };
    }
}
impl<T: Number> From<Vector2<T>> for Vector4<T>  {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x, value.y, T::zero(), T::zero())
    }
}
impl<T: RationalNumber> Rotation<T> for Vector3<T>  {
    fn quaternion(&self) -> Quaternion<T> { Quaternion::from_euler(*self) }
    fn euler(&self) -> Vector3<T> { *self }
}

impl<T: Debug> Debug for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl<T: Debug> Debug for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}
impl<T: Debug> Debug for Vector4<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .field("w", &self.w)
            .finish()
    }
}
macro_rules! impl_fromvec2 {
    ($typea:ident, $typeb:ident) => {
        impl From<Vector2<$typea>> for Vector2<$typeb> {
            fn from(value: Vector2<$typea>) -> Self {
                Self { x: value.x as $typeb, y: value.y as $typeb }
            }
        }
    };
}
macro_rules! impl_fromvec3 {
    ($typea:ident, $typeb:ident) => {
        impl From<Vector3<$typea>> for Vector3<$typeb> {
            fn from(value: Vector3<$typea>) -> Self {
                Self::new(value.x as $typeb, value.y as $typeb, value.z as $typeb)
            }
        }
    };
}
macro_rules! impl_fromvec4 {
    ($typea:ident, $typeb:ident) => {
        impl From<Vector4<$typea>> for Vector4<$typeb> {
            fn from(value: Vector4<$typea>) -> Self {
                Self { x: value.x as $typeb, y: value.y as $typeb, z: value.z as $typeb, w: value.w as $typeb }
            }
        }
    };
}

macro_rules! impl_all_from {
    ($mac:ident, $typea:ident, $($typeb:ident),+) => {
        $(
            $mac!($typea, $typeb);
        )+
    };
}
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}
macro_rules! impl_zero {
    ($($element:tt),+) => {
        fn is_zero(&self) -> bool {
            true $(&& self.$element.is_zero())+
        }
        fn set_zero(&mut self) {
            *self = Self::zero();
        }
        fn zero() -> Self {
            // $(replace_expr!($element T::zero()););
            Self::new($(replace_expr!($element T::zero())),+) 
        }
    };
}
macro_rules! impl_all_from_vec {
    ($mac:ident) => {
        impl_all_from!($mac, f32, f64, i8, i16, i32, i64, u8, u16, u32, u64, usize);
        impl_all_from!($mac, f64, f32, i8, i16, i32, i64, u8, u16, u32, u64, usize);

        impl_all_from!($mac, i8, i16, i32, i64, u8, u16, u32, u64, usize, f32, f64);
        impl_all_from!($mac, i16, i32, i64, u8, u16, u32, u64, usize, f64, f32, i8);

        impl_all_from!($mac, i32, i64, u8, u16, u32, u64, usize, f32, f64, i8, i16);
        impl_all_from!($mac, i64, u8, u16, u32, u64, usize, f64, f32, i8, i16, i32);

        impl_all_from!($mac, u8, u16, u32, u64, usize, f32, f64, i8, i16, i32, i64);
        impl_all_from!($mac, u16, u32, u64, usize, f64, f32, i8, i16, i32, i64, u8);

        impl_all_from!($mac, u32, u64, usize, f32, f64, i8, i16, i32, i64, u8, u16);
        impl_all_from!($mac, u64, usize, f64, f32, i8, i16, i32, i64, u8, u16, u32);
        
        impl_all_from!($mac, usize, u64, f64, f32, i8, i16, i32, i64, u8, u16, u32);
    };
}
impl_all_from_vec!(impl_fromvec2);
impl_all_from_vec!(impl_fromvec3);
impl_all_from_vec!(impl_fromvec4);

impl<T: Zero + Number> Zero for Vector2<T> { impl_zero!(x, y); }
impl<T: Zero + Number> Zero for Vector3<T> { impl_zero!(x, y, z); }
impl<T: Zero + Number> Zero for Vector4<T> { impl_zero!(x, y, z, w); }
impl<T: Sized + Number + std::ops::Neg<Output = T>> Orthogonality for Vector2<T> {
    fn get_orthogonal(&self, polarity: bool) -> Self {
        if polarity {
            Self::new(self.x, -self.y)
        } else {
            Self::new(-self.x, self.y)
        }
    }
}

impl<T: Sized + RationalNumber> Orthonormality for Vector2<T> {
    fn get_orthonormal(&self, polarity: bool, allow_zero: bool) -> Self {
        let len = self.length();
        if len.is_zero() {
            if polarity { Self::new(T::zero(), if allow_zero { T::zero() } else { T::one() }) } else { Self::new(T::zero(), if allow_zero { T::zero() } else { -T::one() }) }
        } else {
            if polarity { Self::new(-self.y/len, self.x/len) } else { Self::new(self.y/len, -self.x/len) }
        }
    }
}


impl<T: One + Number> num_traits::One for Vector2<T> {
    fn is_one(&self) -> bool
        where
            Self: PartialEq, {
        self.x == T::one() && self.y == T::one()
    }
    fn one() -> Self {
        Self::new(T::one(), T::one())
    }
    fn set_one(&mut self) {
        *self = Self::one()
    }
}
impl<T: One + Number> num_traits::One for Vector3<T> {
    fn is_one(&self) -> bool
        where
            Self: PartialEq, {
        self.x == T::one() && self.y == T::one() && self.z== T::one()
    }
    fn one() -> Self {
        Self::new(T::one(), T::one(), T::one())
    }
    fn set_one(&mut self) {
        *self = Self::one()
    }
}
impl<T: One + Number> num_traits::One for Vector4<T> {
    fn is_one(&self) -> bool
        where
            Self: PartialEq, {
        self.x == T::one() && self.y == T::one() && self.z== T::one() && self.w == T::one()
    }
    fn one() -> Self {
        Self::new(T::one(), T::one(), T::one(), T::one())
    }
    fn set_one(&mut self) {
        *self = Self::one()
    }
}