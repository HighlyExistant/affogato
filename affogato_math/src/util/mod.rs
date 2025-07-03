mod normal;
pub mod sdf;

use affogato_core::sets::Real;
pub use normal::*;

pub fn smin_exp<T: Real>(a: T, b: T, k: T) -> T {
    let r = T::exp2(-a/k) + T::exp2(-b/k);
    return -k*T::log2(r);
}

pub fn epsilon_eq<T: Real>(a: T, b: T, epsilon: T) -> bool {
    if a == b {
        true
    } else {
        (a-b).abs() < epsilon
    }
}