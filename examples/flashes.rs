use tasbot_display::tasbot::NUM_PIXELS;
use tasbot_display::{Display, RgbColor};

use structopt::StructOpt;

use std::thread;
use std::time::Duration;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
struct Opts {
    #[structopt(long)]
    brightness: Option<f32>,

    #[structopt(long, default_value = "500", help = "Delay between frames, in milliseconds")]
    delay: u64,
}

fn main() {
    let opts = Opts::from_args();

    let mut display = Display::new(NUM_PIXELS);
    if let Some(brightness) = opts.brightness {
        display.set_brightness(brightness);
    }

    let red = RgbColor::from([255, 0, 0]);
    let green = RgbColor::from([0, 255, 0]);

    loop {
        println!("red");
        for i in 0..NUM_PIXELS {
            display[i as usize] = red;
        }
        display.draw();
        thread::sleep(Duration::from_millis(opts.delay));

        println!("green");
        for i in 0..NUM_PIXELS {
            display[i as usize] = green;
        }
        display.draw();
        thread::sleep(Duration::from_millis(opts.delay));
    }
}
