#![no_std]
#![no_main]
#![feature(abi_avr_interrupt)]

use atmega_hal::clock::Clock;
use core::cell::{self, RefCell};
use embedded_hal::delay::DelayNs;
use embedded_hal_bus::i2c;
use panic_halt as _;

mod as1115;

type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<CoreClock>;
type I2c = atmega_hal::i2c::I2c<CoreClock>;
type Mutex<T> = avr_device::interrupt::Mutex<cell::Cell<T>>;
type Timer3 = atmega_hal::pac::TC3;
type Timer3Prescalar = avr_device::atmega32u4::tc3::tccr3b::CS3_A;

static TOGGLE_COUNT: avr_device::interrupt::Mutex<cell::Cell<u32>> =
    avr_device::interrupt::Mutex::new(cell::Cell::new(0));

#[avr_device::interrupt(atmega32u4)]
fn TIMER1_COMPA() {

}

fn tone(timer3: &Timer3, frequency: u16, duration: u16) {
    // TONE_TIMER_STATE.toggle_count = if duration > 0 {
    //     Some(2 * frequency as u64 * duration as u64 / 1000)
    // } else {
    //     None
    // };

    // WGM3 = 0b0100, CTC mode
    // CS3 = 0b001, prescalar 1 or 0b011, prescalar 64
    // OCR3A = CoreClockHz / TargetHz / Prescalar - 1
    let mut ocr: u32 = CoreClock::FREQ / frequency as u32 / 2 - 1;
    let mut prescalar = Timer3Prescalar::DIRECT;
    if ocr > 0xFFFF {
        ocr = CoreClock::FREQ / frequency as u32 / 2 / 64 - 1;
        prescalar = Timer3Prescalar::PRESCALE_64;
    }

    timer3.tccr3a.write(|w| w.wgm3().bits(0b00));
    timer3
        .tccr3b
        .write(|w| w.cs3().variant(prescalar).wgm3().bits(0b01));
    timer3.ocr3a.write(|w| w.bits(ocr as u16));
    timer3.timsk3.write(|w| w.ocie3a().set_bit());

    unsafe {
        avr_device::interrupt::enable();
    }
}

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

    let switch_pins = [
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

    let mut board_digits = as1115::AS1115::new(i2c::RefCellDevice::new(&i2c_ref_cell), None);
    board_digits.init(3, 3).unwrap();

    let mut board_leds = is31fl3731::IS31FL3731::new(i2c::RefCellDevice::new(&i2c_ref_cell), 0x74);
    board_leds.setup_blocking(&mut delay).unwrap();

    let timer3 = dp.TC3;

    let mut led_num: u8 = 0;
    let mut digit_num: u16 = 0;
    loop {
        if switch_pins[0].is_low() {
            led_num = 0;
            digit_num = 0;
            tone(&timer3, 4000, 100);
        }

        board_leds.pixel_blocking(led_num, 255).unwrap();
        board_digits.display_number(digit_num).unwrap();
        delay.delay_ms(1000);

        board_leds.pixel_blocking(led_num, 0).unwrap();
        led_num = (led_num + 1) % 144;
        digit_num = (digit_num + 1) % 1000;
    }
}
