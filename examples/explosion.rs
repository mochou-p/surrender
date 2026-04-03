// mochou-p/surrender/examples/explosion.rs

#![feature(random)]

use std::iter::repeat_with;
use std::random::random;
use surrender::winit::keyboard::KeyCode;
use surrender::winit::dpi::PhysicalSize;


const WIDTH:  f32 = 640.0;
const HEIGHT: f32 = 360.0;

#[derive(Default)]
struct Game {
    exiting:   bool,
    particles: Vec<((f32, f32), (i8, i8), (u8, u8, u8))>
}

fn main() {
    surrender::App::<Game>::new()
        .window(|_, attributes| {
            attributes
                .with_inner_size(PhysicalSize { width: WIDTH, height: HEIGHT })
                .with_resizable(false)
        })
        .load(|game| {
            game.particles
                .extend(
                    repeat_with(|| (random(..), random(..), random(..), random(..), random(..)))
                        .take(64)
                        .map(|(dx, dy, r, g, b)| (
                            (WIDTH * 0.5, HEIGHT * 0.5),
                            (dx, dy),
                            (r, g, b)
                        ))
                );
        })
        .keyboard(|game, key, down| {
            game.exiting = key == KeyCode::KeyQ && down;
        })
        .update(|game, dt| {
            game.particles
                .iter_mut()
                .for_each(|((x, y), (dx, dy), _)| {
                    if *x <= 0.0 || *x >= WIDTH  { *dx = dx.saturating_neg(); }
                    if *y <= 0.0 || *y >= HEIGHT { *dy = dy.saturating_neg(); }

                    *x += *dx as f32 * dt;
                    *y += *dy as f32 * dt;
                });
        })
        .draw(|game, canvas| {
            game.particles
                .iter()
                .for_each(|((x, y), _, (r, g, b))| {
                    canvas.set_color(*r, *g, *b);
                    canvas.rectangle(*x, *y, 1.0, 1.0);
                });
        })
        .quit_if(|game| game.exiting)
        .run()
}

