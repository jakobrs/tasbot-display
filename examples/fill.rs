use tasbot_display::tasbot::NUM_PIXELS;
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
}

fn main() {
    let opts = Opts::from_args();
    let color = RgbColor::from([opts.red, opts.green, opts.blue]);

    let mut display = Display::new(NUM_PIXELS);

    if let Some(brightness) = opts.brightness {
        display.set_brightness(brightness);
    }

    for i in 0..NUM_PIXELS {
        display.set_pixel(i as usize, color);
    }

    display.draw();
}
