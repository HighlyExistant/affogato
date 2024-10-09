#[repr(transparent)]
pub struct MortonU16(u16);

impl MortonU16 {
    pub fn encode_xy(x: u8, y: u8) -> Self {
        let mut res=x as u32|((y as u32) << 16);
        res=(res|(res<<4))&0x0f0f0f0f;
        res=(res|(res<<2))&0x33333333;
        res=(res|(res<<1))&0x55555555;
        Self((res|(res>>15)) as u16)
    }
}

// uint morton_code(dvec4 xyz) {
//     xyz.x = min(max(xyz.x * RESOLUTION, 0.0), RESOLUTION - 1.0);
//     xyz.y = min(max(xyz.y * RESOLUTION, 0.0), RESOLUTION - 1.0);
//     xyz.z = min(max(xyz.z * RESOLUTION, 0.0), RESOLUTION - 1.0);
//     uint xx = expand_bits(uint(xyz.x));
//     uint yy = expand_bits(uint(xyz.y));
//     uint zz = expand_bits(uint(xyz.z));
//     return xx * 4 + yy * 2 + zz;
// }
#[repr(transparent)]
pub struct MortonU32(u32);

impl MortonU32 {
    pub fn from_f32_xy() {
        
    }
}