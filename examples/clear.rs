use tasbot_display::Display;
use tasbot_display::tasbot::NUM_PIXELS;

fn main() {
    let mut display = Display::new(NUM_PIXELS);

    display.device().clear();
}
