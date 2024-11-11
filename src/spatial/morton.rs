use std::{fmt::Binary, hash::Hash, ops::Deref};

use crate::linear::{UI16Vec2, UI8Vec2, UIVec2};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MortonU16(u16);
impl Hash for MortonU16 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u16(self.0);
    }
}

impl Deref for MortonU16 {
    type Target = u16;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl Binary for MortonU16 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl From<UI8Vec2> for MortonU16{
    fn from(value: UI8Vec2) -> Self {
        Self::encode_xy(value.x, value.y)
    }
}
impl MortonU16 {
    pub fn encode_xy(x: u8, y: u8) -> Self {
        let mut res=x as u32|((y as u32) << 16);
        res=(res|(res<<4))&0x0f0f0f0f;
        res=(res|(res<<2))&0x33333333;
        res=(res|(res<<1))&0x55555555;
        Self((res|(res>>15)) as u16)
    }
    pub fn decode_xy(&self) -> UI8Vec2 {
        let mut res = (self.0 as u32|(self.0 as u32)<<15)&0x55555555;
        res=(res|(res>>1))&0x33333333;
        res=(res|(res>>2))&0x0f0f0f0f;
        res=res|(res>>4);
        let x = (res&0xff) as u8;
        let y = ((res>>16)&0xff) as u8;
        UI8Vec2::new(x, y)
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MortonU32(u32);
impl Hash for MortonU32 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u32(self.0);
    }
}
impl Deref for MortonU32 {
    type Target = u32;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Binary for MortonU32 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl MortonU32 {
    pub fn encode_xy(x: u16, y: u16) -> Self {
        let mut res =(x as u64)|((y as u64)<<32);
        res=(res|(res<<8))&0x00ff00ff00ff00ff;
        res=(res|(res<<4))&0x0f0f0f0f0f0f0f0f;
        res=(res|(res<<2))&0x3333333333333333;
        res=(res|(res<<1))&0x5555555555555555;
        Self((res as u32)|((res as u32)>>31))
    }
    pub fn decode_xy(&self) -> UI16Vec2 {
        let mut res = (self.0 as u64|(self.0 as u64)<<31)&0x5555555555555555;
        res=(res|(res>>1))&0x3333333333333333;
        res=(res|(res>>2))&0x0f0f0f0f0f0f0f0f;
        res=(res|(res>>4))&0x00ff00ff00ff00ff;
        res=res|(res>>8);
        let x = (res&0xffff) as u16;
        let y = ((res>>32)&0xffff) as u16;
        UI16Vec2::new(x, y)
    }
}
impl From<UI16Vec2> for MortonU32 {
    fn from(value: UI16Vec2) -> Self {
        Self::encode_xy(value.x, value.y)
    }
}
#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MortonU64(u64);
impl Hash for MortonU64 {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        state.write_u64(self.0);
    }
}
impl Deref for MortonU64 {
    type Target = u64;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl Binary for MortonU64 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.0.fmt(f)
    }
}

impl MortonU64 {
    #[cfg(target_arch="x86_64")]
    pub fn encode_xy(x: u32, y: u32) -> Self {
        use std::arch::x86_64::_pdep_u64;
        let val = unsafe { _pdep_u64(x as u64, 0x55555555) | _pdep_u64(y as u64,0xaaaaaaaa) };
        Self(val)
    }
    #[cfg(target_arch="x86_64")]
    pub fn decode_xy(&self) -> UIVec2 {
        use std::arch::x86_64::_pext_u64;
        let (x, y) = unsafe { (_pext_u64(self.0, 0x5555555555555555) as u32, _pext_u64(self.0, 0xaaaaaaaaaaaaaaaa) as u32) };
        UIVec2::new(x, y)
    }
}