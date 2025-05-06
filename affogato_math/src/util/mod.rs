mod normal;
pub mod sdf;

pub use normal::*;

use crate::Real;

pub fn smin_exp<T: Real>(a: T, b: T, k: T) -> T {
    let r = T::exp2(-a/k) + T::exp2(-b/k);
    return -k*T::log2(r);
}