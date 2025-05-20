use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{common::*, game::{DisplayState, GameState}, input::InputEvent, modes::GameModeHandler, random::Rand};

pub struct SnakeMode {
    score: u16,
}

impl Default for SnakeMode {
    fn default() -> Self {
        SnakeMode { score: 0 }
    }
}

impl GameModeHandler for SnakeMode
{
    fn short_name(&self) -> &[u8] {
        b"snk"
    }

    fn num_trains(&self) -> usize {
        1
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                // TODO: check train too
                platform.set_cargo();
            }
        }
    }

    fn on_train_event(&mut self, train_index: usize, state: &mut GameState) {
        let train = &mut state.trains[train_index];
        let caboose_loc = train.caboose();
        let last_loc = train.last_loc();

        // Check if train collided with itself
        for i in 1..train.cars() {
            if train[i].loc == train.engine() {
                state.display = DisplayState::Text(*b"ded");
                state.is_over = true;
                return;
            }
        }

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.engine() == platform.track_location() {
                platform.clear_cargo();

                train.add_car(Cargo::Full);

                self.score += 1;
                state.display = DisplayState::Score(self.score);
            }
        }
    }
}
