use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState, MAX_TRAINS, NOMINAL_TRAIN_SIZE},
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
    train::{Train, DEFAULT_SPEED}
};

pub struct FreeplayMode {
    score: u16,
    cur_setting: u8,
}

impl Default for FreeplayMode {
    fn default() -> Self {
        FreeplayMode { score: 0, cur_setting: 0 }
    }
}

impl GameModeHandler for FreeplayMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 0;
        state.display = DisplayState::Score(self.score);
        state.is_over = false;
        state.redraw = true;

        state.init_trains(Cargo::Have(LedPattern::SolidBright), 3, NOMINAL_TRAIN_SIZE as u8);
        state.init_platforms(Cargo::Have(LedPattern::SolidBright));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo(Cargo::Have(LedPattern::SolidBright));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        // TODO: add/remove trains, etc.
        match event {
            InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                state.remove_train();
            },
            InputEvent::DirectionButtonPressed(InputDirection::Right) => {
                state.add_train(Cargo::Have(LedPattern::SolidBright), 3, NOMINAL_TRAIN_SIZE as u8);
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
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();

                self.score += 1;
                state.display = DisplayState::Score(self.score);
            }
        }
    }
}
