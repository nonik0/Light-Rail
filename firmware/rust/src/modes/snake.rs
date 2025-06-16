use random_trait::Random;

use crate::{cargo::*, game_state::*, input::InputEvent, modes::GameModeHandler, random::Rand};

pub struct SnakeMode {
    score: u16,
    counter: u8,
}

impl Default for SnakeMode {
    fn default() -> Self {
        SnakeMode {
            score: 0,
            counter: 0,
        }
    }
}

impl GameModeHandler for SnakeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.counter = 0;
        self.score = 1;
        state.is_over = false;
        state.display = DisplayState::Score(self.score);

        state.init_trains(Cargo::Full(LedPattern::Solid), 1, MAX_CARS as u8);
        state.init_platforms(Cargo::Full(LedPattern::Solid));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo_out(Cargo::Full(LedPattern::Solid));
            }
        }

        if state.is_over {
            self.counter += 1;
            if self.counter == 0 {
                state.display = DisplayState::Text(*b"ded");
            } else if self.counter == u8::MAX >> 1 {
                state.display = DisplayState::Score(self.score);
            }
        }
    }

    fn on_input_event(&mut self, _: InputEvent, state: &mut GameState) {
        if state.is_over {
            self.on_restart(state);
        }
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &mut state.trains[train_index];

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

                train.add_car(Cargo::Full(LedPattern::Solid));

                self.score = train.len() as u16;
                state.display = DisplayState::Score(self.score);
            }
        }
    }
}
