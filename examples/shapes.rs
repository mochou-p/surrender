// mochou-p/surrender/examples/shapes.rs

use surrender::Mode;
use surrender::winit::keyboard::KeyCode;
use surrender::winit::dpi::PhysicalSize;


const WIDTH:  f32 = 176.0;
const HEIGHT: f32 = 176.0;

#[derive(Default)]
struct Game {
    exiting: bool
}

fn main() {
    surrender::App::<Game>::new()
        .window(|_, attributes| {
            attributes
                .with_inner_size(PhysicalSize { width: WIDTH, height: HEIGHT })
                .with_resizable(false)
        })
        .keyboard(|game, key, down| {
            game.exiting = key == KeyCode::KeyQ && down;
        })
        .draw(|_, canvas| {
            canvas.set_color(255, 255, 100);
            for x in 0..(WIDTH/8.0) as usize {
                canvas.point((x * 8) as f32, 0.0);
            }

            canvas.set_color(255, 100, 100);
            canvas.rectangle(Mode::Fill, 16.0, 16.0, 64.0, 64.0);
            canvas.set_color(255, 100, 100);
            canvas.rectangle(Mode::Line, 16.0, 96.0, 64.0, 64.0);

            canvas.set_color(100, 255, 100);
            canvas.   circle(Mode::Fill, 128.0, 48.0, 32.0);
            canvas.set_color(100, 255, 100);
            canvas.   circle(Mode::Line, 128.0, 128.0, 32.0);
        })
        .quit_if(|game| game.exiting)
        .run()
}

