pub mod hardware;
pub use hardware::{display::Display, neopixel_device::NeoPixelDevice};
pub mod tasbot;

use image::Rgb;
pub type RgbColor = Rgb<u8>;
