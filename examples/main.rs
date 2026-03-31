use surrender::{Framebuffer, Color};


fn main() {
    let mut framebuffer = Framebuffer::<80, 60>::new(Color::LIGHT_BLACK);

    framebuffer.   x_line( 1, 5,  3,        Color::ORANGE );
    framebuffer.   y_line( 1, 5,  3,        Color::YELLOW );

    framebuffer.rect_fill( 3, 8, 10, 10,    Color::WHITE  );
    framebuffer.rect_line( 1, 6, 14, 14, 1, Color::BLUE   );
    framebuffer.rect_fill(23, 8, 10, 10,    Color::WHITE  );
    framebuffer.rect_line(21, 6, 14, 14, 6, Color::CYAN   );

    framebuffer[( 1, 26)] = Color::      BLACK  ;
    framebuffer[( 2, 26)] = Color::LIGHT_BLACK  ;
    framebuffer[( 3, 26)] = Color:: DARK_GRAY   ;
    framebuffer[( 4, 26)] = Color::      GRAY   ;
    framebuffer[( 5, 26)] = Color::LIGHT_GRAY   ;
    framebuffer[( 6, 26)] = Color:: DARK_WHITE  ;
    framebuffer[( 7, 26)] = Color::      WHITE  ;

    framebuffer[( 1, 28)] = Color:: DARK_RED    ;
    framebuffer[( 1, 29)] = Color::      RED    ;
    framebuffer[( 1, 30)] = Color::LIGHT_RED    ;
    framebuffer[( 2, 28)] = Color:: DARK_ORANGE ;
    framebuffer[( 2, 29)] = Color::      ORANGE ;
    framebuffer[( 2, 30)] = Color::LIGHT_ORANGE ;
    framebuffer[( 3, 28)] = Color:: DARK_YELLOW ;
    framebuffer[( 3, 29)] = Color::      YELLOW ;
    framebuffer[( 3, 30)] = Color::LIGHT_YELLOW ;
    framebuffer[( 4, 28)] = Color:: DARK_LIME   ;
    framebuffer[( 4, 29)] = Color::      LIME   ;
    framebuffer[( 4, 30)] = Color::LIGHT_LIME   ;
    framebuffer[( 5, 28)] = Color:: DARK_GREEN  ;
    framebuffer[( 5, 29)] = Color::      GREEN  ;
    framebuffer[( 5, 30)] = Color::LIGHT_GREEN  ;
    framebuffer[( 6, 28)] = Color:: DARK_TEAL   ;
    framebuffer[( 6, 29)] = Color::      TEAL   ;
    framebuffer[( 6, 30)] = Color::LIGHT_TEAL   ;
    framebuffer[( 7, 28)] = Color:: DARK_CYAN   ;
    framebuffer[( 7, 29)] = Color::      CYAN   ;
    framebuffer[( 7, 30)] = Color::LIGHT_CYAN   ;
    framebuffer[( 8, 28)] = Color:: DARK_SKY    ;
    framebuffer[( 8, 29)] = Color::      SKY    ;
    framebuffer[( 8, 30)] = Color::LIGHT_SKY    ;
    framebuffer[( 9, 28)] = Color:: DARK_BLUE   ;
    framebuffer[( 9, 29)] = Color::      BLUE   ;
    framebuffer[( 9, 30)] = Color::LIGHT_BLUE   ;
    framebuffer[(10, 28)] = Color:: DARK_PURPLE ;
    framebuffer[(10, 29)] = Color::      PURPLE ;
    framebuffer[(10, 30)] = Color::LIGHT_PURPLE ;
    framebuffer[(11, 28)] = Color:: DARK_MAGENTA;
    framebuffer[(11, 29)] = Color::      MAGENTA;
    framebuffer[(11, 30)] = Color::LIGHT_MAGENTA;
    framebuffer[(12, 28)] = Color:: DARK_CANDY  ;
    framebuffer[(12, 29)] = Color::      CANDY  ;
    framebuffer[(12, 30)] = Color::LIGHT_CANDY  ;

    let mut copy_1 = framebuffer.copy::<4, 4>(1, 1);
    let     copy_2 = copy_1.darker();

    copy_1.lighten();

    framebuffer.paste(&copy_1,  6, 1);
    framebuffer.paste(&copy_2, 11, 1);

    let ppm = framebuffer.as_ppm();
    std::fs::write("image.ppm", ppm).unwrap();

    assert_eq!(Color::RED    + Color::GREEN, Color::YELLOW);
    assert_eq!(Color::YELLOW - Color::RED,   Color::GREEN );
    assert_eq!(Color::YELLOW - Color::GREEN, Color::RED   );

    assert_eq!(-Color::RED,     Color::CYAN  );
    assert_eq!(-Color::WHITE,   Color::BLACK );
    assert_eq!(-Color::PURPLE, !Color::PURPLE);

    use std::ops::Index;
    assert!(
        std::ptr::eq(
            framebuffer.index((12, 34)),
            framebuffer.index([12, 34])
        )
    );
    assert!(
        std::ptr::eq(
            framebuffer.index((12, 34)),
            framebuffer.index(34).index(12)
        )
    );
}

