#![no_std]
#![no_main]

use embedded_graphics::mock_display::ColorMapping;
use embedded_hal::digital::InputPin;
// use embedded_hal::digital::{InputPin, OutputPin};
use panic_halt as _;

use vcc_gnd_yd_rp2040::hal::gpio::{FunctionI2C, Pin};

// use vcc_gnd_yd_rp2040::hal::I2C;

use heapless::String;
use core::fmt::Write;

use hal::fugit::RateExtU32;

use ssd1306::{prelude::*, Ssd1306, size::DisplaySize128x64, I2CDisplayInterface};

use embedded_graphics::{mono_font::MonoTextStyleBuilder, text::Baseline};
use embedded_graphics::pixelcolor::BinaryColor;
use embedded_graphics::mono_font::ascii::FONT_6X10;
use embedded_graphics::{prelude::*, text::Text};

use vcc_gnd_yd_rp2040::{entry, hal};
use vcc_gnd_yd_rp2040::{
    hal::{
        clocks::{init_clocks_and_plls, Clock},
        pac, 
        watchdog::Watchdog,
        Sio,
    },
    Pins, XOSC_CRYSTAL_FREQ,
};


#[entry] // Начало работы прошивки.
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap(); // Видимо тут мы подключаемся к переферии то есть к vcc_gnd_yd_rp2040 пинам чтобы не подключатся вручную
    let core = pac::CorePeripherals::take().unwrap();

    let mut watchdog = Watchdog::new(pac.WATCHDOG);
    
    let clocks = init_clocks_and_plls(
        XOSC_CRYSTAL_FREQ,
        pac.XOSC, // Модуль кварца	Включает внешний генератор и усиливает герцовку подключённых модулей чтобы они работали быстрее
        pac.CLOCKS,
        pac.PLL_SYS, // Умножает частоту для системной шины (обычно до 125 MHz)
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )

    .ok()
    .unwrap();
    
    let sio = Sio::new(pac.SIO);
    let pins = Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());

    let sda_pin: Pin<_, FunctionI2C, _> = pins.gpio18.reconfigure();
    let scl_pin: Pin<_, FunctionI2C, _> = pins.gpio19.reconfigure();

    
    let i2c = hal::I2C::i2c1(
        pac.I2C1,
        sda_pin,
        scl_pin,
        400.kHz(),
        &mut pac.RESETS,
        &clocks.system_clock,
    );
    
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();

    display.init().unwrap();
    let text_style = MonoTextStyleBuilder::new()
    .font(&FONT_6X10)
    .text_color(BinaryColor::On)
    .build();
    
    let mut buf: String<32> = String::new();
    
    let mut counter: i32 = 0; 
    let mut usr_btn = pins.user_key.into_pull_up_input();

    let mut prev_state = false;

    loop {
        let curr_state = usr_btn.is_low().unwrap();
        
        if curr_state == true && prev_state == false {
            let _ = display.clear(BinaryColor::Off);
            counter += 1 ;
            buf.clear();
            write!(buf, "{}", counter).unwrap();
            let text = buf.as_str();
            Text::with_baseline(text, Point::new(0, 16), text_style, Baseline::Top).draw(&mut display).unwrap();
            display.flush().unwrap();
            delay.delay_ms(10);
        }

        prev_state = curr_state;
        delay.delay_ms(60);
    }
}
