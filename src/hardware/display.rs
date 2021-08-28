use std::ops::{Index, IndexMut};

use crate::hardware::neopixel_device::NeoPixelDevice;
use crate::RgbColor;

pub struct Display {
    device: NeoPixelDevice,
    buffer: Vec<RgbColor>,
}

impl Display {
    pub fn new(num_lights: u32) -> Self {
        let black = RgbColor::from([0, 0, 0]);

        Self {
            device: NeoPixelDevice::new(num_lights),
            buffer: vec![black; num_lights as usize],
        }
    }
    pub fn wrap(device: NeoPixelDevice) -> Self {
        let black = RgbColor::from([0, 0, 0]);

        Self {
            buffer: vec![black; device.num_lights as usize],
            device,
        }
    }

    pub fn device(&mut self) -> &mut NeoPixelDevice {
        &mut self.device
    }

    pub fn draw(&mut self) {
        self.device.set_pixels(&self.buffer[..]);
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
