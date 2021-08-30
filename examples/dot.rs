use std::thread;
use std::time::Duration;

//use tasbot_display::tasbot::NUM_PIXELS;
const NUM_PIXELS: u32 = 24;
use tasbot_display::{Display, RgbColor};

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
struct Opts {
    #[structopt(short, long, default_value = "0")]
    red: u8,
    #[structopt(short, long, default_value = "255")]
    green: u8,
    #[structopt(short, long, default_value = "0")]
    blue: u8,

    #[structopt(long)]
    brightness: Option<f32>,

    #[structopt(
        long,
        default_value = "200",
        help = "Delay between steps, in milliseconds"
    )]
    delay: u64,
}

fn main() {
    let opts = Opts::from_args();
    let color = RgbColor::from([opts.red, opts.green, opts.blue]);
    let black = RgbColor::from([0, 0, 0]);

    let mut display = Display::new(NUM_PIXELS);

    if let Some(brightness) = opts.brightness {
        display.set_brightness(brightness);
    }

    loop {
        for i in 0..NUM_PIXELS {
            display.set_pixel(i as usize, black);
            display.set_pixel(((i + 1) % 24) as usize, color);

            display.draw();

            thread::sleep(Duration::from_millis(opts.delay));
        }
    }
}
