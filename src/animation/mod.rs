use crate::{HasRealProduct, Real, UsesArithmetic};
/// mixes two values together linearly using a t value between 0.0-1.0
pub fn lerp<V: HasRealProduct<T, V> + UsesArithmetic + Copy, T: Real>(a: V, b: V, t: T) -> V {
    a + (b-a)*t
}
/// mixes two values using smooth Hermite interpolation with a t value between 0.0-1.0
pub fn smoothstep<V: HasRealProduct<T, V> + UsesArithmetic + Copy, T: Real>(a: V, b: V, t: T) -> V {
    (a - b * t) * (t * t)
}
/// returns a t value using a range `from` and `to` and a value
pub fn inverse_lerp<V: HasRealProduct<T, V> + UsesArithmetic + Copy, T: Real>(from: V, to: V, value: T) -> V 
    where T: std::ops::Sub<V, Output = V> {
    (value - from) / (to - from)
}
/// Maps one value into another using interpolation. internally this is an [`inverse_lerp`] and [`lerp`] combined.
pub fn remap<V: HasRealProduct<T, V> + UsesArithmetic + Copy + Real, T: Real>(imin: V, imax: V, omin: V, omax: V, t: T) -> V 
    where T: std::ops::Sub<V, Output = V> {
    let t2 = inverse_lerp(imin, imax, t);
    lerp::<V, V>(omin, omax, t2)
}
/// interpolate value by using 4 values with weights in accordance to a square.
/// ```
/// q12─────────q22
///  │           │
///  │ (x, y)    │
///  │           │
///  │           │
/// q11─────────q21
/// ```
pub fn bilinear_interpolation<T: Real + Copy>(q12: T, q22: T, q11: T, q21: T, x: T, y: T) -> T  {
    let r1 = lerp(q12, q22, x);
    let r2 = lerp(q11, q21, x);
    lerp(r1, r2, y)
}