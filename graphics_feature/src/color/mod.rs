use affogato_math::{lerp, vector::{DVec3, FVec3}};

#[repr(C)]
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Default, Hash, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8
}
impl Rgb {
    #[inline]
    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
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
}
impl From<FVec3> for Rgb {
    fn from(value: FVec3) -> Self {
        Self {
            r: lerp(0.0, 255.0, value.x()).clamp(0.0, 255.0) as u8,
            g: lerp(0.0, 255.0, value.y()).clamp(0.0, 255.0) as u8,
            b: lerp(0.0, 255.0, value.z()).clamp(0.0, 255.0) as u8,
        }
    }
}
impl From<DVec3> for Rgb {
    fn from(value: DVec3) -> Self {
        Self {
            r: lerp(0.0, 255.0, value.x()).clamp(0.0, 255.0) as u8,
            g: lerp(0.0, 255.0, value.y()).clamp(0.0, 255.0) as u8,
            b: lerp(0.0, 255.0, value.z()).clamp(0.0, 255.0) as u8,
        }
    }
}