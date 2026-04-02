// mochou-p/surrender/examples/main.rs

use surrender::{Framebuffer, Color};


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let fb = Framebuffer::new(20, 10, Color::rgba(255, 127, 0, 100))?;

    fb.write_as_pam("image.pam")?;

    Ok(())
}

