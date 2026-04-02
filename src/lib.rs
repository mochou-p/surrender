// mochou-p/surrender/src/lib.rs

mod color;
mod framebuffer;

use std::fmt::{self, Display, Formatter};

pub use {color::Color, framebuffer::Framebuffer};


pub const CHANNEL_COUNT: usize = 4;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug)]
pub enum Error {
    WidthIsZero,
    HeightIsZero
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", match self {
            Self:: WidthIsZero =>  "width is not non-zero!",
            Self::HeightIsZero => "height is not non-zero!"
        })
    }
}

impl std::error::Error for Error {}

