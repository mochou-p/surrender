// mochou-p/surrender/src/color/mod.rs

#[repr(C)]
pub union Color {
    // NOTE: endianness :grimacing:
    pub channels: [u8; crate::CHANNEL_COUNT],
    pub packed:   u32
}

impl Color {
    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Self { channels: [r, g, b, a] }
    }
}

