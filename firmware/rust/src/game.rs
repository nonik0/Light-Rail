// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::{digital::InputPin, i2c::I2c};
use heapless::Vec;
use is31fl3731::IS31FL3731;

use crate::{
    location::{Cargo, Location},
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
const TRAIN_CARGO_EMPTY: u8 = 200;
const TRAIN_CARGO_FULL: u8 = 50;

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
    trains: heapless::Vec<Train, MAX_TRAINS>,
    //platforms: [Platform; PLATFORM_COUNT],
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
        }
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

    fn update_location(&mut self, loc: Location, cargo: Option<Cargo>) {
        let brightness = match cargo {
            Some(Cargo::Empty) => TRAIN_CARGO_EMPTY,
            Some(Cargo::Full) => TRAIN_CARGO_FULL,
            None => TRACK_EMPTY_PWM,
        };
        self.board_leds
            .pixel_blocking(loc.index, brightness)
            .unwrap();
    }

    // TODO: mode
    pub fn restart(&mut self) {
        self.mode = GameMode::Animation; // TODO
        self.is_over = false;

        self.board_digits.clear().ok();
        self.board_leds.clear_blocking().unwrap();
        self.trains.clear();

        // let mut train = Train::new(Location::new(69), Cargo::Full);
        // train.add_car(Cargo::Empty);
        // train.add_car(Cargo::Empty);
        // self.trains.push(train).unwrap();

        // let mut train2 = Train::new(Location::new(90), Cargo::Full);
        // train2.add_car(Cargo::Empty);
        // train2.add_car(Cargo::Full);
        // train2.add_car(Cargo::Empty);
        // self.trains.push(train2).unwrap();
    }

    pub fn tick(&mut self) {
        self.read_buttons();

        for train in self.trains.iter_mut() {
            train.advance();
        }

        // for platform in self.platforms.iter_mut() {
        //     if platform.is_occupied() && platform.train_present() {
        //         platform.load
        //     }
        // }
    }

    fn read_buttons(&mut self) {
        for (i, button) in self.board_buttons.iter_mut().enumerate() {
            if button.is_low().unwrap() {
                self.board_buzzer.tone((i + 1) as u16 * 1000, 100);
            }
        }
    }
}
