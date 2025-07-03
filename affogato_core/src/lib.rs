pub mod cmath;


#[cfg(test)]
mod test {
    pub use crate::cmath::*;

    #[test]
    fn is_cmath_loaded() {
        acos(0.0);
        acosh(0.0);
        asin(0.0);
        asinh(0.0);
        atan(0.0);
        atan2(0.0, 1.0);
        atanh(0.0);
        cbrt(0.0);
        ceil(0.0);
        cos(0.0);
        erf(0.0);
        exp(0.0);
        exp2(0.0);
        expm1(0.0);
        floor(0.0);
        hypot(0.0, 1.0);
        log(0.0);
        log1p(0.0);
        log10(0.0);
        log2(0.0);
        pow(0.0, 1.0);
        round(0.0);
        sin(0.0);
        sinh(0.0);
        sqrt(0.0);
        tan(0.0);
        tanh(0.0);
        trunc(0.0);
        
        acosf(0.0);
        acoshf(0.0);
        asinf(0.0);
        asinhf(0.0);
        atanf(0.0);
        atan2f(0.0, 1.0);
        atanhf(0.0);
        cbrtf(0.0);
        ceilf(0.0);
        cosf(0.0);
        erff(0.0);
        expf(0.0);
        exp2f(0.0);
        expm1f(0.0);
        floorf(0.0);
        hypotf(0.0, 1.0);
        logf(0.0);
        log1pf(0.0);
        log10f(0.0);
        log2f(0.0);
        powf(0.0, 1.0);
        roundf(0.0);
        sinf(0.0);
        sinhf(0.0);
        sqrtf(0.0);
        tanf(0.0);
        tanhf(0.0);
        truncf(0.0);
    }
}