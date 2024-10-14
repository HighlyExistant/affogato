use std::{fmt::Binary, ops::Deref};

use crate::linear::{UI16Vec2, UI8Vec2};

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct MortonU16(u16);

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