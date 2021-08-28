use tasbot_display::Display;

const NUM_PIXELS: u32 = 24;

fn main() {
    let mut display = Display::new(NUM_PIXELS);

    display.device().clear();
}
