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
        self.buffer
            .extend(&vec![0; self.num_lights as usize * BITS_PER_BIT]);
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

fn convert_to_spi_format(byte: u8) -> [u8; BITS_PER_BIT] {
    let bools: Vec<bool> = (0..8)
        .flat_map(|n| {
            if byte & (1 << (7 - n)) > 0 {
                [true, true, true, true, false, false, false, false]
            } else {
                [true, false, false, false, false, false, false, false]
            }
        })
        .collect();

    [
        eight_bools_to_byte(&bools[0..8]),
        eight_bools_to_byte(&bools[8..16]),
        eight_bools_to_byte(&bools[16..24]),
        eight_bools_to_byte(&bools[24..32]),
        eight_bools_to_byte(&bools[32..40]),
        eight_bools_to_byte(&bools[40..48]),
        eight_bools_to_byte(&bools[48..56]),
        eight_bools_to_byte(&bools[56..64]),
    ]
}

fn eight_bools_to_byte(bools: &[bool]) -> u8 {
    let mut n = 0;
    for (position, bit) in bools.iter().enumerate() {
        if *bit {
            n |= 1 << (7 - position);
        }
    }
    n
}

#[cfg(test)]
mod tests {
    #[test]
    fn eight_bools_to_byte_works() {
        use super::*;

        assert_eq!(
            eight_bools_to_byte(&[false, false, false, false, false, false, false, false]),
            0b0000_0000
        );
        assert_eq!(
            eight_bools_to_byte(&[true, false, false, false, false, false, false, false]),
            0b1000_0000
        );
        assert_eq!(
            eight_bools_to_byte(&[false, true, false, false, false, false, false, false]),
            0b0100_0000
        );
        assert_eq!(
            eight_bools_to_byte(&[false, true, false, false, true, false, false, false]),
            0b0100_1000
        );
    }

    #[test]
    fn convert_to_spi_format_works() {
        use super::*;

        assert_eq!(
            convert_to_spi_format(0b1000_1010),
            [0b1101_0010, 0b0100_1101, 0b0011_0100]
        );
    }
}
