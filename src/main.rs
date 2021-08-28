use tasbot_display::{Display, RgbColor};

const SCREEN_WIDTH: u32 = 30;
const SCREEN_HEIGHT: u32 = 8;
//const NUM_PIXELS: u32 = SCREEN_WIDTH * SCREEN_HEIGHT;
const NUM_PIXELS: u32 = 24;

fn main() {
  let mut display = Display::new(NUM_PIXELS);

  display.draw();
}
