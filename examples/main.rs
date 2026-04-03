// mochou-p/surrender/examples/main.rs

#![feature(random)]

use std::iter::repeat_with;
use std::random::random;
use surrender::winit::keyboard::KeyCode;
use surrender::winit::dpi::PhysicalSize;


#[derive(Default)]
struct Game {
    exiting: bool,
    balls:   Vec<((f32, f32), (u8, u8))>
}

fn main() {
    surrender::App::<Game>::new()
        .window(|_, attributes| {
            attributes
                .with_inner_size(PhysicalSize { width: 800, height: 600 })
                .with_resizable(false)
        })
        .load(|game| {
            game.balls
                .extend(
                    repeat_with(|| (random(..), random(..)))
                        .take(8)
                        .map(|(dx, dy)| ((0.0, 0.0), (dx, dy)))
                );
        })
        .keyboard(|game, key, down| {
            if key == KeyCode::KeyQ && down {
                game.exiting = true;
            }
        })
        .update(|game, dt| {
            game.balls
                .iter_mut()
                .for_each(|((x, y), (dx, dy))| {
                    *x += *dx as f32 * dt;
                    *y += *dy as f32 * dt;
                })
        })
        .draw(|game, _| {
            println!("{:?}", game.balls);
        })
        .quit_if(|game| game.exiting)
        .run()
}

