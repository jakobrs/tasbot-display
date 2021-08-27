use tasbot_display::{Display, RgbColor};

const SCREEN_WIDTH: u32 = 30;
const SCREEN_HEIGHT: u32 = 8;

fn main() {
  let mut display = Display::new(SCREEN_WIDTH * SCREEN_HEIGHT);

  let red = RgbColor::from([255, 0, 0]);
  for index in 0..SCREEN_HEIGHT * SCREEN_WIDTH {
      display[index as usize] = red;
  }
  display.draw();
}