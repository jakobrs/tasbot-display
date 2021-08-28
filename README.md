# tasbot-display

Experiment in handling the TASBot display from Rust

### Repository structure

- `src/hardware` contains the code used for drawing to the display
  - `src/hardware/neopixel_device.rs` contains `NeoPixelDevice`, a data type representing a NeoPixel device
  - `src/hardware/display.rs` contains `Display`, a higher-level struct for drawing to the screen
- `src/tasbot.rs` contains tASBot-specific definitions
- `src/bin` contains binaries
  - `src/bin/image.rs` shows a static image on the screen
- `examples` contains examples
  - `examples/fill.rs` Fills the screen with green
  - `examples/clear.rs` Clears the screen
