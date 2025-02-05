#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]
#![feature(const_trait_impl)]
#![feature(panic_info_message)]

use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::i2c;

//type Adc = atmega_hal::adc::Adc<CoreClock>;
//type Channel = atmega_hal::adc::Channel;
type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<CoreClock>;
type I2c = atmega_hal::i2c::I2c<CoreClock>;

mod common;
mod game;
mod location;
mod panic;
mod platform;
mod random;
mod tone; // TODO: contribute tone library/impl for avr-hal
mod train;

const BASE_DELAY: u32 = 10;
const NUM_BUTTONS: usize = 12;
const DIGITS_I2C_ADDR: u8 = as1115::constants::DEFAULT_ADDRESS;
const DIGITS_COUNT: u8 = 3;
const DIGITS_INTENSITY: u8 = 3;
const LEDS_I2C_ADDR: u8 = is31fl3731::DEFAULT_ADDRESS;

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
    let i2c_ref_cell = RefCell::new(i2c);

    // TODO: potentially create abstraction to simplify usage
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

    let board_buzzer = tone::Timer3Tone::new(dp.TC3, pins.pb4.into_output().downgrade());

    let mut board_digits =
        as1115::AS1115::new(i2c::RefCellDevice::new(&i2c_ref_cell), DIGITS_I2C_ADDR);
    board_digits.init(DIGITS_COUNT, DIGITS_INTENSITY).unwrap();
    board_digits.clear().unwrap();

    // TODO: which pins are not ADC?
    // let mut board_floating_pins = [
    //     pins.pb0.into_analog_input(&mut adc).into_channel(),
    //     pins.pd2.into_analog_input(&mut adc).into_channel(),
    //     pins.pd3.into_analog_input(&mut adc).into_channel(),
    //     pins.pd5.into_analog_input(&mut adc).into_channel(),
    //     pins.pf5.into_analog_input(&mut adc).into_channel(),
    //     pins.pf6.into_analog_input(&mut adc).into_channel(),
    //     pins.pf7.into_analog_input(&mut adc).into_channel(),
    // ];
    // let board_entropy = Adc::new(dp.ADC, Default::default());

    let mut board_leds =
        is31fl3731::IS31FL3731::new(i2c::RefCellDevice::new(&i2c_ref_cell), LEDS_I2C_ADDR);
    board_leds.setup_blocking(&mut delay).unwrap();

    let mut game = game::Game::new(board_buttons, board_buzzer, board_digits, board_leds);
    game.restart();

    loop {
        game.tick();

        if game.is_over() {
            // TODO: mode selection
            game.restart();
        }

        delay.delay_ms(BASE_DELAY);
    }
}
