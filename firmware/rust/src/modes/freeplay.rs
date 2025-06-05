use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState, MAX_TRAINS, NOMINAL_TRAIN_SIZE},
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
    train::{Train, DEFAULT_SPEED},
    NUM_DIGITS,
};

#[derive(PartialEq)]
enum Setting {
    Score,
    Trains,
    TrainCars,
    //TrainSpeed,
}

pub struct FreeplayMode {
    score: u16,
    cur_setting: Setting,
    cur_train_index: u8,
}

impl FreeplayMode {
    fn setting_display(&self, state: &GameState) -> DisplayState {
        match self.cur_setting {
            Setting::Score => DisplayState::Score(self.score),
            Setting::Trains => {
                let num_trains = state.trains.len() as u8;
                let mut text = [b' '; NUM_DIGITS as usize];
                text[0] = b'T';
                text[1] = b'0' + num_trains;
                DisplayState::Text(text)
            }
            Setting::TrainCars => {
                let idx = self.cur_train_index as usize;
                let train_len = state.trains[self.cur_train_index as usize].len();
                let mut text = [b' '; NUM_DIGITS as usize];
                text[0] = b'1' + self.cur_train_index;
                text[1] = b'0' + (train_len as u8 / 10);
                text[2] = b'0' + (train_len as u8 % 10);
                DisplayState::Text(text)
            }
        }
    }

    fn next_setting(&mut self, state: &mut GameState, inc: bool) {
        match self.cur_setting {
            Setting::Score => {
                self.cur_setting = if inc {
                    Setting::Trains
                } else {
                    self.cur_train_index = 0;
                    Setting::TrainCars
                };
            }
            Setting::Trains => {
                self.cur_train_index = 0;
                self.cur_setting = Setting::TrainCars;
            }
            Setting::TrainCars => {
                let num_trains = state.trains.len() as u8;
                if inc {
                    self.cur_train_index += 1;
                    if self.cur_train_index >= num_trains {
                        self.cur_setting = Setting::Score;
                    }
                } else {
                    if self.cur_train_index == 0 {
                        self.cur_setting = Setting::Trains;
                    } else {
                        self.cur_train_index -= 1;
                    }
                }
            }
        }

        state.display = self.setting_display(state);
    }

    fn update_setting(&mut self, state: &mut GameState, inc: bool) {
        match self.cur_setting {
            Setting::Trains => {
                if inc {
                    state.add_train(
                        Cargo::Have(LedPattern::SolidBright),
                        3,
                        NOMINAL_TRAIN_SIZE as u8,
                    );
                } else {
                    state.remove_train();
                }
            }
            Setting::TrainCars => {
                let idx = self.cur_train_index as usize;
                if let Some(train) = state.trains.get_mut(idx) {
                    if inc {
                        train.add_car(Cargo::Have(LedPattern::SolidBright));
                    } else {
                        train.remove_car();
                    }
                }
            }
            _ => {}
        }

        state.display = self.setting_display(state);
    }
}

impl Default for FreeplayMode {
    fn default() -> Self {
        FreeplayMode {
            score: 0,
            cur_setting: Setting::Score,
            cur_train_index: 0,
        }
    }
}

impl GameModeHandler for FreeplayMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 0;
        state.display = DisplayState::Score(self.score);
        state.is_over = false;

        state.init_trains(
            Cargo::Have(LedPattern::SolidBright),
            3,
            NOMINAL_TRAIN_SIZE as u8,
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
            InputEvent::DirectionButtonPressed(InputDirection::Up) => {
                self.next_setting(state, true)
            }
            InputEvent::DirectionButtonPressed(InputDirection::Down) => {
                self.next_setting(state, false)
            }
            InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                self.update_setting(state, false)
            }
            InputEvent::DirectionButtonPressed(InputDirection::Right) => {
                self.update_setting(state, true)
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
                if self.cur_setting == Setting::Score {
                    state.display = DisplayState::Score(self.score);
                }
            }
        }
    }
}
