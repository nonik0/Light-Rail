
use core::ptr::addr_of;
use embedded_hal::delay::DelayNs;

use crate::{Delay, I2c, DIGITS_COUNT, DIGITS_I2C_ADDR, DIGITS_INTENSITY};

#[macro_export]
macro_rules! panic_to_digits {
    ($msg:expr) => {{
        set_panic_msg($msg.as_bytes());
        panic!();
    }};
}

// temporary hack for debugging due to compilation issues with panic handler
// using as both "breadcrumbs" and panic message
// TODO: no scrolling for now, but if it would help debugging
const PANIC_MSG_MAX_LEN: usize = 20;
static mut PANIC_MSG: [u8; PANIC_MSG_MAX_LEN] = [0; PANIC_MSG_MAX_LEN];
pub fn set_panic_msg(code: &[u8]) {
    // this unsafe is safe because we are only calling it from the main function
    // so don't call it from interrupt handlers (or use mutex)
    unsafe {
        for (i, &byte) in code.iter().take(PANIC_MSG_MAX_LEN).enumerate() {
            PANIC_MSG[i] = byte;
        }
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
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        400_000,
    );
    let mut board_digits = as1115::AS1115::new(i2c, DIGITS_I2C_ADDR);
    board_digits.init(DIGITS_COUNT, DIGITS_INTENSITY).unwrap();

    let panic_msg_len = unsafe { PANIC_MSG.iter().position(|&x| x == 0).unwrap_or(PANIC_MSG_MAX_LEN) };
    let mut padded_msg = [b' '; PANIC_MSG_MAX_LEN + DIGITS_COUNT as usize];
    let panic_msg_slice = unsafe { core::slice::from_raw_parts(addr_of!(PANIC_MSG) as *const u8, panic_msg_len) };
    padded_msg[DIGITS_COUNT as usize..DIGITS_COUNT as usize + panic_msg_len].copy_from_slice(panic_msg_slice);

    let mut offset: usize = 0;
    loop {
        let display_slice = &padded_msg[offset..offset + DIGITS_COUNT as usize];
        board_digits.display_ascii(display_slice).unwrap();
        offset = (offset + 1) % (panic_msg_len + DIGITS_COUNT as usize);
        delay.delay_ms(300);
    }
}
