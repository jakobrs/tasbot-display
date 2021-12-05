# tasbot-display

Experiment in handling the TASBot display from Rust

### Repository structure

- `src/hardware` contains the code used for drawing to the display
  - `src/hardware/neopixel_device.rs` contains `NeoPixelDevice`, a data type representing a NeoPixel device
  - `src/hardware/display.rs` contains `Display`, a higher-level struct for drawing to the screen
- `src/tasbot.rs` contains TASBot-specific definitions

### Compiled programs

- Examples
  - `image` shows a static image on the screen, or gifs with `--gif`
  - `fill` fills the screen with green
  - `clear` clears the screen
  - `flashes` fills the screen with red, waits, fills the screen with green, waits, in a loop
  - `dot` shows a moving dot on the screen (for the NeoPixel Ring with 24 LEDs)

Most programs support arguments; pass `--help` for a list of supported arguments. `--brightness`, in particular, should be available for any command which draws non-black pixels.

### Some notes

The brightness is by default capped at 10%. To increase the default brightness, change the definition of `MAX_BRIGHTNESS` in `src/hardware/display.rs`. You can disable the cap completely using `--features dont-cap-brightness`

### Compile instructions

Natively:
```bash
RUSTFLAGS="-C target-cpu=native" cargo build [--release] --examples
```

Cross-compiling:
```
# Use cortex-a53 if compiling for the Pi Zero 2, arm1176jzf-s if compiling for the Pi Zero W
RUSTFLAGS="-C target-cpu=cortex-a53 -C linker=lld" cargo build --target arm-unknown-linux-musleabihf [--release] --examples
```
Note that this assumes you have lld installed.

### Creating tar.gz file

```bash
./package.sh <debug or release> [optional tag to be added to file name]
```

### Setting up your Pi

- GPIO 10 (SPI0 MOSI) is used for communication
- Make sure SPI is enabled in `raspi-config` (under "Interfaces")
