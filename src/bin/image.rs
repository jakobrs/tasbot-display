use std::fs::File;
use std::time::Duration;

use image::gif::GifDecoder;
use image::io::Reader as ImageReader;
use image::{AnimationDecoder, DynamicImage, Frame, ImageError, RgbImage};
use tasbot_display::tasbot::{NUM_PIXELS, PIXEL_POSITIONS};
use tasbot_display::Display;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    file: String,

    #[structopt(long)]
    brightness: Option<f32>,

    #[structopt(long)]
    gif: bool,

    #[structopt(long, default_value = "500")]
    speed: u64,
}

fn main() {
    let opts = Opts::from_args();

    let mut display = Display::new(NUM_PIXELS);

    if let Some(brightness) = opts.brightness {
        display.set_brightness(brightness);
    }

    if opts.gif {
        let file = File::open(&opts.file).unwrap();
        let decoder = GifDecoder::new(file).unwrap();

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
                draw_image(&mut display, image);

                std::thread::sleep(Duration::from_millis(opts.speed));
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

fn draw_image(display: &mut Display, image: &RgbImage) -> () {
    for (x, y, &color) in image.enumerate_pixels() {
        if let Some(i) = PIXEL_POSITIONS[y as usize][x as usize] {
            display[i] = color;
        }
    }

    display.draw();
}
