use random_trait::Random;

use crate::{
    common::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    location::Direction,
    modes::{GameModeHandler},
    random::Rand,
    train::{DEFAULT_SPEED},
    NUM_DIGITS,
};

use super::NUM_MODES;

#[derive(Default)]
pub struct MenuMode {
    index: usize,
}

impl MenuMode {
    fn game_mode(&self) -> [u8; NUM_DIGITS as usize] {
        match self.index {
            1 => *b"ply", // Play
            2 => *b"snk", // Snake
            3 => *b"tme", // Time
            _ => *b"wat",
        }
    }

    fn next_game_mode(&mut self, inc: bool) -> [u8; NUM_DIGITS as usize] {
        let delta = if inc { 1 } else { NUM_MODES - 1 };
        self.index = (self.index + delta) % NUM_MODES;
        if self.index == 0 {
            self.index = if inc { 1 } else { NUM_MODES - 1 };
        }
        self.game_mode()
    }
}

impl GameModeHandler for MenuMode {
    fn on_restart(&mut self, state: &mut GameState) {
        state.is_over = false;
        state.display = if self.index != 0 {
            DisplayState::Text(self.game_mode())
        } else {
            DisplayState::None
        };

        state.init_trains(Cargo::Have(LedPattern::SolidBright), 3, 5);
        state.add_train(
            Cargo::Have(LedPattern::SolidBright),
            5,
            5,
            Some(DEFAULT_SPEED / 2),
        );
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
        match event {
            InputEvent::DirectionButtonPressed(direction) => match direction {
                InputDirection::Up => {
                    state.display = DisplayState::Text(self.next_game_mode(true));
                }
                InputDirection::Down => {
                    state.display = DisplayState::Text(self.next_game_mode(false));
                }
                InputDirection::Right => {
                    state.target_mode_index = self.index; // no-op if index is 0, so user needs to press up/down to select a game mode
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &state.trains[train_index];
        let caboose_loc = train.caboose().loc;
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
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();
            }
        }
    }
}
