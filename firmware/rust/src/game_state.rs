use heapless::Vec;
use random_trait::Random;

use crate::{
    cargo::*,
    location::{NUM_PLATFORMS, NUM_SWITCHES},
    platform::Platform,
    switch::Switch,
    train::{Car, Train, DEFAULT_SPEED},
    Eeprom, Rand, NUM_DIGITS,
};

pub const MAX_CARS: usize = 60;
pub const MAX_TRAINS: usize = 3;
pub const NOMINAL_TRAIN_SIZE: usize = MAX_CARS / MAX_TRAINS;
const DIGITS_MAX_BRIGHTNESS: u8 = 9; //as1115::constants::MAX_INTENSITY;
const LED_BRIGHTNESS_LEVELS: u8 = 10; // 10 levels of brightness between 0 and 255

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayState {
    None,
    Score(u16),
    Text([u8; NUM_DIGITS as usize]),
}

const RED_BRIGHTNESS_LEVELS: [u8; LED_BRIGHTNESS_LEVELS as usize] =
    [0, 14, 28, 42, 56, 71, 85, 99, 113, 127];
const YEL_BRIGHTNESS_LEVELS: [u8; LED_BRIGHTNESS_LEVELS as usize] =
    [0, 28, 57, 85, 114, 142, 171, 199, 228, 255];

pub struct GameSettings {
    eeprom: Eeprom,
    digit_brightness_level: u8,
    car_brightness_level: u8,
    platform_brightness_level: u8,
    switch_brightness_level: u8,
    // game speed?
    // switch animation style?
}

impl GameSettings {
    pub fn new(eeprom: Eeprom) -> Self {
        let mut digit_brightness_level = eeprom.read_byte(0);
        if digit_brightness_level > DIGITS_MAX_BRIGHTNESS {
            digit_brightness_level = 1;
        }

        let mut car_brightness_level = eeprom.read_byte(1);
        if car_brightness_level >= LED_BRIGHTNESS_LEVELS {
            car_brightness_level = 9;
        }

        let mut platform_brightness_level = eeprom.read_byte(2);
        if platform_brightness_level >= LED_BRIGHTNESS_LEVELS {
            platform_brightness_level = 3;
        }
        let mut switch_brightness_level = eeprom.read_byte(3);
        if switch_brightness_level >= LED_BRIGHTNESS_LEVELS {
            switch_brightness_level = 3;
        }

        Self {
            eeprom,
            digit_brightness_level,
            car_brightness_level,
            platform_brightness_level,
            switch_brightness_level,
        }
    }

    pub fn save(&mut self) {
        self.eeprom.write_byte(0, self.digit_brightness_level);
        self.eeprom.write_byte(1, self.car_brightness_level);
        self.eeprom.write_byte(2, self.platform_brightness_level);
        self.eeprom.write_byte(3, self.switch_brightness_level);
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

pub struct GameState {
    pub target_mode_index: usize, // in state so menu mode can manipulate it
    pub is_over: bool,            // stops entity updates
    pub redraw: bool,             // flag to redraw board LEDs
    pub display: DisplayState,
    pub settings: GameSettings,

    // game entities
    pub cars: [Car; MAX_CARS],
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
}

impl GameState {
    pub fn add_train(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8, speed: Option<u8>) {
        if self.trains.is_full() {
            return;
        }

        // TODO: for now, simple allocation method that divides evenly on MAX_TRAINS, only snake allocated single train with max cars
        let cars_ptr = unsafe {
            self.cars
                .as_mut_ptr()
                .add(self.trains.len() * NOMINAL_TRAIN_SIZE)
        };
        let loc = self.rand_platform().track_location();
        let mut train = Train::new(cars_ptr, max_cars, loc, cargo, speed);
        for _ in 1..num_cars {
            train.add_car(cargo);
        }
        self.trains.push(train).unwrap();
        self.redraw = true;
    }

    pub fn remove_train(&mut self) {
        if self.trains.len() > 1 {
            self.trains.pop();
            self.redraw = true;
        }
    }

    /// Initializes the game state with a single train with given parameters.
    pub fn init_trains(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8) {
        // init first train
        if self.trains.len() > 0 {
            while self.trains.len() > 1 {
                self.trains.pop();
            }

            // reuse existing train for smooth transition between modes
            let train = &mut self.trains[0];
            train.init_cars(cargo, num_cars, max_cars);
            train.set_speed(DEFAULT_SPEED);
            self.redraw = true;
        } else {
            self.add_train(cargo, num_cars, max_cars, None);
        }
    }

    pub fn init_platforms(&mut self, cargo: Cargo) {
        for platform in self.platforms.iter_mut() {
            if !platform.is_empty() {
                platform.set_cargo(cargo);
                platform.set_phase_speed(1);
            }
        }
    }

    pub fn clear_platforms(&mut self) {
        for platform in self.platforms.iter_mut() {
            platform.clear_cargo();
        }
    }

    pub fn rand_platform(&self) -> &Platform {
        let rand_platform_index = Rand::default().get_usize() % self.platforms.len();
        &self.platforms[rand_platform_index]
    }
}
