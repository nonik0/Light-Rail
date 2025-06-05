use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState, MAX_CARS},
    input::InputEvent,
    modes::GameModeHandler,
    random::Rand,
    train::DEFAULT_SPEED,
};

pub struct SnakeMode {
    score: u16,
}

impl Default for SnakeMode {
    fn default() -> Self {
        SnakeMode { score: 0 }
    }
}

impl GameModeHandler for SnakeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 1;
        state.is_over = false;
        state.display = DisplayState::Score(self.score);

        state.init_trains(Cargo::Have(LedPattern::SolidBright), 1, MAX_CARS as u8);
        state.init_platforms(Cargo::Have(LedPattern::SolidBright));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                // TODO: check train too
                platform.set_cargo(Cargo::Have(LedPattern::SolidBright));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        if state.is_over {
            self.on_restart(state);
        }
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &mut state.trains[train_index];
        let caboose_loc = train.caboose();
        let last_loc = train.last_loc();

        // Check if train collided with itself
        for i in 1..train.len() {
            if train[i].loc == train.front() {
                state.display = DisplayState::Text(*b"ded");
                state.is_over = true;
                return;
            }
        }

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();

                train.add_car(Cargo::Have(LedPattern::SolidBright));

                self.score = train.len() as u16;
                state.display = DisplayState::Score(self.score);
            }
        }
    }
}
