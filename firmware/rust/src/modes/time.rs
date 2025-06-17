use heapless::Vec;
use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    location::NUM_PLATFORMS,
    modes::GameModeHandler,
    random::Rand,
    NUM_DIGITS,
};

pub struct CargoTimer {
    platform_index: u8,
    ticks_left: u16,
}

pub struct TimeMode {
    score: u16,
    timers: Vec<CargoTimer, { TimeMode::MAX_CARGO as usize }>,
    timer_dots: u8, // indicate time left with the 3 decimal points on display
}

impl TimeMode {
    const MAX_CARGO: u8 = 3;
    const MAX_SPEED: u8 = 15;
    const SPEED_INC: u8 = 5;
    const TIMER_TICKS: u16 = 8000; // ~ 120 seconds with current runtime at 10ms base delay

    fn add_platform_timer(&mut self, platform_index: u8) {
        if self.timers.is_full()
            || self
                .timers
                .iter()
                .any(|t| t.platform_index == platform_index)
        {
            return;
        }

        let timer = CargoTimer {
            platform_index,
            ticks_left: Self::TIMER_TICKS,
        };
        self.timers.push(timer).ok();
    }

    fn remove_platform_timer(&mut self, platform_index: u8) {
        if let Some(index) = self
            .timers
            .iter()
            .position(|t| t.platform_index == platform_index)
        {
            self.timers.remove(index);
        } else {
            crate::panic_with_error!(400);
        }
    }

    fn get_display(&self) -> DisplayState {
        let mut segment_data = [0u8; NUM_DIGITS as usize];
        segment_data[0] = as1115::NUMBERS[((self.score / 100) % 10) as usize];
        segment_data[1] = as1115::NUMBERS[((self.score / 10) % 10) as usize];
        segment_data[2] = as1115::NUMBERS[(self.score % 10) as usize];

        for i in (3 - self.timer_dots as usize)..3 {
            segment_data[i] |= as1115::segments::DP;
        }

        DisplayState::Segments(segment_data)
    }
}

impl Default for TimeMode {
    fn default() -> Self {
        TimeMode {
            score: 0,
            timers: Vec::new(),
            timer_dots: NUM_DIGITS,
        }
    }
}

impl GameModeHandler for TimeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 0;
        self.timer_dots = NUM_DIGITS;
        self.timers.clear();

        state.is_over = false;
        state.init_trains(Cargo::Empty, 3, 5);
        state.clear_platforms();
        state.display = self.get_display();
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        let mut timer_update = false;
        for timer in self.timers.iter_mut() {
            timer.ticks_left = timer.ticks_left.saturating_sub(1);

            let timer_platform = &mut state.platforms[timer.platform_index as usize];
            if timer.ticks_left == (Self::TIMER_TICKS >> 1) {
                timer_platform.set_phase_speed(2);
                if self.timer_dots > 2 {
                    self.timer_dots = 2;
                    timer_update = true;
                }
            } else if timer.ticks_left == (Self::TIMER_TICKS >> 2) {
                timer_platform.set_phase_speed(3);
                if self.timer_dots > 1 {
                    self.timer_dots = 1;
                    timer_update = true;
                }
            } else if timer.ticks_left == (Self::TIMER_TICKS >> 3) {
                timer_platform.set_phase_speed(6);
                if self.timer_dots > 0 {
                    self.timer_dots = 0;
                    timer_update = true;
                }
            } else if timer.ticks_left == 0 {
                state.display = DisplayState::Text(*b"ovr");
                state.is_over = true;
                return;
            }
        }

        if timer_update {
            state.display = self.get_display();
        }

        if !self.timers.is_full() {
            for (platform_index, platform) in state.platforms.iter_mut().enumerate() {
                if platform.is_empty() && Rand::default().get_u16() <= 20 {
                    let led_pattern = match Rand::from_range(0, 3) {
                        0 => LedPattern::Blink1,
                        1 => LedPattern::Blink2,
                        2 => LedPattern::Blink3,
                        _ => LedPattern::Solid,
                    };
                    platform.set_cargo_out(Cargo::Full(led_pattern));
                    self.add_platform_timer(platform_index as u8);
                }
            }
        }

        // if train is stopped
        let train = &mut state.trains[0];
        if train.speed() == 0 {
            let mut cargo_to_place: Vec<Cargo, NUM_PLATFORMS> = Vec::new();

            for (platform_index, platform) in state.platforms.iter_mut().enumerate() {
                // if train is at platform and platform has cargo
                if !platform.is_empty() && train.at_location(platform.track_location()) {
                    let (platform_cargo, is_receiving) = platform.cargo();
                    if let Cargo::Full(pattern) = platform_cargo {
                        // try to unload cargo if platform is receiving cargo
                        if is_receiving {
                            // unload is true only if train has the same type of cargo
                            if train.unload_cargo(Cargo::Full(pattern)) {
                                platform.clear_cargo();
                                self.remove_platform_timer(platform_index as u8);
                                self.score += 1;
                                state.display = self.get_display();

                                if self.score == 3 || self.score == 10 || self.score == 20 {
                                    train.add_car(Cargo::Empty);
                                }

                                // TODO: platform/difficulty increase
                            }
                        } else {
                            if train.load_cargo(platform_cargo) {
                                platform.clear_cargo();
                                self.remove_platform_timer(platform_index as u8);
                                cargo_to_place.push(Cargo::Full(pattern)).ok();
                            }
                        }
                    }
                }
            }

            // find a random empty platform to drop off cargo
            let mut available_platform_indices: Vec<usize, NUM_PLATFORMS> = Vec::new();
            for (i, platform) in state.platforms.iter().enumerate() {
                if platform.is_empty() && !train.at_location(platform.track_location()) {
                    available_platform_indices.push(i).ok();
                }
            }

            for cargo in cargo_to_place {
                if !available_platform_indices.is_empty() {
                    let rand_index = Rand::from_range(0, available_platform_indices.len() as u8 - 1) as usize;
                    let rand_platform_index = available_platform_indices[rand_index];
                    state.platforms[rand_platform_index].set_cargo_in(cargo);
                    self.add_platform_timer(rand_platform_index as u8);
                    available_platform_indices.remove(rand_index);
                }
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        if state.is_over {
            self.on_restart(state);
        }

        match event {
            InputEvent::DirectionButtonPressed(direction) => match direction {
                InputDirection::Left => {
                    let speed = state.trains[0].speed();
                    state.trains[0].set_speed(speed.saturating_sub(Self::SPEED_INC));
                }
                InputDirection::Right => {
                    let speed = state.trains[0].speed();
                    let new_speed = speed.saturating_add(Self::SPEED_INC).min(Self::MAX_SPEED);
                    state.trains[0].set_speed(new_speed);
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn on_train_advance(&mut self, _: usize, _: &mut GameState) {}
}
