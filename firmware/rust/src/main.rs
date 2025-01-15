#![no_std]
#![no_main]
#![allow(dead_code)] // quiet unused warnings

use core::cell::RefCell;
use embedded_hal::delay::DelayNs;
//use embedded_hal::digital::InputPin;
use embedded_hal_bus::i2c;
use panic_halt as _;

mod as1115;

type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<CoreClock>;
type I2c = atmega_hal::i2c::I2c<CoreClock>;

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

    let switch_pin = pins.pb6.into_pull_up_input();

    let mut board_digits = as1115::AS1115::new(i2c::RefCellDevice::new(&i2c_ref_cell), None);
    board_digits.init(3, 3).unwrap();

    //board_digits.display_string("Sup").unwrap();
    //delay.delay_ms(1000);

    let mut board_leds = is31fl3731::IS31FL3731::new(i2c::RefCellDevice::new(&i2c_ref_cell), 0x74);
    board_leds.setup_blocking(&mut delay).unwrap();

    let mut led_num: u8 = 0;
    let mut digit_num: u16 = 0;
    loop {
        if switch_pin.is_low() {
            led_num = 0;
            digit_num = 0;
        }

        board_leds.pixel_blocking(led_num, 255).unwrap();
        board_digits.display_number(digit_num).unwrap();
        delay.delay_ms(1000);

        board_leds.pixel_blocking(led_num, 0).unwrap();
        led_num = (led_num + 1) % 144;
        digit_num = (digit_num + 1) % 1000;
    }
}
