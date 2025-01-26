use crate::{FromPrimitive, HasRealProduct, One, Real, UniversalOperationsOn, UsesArithmetic, Zero};
/// mixes two values together linearly using a t value between 0.0-1.0
pub fn lerp<V, T>(a: V, b: V, t: T) -> V 
    where V: UsesArithmetic + Copy + std::ops::Mul<T, Output = V> {
    a + (b-a)*t
}
/// mixes two values using smooth Hermite interpolation with a t value between 0.0-1.0. Retrieved from https://thebookofshaders.com/glossary/?search=smoothstep
pub fn smoothstep<V, T>(a: V, b: V, t: V) -> V 
    where V: UsesArithmetic + std::ops::Mul<T, Output = V> + Copy + Ord + Zero + One + From<T>,
        T: FromPrimitive {
    let t = ((t - a) / (b - a)).clamp(V::ZERO, V::ONE);
    return t * t * (V::from(T::from_f64(3.0)) - t*T::from_f64(2.0));
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