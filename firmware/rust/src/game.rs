// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::IS31FL3731;

use crate::{
    common::*,
    input::{Buttons, InputEvent},
    location::{Location, NUM_PLATFORMS},
    panic::set_panic_msg,
    platform::Platform,
    tone::Timer3Tone,
    train::Train,
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
const MAX_LOC_UPDATES: usize = crate::train::MAX_UPDATES * MAX_TRAINS + NUM_PLATFORMS;

pub struct Game<I2C>
where
    I2C: I2c,
{
    // board components
    board_buttons: Buttons,
    board_buzzer: Timer3Tone,
    board_digits: AS1115<I2C>,
    board_leds: IS31FL3731<I2C>,

    // game state
    mode: GameMode,
    is_over: bool,
    score: u16,

    // game entities
    platforms: [Platform; NUM_PLATFORMS],
    trains: heapless::Vec<Train, MAX_TRAINS>,
}

impl<I2C> Game<I2C>
where
    I2C: I2c,
{
    // do we need singleton enforcement with ownership?
    pub fn new(
        board_buttons: Buttons,
        board_buzzer: Timer3Tone,
        board_digits: AS1115<I2C>,
        board_leds: IS31FL3731<I2C>,
    ) -> Self {
        set_panic_msg(b"100");
        Self {
            board_buttons,
            board_buzzer,
            board_digits,
            board_leds,
            mode: GameMode::Animation,
            is_over: false,
            score: 0,
            trains: Vec::<Train, MAX_TRAINS>::new(),
            platforms: Platform::take(),
        }
    }

    pub fn is_over(&self) -> bool {
        self.is_over
    }

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
        let event = self.board_buttons.update();

        match event {
            Some(InputEvent::TrackButtonPressed(index)) => {
                self.board_digits
                    .display_number((index + 1) as u16)
                    .unwrap();
                self.board_buzzer.tone((index + 1) as u16 * 1000, 100);
            }
            Some(InputEvent::TrackButtonReleased(index)) => {}
            Some(InputEvent::DirectionButtonPressed(direction)) => {}
            Some(InputEvent::DirectionButtonReleased(_)) => {}
            _ => {}
        }

        let mut all_updates = Vec::<EntityUpdate, MAX_LOC_UPDATES>::new();

        for train in self.trains.iter_mut() {
            if let Some(loc_updates) = train.advance() {
                all_updates.extend(loc_updates.into_iter());
            }
        }

        for platform in self.platforms.iter_mut() {
            if let Some(loc_update) = platform.tick(&self.trains) {
                // update score each time a platform is cleared
                match loc_update.contents {
                    Contents::Platform(Cargo::Empty) => {
                        self.score += 1;
                        self.board_digits.display_number(self.score).unwrap();
                    }
                    _ => {}
                }
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
}
