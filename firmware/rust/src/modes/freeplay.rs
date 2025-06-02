use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState},
    input::InputEvent,
    modes::GameModeHandler,
    random::Rand,
    train::DEFAULT_SPEED
};

pub struct FreeplayMode {
    score: u16,
}

impl Default for FreeplayMode {
    fn default() -> Self {
        FreeplayMode { score: 0 }
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
            train.set_state(3, DEFAULT_SPEED);
        }
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo();
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        // TODO: add/remove trains, etc.
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
