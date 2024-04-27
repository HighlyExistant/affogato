#![allow(unused)]
use core::fmt;
use std::ops::Add;

use num_traits::{AsPrimitive, Bounded, Float, Zero};

use crate::{algebra::Quaternion, FloatingPoint, Number, Rotation, SignedNumber};

use super::FVec2;
pub trait CrossProduct {
    fn cross(&self, other: &Self) -> Self::Product;
    type Product;
}
pub trait Vector {
    fn length(&self) -> Self::Scalar where Self::Scalar: Float { self.dot(self).sqrt() }
    fn distance(&self, other: &Self) -> Self::Scalar where Self::Scalar: Float { self.dot(other).sqrt() }
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
pub trait Orthogonality: Vector {
    /// Gets a vector whos dot dot product with the current vector equals 0.
    /// The polarity means where the sign of the vector will be. true will make y negative
    /// and false will make x negative
    fn get_orthogonal(&self, polarity: bool) -> Self;
}
pub trait Orthonormality: Vector 
    where Self::Scalar: FloatingPoint {
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
        
        impl<T: SignedNumber> std::ops::Neg for $vector<T> {
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
pub struct Vector2<T: Sized + Number> {
    pub x: T,
    pub y: T,
}

impl<T: Sized + Number> Vector2<T> {
    pub fn new(x: T, y: T) -> Self { Self { x, y } }
}

impl<T: Sized + Number> Vector for Vector2<T> { impl_vec!(2, x, y); type Scalar = T; }
impl_ops!(Vector2, x, y);

impl<T: Number> CrossProduct for Vector2<T> {
    fn cross(&self, other: &Self) -> Self::Product {
        (self.x * other.y) - (self.y * other.x)
    }
    type Product = T;
}

#[cfg(not(feature = "glsl"))]
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
}
#[cfg(feature = "glsl")]
#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    padding: T,
}
impl<T: Number> Vector3<T> {
    #[cfg(not(feature = "glsl"))]
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z } }
    #[cfg(feature = "glsl")]
    pub fn new(x: T, y: T, z: T) -> Self { Self { x, y, z, padding: T::zero() } }
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

#[repr(C)]
#[derive(Default, Clone, Copy)]
pub struct Vector4<T: Sized + Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}

impl<T: Sized + Number> Vector4<T> {
    pub fn new(x: T, y: T, z: T, w: T) -> Self { Self { x, y, z, w } }
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
        return Self {x: value.x, y: value.y, z: T::one(), padding: T::zero() };
    }
}
impl<T: Number> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: T::one() }
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

impl<T: FloatingPoint> Rotation<T> for Vector3<T> 
    where f32: AsPrimitive<T>,
    f64: AsPrimitive<T> {
    fn quaternion(&self) -> Quaternion<T> { Quaternion::from_euler(*self) }
    fn euler(&self) -> Vector3<T> { *self }
}

impl<T: fmt::Debug + Number> fmt::Debug for Vector2<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .finish()
    }
}
impl<T: fmt::Debug + Number> fmt::Debug for Vector3<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Vector3")
            .field("x", &self.x)
            .field("y", &self.y)
            .field("z", &self.z)
            .finish()
    }
}
impl<T: fmt::Debug + Number> fmt::Debug for Vector4<T> {
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
macro_rules! impl_zero {
    ($($element:tt),+) => {
        fn is_zero(&self) -> bool {
            true $(&& self.$element.is_zero())+
        }
        fn set_zero(&mut self) {
            *self = Self::zero();
        }
        fn zero() -> Self {
            Self {
                $($element: T::zero()),+
            }
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
impl<T: Sized + SignedNumber> Orthogonality for Vector2<T> {
    fn get_orthogonal(&self, polarity: bool) -> Self {
        if polarity {
            Self::new(self.x, -self.y)
        } else {
            Self::new(-self.x, self.y)
        }
    }
}

impl<T: Sized + FloatingPoint> Orthonormality for Vector2<T> {
    fn get_orthonormal(&self, polarity: bool, allow_zero: bool) -> Self {
        let len = self.length();
        if len.is_zero() {
            if polarity { Self::new(T::zero(), if allow_zero { T::zero() } else { T::one() }) } else { Self::new(T::zero(), if allow_zero { T::zero() } else { -T::one() }) }
        } else {
            if polarity { Self::new(-self.y/len, self.x/len) } else { Self::new(self.y/len, -self.x/len) }
        }
    }
}
