use crate::Framebuffer;


impl<const WIDTH: usize, const HEIGHT: usize> Framebuffer<WIDTH, HEIGHT> {
    fn write_ppm_head(&self, string: &mut String) {
        *string += "P3\n";
        *string += &format!("{WIDTH} {HEIGHT}\n");
        *string += "255\n";
    }

    fn write_ppm_data(&self, string: &mut String) {
        for row in 0..HEIGHT {
            if row != 0 {
                *string += "\n";
            }

            for col in 0..WIDTH {
                if col != 0 {
                    *string += " ";
                }

                *string += &format!(
                    "{} {} {}",
                    self.0[row][col].r,
                    self.0[row][col].g,
                    self.0[row][col].b
                );
            }
        }
    }

    pub fn as_ppm(&self) -> String {
        let mut string = String::new();

        self.write_ppm_head(&mut string);
        self.write_ppm_data(&mut string);

        string
    }
}

