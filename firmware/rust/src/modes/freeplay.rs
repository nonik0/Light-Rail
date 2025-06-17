use as1115::ascii_to_segment;
use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
    NUM_DIGITS,
};

#[derive(PartialEq)]
enum Setting {
    Score,
    RandomSwitching,
    Trains,
    TrainCars,
    TrainSpeed,
}

pub struct FreeplayMode {
    score: u16,
    cur_setting: Setting,
    cur_train_index: u8,
    random_switching: bool,
}

impl FreeplayMode {
    const MAX_SPEED: u8 = 15;
    const SPEED_INC: u8 = 5;

    fn setting_display(&self, state: &GameState) -> DisplayState {
        match self.cur_setting {
            Setting::Score => DisplayState::Score(self.score),
            Setting::RandomSwitching => {
                let mut segments = [b' '; NUM_DIGITS as usize];
                segments[0] = ascii_to_segment(b'Y');
                segments[1] = ascii_to_segment(b'R') | as1115::segments::DP;
                segments[2] = ascii_to_segment(if self.random_switching { b'1' } else { b'0' });
                DisplayState::Segments(segments)
            }
            Setting::Trains => {
                let num_trains = state.trains.len() as u8;
                let mut segments = [b' '; NUM_DIGITS as usize];
                segments[0] = ascii_to_segment(b'T') | as1115::segments::DP;
                segments[1] = 0;
                segments[2] = ascii_to_segment(b'0' + num_trains);
                DisplayState::Segments(segments)
            }
            Setting::TrainCars => {
                let train_len = state.trains[self.cur_train_index as usize].len();
                let mut segments = [b' '; NUM_DIGITS as usize];
                segments[0] = ascii_to_segment(b'1' + self.cur_train_index) | as1115::segments::DP;
                segments[1] = ascii_to_segment(b'0' + (train_len as u8 / 10));
                segments[2] = ascii_to_segment(b'0' + (train_len as u8 % 10));
                DisplayState::Segments(segments)
            }
            Setting::TrainSpeed => {
                let train_speed = state.trains[self.cur_train_index as usize].speed();
                let mut segments = [b' '; NUM_DIGITS as usize];
                segments[0] = ascii_to_segment(b'U');
                segments[1] = ascii_to_segment(b'1' + self.cur_train_index) | as1115::segments::DP;
                segments[2] = ascii_to_segment(b'0' + (train_speed / Self::SPEED_INC));
                DisplayState::Segments(segments)
            }
        }
    }

    fn next_setting(&mut self, state: &mut GameState, inc: bool) {
        if inc {
            match self.cur_setting {
                Setting::Score => {
                    self.cur_setting = Setting::RandomSwitching;
                }
                Setting::RandomSwitching => {
                    self.cur_setting = Setting::Trains;
                }
                Setting::Trains => {
                    self.cur_train_index = 0;
                    self.cur_setting = Setting::TrainCars;
                }
                Setting::TrainCars => {
                    self.cur_train_index += 1;
                    if self.cur_train_index >= state.trains.len() as u8 {
                        self.cur_train_index = 0;
                        self.cur_setting = Setting::TrainSpeed;
                    }
                }
                Setting::TrainSpeed => {
                    self.cur_train_index += 1;
                    if self.cur_train_index >= state.trains.len() as u8 {
                        self.cur_setting = Setting::Score;
                    }
                }
            }
        } else {
            match self.cur_setting {
                Setting::Score => {
                    self.cur_train_index = (state.trains.len() - 1) as u8;
                    self.cur_setting = Setting::TrainSpeed;
                }
                Setting::RandomSwitching => {
                    self.cur_setting = Setting::Score;
                }
                Setting::Trains => {
                    self.cur_setting = Setting::RandomSwitching;
                }
                Setting::TrainCars => {
                    if self.cur_train_index == 0 {
                        self.cur_setting = Setting::Trains;
                    } else {
                        self.cur_train_index -= 1;
                    }
                }
                Setting::TrainSpeed => {
                    if self.cur_train_index == 0 {
                        self.cur_train_index = (state.trains.len() - 1) as u8;
                        self.cur_setting = Setting::TrainCars;
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
            Setting::RandomSwitching => {
                self.random_switching = !self.random_switching;
            }
            Setting::Trains => {
                if inc {
                    state.add_train(Cargo::Full(LedPattern::Solid), 3, TRAIN_SIZE as u8, None);
                } else {
                    state.remove_train();
                }
            }
            Setting::TrainCars => {
                if let Some(train) = state.trains.get_mut(self.cur_train_index as usize) {
                    if inc {
                        train.add_car(Cargo::Full(LedPattern::Solid));
                    } else {
                        train.remove_car();
                    }
                    state.redraw = true;
                }
            }
            Setting::TrainSpeed => {
                if let Some(train) = state.trains.get_mut(self.cur_train_index as usize) {
                    let speed = train.speed();
                    if inc {
                        if speed < Self::MAX_SPEED {
                            train.set_speed(speed + Self::SPEED_INC);
                        }
                    } else {
                        if speed > Self::SPEED_INC {
                            train.set_speed(speed - Self::SPEED_INC);
                        } else {
                            train.set_speed(0);
                        }
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
            random_switching: true,
        }
    }
}

impl GameModeHandler for FreeplayMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 0;
        state.display = DisplayState::Score(self.score);
        state.is_over = false;

        state.init_trains(Cargo::Full(LedPattern::Solid), 3, TRAIN_SIZE as u8);
        state.init_platforms(Cargo::Full(LedPattern::Solid));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo_out(Cargo::Full(LedPattern::Solid));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        match event {
            InputEvent::DirectionButtonPressed(InputDirection::Up) => {
                self.next_setting(state, false)
            }
            InputEvent::DirectionButtonPressed(InputDirection::Down) => {
                self.next_setting(state, true)
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
        if self.random_switching && Rand::default().get_bool() {
            state.train_switch(train_index);
        }

        // Clear cargo if train front is at a platform with cargo
        let train = &state.trains[train_index];
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
