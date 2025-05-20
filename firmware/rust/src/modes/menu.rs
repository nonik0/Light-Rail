use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
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
    fn short_name(&self) -> &[u8] {
        b"mnu"
    }

    fn num_trains(&self) -> usize {
        2
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

    fn on_train_event(&mut self, train_index: usize, state: &mut GameState) {
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
