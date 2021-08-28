use tasbot_display::{Display, NeoPixelDevice, RgbColor};

const SCREEN_WIDTH: u32 = 30;
const SCREEN_HEIGHT: u32 = 8;
//const NUM_PIXELS: u32 = SCREEN_WIDTH * SCREEN_HEIGHT;
const NUM_PIXELS: u32 = 24;

fn main() {
    let mut display = Display::new(NUM_PIXELS);

    let red = RgbColor::from([0, 255, 0]);
    for i in 0..NUM_PIXELS {
        display[i as usize] = red;
    }

    display.draw();
}
