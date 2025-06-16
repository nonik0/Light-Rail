use crate::{

    Eeprom,
};

const DIGITS_MAX_BRIGHTNESS: u8 = 9; //as1115::constants::MAX_INTENSITY;
const LED_BRIGHTNESS_LEVELS: u8 = 6; // 6 levels of brightness between 0 and 255

const RED_BRIGHTNESS_LEVELS: [u8; LED_BRIGHTNESS_LEVELS as usize] = [0, 28, 37, 60, 90, 127];
const YEL_BRIGHTNESS_LEVELS: [u8; LED_BRIGHTNESS_LEVELS as usize] = [0, 50, 100, 150, 200, 255];

pub struct GameSettings {
    eeprom: Eeprom,
    // brightness settings
    digit_brightness_level: u8,
    car_brightness_level: u8,
    platform_brightness_level: u8,
    switch_brightness_level: u8,
    // gameplay settings
    // game_speed: u8, // TODO: add game speed setting
    // other
    buzzer_enabled: bool,
}

impl GameSettings {
    pub fn new(eeprom: Eeprom) -> Self {
        let mut digit_brightness_level = eeprom.read_byte(0);
        if digit_brightness_level > DIGITS_MAX_BRIGHTNESS {
            digit_brightness_level = 1;
        }

        let mut car_brightness_level = eeprom.read_byte(1);
        if car_brightness_level >= LED_BRIGHTNESS_LEVELS {
            car_brightness_level = LED_BRIGHTNESS_LEVELS - 1; // max brightness
        }

        let mut platform_brightness_level = eeprom.read_byte(2);
        if platform_brightness_level >= LED_BRIGHTNESS_LEVELS {
            platform_brightness_level = LED_BRIGHTNESS_LEVELS >> 1; // half brightness
        }

        let mut switch_brightness_level = eeprom.read_byte(3);
        if switch_brightness_level >= LED_BRIGHTNESS_LEVELS {
            switch_brightness_level = (LED_BRIGHTNESS_LEVELS >> 1) - 1; // one level below half brightness
        }
        
        let buzzer_enabled = eeprom.read_byte(4) != 0;

        Self {
            eeprom,
            digit_brightness_level,
            car_brightness_level,
            platform_brightness_level,
            switch_brightness_level,
            buzzer_enabled,
        }
    }

    pub fn save(&mut self) {
        self.eeprom.write_byte(0, self.digit_brightness_level);
        self.eeprom.write_byte(1, self.car_brightness_level);
        self.eeprom.write_byte(2, self.platform_brightness_level);
        self.eeprom.write_byte(3, self.switch_brightness_level);
    }

    #[inline(always)]
    pub fn is_buzzer_enabled(&self) -> bool {
        self.buzzer_enabled
    }

    #[inline(always)]
    pub fn toggle_buzzer(&mut self) {
        self.buzzer_enabled = !self.buzzer_enabled;
    }

    #[inline(always)]
    pub fn digit_brightness_level(&self) -> u8 {
        self.digit_brightness_level
    }

    #[inline(always)]
    pub fn car_brightness(&self) -> u8 {
        YEL_BRIGHTNESS_LEVELS[self.car_brightness_level as usize]
    }

    #[inline(always)]
    pub fn car_brightness_level(&self) -> u8 {
        self.car_brightness_level
    }

    #[inline(always)]
    pub fn platform_brightness(&self) -> u8 {
        RED_BRIGHTNESS_LEVELS[self.platform_brightness_level as usize]
    }

    #[inline(always)]
    pub fn platform_brightness_level(&self) -> u8 {
        self.platform_brightness_level
    }

    #[inline(always)]
    pub fn switch_brightness(&self) -> u8 {
        YEL_BRIGHTNESS_LEVELS[self.switch_brightness_level as usize]
    }

    #[inline(always)]
    pub fn switch_brightness_level(&self) -> u8 {
        self.switch_brightness_level
    }

    pub fn inc_digit_brightness_level(&mut self) {
        if self.digit_brightness_level < DIGITS_MAX_BRIGHTNESS {
            self.digit_brightness_level += 1;
        }
    }

    pub fn dec_digit_brightness_level(&mut self) {
        if self.digit_brightness_level > 0 {
            self.digit_brightness_level -= 1;
        }
    }

    pub fn inc_car_brightness_level(&mut self) {
        if self.car_brightness_level < LED_BRIGHTNESS_LEVELS - 1 {
            self.car_brightness_level += 1;
        }
    }

    pub fn dec_car_brightness_level(&mut self) {
        if self.car_brightness_level > 0 {
            self.car_brightness_level -= 1;
        }
    }

    pub fn inc_platform_brightness_level(&mut self) {
        if self.platform_brightness_level < LED_BRIGHTNESS_LEVELS - 1 {
            self.platform_brightness_level += 1;
        }
    }

    pub fn dec_platform_brightness_level(&mut self) {
        if self.platform_brightness_level > 0 {
            self.platform_brightness_level -= 1;
        }
    }

    pub fn inc_switch_brightness_level(&mut self) {
        if self.switch_brightness_level < LED_BRIGHTNESS_LEVELS - 1 {
            self.switch_brightness_level += 1;
        }
    }

    pub fn dec_switch_brightness_level(&mut self) {
        if self.switch_brightness_level > 0 {
            self.switch_brightness_level -= 1;
        }
    }
}
