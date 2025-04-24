use crate::Real;


pub trait SignedDistance<T> {
    type Distance;
    fn sdf(&self, object: &T) -> Self::Distance;
}

pub fn smin_exp<T: Real>(a: T, b: T, k: T) -> T {
    let r = T::exp2(-a/k) + T::exp2(-b/k);
    return -k*T::log2(r);
}