use tasbot_display::{tasbot::NUM_PIXELS, Display, RgbColor};

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
    #[structopt(long)]
    gamma: Option<f32>,
    #[structopt(long, help = "Gamma on the red channel")]
    red_gamma: Option<f32>,
    #[structopt(long, help = "Gamma on the green channel")]
    green_gamma: Option<f32>,
    #[structopt(long, help = "Gamma on the blue channel")]
    blue_gamma: Option<f32>,
}

fn main() {
    let opts = Opts::from_args();
    let color = RgbColor::from([opts.red, opts.green, opts.blue]);

    let mut display = Display::new(NUM_PIXELS).unwrap();

    if let Some(brightness) = opts.brightness {
        display.set_brightness(brightness);
    }
    if let Some(gamma) = opts.gamma {
        display.set_gamma(gamma);
    }
    {
        let mut gamma = display.gamma().clone();
        if let Some(red_gamma) = opts.red_gamma {
            gamma[0] = red_gamma;
        }
        if let Some(green_gamma) = opts.green_gamma {
            gamma[1] = green_gamma;
        }
        if let Some(blue_gamma) = opts.blue_gamma {
            gamma[2] = blue_gamma;
        }
        display.set_per_channel_gamma(gamma);
    }

    for i in 0..NUM_PIXELS {
        display[i as usize] = color;
    }

    display.draw();
}
