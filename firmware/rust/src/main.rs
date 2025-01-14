#![no_std]
#![no_main]

//use atmega_hal::usart::{Baudrate, Usart};
use embedded_hal::delay::DelayNs;
use is31fl3731::IS31FL3731;
use panic_halt as _;

mod is31fl3731;

type CoreClock = atmega_hal::clock::MHz8;
type Delay = atmega_hal::delay::Delay<crate::CoreClock>;
type I2c = atmega_hal::i2c::I2c<crate::CoreClock>;

#[avr_device::entry]
fn main() -> ! {
    let dp = atmega_hal::Peripherals::take().unwrap();
    let pins = atmega_hal::pins!(dp);

    let mut delay = Delay::new();
    let mut i2c = I2c::new(
        dp.TWI,
        pins.pd1.into_pull_up_input(),
        pins.pd0.into_pull_up_input(),
        400_000,
    );
    let mut boardLeds = is31fl3731::IS31FL3731::new(i2c);
    // let mut serial = Usart::new(
        //     dp.USART1,
        //     pins.pd2,
        //     pins.pd3.into_output(),
        //     Baudrate::<crate::CoreClock>::new(57600),
        // );
        

    boardLeds.begin(&mut delay).unwrap();

    let mut ledNum: u8 = 0;
    loop {
        boardLeds.set_led_pwm(ledNum, 255).unwrap();
        delay.delay_ms(1000);
        boardLeds.set_led_pwm(ledNum, 0).unwrap();
        ledNum = (ledNum + 1) % 144;
    }
}



/* default serial for leonardo
macro_rules! default_serial {
    ($p:expr, $pins:expr, $baud:expr) => {
        $crate::Usart::new(
            $p.USART1,
            $pins.d0, pd2
            $pins.d1.into_output(), pd3
            $crate::hal::usart::BaudrateExt::into_baudrate($baud),
        )
    };
}
*/