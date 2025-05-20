use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{common::*, game::{DisplayState, GameState}, input::InputEvent, modes::GameModeHandler, random::Rand};

pub struct FreeplayMode {
    score: u16,
}

impl Default for FreeplayMode {
    fn default() -> Self {
        FreeplayMode { score: 0 }
    }
}

impl GameModeHandler for FreeplayMode
{
    fn short_name(&self) -> &[u8] {
        b"ply"
    }

    fn num_trains(&self) -> usize {
        1
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

    fn on_train_event(&mut self, train_index: usize, state: &mut GameState) {
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
