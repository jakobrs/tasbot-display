use std::ops::Index;

use crate::hardware::neopixel_device::NeoPixelDevice;
use crate::RgbColor;

const MAX_BRIGHTNESS: f32 = if cfg!(dont_cap_brightness) { 1. } else { 0.1 };
const DEFAULT_BRIGHTNESS: f32 = 0.1;

pub struct Display {
    device: NeoPixelDevice,
    // TODO: Split into separate pre-brightness and post-brightness buffers
    buffer: Vec<RgbColor>,
    brightness: f32,
}

impl Display {
    pub fn new(num_lights: u32) -> Self {
        let black = RgbColor::from([0, 0, 0]);

        Self {
            device: NeoPixelDevice::new(num_lights),
            buffer: vec![black; num_lights as usize],
            brightness: DEFAULT_BRIGHTNESS,
        }
    }
    pub fn wrap(device: NeoPixelDevice) -> Self {
        let black = RgbColor::from([0, 0, 0]);

        Self {
            buffer: vec![black; device.num_lights as usize],
            device,
            brightness: DEFAULT_BRIGHTNESS,
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

        for (index, &color) in self.buffer.iter().enumerate() {
            self.device.set_pixel(index, scale_color(color, brightness));
        }
    }

    pub fn device(&mut self) -> &mut NeoPixelDevice {
        &mut self.device
    }

    pub fn draw(&mut self) {
        self.device.write();
    }

    pub fn get_pixel(&self, index: usize) -> RgbColor {
        self.buffer[index]
    }

    pub fn set_pixel(&mut self, index: usize, color: RgbColor) {
        self.buffer[index] = color;
        self.device
            .set_pixel(index, scale_color(color, self.brightness));
    }
}
impl Index<usize> for Display {
    type Output = RgbColor;

    fn index(&self, index: usize) -> &RgbColor {
        &self.buffer[index]
    }
}

fn scale_color(color: RgbColor, scale: f32) -> RgbColor {
    RgbColor::from([
        ((color[0] as f32) * scale) as u8,
        ((color[1] as f32) * scale) as u8,
        ((color[2] as f32) * scale) as u8,
    ])
}
