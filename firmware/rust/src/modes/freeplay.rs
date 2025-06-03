use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState, MAX_TRAINS},
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
    train::{Train, DEFAULT_SPEED}
};



pub struct FreeplayMode {
    score: u16,
    cur_setting: u8,
}

impl FreeplayMode {
    fn add_train(&mut self, state: &mut GameState) {
        if state.trains.len() < MAX_TRAINS {
            let rand_platform_index = Rand::default().get_usize() % state.platforms.len();
            let rand_platform = &state.platforms[rand_platform_index];
            let rand_speed = 5 + Rand::default().get_u8() % 10;
            let mut train = Train::new(
                rand_platform.track_location(),
                Cargo::Full(LedPattern::SolidBright),
                Some(rand_speed),
            );
            let num_cars = 1 + Rand::default().get_usize() % 3;
            state.trains.push(train).unwrap();
        }

        state.redraw = true;
    }

    fn remove_train(&mut self, state: &mut GameState) {
        if state.trains.len() > 1 {
            state.trains.pop();
        }

        state.redraw = true;
    }
}

impl Default for FreeplayMode {
    fn default() -> Self {
        FreeplayMode { score: 0, cur_setting: 0 }
    }
}

impl GameModeHandler for FreeplayMode {
    fn on_restart(&mut self, state: &mut GameState) {
        state.is_over = false;
        self.score = 1;
        state.display = DisplayState::Score(self.score);

        while state.trains.len() > 1 {
            state.trains.pop();
        }

        for train in state.trains.iter_mut() {
            train.set_state(3, Cargo::Full(LedPattern::SolidBright), DEFAULT_SPEED);
        }

        state.redraw = true;
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo(Cargo::Full(LedPattern::SolidBright));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        // TODO: add/remove trains, etc.
        match event {
            InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                self.remove_train(state);
            },
            InputEvent::DirectionButtonPressed(InputDirection::Right) => {
                self.add_train(state);
            }
            _ => {}
        }
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &state.trains[train_index];
        let caboose_loc = train.caboose();
        let last_loc = train.last_loc();

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.engine() == platform.track_location() {
                platform.clear_cargo();

                self.score += 1;
                state.display = DisplayState::Score(self.score);
            }
        }
    }
}
