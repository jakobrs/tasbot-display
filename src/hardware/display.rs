use std::ops::{Index, IndexMut};

use crate::hardware::neopixel_device::NeoPixelDevice;
use crate::RgbColor;

const MAX_BRIGHTNESS: f32 = if cfg!(feature = "dont-cap-brightness") {
    1.
} else {
    0.1
};
const DEFAULT_BRIGHTNESS: f32 = 0.1;
const DEFAULT_GAMMA: f32 = 2.;

pub struct Display {
    device: NeoPixelDevice,
    // TODO: Split into separate pre-preprocessing and post-preprocessing buffers
    buffer: Vec<RgbColor>,
    brightness: f32,
    gamma: f32,
}

impl Display {
    pub fn new(num_lights: u32) -> Self {
        let black = RgbColor::from([0, 0, 0]);

        Self {
            device: NeoPixelDevice::new(num_lights),
            buffer: vec![black; num_lights as usize],
            brightness: DEFAULT_BRIGHTNESS,
            gamma: DEFAULT_GAMMA,
        }
    }
    pub fn wrap(device: NeoPixelDevice) -> Self {
        let black = RgbColor::from([0, 0, 0]);

        Self {
            buffer: vec![black; device.num_lights as usize],
            device,
            brightness: DEFAULT_BRIGHTNESS,
            gamma: DEFAULT_GAMMA,
        }
    }

    pub fn set_brightness(&mut self, brightness: f32) {
        if brightness > MAX_BRIGHTNESS {
            panic!(
                "Attempted to set brightness to {}, above MAX_BRIGHTNESS of {}",
                brightness, MAX_BRIGHTNESS
            );
        }

        self.brightness = brightness;
    }

    pub fn set_gamma(&mut self, gamma: f32) {
        self.gamma = gamma;
    }

    pub fn device(&mut self) -> &mut NeoPixelDevice {
        &mut self.device
    }

    pub fn draw(&mut self) {
        let buffer_post_brightness: Vec<RgbColor> = self
            .buffer
            .iter()
            .map(|&pixel| {
                let pixel = apply_gamma(&pixel, self.gamma);
                let pixel = scale_color(&pixel, self.brightness);
                pixel
            })
            .collect();
        self.device.set_pixels(&buffer_post_brightness[..]);
    }
}

impl Index<usize> for Display {
    type Output = RgbColor;

    fn index(&self, index: usize) -> &RgbColor {
        &self.buffer[index]
    }
}
impl IndexMut<usize> for Display {
    fn index_mut(&mut self, index: usize) -> &mut RgbColor {
        &mut self.buffer[index]
    }
}

fn apply_gamma(pixel: &image::Rgb<u8>, gamma: f32) -> image::Rgb<u8> {
    RgbColor::from([
        (((pixel[0] as f32) / 255.).powf(gamma) * 255.) as u8,
        (((pixel[1] as f32) / 255.).powf(gamma) * 255.) as u8,
        (((pixel[2] as f32) / 255.).powf(gamma) * 255.) as u8,
    ])
}

fn scale_color(pixel: &RgbColor, scale: f32) -> RgbColor {
    RgbColor::from([
        ((pixel[0] as f32) * scale) as u8,
        ((pixel[1] as f32) * scale) as u8,
        ((pixel[2] as f32) * scale) as u8,
    ])
}
