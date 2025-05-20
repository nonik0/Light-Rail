// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::IS31FL3731;
use static_cell::make_static;

// use embedded_hal::delay::DelayNs;
use random_trait::Random;

use crate::{
    common::*,
    input::{BoardInput, InputDirection, InputEvent},
    location::{Direction, Location, NUM_PLATFORMS, NUM_SWITCHES},
    modes::*,
    panic::trace,
    platform::Platform,
    switch::Switch,
    tone::TimerTone,
    train::Train,
    Rand,
};

const MAX_TRAINS: usize = 5;
const MAX_LOC_UPDATES: usize = crate::train::MAX_UPDATES * MAX_TRAINS + NUM_PLATFORMS;

pub enum DisplayState {
    None,
    Score(u16),
    Text([u8; crate::NUM_DIGITS as usize]),
    //ScrollingText
}

// TOOD: rename to GameState?
pub struct GameState {
    pub target_mode_index: usize, // in state so menu mode can manipulate it
    pub is_over: bool,            // stops entity updates
    pub display: DisplayState,

    // game entities
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
}

pub struct Game<I2C>
where
    I2C: I2c + 'static,
{
    // board components
    board_buzzer: TimerTone,
    board_digits: AS1115<I2C>,
    board_input: BoardInput,
    board_leds: IS31FL3731<I2C>,

    // game mode state
    active_mode_index: usize,
    modes: [&'static mut dyn GameModeHandler; NUM_GAME_MODES + 1],

    // state passed to game modes, changes to state entities are rendered into updates for digits and LEDs
    state: GameState,
}

impl<I2C> Game<I2C>
where
    I2C: I2c + 'static,
{
    pub fn new(
        board_buzzer: TimerTone,
        board_digits: AS1115<I2C>,
        board_input: BoardInput,
        board_leds: IS31FL3731<I2C>,
    ) -> Self {
        let menu_mode = make_static!(MenuMode::default());
        let freeplay_mode = make_static!(FreeplayMode::default());
        let snake_mode = make_static!(SnakeMode::default());

        let state = GameState {
            target_mode_index: 0,
            is_over: false,
            display: DisplayState::None,
            trains: Vec::<Train, MAX_TRAINS>::new(),
            platforms: Platform::take(),
            switches: Switch::take(),
        };

        let mut game = Self {
            board_buzzer,
            board_digits,
            board_input,
            board_leds,
            active_mode_index: 0,
            modes: [menu_mode, freeplay_mode, snake_mode],
            state,
        };

        game.restart();
        game
    }

    fn mode(&self) -> &dyn GameModeHandler {
        self.modes[self.active_mode_index]
    }

    fn restart(&mut self) {
        self.board_digits.clear().ok();
        self.board_leds.clear_blocking().unwrap();

        // instead of clearing the trains, we just adjust the number of trains and set to known state
        let actual_num_trains = self.state.trains.len();
        let target_num_trains = self.mode().num_trains();
        if actual_num_trains > target_num_trains {
            for _ in 0..actual_num_trains - target_num_trains {
                self.state.trains.pop().unwrap();
            }
        } else if actual_num_trains < target_num_trains {
            for _ in 0..target_num_trains - actual_num_trains {
                // TODO: move to separate function
                let rand_platform_index = Rand::default().get_usize() % self.state.platforms.len();
                let rand_platform = &self.state.platforms[rand_platform_index];
                let rand_speed = 5 + Rand::default().get_u8() % 10;
                let mut train = Train::new(
                    rand_platform.track_location(),
                    Cargo::Full,
                    Some(rand_speed),
                );
                let num_cars = 1 + Rand::default().get_usize() % 3;
                for _ in 0..num_cars {
                    train.add_car(Cargo::Full);
                }
                self.state.trains.push(train).unwrap();
            }
        }

        for train in self.state.trains.iter_mut() {
            let actual_num_cars = train.cars();
            let target_num_cars = 3;

            if actual_num_cars > target_num_cars {
                for _ in 0..actual_num_cars - target_num_cars {
                    train.remove_car().unwrap();
                }
            } else if actual_num_cars < target_num_cars {
                for _ in 0..target_num_cars - actual_num_cars {
                    train.add_car(Cargo::Full);
                }
            }
        }

        // self.state.trains.clear();

        // for _ in 0..self.mode().num_trains() {
        //     let rand_platform_index = Rand::default().get_usize() % self.state.platforms.len();
        //     let rand_platform = &self.state.platforms[rand_platform_index];
        //     let rand_speed = 5 + Rand::default().get_u8() % 10;
        //     let mut train = Train::new(rand_platform.track_location(), Cargo::Full, Some(rand_speed));
        //     let num_cars = 1 + Rand::default().get_usize() % 3;
        //     for _ in 0..num_cars {
        //         train.add_car(Cargo::Full);
        //     }
        //     self.state.trains.push(train).unwrap();
        // }
    }

    pub fn tick(&mut self) {


        if let Some(event) = self.board_input.update() {
            // shared events for all game modes
            match event {
                // toggle switches (TODO: maybe specialized for specific modes?)
                InputEvent::SwitchButtonPressed(index) => {
                    self.board_buzzer.tone(4000, 100);
                    let index = index as usize;
                    if index < self.state.switches.len() {
                        self.state.switches[index].switch();
                    }
                }
                // go to menu
                InputEvent::DirectionButtonHeld(InputDirection::Left) => {
                    self.state.target_mode_index = 0;
                    self.active_mode_index = 0;
                    self.state.is_over = false;
                }
                _ => {}
            }

            // mode specific events
            let mode = &mut self.modes[self.active_mode_index];
            mode.on_input_event(event, &mut self.state);
        }

        // no entity updates if game is over, wait for input event to restart or exit mode
        if self.state.is_over {
            return;
        }

        if self.state.target_mode_index != self.active_mode_index {
            self.active_mode_index = self.state.target_mode_index;
            self.restart();
        }

        let mode = &mut self.modes[self.active_mode_index];
        mode.on_game_tick(&mut self.state);

        let mut updates = Vec::<EntityUpdate, MAX_LOC_UPDATES>::new();
        self.advance_trains(&mut updates);
        self.render_updates(&mut updates);
    }

    fn advance_trains(&mut self, updates: &mut Vec<EntityUpdate, MAX_LOC_UPDATES>) {
        let mut event_indices = heapless::Vec::<usize, MAX_TRAINS>::new();
        for (train_index, train) in self.state.trains.iter_mut().enumerate() {
            if let Some(u) = train.advance(&self.state.switches) {
                event_indices.push(train_index).ok();
                updates.extend(u.into_iter());
            }
        }

        let mode = &mut self.modes[self.active_mode_index];
        for &train_index in event_indices.iter() {
            mode.on_train_event(train_index, &mut self.state);
        }
    }

    fn render_updates(&mut self, updates: &mut Vec<EntityUpdate, MAX_LOC_UPDATES>) {
        for platform in self.state.platforms.iter_mut() {
            if let Some(u) = platform.get_update() {
                updates.push(u).ok();
            }
        }

        for switch in self.state.switches.iter_mut() {
            if let Some(u) = switch.get_updates(&self.state.trains) {
                updates.extend(u.into_iter());
            }
        }

        for update in updates {
            self.board_leds
                .pixel_blocking(update.location.index(), update.contents.to_pwm_value())
                .ok();
        }

        match self.state.display {
            DisplayState::None => self.board_digits.clear().unwrap(),
            DisplayState::Score(score) => self.board_digits.display_number(score).unwrap(),
            DisplayState::Text(text) => self.board_digits.display_ascii(&text).unwrap(),
        }
    }
}
