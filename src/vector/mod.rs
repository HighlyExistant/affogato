use std::{fmt::Display, ops::{Div, Index, IndexMut, Neg}};
mod types;
mod polar;
pub use types::*;
pub use polar::*;
use crate::{FloatingPoint, HasNegatives, Number, One, Real, UniversalOperationsOn, Zero};
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
        
        // impl<T: Number + Display> Display for $vector<T> {
        //     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //         f.write_str(stringify!(<$($element),*>))
        //     }
        // }
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
macro_rules! impl_vec {
    ($len_:expr, $($element:tt),+) => {
        fn dot(&self, other: &Self) -> Self::Scalar {
            T::ZERO $(+ self.$element*other.$element)+
        }
        fn project(&self, other: &Self) -> Self where Self::Scalar: FloatingPoint {
            let vector = self.normalize();
            let t = vector.dot(other);
            Self::new($(vector.$element * t),+)
        }
        fn normalize(&self) -> Self where Self::Scalar: FloatingPoint {
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
        const ZERO: Self = Self::new($(replace_expr!($element T::ZERO)),+);
        fn is_zero(&self) -> bool {
            true $(&& self.$element.is_zero())+
        }
    };
}
macro_rules! impl_one {
    ($($element:tt),+) => {
        const ONE: Self = Self::new($(replace_expr!($element T::ONE)),+);
        fn is_one(&self) -> bool {
            true $(&& self.$element.is_one())+
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
pub trait Vector: UniversalOperationsOn<Self::Scalar> + UniversalOperationsOn<Self>{
    type Scalar: Number;
    fn length(&self) -> Self::Scalar where Self::Scalar: FloatingPoint { self.dot(self).sqrt() }
    fn distance(&self, other: &Self) -> Self::Scalar where Self::Scalar: FloatingPoint { self.dot(other).sqrt() }
    /// Direction gives a normalized vector
    /// that points to the given point.
    fn direction_to(&self, point: &Self) -> Self 
        where Self::Scalar: FloatingPoint,
        Self: std::ops::Sub<Output = Self> + Sized + Copy { 
            point.sub(*self).normalize()
    }
    fn dot(&self, other: &Self) -> Self::Scalar;
    fn project(&self, other: &Self) -> Self where Self::Scalar: FloatingPoint;
    fn len(&self) -> usize;
    fn normalize(&self) -> Self where Self::Scalar: FloatingPoint;
    fn get(&self, idx: usize) -> Option<Self::Scalar>;
    fn get_unchecked(&self, idx: usize) -> Self::Scalar;
    fn min(&self, other: &Self) -> Self;
    fn max(&self, other: &Self) -> Self;
}
pub trait CrossProduct: Vector {
    fn cross(&self, other: &Self) -> Self::Product;
    type Product;
}
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vector2<T: Number> {
    pub x: T,
    pub y: T,
}
impl<T: Number> Index<usize> for Vector2<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { std::mem::transmute::<&Self, &[T; 2]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector2<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { std::mem::transmute::<&mut Self, &mut [T; 2]>(self) };
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
    pub fn right() -> Self {
        Self::new(T::ONE, T::ZERO)
    }
    pub fn left() -> Self 
        where T: std::ops::Neg<Output = T> {
        Self::new(-T::ONE, T::ZERO)
    }
    pub fn top() -> Self {
        Self::new(T::ZERO, T::ONE)
    }
    pub fn bottom() -> Self 
        where T: std::ops::Neg<Output = T> {
        Self::new(T::ZERO, -T::ONE)
    }
    pub fn abs(&self) -> Self 
        where T: HasNegatives {
        Self::new(self.x.abs(), self.y.abs())
    }
    pub fn cos(&self)-> T 
        where T: Real {
        self.normalize().dot(&Self::right())
    }
    pub fn sin(&self)-> T 
        where T: Real {
        T::from_f64(std::f64::consts::PI.div(2.0)) - self.cos()
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
}
impl<T: Number> CrossProduct for Vector2<T> {
    fn cross(&self, other: &Self) -> Self::Product {
        (self.x * other.y) - (self.y * other.x)
    }
    type Product = T;
}
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vector3<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    padding: T,
}
impl<T: Number> Index<usize> for Vector3<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { std::mem::transmute::<&Self, &[T; 3]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector3<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { std::mem::transmute::<&mut Self, &mut [T; 3]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Vector3<T> {
    pub const fn new(x: T, y: T, z: T) -> Self {
        Self { x, y, z, padding: T::ZERO }
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
#[repr(C)]
#[derive(Default, Clone, Copy, Debug)]
pub struct Vector4<T: Number> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T,
}
impl<T: Number> Index<usize> for Vector4<T> {
    type Output = T;
    fn index(&self, index: usize) -> &Self::Output {
        let val = unsafe { std::mem::transmute::<&Self, &[T; 4]>(self) };
        &val[index]
    }
}
impl<T: Number> IndexMut<usize> for Vector4<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let val = unsafe { std::mem::transmute::<&mut Self, &mut [T; 4]>(self) };
        &mut val[index]
    }
}
impl<T: Number> Vector4<T> {
    pub const fn new(x: T, y: T, z: T, w: T) -> Self {
        Self { x, y, z, w }
    }
}

impl<T: Number> Vector for Vector2<T> { 
    type Scalar = T;
    impl_vec!(2,x, y); 
}
impl<T: Number> Vector for Vector3<T> { 
    type Scalar = T;
    impl_vec!(3,x, y, z); 
}
impl<T: Number> Vector for Vector4<T> { 
    type Scalar = T;
    impl_vec!(4,x, y, z, w); 
}
impl<T: Number + Display> Display for Vector2<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("<{}, {}>", self.x, self.y).as_str())
    }
}
impl<T: Number + Display> Display for Vector3<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("<{}, {}, {}>", self.x, self.y, self.z).as_str())
    }
}
impl<T: Number + Display> Display for Vector4<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("<{}, {}, {}, {}>", self.x, self.y, self.z, self.w).as_str())
    }
}
impl<T: Zero + Number> Zero for Vector2<T> { impl_zero!(x, y); }
impl<T: Zero + Number> Zero for Vector3<T> { impl_zero!(x, y, z); }
impl<T: Zero + Number> Zero for Vector4<T> { impl_zero!(x, y, z, w); }

impl<T: One + Number> One for Vector2<T> { impl_one!(x, y); }
impl<T: One + Number> One for Vector3<T> { impl_one!(x, y, z); }
impl<T: One + Number> One for Vector4<T> { impl_one!(x, y, z, w); }

impl<T: Number> From<T> for Vector2<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value }
    }
}
impl<T: Number> From<T> for Vector3<T> {
    fn from(value: T) -> Self {
        Self::new(value, value, value)
    }
}
impl<T: Number> From<T> for Vector4<T> {
    fn from(value: T) -> Self {
        Self { x: value, y: value, z: value, w: value }
    }
}
impl<T: Number> From<Vector2<T>> for Vector3<T> {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x, value.y, T::ONE)
    }
}
impl<T: Number> From<Vector3<T>> for Vector4<T> {
    fn from(value: Vector3<T>) -> Self {
        Self { x: value.x, y: value.y, z: value.z, w: T::ONE }
    }
}
impl<T: Number> From<Vector4<T>> for Vector3<T> {
    fn from(value: Vector4<T>) -> Self {
        Self::new(value.x, value.y, value.z)
    }
}
impl<T: Number> From<Vector2<T>> for Vector4<T>  {
    fn from(value: Vector2<T>) -> Self {
        Self::new(value.x, value.y, T::ZERO, T::ONE)
    }
}
impl_ops!(Vector2,x,y);
impl_ops!(Vector3,x,y,z);
impl_ops!(Vector4,x,y,z,w);
impl_all_scalar_ops!(Vector2,x ,y);
impl_all_scalar_ops!(Vector3,x ,y, z);
impl_all_scalar_ops!(Vector4,x ,y, z, w);
impl_all_from_vec!(impl_fromvec2);
impl_all_from_vec!(impl_fromvec3);
impl_all_from_vec!(impl_fromvec4);