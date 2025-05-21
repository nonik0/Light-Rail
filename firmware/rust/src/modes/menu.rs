use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState},
    input::{InputDirection, InputEvent},
    location::Direction,
    modes::{GameModeHandler, SnakeMode},
    platform,
    random::Rand,
    switch,
    train::Train,
    NUM_DIGITS,
};

use super::NUM_GAME_MODES;

#[derive(Default)]
pub struct MenuMode {
    index: usize,
}

impl MenuMode {
    fn next_game_mode(&mut self, inc: bool) -> [u8; NUM_DIGITS as usize] {
        let delta = if inc { 1 } else { NUM_GAME_MODES - 1 };
        self.index = (self.index + delta) % NUM_GAME_MODES;

        match self.index {
            0 => *b"ply",
            1 => *b"snk",
            _ => *b"wat",
        }
    }
}

impl GameModeHandler for MenuMode {
    fn on_restart(&mut self, state: &mut GameState) {
        let actual_num_trains = state.trains.len();
        let target_num_trains = 1;//self.mode().num_trains();
        if actual_num_trains > target_num_trains {
            for _ in 0..actual_num_trains - target_num_trains {
                state.trains.pop().unwrap();
            }
        } else if actual_num_trains < target_num_trains {
            for _ in 0..target_num_trains - actual_num_trains {
                let rand_platform_index = Rand::default().get_usize() % state.platforms.len();
                let rand_platform = &state.platforms[rand_platform_index];
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
                state.trains.push(train).unwrap();
            }
        }

        for train in state.trains.iter_mut() {
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
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo();
                // TODO: score?
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        match event {
            InputEvent::DirectionButtonPressed(direction) => match direction {
                InputDirection::Up => {
                    state.display = DisplayState::Text(self.next_game_mode(true));
                }
                InputDirection::Down => {
                    state.display = DisplayState::Text(self.next_game_mode(false));
                }
                InputDirection::Right => {
                    state.target_mode_index = self.index + 1; // offset by 1 for menu mode
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &state.trains[train_index];
        let caboose_loc = train.caboose();
        let last_loc = train.last_loc();

        // If train just left a switch, randomly switch it
        for switch in state.switches.iter_mut() {
            if caboose_loc == switch.location() {
                continue; // Train is entering, not leaving
            }
            for dir in [Direction::Anode, Direction::Cathode] {
                if switch.active_location(dir) == Some(last_loc) && Rand::default().get_bool() {
                    switch.switch();
                    break;
                }
            }
        }

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.engine() == platform.track_location() {
                platform.clear_cargo();
            }
        }
    }
}
