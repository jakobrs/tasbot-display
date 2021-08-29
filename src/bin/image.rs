use image::io::Reader as ImageReader;
use tasbot_display::tasbot::{NUM_PIXELS, PIXEL_POSITIONS};
use tasbot_display::Display;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    file: String,

    #[structopt(long)]
    brightness: Option<f32>,
}

fn main() {
    let opts = Opts::from_args();

    let image = ImageReader::open(&opts.file)
        .unwrap()
        .decode()
        .unwrap()
        .to_rgb8();

    let mut display = Display::new(NUM_PIXELS);

    if let Some(brightness) = opts.brightness {
        display.set_brightness(brightness);
    }

    for (x, y, &color) in image.enumerate_pixels() {
        if let Some(i) = PIXEL_POSITIONS[y as usize][x as usize] {
            display[i] = color;
        }
    }

    display.draw();
}
