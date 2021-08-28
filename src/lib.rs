use std::ops::{Index, IndexMut};

use image::Rgb;
use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

pub type RgbColor = Rgb<u8>;

pub struct NeoPixelDevice {
    buffer: Vec<u8>,
    spi: Spi,
    num_lights: u32,
}

impl NeoPixelDevice {
    pub fn new(num_lights: u32) -> Self {
        let bus = Bus::Spi0;
        let slave_select = SlaveSelect::Ss0;
        let clock_speed = 3 * 1000 * 1000;
        let mode = Mode::Mode0;

        let spi = Spi::new(bus, slave_select, clock_speed, mode).unwrap();

        Self {
            buffer: vec![],
            spi,
            num_lights,
        }
    }

    fn write(&mut self) {
        let buffer_spi: Vec<u8> = self
            .buffer
            .drain(..)
            .flat_map(convert_to_spi_format)
            .collect();

        self.spi.write(&buffer_spi[..]).unwrap();
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
        self.buffer.extend(&vec![0; self.num_lights as usize]);
        self.write();
    }

    pub fn set_pixels(&mut self, pixels: &[RgbColor]) {
        self.buffer.extend(
            pixels
                .iter()
                .flat_map(|color| [color[1], color[0], color[2]]),
        );
        self.write();
    }
}

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

fn convert_to_spi_format(byte: u8) -> [u8; 3] {
    let bools: Vec<bool> = (0..8).flat_map(|n| {
        if byte & (1 << (8 - n)) > 0 {
            [true, true, false]
        } else {
            [true, false, false]
        }
    }).collect();

    [
        eight_bools_to_byte(&bools[0..8]),
        eight_bools_to_byte(&bools[8..16]),
        eight_bools_to_byte(&bools[16..24]),
    ]
}

fn eight_bools_to_byte(bools: &[bool]) -> u8 {
    let mut n = 0;
    for (position, bit) in bools.iter().enumerate() {
        if *bit {
            n |= 1 << (8 - position);
        }
    }
    n
}
