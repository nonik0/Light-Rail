// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::IS31FL3731;

use embedded_hal::delay::DelayNs;
use random_trait::Random;

use crate::{
    common::*,
    input::{BoardInput, InputDirection, InputEvent},
    location::{Location, NUM_PLATFORMS, NUM_SWITCHES},
    panic::trace,
    platform::Platform,
    Rand,
    switch::Switch,
    tone::TimerTone,
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
    board_buzzer: TimerTone,
    board_digits: AS1115<I2C>,
    board_input: BoardInput,
    board_leds: IS31FL3731<I2C>,

    // game state
    mode: GameMode,
    is_over: bool,
    score: u16,

    // game entities, hold their own state and return location updates for game to render
    platforms: [Platform; NUM_PLATFORMS],
    switches: [Switch; NUM_SWITCHES],
    trains: heapless::Vec<Train, MAX_TRAINS>,
}

impl<I2C> Game<I2C>
where
    I2C: I2c,
{
    // do we need singleton enforcement with ownership?
    pub fn new(
        board_buzzer: TimerTone,
        board_digits: AS1115<I2C>,
        board_input: BoardInput,
        board_leds: IS31FL3731<I2C>,
    ) -> Self {
        Self {
            board_buzzer,
            board_digits,
            board_input,
            board_leds,
            mode: GameMode::Animation,
            is_over: false,
            score: 0,
            trains: Vec::<Train, MAX_TRAINS>::new(),
            platforms: Platform::take(),
            switches: Switch::take(),
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

        // let mut train2 = Train::new(Location::new(90), Cargo::Full);
        // train2.add_car(Cargo::Empty);
        // train2.add_car(Cargo::Full);
        // train2.add_car(Cargo::Empty);
        // self.trains.push(train2).unwrap();
    }

    pub fn tick(&mut self) {
        trace(b"input");
        let event = self.board_input.update();
        match event {
            Some(InputEvent::SwitchButtonPressed(index)) => {
                self.switches[index as usize].switch();

                self.board_digits.display_number((index + 1) as u16).unwrap();
                self.board_buzzer.tone((index + 1) as u16 * 1000, 100);
            }
            Some(InputEvent::DirectionButtonPressed(direction)) => {
                match direction {
                    InputDirection::Up => self.board_digits.display_ascii(b" up").unwrap(),
                    InputDirection::Down => self.board_digits.display_ascii(b" dn").unwrap(),
                    InputDirection::Left => self.board_digits.display_ascii(b" lf").unwrap(),
                    InputDirection::Right => self.board_digits.display_ascii(b" rt").unwrap(),
                }
                self.board_buzzer.tone(4000, 100);
            }
            Some(InputEvent::SwitchButtonReleased(index)) => {}
            Some(InputEvent::DirectionButtonReleased(_)) => {}
            _ => {}
        }

        let mut all_updates = Vec::<EntityUpdate, MAX_LOC_UPDATES>::new();

        trace(b"train");
        for train in self.trains.iter_mut() {
            if let Some(loc_updates) = train.advance(&self.switches) {
                all_updates.extend(loc_updates.into_iter());
            }
        }

        trace(b"platform");
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

        trace(b"switch");
        for switch in self.switches.iter_mut() {
            if let Some(loc_updates) = switch.tick() {
                all_updates.extend(loc_updates.into_iter());
            }
        }

        trace(b"update");
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
