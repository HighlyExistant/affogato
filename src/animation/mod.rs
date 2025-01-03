use crate::{HasRealProduct, Real, UsesArithmetic};

pub fn lerp<V: HasRealProduct<T, V> + UsesArithmetic + Copy, T: Real>(a: V, b: V, t: T) -> V {
    a + (b-a)*t
}
pub fn smoothstep<V: HasRealProduct<T, V> + UsesArithmetic + Copy, T: Real>(a: V, b: V, t: T) -> V {
    (a - b * t) * (t * t)
}
pub fn inverse_lerp<V: HasRealProduct<T, V> + UsesArithmetic + Copy, T: Real>(from: V, to: V, t: T) -> V 
    where T: std::ops::Sub<V, Output = V> {
    (t - from) / (to - from)
}
pub fn remap<V: HasRealProduct<T, V> + UsesArithmetic + Copy + Real, T: Real>(imin: V, imax: V, omin: V, omax: V, t: T) -> V 
    where T: std::ops::Sub<V, Output = V> {
    let t2 = inverse_lerp(imin, imax, t);
    lerp::<V, V>(omin, omax, t2)
}
pub fn bilinear_interpolation<T: Real + Copy>(q12: T, q22: T, q11: T, q21: T, x: T, y: T) -> T  {
    let r1 = lerp(q12, q22, x);
    let r2 = lerp(q11, q21, x);
    lerp(r1, r2, y)
}