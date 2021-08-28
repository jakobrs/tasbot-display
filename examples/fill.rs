use tasbot_display::{Display, RgbColor};
use tasbot_display::tasbot::NUM_PIXELS;

fn main() {
    let mut display = Display::new(NUM_PIXELS);

    let red = RgbColor::from([0, 255, 0]);
    for i in 0..NUM_PIXELS {
        display[i as usize] = red;
    }

    display.draw();
}
