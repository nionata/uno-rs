#![no_std]
#![no_main]

use core::fmt::Write;
use embedded_hal::i2c::I2c as _;
use panic_halt as _;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};

// SSD1309 init sequence. The ssd1306 driver's own init() sends charge pump
// commands (0x8D/0x14) that SSD1309 doesn't support, so we do it manually.
const SSD1309_INIT: &[u8] = &[
    0x00, // control byte: command stream
    0xAE, // display off
    0xD5, 0x80, // clock divide / osc freq
    0xA8, 0x3F, // multiplex ratio = 63
    0xD3, 0x00, // display offset = 0
    0x40, // start line = 0
    0xA1, // seg re-map
    0xC8, // com scan dir
    0xDA, 0x12, // com pins
    0x81, 0x7F, // contrast
    0xD9, 0xF1, // pre-charge
    0xDB, 0x40, // vcomh
    0xA4, // output follows RAM
    0xA6, // normal display
    0x20, 0x02, // page addressing mode
    0xAF, // display on
];

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    let mut led = pins.d13.into_output();

    let mut i2c = arduino_hal::I2c::new(
        dp.TWI,
        pins.a4.into_pull_up_input(),
        pins.a5.into_pull_up_input(),
        50000,
    );

    arduino_hal::delay_ms(100);
    i2c.write(0x3C, SSD1309_INIT).unwrap();

    let interface = I2CDisplayInterface::new(i2c);
    let mut display =
        Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0).into_terminal_mode();

    display.clear().unwrap();
    write!(display, "Hello, Peppernet!").unwrap();

    loop {
        led.toggle();
        arduino_hal::delay_ms(500);
    }
}
