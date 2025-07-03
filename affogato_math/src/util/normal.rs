use crate::vector::{Vector, Vector2, Vector3, Vector4};
use affogato_core::num::{Zero, One};
macro_rules! impl_normalized_no_negatives {
    ($structure:tt, $($types:tt),*) => {
        $(
            impl IsNormalized for Vector2<$types> {
                fn normalized(&self) -> bool {
                    self.x() == 1 && self.y() == 0 ||
                    self.x() == 0 && self.y() == 1
                }
            }
            impl IsNormalized for Vector3<$types> {
                fn normalized(&self) -> bool {
                    self.x() == 1 && self.y() == 0 && self.z() == 0 ||
                    self.x() == 0 && self.y() == 1 && self.z() == 0 || 
                    self.x() == 0 && self.y() == 0 && self.z() == 1
                }
            }
            impl IsNormalized for Vector4<$types> {
                fn normalized(&self) -> bool {
                    self.x() == 1 && self.y() == 0 && self.z() == 0 && self.w() == 0 ||
                    self.x() == 0 && self.y() == 1 && self.z() == 0 && self.w() == 0 || 
                    self.x() == 0 && self.y() == 0 && self.z() == 1 && self.w() == 0 || 
                    self.x() == 0 && self.y() == 0 && self.z() == 0 && self.w() == 1
                }
            }
        )*
    };
}
macro_rules! impl_normalized_vec {
    ($structure:tt, $($types:tt),*) => {
        $(
            impl IsNormalized for Vector2<$types> {
                fn normalized(&self) -> bool {
                    let x = self.x().abs();
                    let y = self.y().abs();
                    x == $types::ONE && y == $types::ZERO ||
                    x == $types::ZERO && y == $types::ONE
                }
            }
            impl IsNormalized for Vector3<$types> {
                fn normalized(&self) -> bool {
                    let x = self.x().abs();
                    let y = self.y().abs();
                    let z = self.z().abs();
                    x == 1 && y == 0 && z == 0 ||
                    x == 0 && y == 1 && z == 0 || 
                    x == 0 && y == 0 && z == 1
                }
            }
            impl IsNormalized for Vector4<$types> {
                fn normalized(&self) -> bool {
                    let x = self.x().abs();
                    let y = self.y().abs();
                    let z = self.z().abs();
                    let w = self.w().abs();
                    x == 1 && y == 0 && z == 0 && w == 0 ||
                    x == 0 && y == 1 && z == 0 && w == 0 || 
                    x == 0 && y == 0 && z == 1 && w == 0 || 
                    x == 0 && y == 0 && z == 0 && w == 1
                }
            }
        )*
    };
}
macro_rules! impl_normalized_fvec {
    ($structure:tt, $($types:tt),*) => {
        impl IsNormalized for Vector2<f32> {
            fn normalized(&self) -> bool {
                self.length() == 1.0
            }
        }
        impl IsNormalized for Vector3<f32> {
            fn normalized(&self) -> bool {
                self.length() == 1.0
            }
        }
        impl IsNormalized for Vector4<f32> {
            fn normalized(&self) -> bool {
                self.length() == 1.0
            }
        }
    };
}
pub trait IsNormalized {
    fn normalized(&self) -> bool;
}
impl_normalized_no_negatives!(u8, u16, u32, u64, usize);
impl_normalized_vec!(i8, i16, i32, i64, isize);
impl_normalized_fvec!(f32, f64);