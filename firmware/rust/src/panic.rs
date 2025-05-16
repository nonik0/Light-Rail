use core::ptr::addr_of;
use atmega_hal::delay;
use embedded_hal::delay::DelayNs;

use crate::{Delay, I2c, NUM_DIGITS, DIGITS_I2C_ADDR, DIGITS_INTENSITY};

#[macro_export]
macro_rules! panic_with_error {
    ($error_code:expr) => {{
        crate::panic::set_error_code($error_code);
        panic!();
    }};
}

static mut ERROR_CODE: u16 = 0;
pub fn set_error_code(error_code: u16) {
    unsafe {
        ERROR_CODE = error_code;
    }
}

pub fn trace(code: &[u8]) {
    // this unsafe is safe because we are only calling it from the main function
    // so don't call it from interrupt handlers (or use mutex)
    // unsafe {
    //     for (i, &byte) in code.iter().take(PANIC_MSG_MAX_LEN - 1).enumerate() {
    //         PANIC_MSG[i] = byte;
    //     }
    //     PANIC_MSG[code.len().min(PANIC_MSG_MAX_LEN - 1)] = 0; // Add zero byte at the end
    // }
}

#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { atmega_hal::Peripherals::steal() };
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
    let mut board_digits = as1115::AS1115::new(i2c, DIGITS_I2C_ADDR);
    board_digits.init(NUM_DIGITS, DIGITS_INTENSITY).unwrap();

    let mut error_code: u16 = 0;
    unsafe {
        error_code = ERROR_CODE;
    }

    loop {
        board_digits.display_ascii(b"err").unwrap();
        delay.delay_ms(500);
        board_digits.display_number(error_code).unwrap();
        delay.delay_ms(1000);
    }
}

/*
#[macro_export]
macro_rules! panic_with_error {
    ($msg:expr) => {{
        crate::panic::trace($msg.as_bytes());
        panic!();
    }};
}

// temporary hack for debugging due to compilation issues with panic handler
// using as both "breadcrumbs" and panic message
// TODO: no scrolling for now, but if it would help debugging
const PANIC_MSG_MAX_LEN: usize = 32; // Reduce size to save memory
static mut PANIC_MSG: [u8; PANIC_MSG_MAX_LEN] = [0; PANIC_MSG_MAX_LEN];
pub fn trace(code: &[u8]) {
    // this unsafe is safe because we are only calling it from the main function
    // so don't call it from interrupt handlers (or use mutex)
    unsafe {
        for (i, &byte) in code.iter().take(PANIC_MSG_MAX_LEN - 1).enumerate() {
            PANIC_MSG[i] = byte;
        }
        PANIC_MSG[code.len().min(PANIC_MSG_MAX_LEN - 1)] = 0; // Add zero byte at the end
    }
}

// TODO: investigate linker issue with panic handler
#[panic_handler]
fn panic(_: &core::panic::PanicInfo) -> ! {
    avr_device::interrupt::disable();

    let dp = unsafe { atmega_hal::Peripherals::steal() };
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
    let mut board_digits = as1115::AS1115::new(i2c, DIGITS_I2C_ADDR);
    board_digits.init(NUM_DIGITS, DIGITS_INTENSITY).unwrap();

    let panic_msg_len = unsafe { PANIC_MSG.iter().position(|&x| x == 0).unwrap_or(PANIC_MSG_MAX_LEN) };

    // Allocate `padded_msg` dynamically to reduce stack usage
    let mut padded_msg = heapless::Vec::<u8, { PANIC_MSG_MAX_LEN + NUM_DIGITS as usize }>::new();
    padded_msg.extend_from_slice(&[b' '; NUM_DIGITS as usize]).unwrap();
    unsafe {
        padded_msg.extend_from_slice(&PANIC_MSG[..panic_msg_len]).unwrap();
    }
    padded_msg.extend_from_slice(&[b' '; NUM_DIGITS as usize]).unwrap();

    let mut offset: usize = 0;
    loop {
        let display_slice = &padded_msg[offset..offset + NUM_DIGITS as usize];
        board_digits.display_ascii(display_slice).unwrap();
        offset = (offset + 1) % (panic_msg_len + NUM_DIGITS as usize);
        delay.delay_ms(300);
    }
    loop {
        delay.delay_ms(1000);
    }
}
*/