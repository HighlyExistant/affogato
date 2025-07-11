mod types;
mod polar;
mod vec2;
mod vec3;
mod vec4;
use core::ops::{Div, Index, IndexMut, Neg, Sub};

use affogato_core::{groups::vector_spaces::{InnerProduct, NormedVectorSpace, VectorSpace}, num::{Bounds, FloatingPoint, Number, One, Signed, UniversalOperationsOn, Zero}, sets::Real};
#[cfg(feature="serde")]
use serde::{Serialize, Deserialize};

use bytemuck::{Pod, Zeroable};

pub use types::*;
pub use polar::*;
pub use vec2::Vector2;
pub use vec3::Vector3;
pub use vec4::Vector4;
pub(crate) mod impl_macros {
    macro_rules! impl_ops {
        ($vector:ident, $($element:tt),+) => {
            impl<T: Number> core::ops::Add for $vector <T>  {
                fn add(self, rhs: Self) -> Self::Output {
                    Self::new($(self.$element + rhs.$element),+)
                }
                type Output = Self;
            } 
            impl<T: Number> core::ops::Rem for $vector<T>  {
                fn rem(self, rhs: Self) -> Self::Output {
                    Self::new($(self.$element % rhs.$element),+)
                }
                type Output = Self;
            }
            
            impl<T: Number + core::ops::Neg<Output = T>> core::ops::Neg for $vector<T> {
                fn neg(self) -> Self::Output {
                    Self::new($(-self.$element),+)
                }
                type Output = Self;
            }
            
            impl<T: Number> core::ops::Sub for $vector<T>  {
                fn sub(self, rhs: Self) -> Self::Output {
                    Self::new($(self.$element - rhs.$element),+)
                }
                type Output = Self;
            }
            
            impl<T: Number> core::ops::Mul for $vector<T>  {
                fn mul(self, rhs: Self) -> Self::Output {
                    Self::new($(self.$element * rhs.$element),+)
                }
                type Output = Self;
            }
            
            impl<T: Number> core::ops::Div for $vector<T>  {
                fn div(self, rhs: Self) -> Self::Output {
                    Self::new($(self.$element / rhs.$element),+)
                }
                type Output = Self;
            }

            // Operations on scalar values
            impl<T: Number> core::ops::Add<T> for $vector<T>  {
                fn add(self, rhs: T) -> Self::Output {
                    Self::new($(self.$element + rhs),+)
                }
                type Output = Self;
            }
            impl<T: Number> core::ops::Sub<T> for $vector<T>  {
                fn sub(self, rhs: T) -> Self::Output {
                    Self::new($(self.$element - rhs),+)
                }
                type Output = Self;
            }
            impl<T: Number> core::ops::Mul<T> for $vector<T>  {
                fn mul(self, rhs: T) -> Self::Output {
                    Self::new($(self.$element * rhs),+)
                }
                type Output = Self;
            }
            impl<T: Number> core::ops::Div<T> for $vector<T>  {
                fn div(self, rhs: T) -> Self::Output {
                    Self::new($(self.$element / rhs),+)
                }
                type Output = Self;
            }
            impl<T: Number> core::ops::Rem<T> for $vector<T>  {
                fn rem(self, rhs: T) -> Self::Output {
                    Self::new($(self.$element % rhs),+)
                }
                type Output = Self;
            }
            impl<T: Number> core::ops::AddAssign for $vector<T>  {
                fn add_assign(&mut self, rhs: Self) {
                    $(self.$element += rhs.$element);+
                }
            }
            impl<T: Number> core::ops::SubAssign for $vector<T>  {
                fn sub_assign(&mut self, rhs: Self) {
                    $(self.$element -= rhs.$element);+
                }
            }
            impl<T: Number> core::ops::MulAssign for $vector<T>  {
                fn mul_assign(&mut self, rhs: Self) {
                    $(self.$element *= rhs.$element);+
                }
            }
            impl<T: Number> core::ops::DivAssign for $vector<T>  {
                fn div_assign(&mut self, rhs: Self) {
                    $(self.$element /= rhs.$element);+
                }
            }
            impl<T: Number> core::ops::RemAssign for $vector<T>  {
                fn rem_assign(&mut self, rhs: Self) {
                    $(self.$element %= rhs.$element);+
                }
            }
            impl<T: Number> core::ops::AddAssign<T> for $vector<T>  {
                fn add_assign(&mut self, rhs: T) {
                    $(self.$element += rhs);+
                }
            }
            impl<T: Number> core::ops::SubAssign<T> for $vector<T>  {
                fn sub_assign(&mut self, rhs: T) {
                    $(self.$element -= rhs);+
                }
            }
            impl<T: Number> core::ops::MulAssign<T> for $vector<T>  {
                fn mul_assign(&mut self, rhs: T) {
                    $(self.$element *= rhs);+
                }
            }
            impl<T: Number> core::ops::DivAssign<T> for $vector<T>  {
                fn div_assign(&mut self, rhs: T) {
                    $(self.$element /= rhs);+
                }
            }
            impl<T: Number> core::ops::RemAssign<T> for $vector<T>  {
                fn rem_assign(&mut self, rhs: T) {
                    $(self.$element %= rhs);+
                }
            }
            impl<T: Number> core::cmp::PartialEq<T>  for $vector<T>  {
                fn eq(&self, other: &T) -> bool {
                    true $(&& self.$element == *other)+
                }
                fn ne(&self, other: &T) -> bool {
                    true $(&& self.$element == *other)+
                }
            }
            impl<T: Number> core::cmp::PartialEq<Self>  for $vector<T>  {
                fn eq(&self, other: &Self) -> bool {
                    true $(&& self.$element == other.$element)+
                }
                fn ne(&self, other: &Self) -> bool {
                    true $(&& self.$element == other.$element)+
                }
            }
            impl<T: Number> core::cmp::Eq for $vector <T> {}
            
            unsafe impl<T: Number> Zeroable for $vector <T> {
                fn zeroed() -> Self {
                    Self::ZERO
                }
            }
            unsafe impl<T: Number + Pod> Pod for $vector <T> {}
            // impl<T: Number + Display> Display for $vector<T> {
            //     fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
            //         f.write_str(stringify!(<$($element),*>))
            //     }
            // }
        };
    }
    macro_rules! vector_permutations {
        ($ret:tt, $($x:tt),*) => {
            paste::paste! {
                pub fn  [<$($x)*>](&self) -> $ret <T>
                    where T: Real {
                    $ret::<T>::new($(self.$x),*)
                }
            }
        };
    }
    macro_rules! impl_scalar_ops {
        ($structure: ident, $typea: ident, $($element:tt),+) => {
            impl core::ops::Add<$structure<$typea>> for $typea {
                type Output = $structure<$typea>;
                fn add(self, rhs: $structure<$typea>) -> Self::Output {
                    $structure::<$typea>::new($(rhs.$element + self),+)

                }
            }
            impl core::ops::Sub<$structure<$typea>> for $typea {
                type Output = $structure<$typea>;
                fn sub(self, rhs: $structure<$typea>) -> Self::Output {
                    $structure::<$typea>::new($(rhs.$element - self),+)
                }
            }
            impl core::ops::Mul<$structure<$typea>> for $typea {
                type Output = $structure<$typea>;
                fn mul(self, rhs: $structure<$typea>) -> Self::Output {
                    $structure::<$typea>::new($(rhs.$element * self),+)
                }
            }
            impl core::ops::Div<$structure<$typea>> for $typea {
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
            impl_scalar_ops!($structure, u128, $($element),+);
            impl_scalar_ops!($structure, usize, $($element),+);
            impl_scalar_ops!($structure, i8, $($element),+);
            impl_scalar_ops!($structure, i16, $($element),+);
            impl_scalar_ops!($structure, i32, $($element),+);
            impl_scalar_ops!($structure, i64, $($element),+);
            impl_scalar_ops!($structure, i128, $($element),+);
            impl_scalar_ops!($structure, isize, $($element),+);
            impl_scalar_ops!($structure, f32, $($element),+);
            impl_scalar_ops!($structure, f64, $($element),+);
        };
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
    pub(crate) use impl_ops;
    pub(crate) use vector_permutations;
    pub(crate) use impl_scalar_ops;
    pub(crate) use impl_all_scalar_ops;
    pub(crate) use impl_fromvec2;
    pub(crate) use impl_fromvec3;
    pub(crate) use impl_fromvec4;
    pub(crate) use impl_all_from;
    pub(crate) use impl_all_from_vec;
}

// macro_rules! impl_vec {
//     ($len_:expr, $($element:tt),+) => {
//         fn dot(&self, other: &Self) -> Self::Scalar {
//             T::ZERO $(+ self.$element*other.$element)+
//         }
//         fn project(&self, other: &Self) -> Self where Self::Scalar: Real {
//             let vector = self.normalize();
//             let t = vector.dot(other);
//             Self::new($(vector.$element * t),+)
//         }
//         fn normalize(&self) -> Self where Self::Scalar: Real {
//             let magnitude = self.length();
//             Self::new($(self.$element / magnitude),+)
//         }
//         fn get_unchecked(&self, idx: usize) -> Self::Scalar { [$(self.$element),+][idx] }
//         fn get(&self, idx: usize) -> Option<Self::Scalar> { 
//             let arr = [$(self.$element),+];
//             if idx > arr.len() {
//                     None 
//                 } else { 
//                     Some(arr[idx]) 
//                 }
//         }
//         fn len(&self) -> usize { $len_ }
//         fn min(&self, other: &Self) -> Self {
//             Self::new(
//                 $(
//                     if self.$element < other.$element {
//                         self.$element
//                     } else {
//                         other.$element
//                     }
//                 ),+
//             )
//         }
//         fn max(&self, other: &Self) -> Self {
//             Self::new(
//                 $(
//                     if self.$element > other.$element {
//                         self.$element
//                     } else {
//                         other.$element
//                     }
//                 ),+
//             )
//         }
//     }
// }
// macro_rules! replace_expr {
//     ($_t:tt $sub:expr) => {$sub};
// }
// macro_rules! impl_zero {
//     ($($element:tt),+) => {
//         const ZERO: Self = Self::new($(replace_expr!($element T::ZERO)),+);
//         fn is_zero(&self) -> bool {
//             true $(&& self.$element.is_zero())+
//         }
//     };
// }
// macro_rules! impl_one {
//     ($($element:tt),+) => {
//         const ONE: Self = Self::new($(replace_expr!($element T::ONE)),+);
//         fn is_one(&self) -> bool {
//             true $(&& self.$element.is_one())+
//         }
//     };
// }

// use crate::{epsilon_eq, vector::impl_macros::{impl_all_from, impl_all_from_vec, impl_all_scalar_ops, impl_fromvec2, impl_fromvec3, impl_fromvec4, impl_ops, impl_scalar_ops, vector_permutations}};
// pub trait Vector 
//     where Self: 
//     UniversalOperationsOn<Self::Scalar> + UniversalOperationsOn<Self> +
//     Clone + Copy +
//     Zero {
//     type Scalar: Number;
//     fn length(&self) -> Self::Scalar where Self::Scalar: Real { self.length_squared().sqrt() }
//     #[inline]
//     fn length_squared(&self) -> Self::Scalar where Self::Scalar: Real { self.dot(self) }
//     fn distance(&self, other: &Self) -> Self::Scalar where Self::Scalar: Real { (self.clone()-other.clone()).length() }
//     /// Direction gives a normalized vector that points to the given point.
//     fn direction_to(&self, point: &Self) -> Self 
//         where Self::Scalar: Real,
//         Self: core::ops::Sub<Output = Self> + Sized { 
//             point.clone().sub(self.clone()).normalize()
//     }
//     fn point_at(&self, point: &Self, distance: Self::Scalar) -> Self 
//     where Self::Scalar: Real,
//     Self: core::ops::Sub<Output = Self> + Sized {
//         self.direction_to(point).mul(distance)+self.clone()
//     }
//     /// The dot product is a common linear algebra function which is defined as
//     /// the sum of the products of each respective scalar value in the vector.
//     /// # Properties of the Dot Product
//     /// * The dot product is commutative
//     /// * The angle between the two vectors is greater than 90 degrees if the dot product is negative
//     /// * The vectors are perpendicular if the dot product equals 0
//     /// * The dot product of two normalized vectors, returns the cosine of the angle between those vectors.
//     fn dot(&self, other: &Self) -> Self::Scalar;
//     fn project(&self, other: &Self) -> Self where Self::Scalar: Real;
//     /// the amount of scalar values this vector has.
//     fn len(&self) -> usize;
//     fn normalize(&self) -> Self where Self::Scalar: Real;
//     // retrieves a point inside the vector, checking whether it is out of bounds
//     fn get(&self, idx: usize) -> Option<Self::Scalar>;
//     // retrieves a point inside the vector
//     fn get_unchecked(&self, idx: usize) -> Self::Scalar;
//     fn min(&self, other: &Self) -> Self;
//     fn max(&self, other: &Self) -> Self;
// }
// pub trait OuterProduct: Vector {
//     fn cross(&self, other: &Self) -> Self::Product;
//     type Product;
// }
// #[repr(C)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
// #[derive(Default, Clone, Copy, Debug, Hash)]
// pub struct Vector2<T: Number> {
//     x: T,
//     y: T,
// }
// impl<T: Number> Index<usize> for Vector2<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         let val = unsafe { core::mem::transmute::<&Self, &[T; 2]>(self) };
//         &val[index]
//     }
// }
// impl<T: Number> IndexMut<usize> for Vector2<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 2]>(self) };
//         &mut val[index]
//     }
// }
// impl<T: Number> Vector2<T> {
//     pub const fn new(x: T, y: T) -> Self {
//         Self { x, y }
//     }
//     pub fn rotate_90(&self) -> Self 
//         where T: Neg<Output = T> {
//         Self::new(-self.y, self.x)
//     }
//     pub fn rotate_180(&self) -> Self 
//         where T: Neg<Output = T> {
//         Self::new(self.x, -self.y)
//     }
//     pub fn rotate_270(&self) -> Self 
//         where T: Neg<Output = T> {
//         Self::new(self.y, -self.x)
//     }
//     /// Returns a vector pointing to the right of the graph <1, 0>
//     pub fn right() -> Self {
//         Self::new(T::ONE, T::ZERO)
//     }
//     /// Returns a vector pointing to the left of the graph <-1, 0>
//     pub fn left() -> Self 
//         where T: core::ops::Neg<Output = T> {
//         Self::new(-T::ONE, T::ZERO)
//     }
//     /// Returns a vector pointing to the left of the graph <0, 1>
//     pub fn top() -> Self {
//         Self::new(T::ZERO, T::ONE)
//     }
//     /// Returns a vector pointing to the left of the graph <0, -1>
//     pub fn bottom() -> Self 
//         where T: core::ops::Neg<Output = T> {
//         Self::new(T::ZERO, -T::ONE)
//     }
//     pub fn abs(&self) -> Self 
//         where T: Signed {
//         Self::new(self.x.abs(), self.y.abs())
//     }
//     pub fn cos(&self)-> T 
//         where T: Real {
//         self.normalize().dot(&Self::right())
//     }
//     pub fn sin(&self)-> T 
//         where T: Real {
//         T::from_f64(core::f64::consts::PI.div(2.0)) - self.cos()
//     }
//     pub fn tan(&self)-> T 
//         where T: Real {
//         let normalize = self.normalize();
//         normalize.y.div(normalize.x)
//     }
//     pub fn angle(&self) -> T 
//         where T: Real {
//         self.cos().acos()
//     }
//     pub fn from_angle(angle: T) -> Self
//         where T: Real {
//         Self::new(angle.cos(), angle.sin())
//     }

//     #[inline]
//     pub fn x(&self) -> T {
//         self.x
//     }

//     #[inline]
//     pub const fn y(&self) -> T {
//         self.y
//     }

//     #[inline]
//     pub const fn set_x(&mut self, x: T) {
//         self.x = x;
//     }

//     #[inline]
//     pub fn set_y(&mut self, y: T) {
//         self.y = y;
//     }

//     vector_permutations!(Vector2, x, y);
//     vector_permutations!(Vector2, y, x);
//     #[cfg(feature="rand")]
//     pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
//         where T: rand::distr::uniform::SampleUniform {
//         Vector2::new(generator.random_range(range.clone()), generator.random_range(range.clone()))
//     }
//     pub fn epsilon_eq(&self, other: Self, epsilon: T) -> bool 
//         where T: Real {
//         epsilon_eq(self.x, other.x, epsilon) &&
//         epsilon_eq(self.y, other.y, epsilon)
//     }
// }
// impl<T: Number> OuterProduct for Vector2<T> {
//     /// In 2 dimensions there is no cross product as we understand it in 3d. Instead of returning
//     /// a vector, it returns a scalar value. The absolute value of this scalar represents the area 
//     /// of the parallelogram formed by the 2 vectors.
//     fn cross(&self, other: &Self) -> Self::Product {
//         (self.x * other.y) - (self.y * other.x)
//     }
//     type Product = T;
// }

// impl<T: Signed + Number> Signed for Vector2<T> {
//     fn abs(self) -> Self {
//         Self::new(self.x.abs(), self.y.abs())
//     }
//     fn flip_sign(self) -> Self {
//         Self::new(-self.x, -self.y)
//     }
//     fn is_negative(self) -> bool {
//         self.x.is_negative() &&
//         self.y.is_negative() 
//     }
//     fn is_positive(self) -> bool {
//         self.x.is_positive() &&
//         self.y.is_positive() 
//     }
// }
// #[repr(C)]
// #[cfg(feature="glsl")]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
// #[derive(Default, Clone, Copy, Debug, Hash)]
// pub struct Vector3<T: Number> {
//     x: T,
//     y: T,
//     z: T,
//     padding: T,
// }

// #[repr(C)]
// #[cfg(not(feature="glsl"))]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
// #[derive(Default, Clone, Copy, Debug, Hash)]
// pub struct Vector3<T: Number> {
//     x: T,
//     y: T,
//     z: T,
// }

// impl<T: Number> Index<usize> for Vector3<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         let val = unsafe { core::mem::transmute::<&Self, &[T; 3]>(self) };
//         &val[index]
//     }
// }
// impl<T: Number> IndexMut<usize> for Vector3<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 3]>(self) };
//         &mut val[index]
//     }
// }
// impl<T: Number> Vector3<T> {
//     #[cfg(feature="glsl")]
//     pub const fn new(x: T, y: T, z: T) -> Self {
//         Self { x, y, z, padding: T::ZERO }
//     }
//     #[cfg(not(feature="glsl"))]
//     pub const fn new(x: T, y: T, z: T) -> Self {
//         Self { x, y, z }
//     }
//     /// Returns a vector pointing to the right of the graph <1, 0, 0>
//     pub const fn right() -> Self {
//         Self::new(T::ONE, T::ZERO, T::ZERO)
//     }
//     /// Returns a vector pointing to the left of the graph <-1, 0, 0>
//     pub fn left() -> Self 
//         where T: core::ops::Neg<Output = T> {
//         Self::new(-T::ONE, T::ZERO, T::ZERO)
//     }
//     /// Returns a vector pointing to the top of the graph <0, 1, 0>
//     pub const fn top() -> Self {
//         Self::new(T::ZERO, T::ONE, T::ZERO)
//     }
//     /// Returns a vector pointing to the top of the graph <0, -1, 0>
//     pub fn bottom() -> Self 
//         where T: core::ops::Neg<Output = T> {
//         Self::new(T::ZERO, -T::ONE, T::ZERO)
//     }
//     /// Returns a vector pointing forward to the graph <0, 0, 1>
//     pub const fn forward() -> Self {
//         Self::new(T::ZERO, T::ZERO, T::ONE)
//     }
//     /// Returns a vector pointing backward to the graph <0, 0, -1>
//     pub fn backward() -> Self
//         where T: core::ops::Neg<Output = T> {
//         Self::new(T::ZERO, T::ZERO, -T::ONE)
//     }

//     #[inline]
//     pub const fn x(&self) -> T {
//         self.x
//     }

//     #[inline]
//     pub const fn y(&self) -> T {
//         self.y
//     }

//     #[inline]
//     pub const fn z(&self) -> T {
//         self.z
//     }

//     #[inline]
//     pub fn set_x(&mut self, x: T) {
//         self.x = x;
//     }

//     #[inline]
//     pub fn set_y(&mut self, y: T) {
//         self.y = y;
//     }

//     #[inline]
//     pub fn set_z(&mut self, z: T) {
//         self.z = z;
//     }

//     vector_permutations!(Vector2, x, y);
//     vector_permutations!(Vector2, y, x);
//     vector_permutations!(Vector2, x, z);
//     vector_permutations!(Vector2, z, x);
//     vector_permutations!(Vector2, y, z);
//     vector_permutations!(Vector2, z, y);
    
//     /// from: https://en.wikipedia.org/wiki/Distance_from_a_point_to_a_line
//     pub fn line_distance(&self, a: Self, b: Self) -> T 
//         where T: Real {
//         let dir_ba = b-a;
//         let dir_pa = *self-a;
//         dir_pa.cross(&dir_ba).length().div(dir_ba.length())
//     }
//     pub fn signed_plane_distance(&self, a: Self, b: Self, c: Self) -> T 
//         where T: Real {
//         let normal = b.sub(a).cross(&c.sub(a)).normalize();
//         normal.dot(&self.sub(a))
//     }
//     pub fn epsilon_eq(&self, p: Self, epsilon: T) -> bool 
//         where T: Real {
//         let p = (self.clone()-p).abs();
//         p.x <= epsilon &&
//         p.y <= epsilon &&
//         p.z <= epsilon 
//     }
//     #[cfg(feature="rand")]
//     pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
//         where T: rand::distr::uniform::SampleUniform {
//         Vector3::new(generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()))
//     }
// }
// impl<T: Signed + Number> Signed for Vector3<T> {
//     fn abs(self) -> Self {
//         Self::new(self.x.abs(), self.y.abs(), self.z.abs())
//     }
//     fn flip_sign(self) -> Self {
//         Self::new(-self.x, -self.y, -self.z)
//     }
//     fn is_negative(self) -> bool {
//         self.x.is_negative() &&
//         self.y.is_negative() &&
//         self.z.is_negative() 
//     }
//     fn is_positive(self) -> bool {
//         self.x.is_positive() &&
//         self.y.is_positive() &&
//         self.z.is_positive() 
//     }
// }
// impl<T: Number> OuterProduct for Vector3<T> {
//     type Product = Self;
//     /// The outer product, also known as the cross product, is used to find a vector 
//     /// perpendicular to 2 vectors. 
//     /// # Properties of the Cross Product
//     /// * finds a vector perpendicular to the 2 given vectors.
//     /// * If the vectors are collinear it will give you a 0 vector.
//     fn cross(&self, other: &Self) -> Self::Product {
//         Self::new(
//             (self.y * other.z) - (self.z * other.y),
//             (self.z * other.x) - (self.x * other.z),
//             (self.x * other.y) - (self.y * other.x),
//         )
//     }
// }
// #[repr(C)]
// #[cfg_attr(feature="serde", derive(Serialize, Deserialize))]
// #[derive(Default, Clone, Copy, Debug, Hash)]
// pub struct Vector4<T: Number> {
//     x: T,
//     y: T,
//     z: T,
//     w: T,
// }
// impl<T: Number> Index<usize> for Vector4<T> {
//     type Output = T;
//     fn index(&self, index: usize) -> &Self::Output {
//         let val = unsafe { core::mem::transmute::<&Self, &[T; 4]>(self) };
//         &val[index]
//     }
// }
// impl<T: Number> IndexMut<usize> for Vector4<T> {
//     fn index_mut(&mut self, index: usize) -> &mut Self::Output {
//         let val = unsafe { core::mem::transmute::<&mut Self, &mut [T; 4]>(self) };
//         &mut val[index]
//     }
// }
// impl<T: Number> Vector4<T> {
//     pub const fn new(x: T, y: T, z: T, w: T) -> Self {
//         Self { x, y, z, w }
//     }
    
//     pub fn xyz(&self) -> Vector3<T> {
//         Vector3::new(self.x, self.y, self.z)
//     }

//     #[inline]
//     pub const fn x(&self) -> T {
//         self.x
//     }

//     #[inline]
//     pub const fn y(&self) -> T {
//         self.y
//     }

//     #[inline]
//     pub const fn z(&self) -> T {
//         self.z
//     }

//     #[inline]
//     pub const fn w(&self) -> T {
//         self.w
//     }

//     #[inline]
//     pub fn set_x(&mut self, x: T) {
//         self.x = x;
//     }

//     #[inline]
//     pub fn set_y(&mut self, y: T) {
//         self.y = y;
//     }

//     #[inline]
//     pub fn set_z(&mut self, z: T) {
//         self.z = z;
//     }

//     #[inline]
//     pub fn set_w(&mut self, w: T) {
//         self.w = w;
//     }
//     pub fn epsilon_eq(&self, p: Self, epsilon: T) -> bool 
//         where T: Real {
//         let p = (self.clone()-p).abs();
//         p.x <= epsilon &&
//         p.y <= epsilon &&
//         p.z <= epsilon &&
//         p.w <= epsilon
//     }

//     #[cfg(feature="rand")]
//     pub fn random(generator: &mut impl rand::Rng, range: core::ops::Range<T>) -> Self 
//         where T: rand::distr::uniform::SampleUniform {
//         Vector4::new(generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()), generator.random_range(range.clone()))
//     }
// }

// impl<T: Signed + Number> Signed for Vector4<T> {
//     fn abs(self) -> Self {
//         Self::new(self.x.abs(), self.y.abs(), self.z.abs(), self.w.abs())
//     }
//     fn flip_sign(self) -> Self {
//         Self::new(-self.x, -self.y, -self.z, -self.w)
//     }
//     fn is_negative(self) -> bool {
//         self.x.is_negative() &&
//         self.y.is_negative() &&
//         self.z.is_negative() &&
//         self.w.is_negative() 
//     }
//     fn is_positive(self) -> bool {
//         self.x.is_positive() &&
//         self.y.is_positive() &&
//         self.z.is_positive() &&
//         self.w.is_positive() 
//     }
// }
// impl<T: Number> Vector for Vector2<T> { 
//     type Scalar = T;
//     impl_vec!(2,x, y); 
// }
// impl<T: Number> Vector for Vector3<T> { 
//     type Scalar = T;
//     impl_vec!(3,x, y, z); 
// }
// impl<T: Number> Vector for Vector4<T> { 
//     type Scalar = T;
//     impl_vec!(4,x, y, z, w); 
// }
// impl<T: Zero + Number> Zero for Vector2<T> { impl_zero!(x, y); }
// impl<T: Zero + Number> Zero for Vector3<T> { impl_zero!(x, y, z); }
// impl<T: Zero + Number> Zero for Vector4<T> { impl_zero!(x, y, z, w); }

// impl<T: One + Number> One for Vector2<T> { impl_one!(x, y); }
// impl<T: One + Number> One for Vector3<T> { impl_one!(x, y, z); }
// impl<T: One + Number> One for Vector4<T> { impl_one!(x, y, z, w); }

// impl<T: Number> From<T> for Vector2<T> {
//     fn from(value: T) -> Self {
//         Self { x: value, y: value }
//     }
// }
// impl<T: Number> From<T> for Vector3<T> {
//     fn from(value: T) -> Self {
//         Self::new(value, value, value)
//     }
// }
// impl<T: Number> From<T> for Vector4<T> {
//     fn from(value: T) -> Self {
//         Self { x: value, y: value, z: value, w: value }
//     }
// }
// impl<T: Number> From<Vector2<T>> for Vector3<T> {
//     fn from(value: Vector2<T>) -> Self {
//         Self::new(value.x, value.y, T::ONE)
//     }
// }
// impl<T: Number> From<Vector3<T>> for Vector4<T> {
//     fn from(value: Vector3<T>) -> Self {
//         Self { x: value.x, y: value.y, z: value.z, w: T::ONE }
//     }
// }
// impl<T: Number> From<Vector4<T>> for Vector3<T> {
//     fn from(value: Vector4<T>) -> Self {
//         Self::new(value.x, value.y, value.z)
//     }
// }
// impl<T: Number> From<Vector2<T>> for Vector4<T>  {
//     fn from(value: Vector2<T>) -> Self {
//         Self::new(value.x, value.y, T::ZERO, T::ONE)
//     }
// }

// impl<T: Number> From<(T, T)> for Vector2<T> {
//     fn from(value: (T, T)) -> Self {
//         Self::new(value.0, value.1)
//     }
// }
// impl<T: Number> From<(T, T, T)> for Vector3<T> {
//     fn from(value: (T, T, T)) -> Self {
//         Self::new(value.0, value.1, value.2)
//     }
// }
// impl<T: Number> From<(T, T, T, T)> for Vector4<T> {
//     fn from(value: (T, T, T, T)) -> Self {
//         Self::new(value.0, value.1, value.2, value.3)
//     }
// }
// impl<T: Number> From<[T; 2]> for Vector2<T> {
//     fn from(value: [T; 2]) -> Self {
//         Self::new(value[0], value[1])
//     }
// }
// impl<T: Number> From<[T; 3]> for Vector3<T> {
//     fn from(value: [T; 3]) -> Self {
//         Self::new(value[0], value[1], value[2])
//     }
// }
// impl<T: Number> From<[T; 4]> for Vector4<T> {
//     fn from(value: [T; 4]) -> Self {
//         Self::new(value[0], value[1], value[2], value[3])
//     }
// }

// impl<T: Number> From<Vector2<T>> for (T, T)  {
//     fn from(value: Vector2<T>) -> Self {
//         (value.x, value.y)
//     }
// }
// impl<T: Number> From<Vector3<T>> for (T, T, T)  {
//     fn from(value: Vector3<T>) -> Self {
//         (value.x, value.y, value.z)
//     }
// }
// impl<T: Number> From<Vector4<T>> for (T, T, T, T)  {
//     fn from(value: Vector4<T>) -> Self {
//         (value.x, value.y, value.z, value.w)
//     }
// }

// impl<T: Number> From<Vector2<T>> for [T; 2]  {
//     fn from(value: Vector2<T>) -> Self {
//         [value.x, value.y]
//     }
// }
// impl<T: Number> From<Vector3<T>> for [T; 3]  {
//     fn from(value: Vector3<T>) -> Self {
//         [value.x, value.y, value.z]
//     }
// }
// impl<T: Number> From<Vector4<T>> for [T; 4]  {
//     fn from(value: Vector4<T>) -> Self {
//         [value.x, value.y, value.z, value.w]
//     }
// }

/// calculates the direction of reflection of an incident vector, where `incident` is the incident vector and
/// `normal` is the normal of the surface it is reflecting on. Important to note that
/// both the incident and normal should be normalized vectors. The following snippet was
/// retrieved from https://thebookofshaders.com/glossary/?search=reflect
pub fn reflect<V: VectorSpace>(incident: &V, normal: &V) -> V 
    where V: InnerProduct, V::Scalar: Real {
    use affogato_core::num::FromPrimitive;
    incident.clone() - (normal.clone())*V::Scalar::from_f64(2.0)*incident.dot(normal)
}
/// calculates the refraction of an incident vector, where `incident` is the incident vector,
/// `normal` is the normal of the surface it is reflecting on and `eta` is the ratio of indices of
/// refraction. Important to note that both the incident and normal should be normalized vectors.
/// retrieved from https://raytracing.github.io/books/RayTracingInOneWeekend.html#dielectrics/refraction
pub fn refract<V: VectorSpace + Neg<Output=V> + Zero + One>(incident: &V, normal: &V, eta: V::Scalar) -> V 
    where V: InnerProduct + NormedVectorSpace, V::Scalar: Real {
    // commented code from https://thebookofshaders.com/glossary/?search=refract
    // let ni = normal.dot(incident);
    // let k = V::Scalar::ONE - eta*eta*(V::Scalar::ONE - ni*ni);
    // if k < V::Scalar::ZERO {
    //     V::ZERO
    // } else {
    //     (incident.clone()*eta)-(normal.clone()*(eta*ni + k.sqrt()))
    // }
    let cos_theta = (-incident.clone()).dot(normal).min(V::Scalar::ONE);
    let r_out_perp = (incident.clone()+(normal.clone()*cos_theta))*eta;
    let r_out_parallel = normal.clone()*(-(V::Scalar::ONE.sub(r_out_perp.length_squared()).abs()).sqrt());
    r_out_perp+r_out_parallel
}
// impl_ops!(Vector2,x,y);
// impl_ops!(Vector3,x,y,z);
// impl_ops!(Vector4,x,y,z,w);
// impl_all_scalar_ops!(Vector2,x ,y);
// impl_all_scalar_ops!(Vector3,x ,y, z);
// impl_all_scalar_ops!(Vector4,x ,y, z, w);
// impl_all_from_vec!(impl_fromvec2);
// impl_all_from_vec!(impl_fromvec3);
// impl_all_from_vec!(impl_fromvec4);
// Features
pub use affogato_core::groups::{vector_spaces::*};

#[cfg(feature = "alloc")]
mod alloc_feature {
    use core::fmt::Display;
    use affogato_core::num::Number;

    use crate::vector::{vec2::Vector2, vec3::Vector3, vec4::Vector4};

    extern crate alloc;
    impl<T: Number> From<Vector2<T>> for alloc::vec::Vec<T> {
        fn from(value: Vector2<T>) -> Self {
            alloc::vec![
                value.x(), value.y(), 
            ]
        }
    }
    impl<T: Number> From<Vector3<T>> for alloc::vec::Vec<T> {
        fn from(value: Vector3<T>) -> Self {
            alloc::vec![
                value.x(), value.y(), value.z(), 
            ]
        }
    }
    impl<T: Number> From<Vector4<T>> for alloc::vec::Vec<T> {
        fn from(value: Vector4<T>) -> Self {
            alloc::vec![
                value.x(), value.y(), value.z(), value.w(),
            ]
        }
    }
    impl<T: Number + Display> Display for Vector2<T> {
        fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
            f.write_str(alloc::format!("<{}, {}>", self.x(), self.y()).as_str())
        }
    }
    impl<T: Number + Display> Display for Vector3<T> {
        fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
            f.write_str(alloc::format!("<{}, {}, {}>", self.x(), self.y(), self.z()).as_str())
        }
    }
    impl<T: Number + Display> Display for Vector4<T> {
        fn fmt(&self, f: &mut alloc::fmt::Formatter<'_>) -> alloc::fmt::Result {
            f.write_str(alloc::format!("<{}, {}, {}, {}>", self.x(), self.y(), self.z(), self.w()).as_str())
        }
    }
}
// #[cfg(feature = "alloc")]
// pub use alloc_feature::*;

// mod serde_feature {
//     use serde::{ser::SerializeStruct, Serialize};

//     use crate::{vector::Vector2, Number};

    
//     impl<T: Number + Serialize> serde::Serialize for Vector2<T> {
//         fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
//             where
//                 S: serde::Serializer {
//             let mut serialize = serializer.serialize_struct("Vector2", 2)?;
//             serialize.serialize_field("x", &self.x)?;
//             serialize.serialize_field("y", &self.y)?;
//             serialize.end()
//         }
//     }
// }