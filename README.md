# tasbot-display

Experiment in handling the TASBot display from Rust

### Repository structure

- `src/hardware` contains the code used for drawing to the display
  - `src/hardware/neopixel_device.rs` contains `NeoPixelDevice`, a data type representing a NeoPixel device
  - `src/hardware/display.rs` contains `Display`, a higher-level struct for drawing to the screen
- `src/tasbot.rs` contains TASBot-specific definitions

### Compiled programs

- Primary binaries
  - `image` shows a static image on the screen, or gifs with `--gif`
- Examples
  - `fill` fills the screen with green
  - `clear` clears the screen
  - `flashes` fills the screen with red, waits, fills the screen with green, waits, in a loop
  - `dot` shows a moving dot on the screen (for the NeoPixel Ring with 24 LEDs)

Most programs support arguments; pass `--help` for a list of supported arguments. `--brightness`, in particular, should be available for any command which draws non-black pixels.

### Some notes

The brightness is by default capped at 10%. To increase the default brightness, change the definition of `MAX_BRIGHTNESS` in `src/hardware/display.rs`. You can disable the cap completely using `--features dont-cap-brightness`
