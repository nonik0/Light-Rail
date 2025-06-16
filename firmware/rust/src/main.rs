#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(const_trait_impl)]
#![feature(panic_info_message)]
#![feature(type_alias_impl_trait)]

// TODO: can print to digits before panic halting
#[cfg(not(feature = "panic_to_digits"))]
#[macro_export]
macro_rules! panic_with_error {
    ($error_code:expr) => {
        panic!()
    };
}

use atmega_hal::adc;
use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::i2c::{self};
#[cfg(not(feature = "panic_to_digits"))]
use panic_halt as _;

type Adc = atmega_hal::adc::Adc<CoreClock>;
#[cfg(feature = "atmega32u4")]
type CoreClock = atmega_hal::clock::MHz8;
#[cfg(feature = "atmega328p")]
type CoreClock = atmega_hal::clock::MHz16;
type Delay = atmega_hal::delay::Delay<CoreClock>;
type Eeprom = atmega_hal::eeprom::Eeprom;
type I2c = atmega_hal::i2c::I2c<CoreClock>;

mod cargo;
mod game;
mod game_settings;
mod game_state;
mod input;
mod location;
mod modes;
#[cfg(feature = "panic_to_digits")]
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
const DIGITS_I2C_ADDR: u8 = as1115::DEFAULT_ADDRESS;
const LEDS_I2C_ADDR: u8 = is31fl3731::DEFAULT_ADDRESS;

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
    let i2c_ref_cell = RefCell::new(i2c);

    #[cfg(feature = "atmega32u4")]
    let board_buzzer = tone::TimerTone::new(dp.TC3, pins.pb4.into_output().downgrade());
    #[cfg(feature = "atmega328p")]
    let board_buzzer = tone::TimerTone::new();

    let eeprom = Eeprom::new(dp.EEPROM);
    let settings = game_settings::GameSettings::new(eeprom);

    let mut board_digits =
        as1115::AS1115::new(i2c::RefCellDevice::new(&i2c_ref_cell), DIGITS_I2C_ADDR);
    board_digits.init(settings.digit_brightness_level()).ok();
    board_digits.clear().ok();

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

    let mut board_leds =
        is31fl3731::IS31FL3731::new(i2c::RefCellDevice::new(&i2c_ref_cell), LEDS_I2C_ADDR);
    board_leds.setup_blocking(&mut delay).unwrap(); // TODO: why does OK hang???
    board_leds.clear_blocking().ok();

    // generate random seed from ADC temperature sensor
    let mut adc = Adc::new(dp.ADC, Default::default());
    let mut seed: u32 = 0;
    for i in 0..8 {
        // using 4 LSB bits x 8 readings for 32 bit seed
        let reading = adc.read_blocking(&adc::channel::Temperature);
        let lsb = (reading & 0xF) as u32;
        seed |= lsb << (i * 4);
    }
    random::Rand::seed(seed);

    let cars = [train::Car::default(); game_state::MAX_CARS];
    let mut game = game::Game::new(
        board_buzzer,
        board_digits,
        board_input,
        board_leds,
        cars,
        settings,
    );
    game.restart();

    loop {
        game.tick();
        delay.delay_ms(BASE_DELAY);
    }
}
