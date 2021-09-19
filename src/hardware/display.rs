use std::ops::{Index, IndexMut};

use crate::{hardware::neopixel_device::NeoPixelDevice, RgbColor};
use log::info;

pub const MAX_BRIGHTNESS: f32 = if cfg!(feature = "dont-cap-brightness") {
    1.
} else {
    0.1
};
pub const DEFAULT_BRIGHTNESS: f32 = 0.1;
pub const DEFAULT_GAMMA: [f32; 3] = [2., 2., 2.];

pub struct Display {
    device: NeoPixelDevice,
    // TODO: Split into separate pre-preprocessing and post-preprocessing buffers
    buffer: Vec<RgbColor>,
    brightness: f32,
    gamma: [f32; 3],
}

impl Display {
    pub fn new(num_lights: u32) -> rppal::spi::Result<Self> {
        let black = RgbColor::from([0, 0, 0]);

        Ok(Self {
            device: NeoPixelDevice::new(num_lights)?,
            buffer: vec![black; num_lights as usize],
            brightness: DEFAULT_BRIGHTNESS,
            gamma: DEFAULT_GAMMA,
        })
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
            info!(
                "Attempted to set brightness to {}, above MAX_BRIGHTNESS of {}",
                brightness, MAX_BRIGHTNESS
            );
        }

        self.brightness = brightness.clamp(0., MAX_BRIGHTNESS);
    }

    pub fn brightness(&self) -> f32 {
        self.brightness
    }

    pub fn set_gamma(&mut self, gamma: f32) {
        self.set_per_channel_gamma([gamma; 3]);
    }

    pub fn set_per_channel_gamma(&mut self, gamma: [f32; 3]) {
        self.gamma = gamma;
    }

    pub fn gamma(&self) -> &[f32; 3] {
        &self.gamma
    }

    pub fn device(&self) -> &NeoPixelDevice {
        &self.device
    }

    pub fn device_mut(&mut self) -> &mut NeoPixelDevice {
        &mut self.device
    }

    pub fn draw(&mut self) {
        let buffer_post_brightness: Vec<RgbColor> = self
            .buffer
            .iter()
            .map(|&pixel| {
                let pixel = scale_color(pixel, self.brightness);
                let pixel = apply_gamma(pixel, self.gamma);
                pixel
            })
            .collect();
        self.device.set_pixels(&buffer_post_brightness[..]);
    }

    pub fn get_buffer(&self) -> &[RgbColor] {
        &self.buffer
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

fn apply_gamma(pixel: RgbColor, gamma: [f32; 3]) -> image::Rgb<u8> {
    RgbColor::from([
        (((pixel[0] as f32) / 255.).powf(gamma[0]) * 255.) as u8,
        (((pixel[1] as f32) / 255.).powf(gamma[1]) * 255.) as u8,
        (((pixel[2] as f32) / 255.).powf(gamma[2]) * 255.) as u8,
    ])
}

fn scale_color(pixel: RgbColor, scale: f32) -> RgbColor {
    RgbColor::from([
        ((pixel[0] as f32) * scale) as u8,
        ((pixel[1] as f32) * scale) as u8,
        ((pixel[2] as f32) * scale) as u8,
    ])
}
