mod color;
mod image;
mod shape;
mod transfer;

pub use color::Color;


#[derive(Debug, Clone)]
pub struct Framebuffer<const WIDTH: usize, const HEIGHT: usize>(
    [[Color; WIDTH]; HEIGHT]
);

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Index<usize>
for Framebuffer<WIDTH, HEIGHT> {
    type Output = [Color; WIDTH];

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::IndexMut<usize>
for Framebuffer<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.0[index]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Index<[usize; 2]>
for Framebuffer<WIDTH, HEIGHT> {
    type Output = Color;

    fn index(&self, index: [usize; 2]) -> &Self::Output {
        &self.0[index[1]][index[0]]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Index<(usize, usize)>
for Framebuffer<WIDTH, HEIGHT> {
    type Output = Color;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::IndexMut<[usize; 2]>
for Framebuffer<WIDTH, HEIGHT> {
    fn index_mut(&mut self, index: [usize; 2]) -> &mut Self::Output {
        &mut self.0[index[1]][index[0]]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::IndexMut<(usize, usize)>
for Framebuffer<WIDTH, HEIGHT> {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
        &mut self.0[y][x]
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Framebuffer<WIDTH, HEIGHT> {
    pub fn new(c: Color) -> Self {
        Self([[c; WIDTH]; HEIGHT])
    }

    pub fn clear(&mut self, c: Color) {
        self.0.as_flattened_mut().fill(c);
    }
}

