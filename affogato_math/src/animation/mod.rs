use affogato_core::{num::{FromPrimitive, One, UniversalOperationsOn, Zero}, sets::Real};

/// mixes two values together linearly using a t value between 0.0-1.0
/// # Example
/// ``` no_run,ignore
/// let a = 10.0;
/// let b = 20.0;
/// assert!(lerp(a, b, 0.5) == 15.0);
/// assert!(lerp(a, b, 0.75) == 17.5);
/// ```
/// this also works with vectors, and every type which implements
/// [`core::ops::Sub`] and can be multiplied by [t].
/// ``` no_run,ignore
/// let a = FVec2::new(10.0, 20.0);
/// let b = FVec2::new(30.0, 7.0);
/// assert!(lerp(a, b, 0.5) == FVec2::new(20.0, 13.5))
/// ```
pub fn lerp<V, T>(a: V, b: V, t: T) -> V 
    where V: UniversalOperationsOn<V> + Copy + core::ops::Mul<T, Output = V> {
    a + (b-a)*t
}
/// mixes two values using smooth Hermite interpolation and returns a t value between 0.0-1.0. Retrieved from [the book of shaders](https://thebookofshaders.com/glossary/?search=smoothstep)
/// # Example
/// by using edge0 = 0.0 and edge1 = 1.0, it will just do the classic ease and out you would expect.
/// ``` no_run,ignore
/// // Using the image crate:
/// let mut image = image::ImageBuffer::<Rgb<u8>, Vec<_>>::new(512, 512);
/// for x in 0..image.width() {
///     let value = smoothstep(0.0, 1.0, inverse_lerp(0.0, (image.width()-1) as f32, x as f32));
///     println!("{value}");
///     image.put_pixel(x, lerp(0.0, (image.height()-1) as f32, value as f32) as u32, Rgb([255; 3]));
/// }
/// image.save("save3.png").unwrap();
/// ```
/// # Panics
/// * Results of smoothstep are undefined if edge0 >= edge1
pub fn smoothstep<V>(edge0: V, edge1: V, x: V) -> V 
    where V: UniversalOperationsOn<V> + core::ops::Mul<V, Output = V> + Copy + Zero + One + PartialOrd + FromPrimitive {
    assert!(!(edge0 >= edge1), "results of smoothstep are undefined if `edge0 >= edge1`");
    let mut t = ((x - edge0) / (edge1 - edge0));
    if t < V::ZERO {
        t = V::ZERO;
    } else if t > V::ONE {
        t = V::ONE;
    }
    return t * t * (V::from_f64(3.0) - t*V::from_f64(2.0));
}
/// returns a t value using a range `from` and `to` and a value
pub fn inverse_lerp<V: UniversalOperationsOn<V> + core::ops::Mul<T> + Copy, T: Real>(from: V, to: V, value: T) -> V 
    where T: core::ops::Sub<V, Output = V> {
    (value - from) / (to - from)
}
/// Maps one value into another using interpolation. internally this is an [`inverse_lerp`] and [`lerp`] combined.
pub fn remap<V: core::ops::Mul<T> + UniversalOperationsOn<V> + Copy + Real, T: Real>(imin: V, imax: V, omin: V, omax: V, t: T) -> V 
    where T: core::ops::Sub<V, Output = V> {
    let t2 = inverse_lerp(imin, imax, t);
    lerp::<V, V>(omin, omax, t2)
}
/// interpolate value by using 4 values with weights in accordance to a square.
/// ```no_run,ignore
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

#[cfg(test)]
mod tests {
    use crate::{lerp, smoothstep, vector::FVec2};

    #[test]
    fn test_lerp() {
        let a = 10.0;
        let b = 20.0;
        assert!(lerp(a, b, 0.5) == 15.0);
        assert!(lerp(a, b, 0.75) == 17.5);
        let a = FVec2::new(10.0, 20.0);
        let b = FVec2::new(30.0, 7.0);
        assert!(lerp(a, b, 0.5) == FVec2::new(20.0, 13.5))
    }
}