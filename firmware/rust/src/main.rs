#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(const_trait_impl)]
#![feature(panic_info_message)]
#![feature(type_alias_impl_trait)]
#![allow(unused)]

use atmega_hal::adc;
use atmega_hal::port::{mode::Input, *};
use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::i2c::{self, RefCellDevice};
use random::Rand;
use random_trait::Random;
use static_cell::make_static;

type Adc = atmega_hal::adc::Adc<CoreClock>;
type Channel = atmega_hal::adc::Channel;
#[cfg(feature = "atmega32u4")]
type CoreClock = atmega_hal::clock::MHz8;
#[cfg(feature = "atmega328p")]
type CoreClock = atmega_hal::clock::MHz16;
type Delay = atmega_hal::delay::Delay<CoreClock>;
type I2c = atmega_hal::i2c::I2c<CoreClock>;

mod common;
mod game;
mod input;
mod location;
mod modes;
mod panic;
mod platform;
mod random;
mod switch;
#[cfg_attr(not(feature = "atmega32u4"), path = "notone.rs")]
mod tone;
mod train;

const BASE_DELAY: u32 = 10;
const NUM_BUTTONS: usize = 12;
const NUM_DIGITS: u8 = 3;
const DIGITS_I2C_ADDR: u8 = as1115::constants::DEFAULT_ADDRESS;
const LEDS_I2C_ADDR: u8 = is31fl3731::DEFAULT_ADDRESS;
const DIGITS_INTENSITY: u8 = 3;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);
    let mut delay = Delay::new();
    let i2c = I2c::new(
        dp.TWI,
        #[cfg(feature = "atmega32u4")]
        pins.pd1.into_pull_up_input(),
        #[cfg(feature = "atmega328p")]
        pins.pc4.into_pull_up_input(),
        #[cfg(feature = "atmega32u4")]
        pins.pd0.into_pull_up_input(),
        #[cfg(feature = "atmega328p")]
        pins.pc5.into_pull_up_input(),
        400_000,
    );
    let i2c_ref_cell = make_static!(RefCell::new(i2c));

    #[cfg(feature = "atmega32u4")]
    let board_buzzer = tone::TimerTone::new(dp.TC3, pins.pb4.into_output().downgrade());
    #[cfg(feature = "atmega328p")]
    let board_buzzer = tone::TimerTone::new();

    let digits_i2c = i2c::RefCellDevice::new(i2c_ref_cell);
    let mut board_digits = as1115::AS1115::new(digits_i2c, DIGITS_I2C_ADDR);
    board_digits.init(NUM_DIGITS, DIGITS_INTENSITY).unwrap();
    board_digits.clear().unwrap();

    #[cfg(feature = "atmega32u4")]
    let input_pins = [
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
    #[cfg(feature = "atmega328p")]
    let input_pins = [
        pins.pd0.into_pull_up_input().downgrade(),
        pins.pd1.into_pull_up_input().downgrade(),
        pins.pd2.into_pull_up_input().downgrade(),
        pins.pd3.into_pull_up_input().downgrade(),
        pins.pd4.into_pull_up_input().downgrade(),
        pins.pd5.into_pull_up_input().downgrade(),
        pins.pd6.into_pull_up_input().downgrade(),
        pins.pd7.into_pull_up_input().downgrade(),
        pins.pb0.into_pull_up_input().downgrade(),
        pins.pb1.into_pull_up_input().downgrade(),
        pins.pb2.into_pull_up_input().downgrade(),
        pins.pb3.into_pull_up_input().downgrade(),
    ];
    let board_input = input::BoardInput::new(input_pins);

    let leds_i2c = i2c::RefCellDevice::new(i2c_ref_cell);
    let mut board_leds = is31fl3731::IS31FL3731::new(leds_i2c, LEDS_I2C_ADDR);
    board_leds.setup_blocking(&mut delay).unwrap();
    board_leds.clear_blocking().unwrap();

    // generate random seed from ADC temperature sensor
    panic::trace(b"seed");
    let mut adc = Adc::new(dp.ADC, Default::default());
    let mut seed: u32 = 0;
    for i in 0..8 {
        // using 4 LSB bits x 8 readings for 32 bit seed
        let reading = adc.read_blocking(&adc::channel::Temperature);
        let lsb = (reading & 0xF) as u32;
        seed |= lsb << (i * 4);
    }
    random::Rand::seed(seed);
    board_digits
        .display_number(Rand::default().get_u8() as u16)
        .unwrap();

    panic::trace(b"game");
    let mut game = game::Game::new(board_buzzer, board_digits, board_input, board_leds);

    loop {
        game.tick();
        delay.delay_ms(BASE_DELAY);
    }
}
