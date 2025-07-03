use affogato_math::{lerp, vector::{DVec3, DVec4, FVec3, FVec4}};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8
}
impl Rgba {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { r, g, b, a }
    }
    #[inline]
    pub const fn r(&self) -> u8 {
        self.r
    }
    #[inline]
    pub const fn g(&self) -> u8 {
        self.g
    }
    #[inline]
    pub const fn b(&self) -> u8 {
        self.b
    }
    #[inline]
    pub const fn a(&self) -> u8 {
        self.a
    }
}
impl From<FVec4> for Rgba {
    fn from(value: FVec4) -> Self {
        Self {
            r: lerp(0.0, 255.0, value.x()).clamp(0.0, 255.0) as u8,
            g: lerp(0.0, 255.0, value.y()).clamp(0.0, 255.0) as u8,
            b: lerp(0.0, 255.0, value.z()).clamp(0.0, 255.0) as u8,
            a: lerp(0.0, 255.0, value.w()).clamp(0.0, 255.0) as u8,
        }
    }
}
impl From<DVec4> for Rgba {
    fn from(value: DVec4) -> Self {
        Self {
            r: lerp(0.0, 255.0, value.x()).clamp(0.0, 255.0) as u8,
            g: lerp(0.0, 255.0, value.y()).clamp(0.0, 255.0) as u8,
            b: lerp(0.0, 255.0, value.z()).clamp(0.0, 255.0) as u8,
            a: lerp(0.0, 255.0, value.w()).clamp(0.0, 255.0) as u8,
        }
    }
}