// mochou-p/surrender/examples/main.rs

#[derive(Default)]
struct Game;

fn main() {
    surrender::App::<Game>::new()
        .window(|_, attributes| {
            attributes
                .with_resizable(false)
        })
        .load(|_| {})
        .update(|_| {})
        .draw(|_, _| {})
        .quit(|_| {})
        .run()
}

