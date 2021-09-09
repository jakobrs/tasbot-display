use std::{
    fs::File,
    time::{Duration, Instant},
};

use image::{
    gif::GifDecoder, io::Reader as ImageReader, AnimationDecoder, DynamicImage, Frame, ImageError,
    RgbImage,
};
use rppal::spi;
use tasbot_display::{
    tasbot::{NUM_PIXELS, PIXEL_POSITIONS, SCREEN_HEIGHT, SCREEN_WIDTH},
    Display, NeoPixelDevice,
};
use thiserror::Error;

use structopt::StructOpt;

#[derive(StructOpt)]
#[structopt(setting = structopt::clap::AppSettings::DeriveDisplayOrder)]
struct Opts {
    #[structopt(help = "Path to the image file")]
    file: String,

    #[structopt(long, help = "Brightness")]
    brightness: Option<f32>,

    #[structopt(long, help = "Gamma")]
    gamma: Option<f32>,
    #[structopt(long, help = "Gamma on the red channel")]
    red_gamma: Option<f32>,
    #[structopt(long, help = "Gamma on the green channel")]
    green_gamma: Option<f32>,
    #[structopt(long, help = "Gamma on the blue channel")]
    blue_gamma: Option<f32>,

    #[structopt(long, help = "Enable if the image is a gif")]
    gif: bool,

    #[structopt(
        long,
        default_value = "1",
        help = "Divides delay between frames by <speedup>"
    )]
    speedup: f32,

    #[structopt(long, help = "Don't loop the gif")]
    noloop: bool,

    #[structopt(long, help = "Which SPI bus to use", default_value = "0", parse(try_from_str = parse_spi_bus))]
    bus: spi::Bus,
}

#[derive(Debug, Error)]
enum ParseSpiBusError {
    #[error("{0}")]
    ParseIntError(#[from] std::num::ParseIntError),
    #[error("bus number out of range (0-6)")]
    OutOfRangeError,
}

fn parse_spi_bus(bus: &str) -> Result<spi::Bus, ParseSpiBusError> {
    match bus.parse()? {
        0 => Ok(spi::Bus::Spi0),
        1 => Ok(spi::Bus::Spi1),
        2 => Ok(spi::Bus::Spi2),
        3 => Ok(spi::Bus::Spi3),
        4 => Ok(spi::Bus::Spi4),
        5 => Ok(spi::Bus::Spi5),
        6 => Ok(spi::Bus::Spi6),
        _ => Err(ParseSpiBusError::OutOfRangeError),
    }
}

fn main() {
    let opts = Opts::from_args();

    let device = NeoPixelDevice::new_on_bus(NUM_PIXELS, opts.bus).unwrap();
    let mut display = Display::wrap(device);
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

    if opts.gif {
        let file = File::open(&opts.file).unwrap();
        let decoder = GifDecoder::new(file).unwrap();

        // Takes each frame, converts the buffer to an RgbImage, and collects the result.
        let frames: Vec<(Frame, RgbImage)> = decoder
            .into_frames()
            .map(|frame| {
                let frame = frame?;
                let buffer = frame.buffer();

                let dynamic_image = DynamicImage::ImageRgba8(buffer.clone());
                let rgb_image = dynamic_image.into_rgb8();

                Ok((frame, rgb_image))
            })
            .collect::<Result<Vec<(Frame, RgbImage)>, ImageError>>()
            .unwrap();

        loop {
            for (frame, image) in frames.iter() {
                let now = Instant::now();

                draw_image(&mut display, image);

                let (numer, denom) = frame.delay().numer_denom_ms();
                let duration = Duration::from_millis(numer as u64).div_f32(opts.speedup) / denom;
                std::thread::sleep(duration.saturating_sub(now.elapsed()));
            }

            if opts.noloop {
                break;
            }
        }
    } else {
        let image = ImageReader::open(&opts.file)
            .unwrap()
            .decode()
            .unwrap()
            .to_rgb8();

        draw_image(&mut display, &image);
    }
}

fn draw_image(display: &mut Display, image: &RgbImage) {
    for (x, y, &color) in image.enumerate_pixels() {
        let x = x as usize;
        let y = y as usize;
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            continue;
        }

        if let Some(i) = PIXEL_POSITIONS[y][x] {
            display[i] = color;
        }
    }

    display.draw();
}
