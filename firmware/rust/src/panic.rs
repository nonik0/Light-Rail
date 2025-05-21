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

// #[panic_handler]
// fn panic(_: &core::panic::PanicInfo) -> ! {
//     avr_device::interrupt::disable();

//     let dp = unsafe { atmega_hal::Peripherals::steal() };
//     let pins = atmega_hal::pins!(dp);
//     let mut delay = Delay::new();
//     let i2c = I2c::new(
//         dp.TWI,
//         #[cfg(feature = "atmega32u4")]
//         pins.pd1.into_pull_up_input(),
//         #[cfg(feature = "atmega328p")]
//         pins.pc4.into_pull_up_input(),
//         #[cfg(feature = "atmega32u4")]
//         pins.pd0.into_pull_up_input(),
//         #[cfg(feature = "atmega328p")]
//         pins.pc5.into_pull_up_input(),
//         400_000,
//     );
//     let mut board_digits = as1115::AS1115::new(i2c, DIGITS_I2C_ADDR);
//     board_digits.init(NUM_DIGITS, DIGITS_INTENSITY).unwrap();

//     let mut error_code: u16 = 0;
//     unsafe {
//         error_code = ERROR_CODE;
//     }

//     loop {
//         board_digits.display_ascii(b"err").unwrap();
//         delay.delay_ms(500);
//         board_digits.display_number(error_code).unwrap();
//         delay.delay_ms(1000);
//     }
// }
