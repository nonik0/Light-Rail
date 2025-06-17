use random_trait::Random;

use crate::{
    cargo::*, game_state::*, input::InputEvent, modes::GameModeHandler, random::Rand,
    train::DEFAULT_SPEED,
};

const START_SPEED: u8 = 5;

pub struct JuggleMode {
    score: u16,
    counter: u8,
}

impl Default for JuggleMode {
    fn default() -> Self {
        JuggleMode {
            score: 0,
            counter: 0,
        }
    }
}

impl GameModeHandler for JuggleMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.counter = 0;
        self.score = 0;
        state.is_over = false;
        state.display = DisplayState::Score(self.score);

        state.init_trains(Cargo::Full(LedPattern::Solid), 3, MAX_CARS as u8);
        state.trains[0].set_speed(START_SPEED);
        state.add_train(Cargo::Full(LedPattern::Solid), 3, 5, Some(START_SPEED));
        state.init_platforms(Cargo::Full(LedPattern::Solid));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        if state.is_over {
            self.counter += 1;
            if self.counter == 0 {
                state.display = DisplayState::Text(*b" GG");
            } else if self.counter == u8::MAX >> 1 {
                state.display = DisplayState::Score(self.score);
            }
            return;
        }

        self.counter += 1;
        if self.counter == 0 {
            self.score += 1;

            match self.score {
                5 => state.trains[0].set_speed(START_SPEED + 3),
                10 => state.trains[1].set_speed(START_SPEED + 3),
                15 => state.add_train(Cargo::Full(LedPattern::Solid), 3, 5, Some(START_SPEED+3)),
                20 => state.trains[1].set_speed(START_SPEED + 6),
                25 => state.trains[2].set_speed(START_SPEED + 6),
                30 => state.trains[3].set_speed(START_SPEED + 6),
                35 => state.trains[1].set_speed(START_SPEED + 6),
                40 => state.trains[2].set_speed(START_SPEED + 6),
                45 => state.trains[3].set_speed(START_SPEED + 6),
                _ => {}
            }

            state.display = DisplayState::Score(self.score);
        }

        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo_out(Cargo::Full(LedPattern::Solid));
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

        // TODO: this doesn't work here, but it does it snake mode?
        // // Check if train collided with itself
        // for i in 1..train.len() {
        //     if train[i].loc == train.front() {
        //         state.display = DisplayState::Text(*b"abc");
        //         state.is_over = true;
        //         return;
        //     }
        // }

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();
            }

            // TODO: train increases in length after picking up X cargo
        }

        // Check if train collided with another train
        let train_front = train.front();
        for (other_index, other_train) in state.trains.iter().enumerate() {
            if train_index != other_index && other_train.at_location(train_front) {
                state.display = DisplayState::Text(*b" GG");
                state.is_over = true;
                return;
            }
        }
    }
}
