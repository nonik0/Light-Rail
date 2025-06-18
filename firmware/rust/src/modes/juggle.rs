use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
};

const START_SPEED: u8 = 5;

pub struct JuggleMode {
    counter: u8,
    score: u16,
}

impl Default for JuggleMode {
    fn default() -> Self {
        JuggleMode {
            counter: 0,
            score: 0,
        }
    }
}

impl GameModeHandler for JuggleMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.counter = 0;
        self.score = 0;
        state.is_over = false;
        state.is_paused = false;
        state.display = DisplayState::Score(self.score);

        state.init_trains(Cargo::Full(LedPattern::Solid), 3, MAX_CARS as u8);
        state.trains[0].set_speed(START_SPEED);
        state.add_train(Cargo::Full(LedPattern::Solid), 4, 5, Some(START_SPEED));
        state.init_platforms(Cargo::Full(LedPattern::Solid));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        if state.is_over || state.is_paused  {
            self.counter += 1;
            if self.counter == 0 {
                state.display = if state.is_paused {
                    DisplayState::PAUSE
                } else {
                    DisplayState::GG
                }
            } else if self.counter == u8::MAX >> 1 {
                state.display = DisplayState::Score(self.score);
            }
            return;
        }

        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo_out(Cargo::Full(LedPattern::Solid));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        if state.is_over {
            self.on_restart(state);
        }

        match event {
            InputEvent::DirectionButtonPressed(direction) => match direction {
                InputDirection::Up | InputDirection::Down => {
                    state.is_paused = !state.is_paused;
                }
                _ => {}
            },
            _ => {}
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
        let mut score_updated = false;
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();

                score_updated = true;
                self.score += 1;
                state.display = DisplayState::Score(self.score);
            }
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

        // difficulty scaling
        if score_updated {
            match self.score {
                05 => state.trains[0].set_speed(START_SPEED + 3),
                10 => state.trains[1].set_speed(START_SPEED + 3),
                15 => state.trains[0].set_speed(START_SPEED + 6),
                20 => state.trains[1].set_speed(START_SPEED + 6),
                25 => state.trains[0].set_speed(START_SPEED + 10),
                30 => state.trains[1].set_speed(START_SPEED + 10),
                35 => {
                    state.add_train(Cargo::Full(LedPattern::Solid), 5, 5, Some(START_SPEED));
                    state.trains[0].set_speed(START_SPEED);
                    state.trains[1].set_speed(START_SPEED);
                }
                40 => state.trains[0].set_speed(START_SPEED + 3),
                45 => state.trains[1].set_speed(START_SPEED + 3),
                50 => state.trains[2].set_speed(START_SPEED + 3),
                55 => state.trains[0].set_speed(START_SPEED + 6),
                60 => state.trains[1].set_speed(START_SPEED + 6),
                65 => state.trains[2].set_speed(START_SPEED + 6),
                70 => state.trains[0].set_speed(START_SPEED + 10),
                75 => state.trains[1].set_speed(START_SPEED + 10),
                80 => state.trains[2].set_speed(START_SPEED + 10),
                _ => {}
            }
        }
    }
}
