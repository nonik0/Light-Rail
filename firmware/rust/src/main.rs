#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(panic_info_message)]
// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::i2c;

type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<CoreClock>;
type I2c = atmega_hal::i2c::I2c<CoreClock>;

// TODO: formalize into own crates
mod as1115;
mod tone;

mod game;
mod platform;
mod train;
mod track;

const DIGITS_I2C_ADDR: u8 = 0x00;
const DIGITS_COUNT: u8 = 3;
const DIGITS_INTENSITY: u8 = 3;
const LEDS_I2C_ADDR: u8 = 0x74;
const LEDS_COUNT: u8 = 144;
const HELLO_MSG: &str = "   HELLO   ";
const ERROR_MSG: &str = "   OSHIT   ";

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let mut delay = Delay::new();
    let i2c = I2c::new(
        dp.TWI,
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        400_000,
    );
    let i2c_ref_cell = RefCell::new(i2c); // not Send/thread safe

    let board_buttons = [
        pins.pb6.into_pull_up_input().downgrade(),
        pins.pb7.into_pull_up_input().downgrade(),
        pins.pc6.into_pull_up_input().downgrade(),
        pins.pc7.into_pull_up_input().downgrade(),
        pins.pd4.into_pull_up_input().downgrade(),
        pins.pe2.into_pull_up_input().downgrade(),
        pins.pd6.into_pull_up_input().downgrade(),
        pins.pd7.into_pull_up_input().downgrade(),
        pins.pf4.into_pull_up_input().downgrade(),
        pins.pf1.into_pull_up_input().downgrade(),
        pins.pf0.into_pull_up_input().downgrade(),
        pins.pe6.into_pull_up_input().downgrade(),
    ];

    let mut board_digits =
        as1115::AS1115::new(i2c::RefCellDevice::new(&i2c_ref_cell), DIGITS_I2C_ADDR);
    board_digits.init(DIGITS_COUNT, DIGITS_INTENSITY).unwrap();
    board_digits.clear().unwrap();

    let mut board_leds =
        is31fl3731::IS31FL3731::new(i2c::RefCellDevice::new(&i2c_ref_cell), LEDS_I2C_ADDR);
    board_leds.setup_blocking(&mut delay).unwrap();

    //let board_piezo = tone::Timer3Tone::new(dp.TC3, pins.pb4.into_output());
    let mut board_piezo = tone::Timer1Tone::new(dp.TC1, pins.pb4.into_output().downgrade());

    board_digits.display_ascii(b"ohi").unwrap();
    delay.delay_ms(1000);

    let mut led_num: u8 = 0;
    let mut digit_num: u16 = 0;
    loop {
        // beep if any button is pressed
        let mut any_button_pressed = false;
        for (i, button) in board_buttons.iter().enumerate() {
            if button.is_low() {
                any_button_pressed = true;
                board_digits.display_number((i + 1) as u16).unwrap();
                board_piezo.tone((i + 1) as u16 * 1000, 0);
                break;
            }
        }

        if !any_button_pressed {
            board_piezo.no_tone();
        }

        board_leds.pixel_blocking(led_num, 255).unwrap();
        board_digits.display_number(digit_num).unwrap();
        delay.delay_ms(500);

        board_leds.pixel_blocking(led_num, 0).unwrap();
        led_num = (led_num + 1) % 144;
        digit_num = (digit_num + 1) % 1000;
    }
}

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { atmega_hal::Peripherals::steal() };
    let pins = atmega_hal::pins!(dp);
    let mut delay = Delay::new();
    let i2c = I2c::new(
        dp.TWI,
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        400_000,
    );
    let mut board_digits = as1115::AS1115::new(i2c, DIGITS_I2C_ADDR);
    board_digits.init(DIGITS_COUNT, DIGITS_INTENSITY).unwrap();

    let mut offset: usize = 0;
    loop {
        board_digits
            .display_string(&ERROR_MSG[offset..offset + DIGITS_COUNT as usize])
            .unwrap();
        offset = (offset + 1) % (ERROR_MSG.len() - DIGITS_COUNT as usize);
        delay.delay_ms(300);
    }
}
