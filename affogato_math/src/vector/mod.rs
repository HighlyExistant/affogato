mod types;
mod polar;
mod vec2;
mod vec3;
mod vec4;
mod vec5;
use core::ops::{Div, Index, IndexMut, Neg, Sub};

use affogato_core::{num::{Bounds, FloatingPoint, Number, One, Signed, UniversalOperationsOn, Zero}, sets::Real};
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
                    Self::new(value.x as $typeb, value.y as $typeb)
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
                    Self::new(value.x as $typeb, value.y as $typeb, value.z as $typeb, value.w as $typeb)
                }
            }
        };
    }
    macro_rules! impl_fromvec5 {
        ($typea:ident, $typeb:ident) => {
            impl From<Vector5<$typea>> for Vector5<$typeb> {
                fn from(value: Vector5<$typea>) -> Self {
                    Self::new(value.x as $typeb, value.y as $typeb, value.z as $typeb, value.w as $typeb, value.a as $typeb)
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
    pub(crate) use impl_fromvec5;
    pub(crate) use impl_all_from;
    pub(crate) use impl_all_from_vec;
}

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

pub fn project<V: VectorSpace + InnerProduct>(vector: &V, project_onto: &V) -> V {
    use core::ops::{Mul, Div};
    project_onto.mul(vector.dot(project_onto).div(project_onto.length_squared()))
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