use tasbot_display::{Display, RgbColor};
use tasbot_display::tasbot::NUM_PIXELS;

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
}

fn main() {
    let opts = Opts::from_args();
    let color = RgbColor::from([opts.red, opts.green, opts.blue]);

    let mut display = Display::new(NUM_PIXELS);

    for i in 0..NUM_PIXELS {
        display[i as usize] = color;
    }

    display.draw();
}
