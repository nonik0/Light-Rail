
// reference implementations:
// https://github.com/gleich/is31fl3731/ => TODO: PR to update this lib to embedded-hal v1
// https://github.com/adafruit/Adafruit_IS31FL3731

use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

pub struct IS31FL3731<I2C> {
    pub i2c: I2C,
    pub address: u8,
    frame: u8,
}

impl<I2C, E> IS31FL3731<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C) -> Self {
        Self {
            i2c,
            address: 0x74,
            frame: 0,
        }
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub fn begin(&mut self, delay: &mut dyn DelayNs) -> Result<(), Error<E>> {
        self.sleep(true)?;
        delay.delay_ms(10);
        self.mode(addresses::PICTURE_MODE)?;
        self.show_frame(self.frame)?;
        for frame in 0..constants::MAX_FRAMES {
            self.clear(Some(frame))?;
            for col in 0..18 {
                self.write_register(frame, addresses::ENABLE_OFFSET + col, 0xFF)?;
            }
        }
        self.audio_sync(false)?;
        self.sleep(false)?;
        Ok(())
    }

    pub fn set_led_pwm(&mut self, lednum: u8, brightness: u8) -> Result<(), Error<E>> {
        if lednum >= constants::MAX_LEDS {
            return Err(Error::InvalidLocation(lednum.into()));
        }
        self.write_register(self.frame, addresses::COLOR_OFFSET + lednum, brightness)?;
        Ok(())
    }

    pub fn audio_sync(&mut self, yes: bool) -> Result<(), E> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::AUDIOSYNC,
            if yes { 1 } else { 0 },
        )?;
        Ok(())
    }

    pub fn clear(&mut self, frame: Option<u8>) -> Result<(), E> {
        let frame = frame.unwrap_or(self.frame);
        self.set_bank(frame)?;
        let mut payload = [0; 25];
        for row in 0..(constants::MAX_ROWS - 1) {
            payload[0] = addresses::COLOR_OFFSET + row * 24;
            self.i2c.write(self.address, &payload)?;
        }
        Ok(())
    }

    pub fn set_frame(&mut self, frame: u8) -> Result<(), Error<E>> {
        if frame > constants::MAX_FRAMES {
            return Err(Error::InvalidFrame(frame.into()));
        }
        self.frame = frame;
        Ok(())
    }

    pub fn show_frame(&mut self, frame: u8) -> Result<(), Error<E>> {
        if frame > constants::MAX_FRAMES {
            return Err(Error::InvalidFrame(frame.into()));
        }
        self.frame = frame;
        self.write_register(addresses::CONFIG_BANK, addresses::FRAME, frame)?;
        Ok(())
    }

    pub fn mode(&mut self, mode: u8) -> Result<(), E> {
        self.write_register(addresses::CONFIG_BANK, addresses::MODE_REGISTER, mode)?;
        Ok(())
    }



    pub fn sleep(&mut self, yes: bool) -> Result<(), E> {
        self.write_register(
            addresses::CONFIG_BANK,
            addresses::SHUTDOWN,
            if yes { 0 } else { 1 },
        )?;
        Ok(())
    }

    fn set_bank(&mut self, bank: u8) -> Result<(), E> {
        self.i2c
            .write(self.address, &[addresses::BANK_ADDRESS, bank])?;
        Ok(())
    }

    fn write_register(&mut self, bank: u8, register: u8, value: u8) -> Result<(), E> {
        self.set_bank(bank)?;
        self.i2c.write(self.address, &[register, value])?;
        Ok(())
    }
}

pub mod constants {
    pub const MAX_LEDS: u8 = 144;
    pub const MAX_ROWS: u8 = 7;
    pub const MAX_FRAMES: u8 = 8;
}

pub mod addresses {
    pub const MODE_REGISTER: u8 = 0x00;
    pub const FRAME: u8 = 0x01;
    // pub const AUTOPLAY1: u8 = 0x02;
    // pub const AUTOPLAY2: u8 = 0x03;
    // pub const BLINK: u8 = 0x05;
    pub const AUDIOSYNC: u8 = 0x06;
    // pub const BREATH1: u8 = 0x08;
    // pub const BREATH2: u8 = 0x09;
    pub const SHUTDOWN: u8 = 0x0A;
    // pub const GAIN: u8 = 0x0B;
    // pub const ADC: u8 = 0x0C;

    pub const CONFIG_BANK: u8 = 0x0B;
    pub const BANK_ADDRESS: u8 = 0xFD;

    pub const PICTURE_MODE: u8 = 0x00;
    // pub const AUTOPLAY_MODE: u8 = 0x08;
    // pub const AUDIOPLAY_MODE: u8 = 0x18;

    pub const ENABLE_OFFSET: u8 = 0x00;
    // pub const BLINK_OFFSET: u8 = 0x12;
    pub const COLOR_OFFSET: u8 = 0x24;
}

#[derive(Clone, Copy, Debug)]
pub enum Error<E> {
    I2cError(E),
    InvalidLocation(u8),
    InvalidFrame(u8),
}

// impl i2c::Error for Error<I2cError> {
//     fn kind(&self) -> i2c::ErrorKind {
//         match self {
//             Error::I2cError(_) => i2c::ErrorKind::I2c,
//             Error::InvalidLocation(_) => i2c::ErrorKind::Other,
//             Error::InvalidFrame(_) => i2c::ErrorKind::Other,
//         }
//     }
// }

impl<E> From<E> for Error<E> {
    fn from(error: E) -> Self {
        Error::I2cError(error)
    }
}
