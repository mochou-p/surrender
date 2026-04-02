// mochou-p/surrender/src/framebuffer/image.rs

use std::convert::AsRef;
use std::fs::File;
use std::io::{self, BufWriter, Write as _};
use std::path::Path;
use crate::Framebuffer;


impl Framebuffer {
    pub fn write_as_pam(&self, path: impl AsRef<Path>) -> io::Result<()> {
        let     file   = File::create(path)?;
        let mut writer = BufWriter::new(file);

        write!(
            writer,
            "P7\nWIDTH {}\nHEIGHT {}\nDEPTH {}\nMAXVAL {}\nTUPLTYPE RGB_ALPHA\nENDHDR\n",
            self.width,
            self.height,
            crate::CHANNEL_COUNT,
            u8::MAX
        )?;

        writer.write_all(&self.buffer)?;
        writer.flush()
    }
}

