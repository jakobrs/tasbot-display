use tasbot_display::{tasbot::NUM_PIXELS, Display};

fn main() {
    let mut display = Display::new(NUM_PIXELS).unwrap();

    display.device_mut().clear();
}
