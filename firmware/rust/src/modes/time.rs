use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState},
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
    train::DEFAULT_SPEED,
};

pub struct TimeMode {
    score: u16,
}

impl Default for TimeMode {
    fn default() -> Self {
        TimeMode { score: 0 }
    }
}

impl GameModeHandler for TimeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 1;
        state.is_over = false;
        state.display = DisplayState::Score(self.score);

        for platform in state.platforms.iter_mut() {
            platform.clear_cargo();
        }

        while state.trains.len() > 1 {
            state.trains.pop();
        }
        state.trains[0].set_state(5, Cargo::Empty, DEFAULT_SPEED);

        state.redraw = true;
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 20 {
                // TODO: check train too
                platform.set_cargo(Cargo::Full(LedPattern::SolidBright));
            }
        }

        // check if train can pick up cargo
        let train = &mut state.trains[0];
        if train.speed() == 0 {
            // Add cargo to train if there is capacity
            for platform in state.platforms.iter_mut() {
                if !platform.is_empty() {
                    // train picks up cargo if car at platform's track location is empty
                    if train.cargo_at_location(platform.track_location()) == Some(Cargo::Empty) {
                        train.set_cargo_at_location(platform.track_location(), platform.cargo());
                        platform.clear_cargo();

                        self.score += 1;
                        state.display = DisplayState::Score(self.score);
                    }
                }
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        match event {
            InputEvent::DirectionButtonPressed(direction) => match direction {
                InputDirection::Left => {
                    let speed = state.trains[0].speed();
                    if speed > 5 {
                        state.trains[0].set_speed(speed - 5);
                    } else {
                        state.trains[0].set_speed(0);
                    }
                }
                InputDirection::Right => {
                    let speed = state.trains[0].speed();
                    if speed > 20 {
                        state.trains[0].set_speed(25);
                    } else {
                        state.trains[0].set_speed(speed + 5);
                    }
                }
                _ => {}
            },
            _ => {}
        }
    }

    // fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
    //     let train = &mut state.trains[train_index];
    //     let caboose_loc = train.caboose();
    //     let last_loc = train.last_loc();
    // }
}
