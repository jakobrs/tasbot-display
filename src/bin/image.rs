use std::fs::File;
use std::time::Duration;

use image::gif::GifDecoder;
use image::io::Reader as ImageReader;
use image::{AnimationDecoder, DynamicImage, Frame, ImageError, RgbImage};
use tasbot_display::tasbot::{NUM_PIXELS, PIXEL_POSITIONS, SCREEN_HEIGHT, SCREEN_WIDTH};
use tasbot_display::Display;

use structopt::StructOpt;

#[derive(StructOpt)]
struct Opts {
    #[structopt(help = "Path to the image file")]
    file: String,

    #[structopt(long, help = "Brightness")]
    brightness: Option<f32>,

    #[structopt(long, help = "Enable if the image is a gif")]
    gif: bool,

    #[structopt(long, default_value = "1", help = "Divides delay between frames by <speedup>")]
    speedup: u32,
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
                draw_image(&mut display, image);

                let (numer, denom) = frame.delay().numer_denom_ms();
                let duration = Duration::from_millis(numer as u64) / denom / opts.speedup;

                std::thread::sleep(duration);
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
