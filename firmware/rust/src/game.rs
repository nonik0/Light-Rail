// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::{digital::InputPin, i2c::I2c};
use heapless::Vec;
use is31fl3731::IS31FL3731;

use crate::{
    common::*,
    location::{Location, NUM_PLATFORMS},
    platform::Platform,
    tone::Timer3Tone,
    train::Train,
    NUM_BUTTONS,
};

#[derive(Copy, Clone)]
enum GameMode {
    Animation,
    Freeplay,
    Puzzle,
    Race,
    Survival,
}

const MAX_TRAINS: usize = 5;
const TRACK_EMPTY_PWM: u8 = 0;
const TRAIN_CARGO_EMPTY_PWM: u8 = 200;
const TRAIN_CARGO_FULL_PWM: u8 = 50;
const PLATFORM_EMPTY_PWM: u8 = 0;
const PLATFORM_FULL_PWM: u8 = 50;
const MAX_LOC_UPDATES: usize = crate::train::MAX_UPDATES * MAX_TRAINS + NUM_PLATFORMS;

pub struct Game<I2C, ButtonPin>
where
    I2C: I2c,
    ButtonPin: InputPin,
{
    // board components
    board_buttons: [ButtonPin; NUM_BUTTONS],
    board_buzzer: Timer3Tone,
    board_digits: AS1115<I2C>,
    board_leds: IS31FL3731<I2C>,

    // game state
    mode: GameMode,
    is_over: bool,
    platforms: [Platform; NUM_PLATFORMS],
    trains: heapless::Vec<Train, MAX_TRAINS>,
}

impl<I2C, ButtonPin> Game<I2C, ButtonPin>
where
    I2C: I2c,
    ButtonPin: InputPin,
{
    // do we need singleton enforcement with ownership?
    pub fn new(
        board_buttons: [ButtonPin; NUM_BUTTONS],
        board_buzzer: Timer3Tone,
        board_digits: AS1115<I2C>,
        board_leds: IS31FL3731<I2C>,
    ) -> Self {
        Self {
            board_buttons,
            board_buzzer,
            board_digits,
            board_leds,
            mode: GameMode::Animation,
            is_over: false,
            trains: Vec::<Train, MAX_TRAINS>::new(),
            platforms: Platform::take(),
        }
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    // fn update_location(&mut self, loc: Location, cargo: Option<Cargo>) {
    //     let brightness = match cargo {
    //         Some(Cargo::Empty) => TRAIN_CARGO_EMPTY_PWM,
    //         Some(Cargo::Full) => TRAIN_CARGO_FULL_PWM,
    //         None => TRACK_EMPTY_PWM,
    //     };
    //     self.board_leds
    //         .pixel_blocking(loc.index(), brightness)
    //         .unwrap();
    // }

    // TODO: mode
    pub fn restart(&mut self) {
        self.mode = GameMode::Animation; // TODO
        self.is_over = false;

        self.board_digits.clear().ok();
        self.board_leds.clear_blocking().unwrap();
        self.trains.clear();

        let mut train = Train::new(Location::new(69), Cargo::Full);
        train.add_car(Cargo::Empty);
        train.add_car(Cargo::Empty);
        self.trains.push(train).unwrap();

        let mut train2 = Train::new(Location::new(90), Cargo::Full);
        train2.add_car(Cargo::Empty);
        train2.add_car(Cargo::Full);
        train2.add_car(Cargo::Empty);
        self.trains.push(train2).unwrap();
    }

    pub fn tick(&mut self) {
        self.read_buttons();

        let mut all_updates = Vec::<EntityUpdate, MAX_LOC_UPDATES>::new();

        for train in self.trains.iter_mut() {
            if let Some(loc_updates) = train.advance() {
                all_updates.extend(loc_updates.into_iter());
            }
        }

        for platform in self.platforms.iter_mut() {
            if let Some(loc_update) = platform.tick() {
                all_updates.push(loc_update).unwrap();
            }
        }

        for loc_update in all_updates.iter() {
            self.board_leds
                .pixel_blocking(
                    loc_update.location.index(),
                    loc_update.contents.to_pwm_value(),
                )
                .unwrap();
        }
    }

    fn read_buttons(&mut self) {
        for (i, button) in self.board_buttons.iter_mut().enumerate() {
            if button.is_low().unwrap() {
                self.board_buzzer.tone((i + 1) as u16 * 1000, 100);
            }
        }
    }
}
