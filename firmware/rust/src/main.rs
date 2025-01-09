#![no_std]
#![no_main]

use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use panic_halt as _;

type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<crate::CoreClock>;
type I2c = atmega_hal::i2c::I2c<crate::CoreClock>;

fn delay_ms(ms: u16) {
    Delay::new().delay_ms(u32::from(ms))
}

#[allow(dead_code)]
fn delay_us(us: u32) {
    Delay::new().delay_us(us)
}

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut led = pins.pb7.into_output();
    let mut i2c = I2c::new(
        dp.TWI,
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        400_000,
    );
    let mut serial = Usart::new(
        dp.USART1,
        pins.pd2,
        pins.pd3.into_output(),
        Baudrate::<crate::CoreClock>::new(57600),
    );

    ufmt::uwriteln!(&mut serial, "Write direction test:\r").unwrap();
    i2c.i2cdetect(&mut serial, atmega_hal::i2c::Direction::Write)
        .unwrap();
    ufmt::uwriteln!(&mut serial, "\r\nRead direction test:\r").unwrap();
    i2c.i2cdetect(&mut serial, atmega_hal::i2c::Direction::Read)
        .unwrap();

    loop {
        led.toggle();
        delay_ms(1000);
    }
}