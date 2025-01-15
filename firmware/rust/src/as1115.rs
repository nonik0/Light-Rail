// reference implementation:
// https://github.com/blemasle/arduino-as1115

//use embedded_hal::delay::DelayNs;
use embedded_hal::i2c::I2c;

pub struct AS1115<I2C> {
    pub i2c: I2C,
    pub address: u8,
    pub num_digits: u8,
}

impl<I2C, E> AS1115<I2C>
where
    I2C: I2c<Error = E>,
{
    pub fn new(i2c: I2C, address: Option<u8>) -> Self {
        Self {
            i2c,
            address: address.unwrap_or(constants::DEFAULT_ADDRESS),
            num_digits: 0,
        }
    }

    pub fn destroy(self) -> I2C {
        self.i2c
    }

    pub fn init(&mut self, num_digits: u8, intensity: u8) -> Result<(), AS1115Error<E>> {
        self.num_digits = constants::MAX_DIGITS.min(num_digits);

        self.write_register_to_addr(constants::DEFAULT_ADDRESS, addresses::SHUTDOWN, constants::shutdown_mode::NORMAL_OPERATION | constants::shutdown_mode::RESET_FEATURE)?;

        if self.address != constants::DEFAULT_ADDRESS {
            self.write_register_to_addr(constants::DEFAULT_ADDRESS, addresses::SELF_ADDRESSING, constants::self_addressing::USER_SET_ADDR)?;
        }

        self.write_register(addresses::DECODE_MODE, constants::decode_mode::NO_DECODE)?;
        self.write_register(addresses::SCAN_LIMIT, num_digits - 1)?;
        self.set_intensity(intensity)?;

        Ok(())
    }

    // TOOD: debug
    pub fn display_string(&mut self, string: &str) -> Result<(), AS1115Error<E>> {
        let mut index = 0;
        for c in string.chars() {
            let digit = match c {
                '0'..='9' => DIGITS[(c as u8 - b'0') as usize],
                'a'..='z' => LETTERS[(c as u8 - b'a') as usize],
                'A'..='Z' => LETTERS[(c as u8 - b'A') as usize],
                _ => 0,
            };
            self.display_digit(index, DIGITS[digit as usize])?;
            index += 1;
            if index >= self.num_digits {
                break;
            }
        }
        Ok(())
    }

    pub fn display_number(&mut self, number: u16) -> Result<(), AS1115Error<E>> {
        let mut num = number;
        for i in 0..self.num_digits {
            let digit = num % 10;
            self.display_digit(self.num_digits - 1 - i, DIGITS[digit as usize])?;
            num /= 10;
        }
        Ok(())
    }

    pub fn display_digit(&mut self, digit: u8, value: u8) -> Result<(), AS1115Error<E>> {
        if digit >= self.num_digits {
            return Err(AS1115Error::InvalidLocation(digit));
        }
        self.write_register(addresses::DIGIT_OFFSET + digit, value)?;
        Ok(())
    }

    pub fn set_intensity(&mut self, intensity: u8) -> Result<(), AS1115Error<E>> {
        // TODO: max intensity?
        self.write_register(addresses::GLOBAL_INTENSITY, intensity)?;
        Ok(())
    }

    fn write_register(&mut self, register: u8, value: u8) -> Result<(), AS1115Error<E>> {
        self.write_register_to_addr(self.address, register, value)?;
        Ok(())
    }

    fn write_register_to_addr(&mut self, address: u8, register: u8, value: u8) -> Result<(), AS1115Error<E>> {
        self.i2c.write(address, &[register, value])?;
        Ok(())
    }
}

pub mod constants {
    pub const DEFAULT_ADDRESS: u8 = 0x00;
    pub const MAX_DIGITS: u8 = 8;

    pub mod decode_mode {
        pub const NO_DECODE: u8 = 0x00;
        // pub const DECODE_DIGIT_0: u8 = 0x01;
        // pub const DECODE_DIGIT_1: u8 = 0x02;
    }

    pub mod self_addressing {
        pub const FACTORY_SET_ADDR: u8 = 0x00;
        pub const USER_SET_ADDR: u8 = 0x01;
    }

    pub mod shutdown_mode {
        pub const SHUTDOWN_MODE: u8 = 0x00;
        pub const NORMAL_OPERATION: u8 = 0x01;
        pub const RESET_FEATURE: u8 = 0x00;
        pub const PRESERVE_FEATURE: u8 = 0x80;
    }
}

pub const DIGITS: [u8; 16] = [
    0x7E, 0x30, 0x6D, 0x79, 0x33, 0x5B, 0x5F, 0x70,
    0x7F, 0x7B, 0x77, 0x1F, 0x4E, 0x3D, 0x4F, 0x47,
];
pub const LETTERS: [u8; 26] = [
    0x77, 0x1F, 0x4E, 0x3D, 0x4F, 0x47, 0x5E, 0x37,
    0x30, 0x3C, 0x2F, 0x0E, 0x54, 0x15, 0x1D, 0x67,
    0x73, 0x05, 0x5B, 0x0F, 0x3E, 0x1C, 0x2A, 0x49,
    0x3B, 0x25,
];

// TODO: try enum?
pub mod addresses {
    pub const DIGIT_OFFSET: u8 = 0x01;
    pub const DECODE_MODE: u8 = 0x09;
    pub const GLOBAL_INTENSITY: u8 = 0x0A;
    pub const SCAN_LIMIT: u8 = 0x0B;
    pub const SHUTDOWN: u8 = 0x0C;
    pub const SELF_ADDRESSING: u8 = 0x2D; // bit 5 is set?
    pub const FEATURE: u8 = 0x0E;
    pub const DISPLAY_TEST_MODE: u8 = 0x0F;
    pub const DIG01_INTENSITY: u8 = 0x10;
    pub const DIG23_INTENSITY: u8 = 0x11;
    pub const DIG45_INTENSITY: u8 = 0x12;
    pub const DIG67_INTENSITY: u8 = 0x13;
    pub const DIAG_DIGIT_0: u8 = 0x14;
    pub const DIAG_DIGIT_1: u8 = 0x15;
    pub const DIAG_DIGIT_2: u8 = 0x16;
    pub const DIAG_DIGIT_3: u8 = 0x17;
    pub const DIAG_DIGIT_4: u8 = 0x18;
    pub const DIAG_DIGIT_5: u8 = 0x19;
    pub const DIAG_DIGIT_6: u8 = 0x1A;
    pub const DIAG_DIGIT_7: u8 = 0x1B;
    pub const KEY_A: u8 = 0x1C;
    pub const KEY_B: u8 = 0x1D;
}

#[derive(Clone, Copy, Debug)]
pub enum AS1115Error<E> {
    I2cError(E),
    InvalidLocation(u8),
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

impl<E> From<E> for AS1115Error<E> {
    fn from(error: E) -> Self {
        AS1115Error::I2cError(error)
    }
}
