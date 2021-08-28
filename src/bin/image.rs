use image::io::Reader as ImageReader;
use tasbot_display::tasbot::{NUM_PIXELS, PIXEL_POSITIONS};
use tasbot_display::Display;

fn main() {
    let image = ImageReader::open("./a.png")
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let mut display = Display::new(NUM_PIXELS);

    for (x, y, &color) in image.enumerate_pixels() {
        if let Some(i) = PIXEL_POSITIONS[y as usize][x as usize] {
            display[i] = color;
        }
    }

    display.draw();
}
