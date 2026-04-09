# OLED Display Counter

Button-controlled counter displayed on a mini OLED screen (128x64). Each press of the onboard USER button increments the counter with proper debouncing. Demonstrates I2C communication and embedded graphics in Rust.

## Features

- I2C communication with SSD1306 OLED display (128x64)
- Button debouncing for reliable input
- Real-time counter display with embedded graphics
- Buffered graphics mode for smooth updates
- Low-level Rust firmware (no_std)

## Hardware

- **Board:** YD-RP2040
- **Microcontroller:** RP2040 (Dual ARM Cortex-M0+ @ 125 MHz)
- **Components:**
  - SSD1306 OLED Display 128x64 (I2C interface)
  - Onboard USER button

## Wiring Diagram

| Component | YD-RP2040 Pin | Notes |
|-----------|---------------|-------|
| OLED VCC  | 3.3V          | Power supply |
| OLED GND  | GND           | Ground |
| OLED SDA  | GPIO18        | I2C Data line |
| OLED SCL  | GPIO19        | I2C Clock line |
| Button    | USER KEY      | Onboard (no wiring needed) |

### I2C Configuration
- **Bus:** I2C1
- **Speed:** 400 kHz (Fast mode)
- **Address:** 0x3C (SSD1306 default)

## How to Build

1. Install Rust and the ARM Cortex-M0+ target:
```bash
rustup target add thumbv6m-none-eabi
```

2. Build the project:
```bash
cargo build --release
```

3. Convert ELF to UF2:
```bash
cargo install elf2uf2-rs
elf2uf2-rs target/thumbv6m-none-eabi/release/hello_rust hello_rust.uf2
```

## How to Flash

1. Hold **BOOT** button on YD-RP2040
2. Press **RESET** button (or connect USB while holding BOOT)
3. Board appears as USB drive **RPI-RP2**
4. Copy **hello_rust.uf2** to the drive
5. Board will reboot automatically

## Built With

**Core Dependencies:**
- **cortex-m** v0.7.7 - Low-level ARM Cortex-M primitives
- **cortex-m-rt** v0.7.5 - Startup code and runtime
- **panic-halt** v1.0.0 - Panic handler
- **rp2040-hal** v0.11.0 - Hardware abstraction layer
- **vcc-gnd-yd-rp2040** v0.6.0 - Board support package

**Display & Graphics:**
- **ssd1306** v0.10.0 - OLED display driver
- **embedded-graphics** v0.8.0 - 2D graphics primitives and text
- **display-interface-i2c** v0.5.0 - I2C display interface
- **heapless** v0.9.1 - Stack-allocated String (no heap allocation)

**Utilities:**
- **embedded-hal** v1.0.0 - Hardware abstraction traits
- **fugit** v0.3.7 - Time and frequency types
- **usb-device** v0.3.2 - USB device framework
- **usbd-serial** v0.2.2 - USB serial implementation

## How it Works

1. **I2C Initialization:** Sets up I2C1 bus at 400 kHz on GPIO18 (SDA) and GPIO19 (SCL)
2. **Display Setup:** Initializes SSD1306 in buffered graphics mode for smooth rendering
3. **Button Monitoring:** Continuously checks USER button state with debouncing
4. **Counter Logic:**
   - Detects button press (transition from HIGH to LOW)
   - Increments counter by 1
   - Clears display buffer
   - Converts integer to string using heapless::String
   - Renders text at position (0, 16) using 6x10 pixel font
   - Flushes buffer to display
5. **Debouncing:** Uses state comparison with 60ms polling interval to prevent double-counting

## Display Details

- **Font:** 6x10 ASCII monospace (FONT_6X10)
- **Text Color:** White (BinaryColor::On)
- **Background:** Black (cleared on each update)
- **Position:** Top-left corner with 16px vertical offset
- **Update rate:** On button press only (not continuous refresh)

## Troubleshooting

| Issue | Solution |
|-------|----------|
| Display shows nothing | Check I2C wiring (SDA/SCL), verify 3.3V power |
| Counter doesn't increment | Check button debouncing delay, verify USER button connection |
| I2C errors | Reduce I2C speed to 100 kHz, check pull-up resistors |
| Display flickers | Ensure proper buffered mode initialization |

## Learning Points

This project demonstrates:
- I2C peripheral configuration and communication
- Working with external I2C devices (SSD1306)
- Buffered graphics rendering
- Button debouncing techniques
- Stack-allocated string formatting in no_std environment
- Embedded graphics text rendering

## License

MIT
