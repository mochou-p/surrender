// mochou-p/surrender/src/framebuffer/mod.rs

mod image;

use std::num::NonZero;
use crate::Color;


pub struct Framebuffer {
    pub width:  NonZero<usize>,
    pub height: NonZero<usize>,
    pub buffer: Box<[u8]>
}

impl Framebuffer {
    pub fn new(width: usize, height: usize, color: Color) -> crate::Result<Self> {
        let Some(width ) = NonZero::<usize>::new(width ) else { return Err(crate::Error:: WidthIsZero); };
        let Some(height) = NonZero::<usize>::new(height) else { return Err(crate::Error::HeightIsZero); };

        let pixel_count  = width.get() * height.get();
        let pixels       = unsafe {
            color.channels
                .iter()
                .cycle()
                .take(pixel_count * crate::CHANNEL_COUNT)
                .cloned()
                .collect::<Vec<u8>>()
        };

        Ok(Self { width, height, buffer: pixels.into_boxed_slice() })
    }
}

