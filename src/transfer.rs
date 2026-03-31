use crate::{Framebuffer, Color};


impl<const WIDTH: usize, const HEIGHT: usize> Framebuffer<WIDTH, HEIGHT> {
    pub fn copy<const W: usize, const H: usize>(
        &self, x: usize, y: usize
    ) -> Framebuffer<W, H> {
        let mut framebuffer = Framebuffer::<W, H>::new(Color::BLACK);

        for row in 0..H {
            framebuffer.0[row].copy_from_slice(&self.0[row+y][x..x+W]);
        }

        framebuffer
    }

    pub fn paste<const W: usize, const H: usize>(
        &mut self, f: &Framebuffer<W, H>, x: usize, y: usize
    ) {
        for row in 0..H {
            self.0[row+y][x..x+W].copy_from_slice(&f.0[row]);
        }
    }
}

