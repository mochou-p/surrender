use crate::Framebuffer;


#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8
}

unsafe impl Send for Color {}
unsafe impl Sync for Color {}

impl Default for Color {
    fn default() -> Self {
        Self::BLACK
    }
}

impl From<[u8; 3]> for Color {
    fn from(value: [u8; 3]) -> Self {
        Self { r: value[0], g: value[1], b: value[2] }
    }
}

impl From<Color> for [u8; 3] {
    fn from(value: Color) -> Self {
        [value.r, value.g, value.b]
    }
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b }
    }
}

impl From<Color> for (u8, u8, u8) {
    fn from(value: Color) -> Self {
        (value.r, value.g, value.b)
    }
}

impl std::ops::Index<usize> for Color {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.r,
            1 => &self.g,
            2 => &self.b,
            _ => panic!("cannot index Color with index > 2")
        }
    }
}

impl std::ops::IndexMut<usize> for Color {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        match index {
            0 => &mut self.r,
            1 => &mut self.g,
            2 => &mut self.b,
            _ => panic!("cannot index Color with index > 2")
        }
    }
}

impl std::ops::Add for Color {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.r = self.r.saturating_add(rhs.r);
        self.g = self.g.saturating_add(rhs.g);
        self.b = self.b.saturating_add(rhs.b);
        self
    }
}

impl std::ops::Sub for Color {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.r = self.r.saturating_sub(rhs.r);
        self.g = self.g.saturating_sub(rhs.g);
        self.b = self.b.saturating_sub(rhs.b);
        self
    }
}

impl std::ops::AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        *self = *self + rhs;
    }
}

impl std::ops::SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        *self = *self - rhs;
    }
}

impl std::ops::Neg for Color {
    type Output = Self;

    fn neg(self) -> Self {
        Color::WHITE - self
    }
}

impl std::ops::Not for Color {
    type Output = Self;

    fn not(self) -> Self {
        -self
    }
}

impl Color {
    pub const       BLACK:   Self = Self { r:   0, g:   0, b:   0 };
    pub const LIGHT_BLACK:   Self = Self { r:  43, g:  43, b:  43 };

    pub const  DARK_GRAY:    Self = Self { r:  85, g:  85, b:  85 };
    pub const       GRAY:    Self = Self { r: 128, g: 128, b: 128 };
    pub const LIGHT_GRAY:    Self = Self { r: 170, g: 170, b: 170 };

    pub const  DARK_WHITE:   Self = Self { r: 213, g: 213, b: 213 };
    pub const       WHITE:   Self = Self { r: 255, g: 255, b: 255 };

    pub const  DARK_RED:     Self = Self { r: 128, g:   0, b:   0 };
    pub const       RED:     Self = Self { r: 255, g:   0, b:   0 };
    pub const LIGHT_RED:     Self = Self { r: 255, g: 128, b: 128 };

    pub const  DARK_ORANGE:  Self = Self { r: 128, g:  64, b:   0 };
    pub const       ORANGE:  Self = Self { r: 255, g: 128, b:   0 };
    pub const LIGHT_ORANGE:  Self = Self { r: 255, g: 191, b: 128 };

    pub const  DARK_YELLOW:  Self = Self { r: 128, g: 128, b:   0 };
    pub const       YELLOW:  Self = Self { r: 255, g: 255, b:   0 };
    pub const LIGHT_YELLOW:  Self = Self { r: 255, g: 255, b: 128 };

    pub const  DARK_LIME:    Self = Self { r:  64, g: 128, b:   0 };
    pub const       LIME:    Self = Self { r: 128, g: 255, b:   0 };
    pub const LIGHT_LIME:    Self = Self { r: 191, g: 255, b: 128 };

    pub const  DARK_GREEN:   Self = Self { r:   0, g: 128, b:   0 };
    pub const       GREEN:   Self = Self { r:   0, g: 255, b:   0 };
    pub const LIGHT_GREEN:   Self = Self { r: 128, g: 255, b: 128 };

    pub const  DARK_TEAL:    Self = Self { r:   0, g: 128, b:  64 };
    pub const       TEAL:    Self = Self { r:   0, g: 255, b: 128 };
    pub const LIGHT_TEAL:    Self = Self { r: 128, g: 255, b: 191 };

    pub const  DARK_CYAN:    Self = Self { r:   0, g: 128, b: 128 };
    pub const       CYAN:    Self = Self { r:   0, g: 255, b: 255 };
    pub const LIGHT_CYAN:    Self = Self { r: 128, g: 255, b: 255 };

    pub const  DARK_SKY:     Self = Self { r:   0, g:  64, b: 128 };
    pub const       SKY:     Self = Self { r:   0, g: 128, b: 255 };
    pub const LIGHT_SKY:     Self = Self { r: 128, g: 191, b: 255 };

    pub const  DARK_BLUE:    Self = Self { r:   0, g:   0, b: 128 };
    pub const       BLUE:    Self = Self { r:   0, g:   0, b: 255 };
    pub const LIGHT_BLUE:    Self = Self { r: 128, g: 128, b: 255 };

    pub const  DARK_PURPLE:  Self = Self { r:  64, g:   0, b: 128 };
    pub const       PURPLE:  Self = Self { r: 128, g:   0, b: 255 };
    pub const LIGHT_PURPLE:  Self = Self { r: 191, g: 128, b: 255 };

    pub const  DARK_MAGENTA: Self = Self { r: 128, g:   0, b: 128 };
    pub const       MAGENTA: Self = Self { r: 255, g:   0, b: 255 };
    pub const LIGHT_MAGENTA: Self = Self { r: 255, g: 128, b: 255 };

    pub const  DARK_CANDY:   Self = Self { r: 128, g:   0, b:  64 };
    pub const       CANDY:   Self = Self { r: 255, g:   0, b: 128 };
    pub const LIGHT_CANDY:   Self = Self { r: 255, g: 128, b: 191 };

    pub fn darker(self) -> Self {
        match self {
            Self::      BLACK   |
            Self::LIGHT_BLACK   => Self::      BLACK  ,
            Self:: DARK_GRAY    => Self::LIGHT_BLACK  ,
            Self::      GRAY    => Self:: DARK_GRAY   ,
            Self::LIGHT_GRAY    => Self::      GRAY   ,
            Self:: DARK_WHITE   => Self::LIGHT_GRAY   ,
            Self::      WHITE   => Self:: DARK_WHITE  ,

            Self:: DARK_RED     |
            Self:: DARK_ORANGE  |
            Self:: DARK_YELLOW  |
            Self:: DARK_LIME    |
            Self:: DARK_GREEN   |
            Self:: DARK_TEAL    |
            Self:: DARK_CYAN    |
            Self:: DARK_SKY     |
            Self:: DARK_BLUE    |
            Self:: DARK_PURPLE  |
            Self:: DARK_MAGENTA |
            Self:: DARK_CANDY   => Self::      BLACK  ,

            Self::      RED     => Self:: DARK_RED    ,
            Self::      ORANGE  => Self:: DARK_ORANGE ,
            Self::      YELLOW  => Self:: DARK_YELLOW ,
            Self::      LIME    => Self:: DARK_LIME   ,
            Self::      GREEN   => Self:: DARK_GREEN  ,
            Self::      TEAL    => Self:: DARK_TEAL   ,
            Self::      CYAN    => Self:: DARK_CYAN   ,
            Self::      SKY     => Self:: DARK_SKY    ,
            Self::      BLUE    => Self:: DARK_BLUE   ,
            Self::      PURPLE  => Self:: DARK_PURPLE ,
            Self::      MAGENTA => Self:: DARK_MAGENTA,
            Self::      CANDY   => Self:: DARK_CANDY  ,

            Self::LIGHT_RED     => Self::      RED    ,
            Self::LIGHT_ORANGE  => Self::      ORANGE ,
            Self::LIGHT_YELLOW  => Self::      YELLOW ,
            Self::LIGHT_LIME    => Self::      LIME   ,
            Self::LIGHT_GREEN   => Self::      GREEN  ,
            Self::LIGHT_TEAL    => Self::      TEAL   ,
            Self::LIGHT_CYAN    => Self::      CYAN   ,
            Self::LIGHT_SKY     => Self::      SKY    ,
            Self::LIGHT_BLUE    => Self::      BLUE   ,
            Self::LIGHT_PURPLE  => Self::      PURPLE ,
            Self::LIGHT_MAGENTA => Self::      MAGENTA,
            Self::LIGHT_CANDY   => Self::      CANDY  ,

            _                   => Self::      BLACK
        }
    }

    pub fn lighter(self) -> Self {
        match self {
            Self::      WHITE   |
            Self:: DARK_WHITE   => Self::      WHITE  ,
            Self::LIGHT_GRAY    => Self:: DARK_WHITE  ,
            Self::      GRAY    => Self::LIGHT_GRAY   ,
            Self:: DARK_GRAY    => Self::      GRAY   ,
            Self::LIGHT_BLACK   => Self:: DARK_GRAY   ,
            Self::      BLACK   => Self::LIGHT_BLACK  ,

            Self::LIGHT_RED     |
            Self::LIGHT_ORANGE  |
            Self::LIGHT_YELLOW  |
            Self::LIGHT_LIME    |
            Self::LIGHT_GREEN   |
            Self::LIGHT_TEAL    |
            Self::LIGHT_CYAN    |
            Self::LIGHT_SKY     |
            Self::LIGHT_BLUE    |
            Self::LIGHT_PURPLE  |
            Self::LIGHT_MAGENTA |
            Self::LIGHT_CANDY   => Self::      WHITE  ,

            Self::      RED     => Self::LIGHT_RED    ,
            Self::      ORANGE  => Self::LIGHT_ORANGE ,
            Self::      YELLOW  => Self::LIGHT_YELLOW ,
            Self::      LIME    => Self::LIGHT_LIME   ,
            Self::      GREEN   => Self::LIGHT_GREEN  ,
            Self::      TEAL    => Self::LIGHT_TEAL   ,
            Self::      CYAN    => Self::LIGHT_CYAN   ,
            Self::      SKY     => Self::LIGHT_SKY    ,
            Self::      BLUE    => Self::LIGHT_BLUE   ,
            Self::      PURPLE  => Self::LIGHT_PURPLE ,
            Self::      MAGENTA => Self::LIGHT_MAGENTA,
            Self::      CANDY   => Self::LIGHT_CANDY  ,

            Self:: DARK_RED     => Self::      RED    ,
            Self:: DARK_ORANGE  => Self::      ORANGE ,
            Self:: DARK_YELLOW  => Self::      YELLOW ,
            Self:: DARK_LIME    => Self::      LIME   ,
            Self:: DARK_GREEN   => Self::      GREEN  ,
            Self:: DARK_TEAL    => Self::      TEAL   ,
            Self:: DARK_CYAN    => Self::      CYAN   ,
            Self:: DARK_SKY     => Self::      SKY    ,
            Self:: DARK_BLUE    => Self::      BLUE   ,
            Self:: DARK_PURPLE  => Self::      PURPLE ,
            Self:: DARK_MAGENTA => Self::      MAGENTA,
            Self:: DARK_CANDY   => Self::      CANDY  ,

            _                   => Self::      WHITE
        }
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Self { r, g, b }
    }

    pub fn grayscale(l: u8) -> Self {
        Self { r: l, g: l, b: l }
    }

    pub fn darken(&mut self) {
        *self = self.darker();
    }

    pub fn lighten(&mut self) {
        *self = self.lighter();
    }
}

impl<const WIDTH: usize, const HEIGHT: usize> Framebuffer<WIDTH, HEIGHT> {
    pub fn darken(&mut self) {
        self.0
            .as_flattened_mut()
            .iter_mut()
            .for_each(Color::darken);
    }

    pub fn lighten(&mut self) {
        self.0
            .as_flattened_mut()
            .iter_mut()
            .for_each(Color::lighten);
    }

    pub fn darker(&self) -> Self {
        let mut clone = self.clone();
        clone.darken();
        clone
    }

    pub fn lighter(&self) -> Self {
        let mut clone = self.clone();
        clone.lighten();
        clone
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::AddAssign
for Framebuffer<WIDTH, HEIGHT> {
    fn add_assign(&mut self, rhs: Self) {
        self.0
            .as_flattened_mut()
            .iter_mut()
            .zip(rhs.0.as_flattened().iter())
            .for_each(|(a, b)| *a += *b);
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::SubAssign
for Framebuffer<WIDTH, HEIGHT> {
    fn sub_assign(&mut self, rhs: Self) {
        self.0
            .as_flattened_mut()
            .iter_mut()
            .zip(rhs.0.as_flattened().iter())
            .for_each(|(a, b)| *a -= *b);
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Add
for Framebuffer<WIDTH, HEIGHT> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Sub
for Framebuffer<WIDTH, HEIGHT> {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self -= rhs;
        self
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Neg
for Framebuffer<WIDTH, HEIGHT> {
    type Output = Self;

    fn neg(mut self) -> Self {
        self.0
            .as_flattened_mut()
            .iter_mut()
            .for_each(|c| *c = -*c);

        self
    }
}

impl<const WIDTH: usize, const HEIGHT: usize>
std::ops::Not
for Framebuffer<WIDTH, HEIGHT> {
    type Output = Self;

    fn not(mut self) -> Self {
        self.0
            .as_flattened_mut()
            .iter_mut()
            .for_each(|c| *c = !*c);

        self
    }
}

