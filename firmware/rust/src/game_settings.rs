use crate::Eeprom;

const DIGITS_MAX_BRIGHTNESS: u8 = 9; //as1115::constants::MAX_INTENSITY;
const DIGITS_BRIGHTNESS_DEFAULT: u8 = 1;

const LED_BRIGHTNESS_LEVEL_COUNT: u8 = 6; // 6 levels of brightness between 0 and 255
const RED_BRIGHTNESS_LEVELS: [u8; LED_BRIGHTNESS_LEVEL_COUNT as usize] = [0, 28, 37, 60, 90, 127]; // reds are brighter
const YEL_BRIGHTNESS_LEVELS: [u8; LED_BRIGHTNESS_LEVEL_COUNT as usize] = [0, 50, 100, 150, 200, 255];

const CAR_BRIGHTNESS_LEVEL_DEFAULT: u8 = LED_BRIGHTNESS_LEVEL_COUNT - 1; // yellow, max brightness
const PLATFORM_BRIGHTNESS_LEVEL_DEFAULT: u8 = LED_BRIGHTNESS_LEVEL_COUNT >> 1; // red, half brightness
const SWITCH_BRIGHTNESS_LEVEL_DEFAULT: u8 = 1; // red, min brightness

const GAME_SPEED_LEVEL_COUNT: u8 = 3;
const GAME_SPEED_LEVELS: [u8; GAME_SPEED_LEVEL_COUNT as usize] = [20, 10, 5];
const GAME_SPEED_LEVEL_DEFAULT: u8 = 1;

const AUTOSTOP_LEVEL_COUNT: u8 = 3; // the higher the level the faster the speeds the train will stop
const AUTOSTOP_LEVEL_DEFAULT: u8 = 2;

// TODO: cleanup/refactor, use helper trait for ops on settings
pub struct GameSettings {
    eeprom: Eeprom,
    // brightness settings
    digit_brightness_level: u8,
    car_brightness_level: u8,
    platform_brightness_level: u8,
    switch_brightness_level: u8,
    // hardware settings
    buzzer_enabled: bool,    
    // gameplay settings
    game_speed_level: u8,
    autostop_level: u8,
}

impl GameSettings {
    pub fn new(eeprom: Eeprom) -> Self {
        let mut digit_brightness_level = eeprom.read_byte(0);
        if digit_brightness_level > DIGITS_MAX_BRIGHTNESS {
            digit_brightness_level = DIGITS_BRIGHTNESS_DEFAULT;
        }

        let mut car_brightness_level = eeprom.read_byte(1);
        if car_brightness_level >= LED_BRIGHTNESS_LEVEL_COUNT {
            car_brightness_level = CAR_BRIGHTNESS_LEVEL_DEFAULT;
        }

        let mut platform_brightness_level = eeprom.read_byte(2);
        if platform_brightness_level >= LED_BRIGHTNESS_LEVEL_COUNT {
            platform_brightness_level = PLATFORM_BRIGHTNESS_LEVEL_DEFAULT;
        }

        let mut switch_brightness_level = eeprom.read_byte(3);
        if switch_brightness_level >= LED_BRIGHTNESS_LEVEL_COUNT {
            switch_brightness_level = SWITCH_BRIGHTNESS_LEVEL_DEFAULT;
        }
        
        let buzzer_enabled = eeprom.read_byte(4) != 0;
        
        let mut game_speed_level= eeprom.read_byte(5);
        if game_speed_level >= GAME_SPEED_LEVEL_COUNT {
            game_speed_level = GAME_SPEED_LEVEL_DEFAULT;
        }

        let mut autostop_level = eeprom.read_byte(6);
        if autostop_level > AUTOSTOP_LEVEL_COUNT {
            autostop_level = AUTOSTOP_LEVEL_DEFAULT;
        }

        Self {
            eeprom,
            digit_brightness_level,
            car_brightness_level,
            platform_brightness_level,
            switch_brightness_level,
            buzzer_enabled,
            game_speed_level,
            autostop_level
        }
    }

    pub fn save(&mut self) {
        self.eeprom.write_byte(0, self.digit_brightness_level);
        self.eeprom.write_byte(1, self.car_brightness_level);
        self.eeprom.write_byte(2, self.platform_brightness_level);
        self.eeprom.write_byte(3, self.switch_brightness_level);
        self.eeprom.write_byte(4, self.buzzer_enabled as u8);
        self.eeprom.write_byte(5, self.game_speed_level);
        self.eeprom.write_byte(6, self.autostop_level);
    }

    // digit brightness level
    #[inline(always)]
    pub fn digit_brightness_level(&self) -> u8 {
        self.digit_brightness_level
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

    // car brightness level
    #[inline(always)]
    pub fn car_brightness(&self) -> u8 {
        YEL_BRIGHTNESS_LEVELS[self.car_brightness_level as usize]
    }

    #[inline(always)]
    pub fn car_brightness_level(&self) -> u8 {
        self.car_brightness_level
    }

    pub fn inc_car_brightness_level(&mut self) {
        if self.car_brightness_level < LED_BRIGHTNESS_LEVEL_COUNT - 1 {
            self.car_brightness_level += 1;
        }
    }

    pub fn dec_car_brightness_level(&mut self) {
        if self.car_brightness_level > 0 {
            self.car_brightness_level -= 1;
        }
    }

    // platform brightness level
    #[inline(always)]
    pub fn platform_brightness(&self) -> u8 {
        RED_BRIGHTNESS_LEVELS[self.platform_brightness_level as usize]
    }

    #[inline(always)]
    pub fn platform_brightness_level(&self) -> u8 {
        self.platform_brightness_level
    }

    pub fn inc_platform_brightness_level(&mut self) {
        if self.platform_brightness_level < LED_BRIGHTNESS_LEVEL_COUNT - 1 {
            self.platform_brightness_level += 1;
        }
    }

    pub fn dec_platform_brightness_level(&mut self) {
        if self.platform_brightness_level > 0 {
            self.platform_brightness_level -= 1;
        }
    }

    // switch brightness level
    #[inline(always)]
    pub fn switch_brightness(&self) -> u8 {
        YEL_BRIGHTNESS_LEVELS[self.switch_brightness_level as usize]
    }

    #[inline(always)]
    pub fn switch_brightness_level(&self) -> u8 {
        self.switch_brightness_level
    }

    pub fn inc_switch_brightness_level(&mut self) {
        if self.switch_brightness_level < LED_BRIGHTNESS_LEVEL_COUNT - 1 {
            self.switch_brightness_level += 1;
        }
    }

    pub fn dec_switch_brightness_level(&mut self) {
        if self.switch_brightness_level > 0 {
            self.switch_brightness_level -= 1;
        }
    }

    // buzzer enabled
    #[inline(always)]
    pub fn is_buzzer_enabled(&self) -> bool {
        self.buzzer_enabled
    }

    #[inline(always)]
    pub fn toggle_buzzer(&mut self) {
        self.buzzer_enabled = !self.buzzer_enabled;
    }

    // game speed
    #[inline(always)]
    pub fn game_speed(&self) -> u32 {
        GAME_SPEED_LEVELS[self.game_speed_level as usize] as u32
    }

    #[inline(always)]
    pub fn game_speed_level(&self) -> u8 {
        self.game_speed_level
    }

    pub fn inc_game_speed(&mut self) {
        if self.game_speed_level < GAME_SPEED_LEVEL_COUNT - 1 {
            self.game_speed_level += 1;
        }
    }

    pub fn dec_game_speed(&mut self) {
        if self.game_speed_level > 0 {
            self.game_speed_level -= 1;
        }
    }

    // autostop level
    pub fn autostop_level(&self) -> u8 {
        self.autostop_level
    }

    pub fn inc_autostop_level(&mut self) {
        if self.autostop_level < AUTOSTOP_LEVEL_COUNT - 1 {
            self.autostop_level += 1;
        }
    }

    pub fn dec_autostop_level(&mut self) {
        if self.autostop_level > 0 {
            self.autostop_level -= 1;
        }
    }
}
