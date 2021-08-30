use crate::RgbColor;

use rppal::spi::{Bus, Mode, SlaveSelect, Spi};

const BITS_PER_BIT: usize = 8;

pub struct NeoPixelDevice {
    buffer: Vec<u8>,
    spi: Spi,
    pub(crate) num_lights: u32,
}

impl NeoPixelDevice {
    pub fn new(num_lights: u32) -> Self {
        let bus = Bus::Spi0;
        let slave_select = SlaveSelect::Ss0;
        //let clock_speed = 3 * 1000 * 1000;
        let clock_speed = 6_400_000;
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
        self.buffer.extend(&vec![0; self.num_lights as usize * 3]);
        self.write();
    }

    pub fn set_pixels(&mut self, pixels: &[RgbColor]) {
        self.buffer.clear();
        self.buffer.extend(
            pixels
                .iter()
                .flat_map(|color| [color[1], color[0], color[2]]),
        );
        self.write();
    }
}

const ZERO_BIT_PATTERN: u8 = 0b1000_0000;
const ONE_BIT_PATTERN: u8 = 0b1111_0000;

fn convert_to_spi_format(byte: u8) -> [u8; BITS_PER_BIT] {
    fn bit_to_spi_byte(byte: u8, bit: u8) -> u8 {
        if byte & (1 << (7 - bit)) > 0 {
            ONE_BIT_PATTERN
        } else {
            ZERO_BIT_PATTERN
        }
    }

    [
        bit_to_spi_byte(byte, 0),
        bit_to_spi_byte(byte, 1),
        bit_to_spi_byte(byte, 2),
        bit_to_spi_byte(byte, 3),
        bit_to_spi_byte(byte, 4),
        bit_to_spi_byte(byte, 5),
        bit_to_spi_byte(byte, 6),
        bit_to_spi_byte(byte, 7),
    ]
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert_to_spi_format_works() {
        use super::*;

        assert_eq!(
            convert_to_spi_format(0b1000_1010),
            [
                ONE_BIT_PATTERN,
                ZERO_BIT_PATTERN,
                ZERO_BIT_PATTERN,
                ZERO_BIT_PATTERN,
                ONE_BIT_PATTERN,
                ZERO_BIT_PATTERN,
                ONE_BIT_PATTERN,
                ZERO_BIT_PATTERN,
            ]
        );
    }
}
