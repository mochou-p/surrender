use crate::{Framebuffer, Color};


impl<const WIDTH: usize, const HEIGHT: usize> Framebuffer<WIDTH, HEIGHT> {
    pub fn x_line(
        &mut self,
        l: usize,
        r: usize,
        y: usize,
        c: Color
    ) {
        self.0[y][l..r].fill(c);
    }

    pub fn y_line(
        &mut self,
        u: usize,
        d: usize,
        x: usize,
        c: Color
    ) {
        for row in u..d {
            self.0[row][x] = c;
        }
    }

    pub fn rect_fill(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        c: Color
    ) {
        for row in y..y+h {
            self.0[row][x..x+w].fill(c);
        }
    }

    pub fn rect_line(
        &mut self,
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        t: usize,
        c: Color
    ) {
        for row in y+1-t..y+1 {
            self.0[row][x+1-t..x+w+t-1].fill(c);
        }

        for row in y+1..y+h-1 {
            self.0[row][x+1-t..x+1    ].fill(c);
            self.0[row][x+w-1..x+w+t-1].fill(c);
        }

        for row in y+h-1..y+h+t-1 {
            self.0[row][x+1-t..x+w+t-1].fill(c);
        }
    }
}

