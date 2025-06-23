use core::{fmt::Display, num::{NonZeroU128, NonZeroU16, NonZeroU32, NonZeroU64, NonZeroU8, NonZeroUsize}};

use super::One;

pub trait Natural {}

const NATURAL_ZERO_ERROR: &'static str = "Natural numbers cannot be zero";
#[cfg(feature="alloc")]
extern crate alloc;
macro_rules! impl_natural {
    ($($structure:tt, $value:tt, $raw:tt),*) => {
        $(
            #[repr(transparent)]
            #[derive(Debug, Clone, Copy)]
            pub struct $structure($value);
            impl core::ops::Add for $structure {
                type Output = Self;
                fn add(self, rhs: Self) -> Self::Output {
                    Self(self.0.saturating_add(rhs.0.get()))
                }
            }
            impl core::ops::Sub for $structure {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self::Output {
                    Self($value::new(self.0.get().sub(rhs.0.get())).expect(NATURAL_ZERO_ERROR))
                }
            }
            impl core::ops::Mul for $structure {
                type Output = Self;
                fn mul(self, rhs: Self) -> Self::Output {
                    Self(self.0.saturating_mul(rhs.0))
                }
            }
            impl core::ops::Div for $structure {
                type Output = Self;
                fn div(self, rhs: Self) -> Self::Output {
                    Self($value::new(self.0.get().div(rhs.0.get())).expect(NATURAL_ZERO_ERROR))
                }
            }

            impl One for $structure {
                const ONE: Self = $structure::new(unsafe { $value::new_unchecked(1) });
                fn is_one(&self) -> bool {
                    self.0.get() == 1
                }
            }
            #[allow(dead_code)]
            impl $structure {
                pub fn abs_diff(&self, rhs: Self) -> Self {
                    Self($value::new(self.0.get().abs_diff(rhs.0.get())).expect(NATURAL_ZERO_ERROR))
                }
                pub const fn new(value: $value) -> Self {
                    Self(value)
                }
            }
            impl From<$raw> for $structure {
                fn from(value: $raw) -> Self {
                    Self($value::new(value).expect(NATURAL_ZERO_ERROR))
                }
            }
            impl Natural for $structure {}
            
            #[cfg(feature="alloc")]
            impl Display for $structure {
                fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                    f.write_str(alloc::format!("{}", self.0.get()).as_str())
                }
            }
        )*
    };
}
impl PartialEq for NaturalU16 {
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}
impl PartialOrd for NaturalU16 {
    fn ge(&self, other: &Self) -> bool {
        self.0.ge(&other.0)
    }
    fn gt(&self, other: &Self) -> bool {
        self.0.gt(&other.0)
    }
    fn le(&self, other: &Self) -> bool {
        self.0.le(&other.0)
    }
    fn lt(&self, other: &Self) -> bool {
        self.0.lt(&other.0)
    }
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        if other.gt(other) {
            Some(core::cmp::Ordering::Greater)
        } else if other.lt(other) {
            Some(core::cmp::Ordering::Less)
        } else {
            Some(core::cmp::Ordering::Equal)
        }
    }
}

impl_natural!(NaturalU8, NonZeroU8, u8, NaturalU16, NonZeroU16, u16, NaturalU32, NonZeroU32, u32, NaturalU64, NonZeroU64, u64, NaturalU128, NonZeroU128, u128, NaturalUsize, NonZeroUsize, usize);

