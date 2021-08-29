use tasbot_display::tasbot::NUM_PIXELS;
use tasbot_display::Display;

fn main() {
    let mut display = Display::new(NUM_PIXELS);

    display.device().clear();
}
