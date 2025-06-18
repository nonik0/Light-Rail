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
    counter: u8,
    score: u16,
    timers: Vec<CargoTimer, { TimeMode::MAX_TIMERS as usize }>,
    timer_dots: u8, // indicate time left with the 3 decimal points on display
}

impl TimeMode {
    const MAX_TIMERS: u8 = 5;
    const MAX_SPEED: u8 = 15;
    const SPEED_INC: u8 = 5;
    const DEFAULT_TIMER_TICKS: u16 = 8000; // ~ 120 seconds with current runtime at 10ms base delay

    // difficulty calc functions
    #[inline(always)]
    fn cargo_ticks_left(&self, cargo: Cargo) -> u16 {
        match cargo {
            Cargo::Full(LedPattern::Blink1) => Self::DEFAULT_TIMER_TICKS >> 0,
            Cargo::Full(LedPattern::Blink2) => Self::DEFAULT_TIMER_TICKS >> 1,
            Cargo::Full(LedPattern::Blink3) => Self::DEFAULT_TIMER_TICKS >> 2,
            _ => Self::DEFAULT_TIMER_TICKS,
        }
    }

    #[inline(always)]
    fn led_pattern_options(&self) -> u8 {
        let count = (self.score / 5 + 1) as u8;
        return if count > 5 { 5 } else { count };
    }

    #[inline(always)]
    fn max_timer_count(&self) -> u8 {
        (3 + self.score / 15) as u8
    }

    #[inline(always)]
    fn spawn_chance(&self) -> u16 {
        20 + self.score / 10
    }

    fn add_platform_timer(&mut self, platform_index: u8, ticks_left: u16) {
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
            ticks_left,
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

    #[inline(always)]
    fn add_timer_indicators(&self, segment_data: &mut [u8; NUM_DIGITS as usize]) {
        for i in (3 - self.timer_dots as usize)..3 {
            segment_data[i] |= as1115::segments::DP;
        }
    }

    fn score_display(&self) -> DisplayState {
        let mut segment_data = [0u8; NUM_DIGITS as usize];
        segment_data[0] = as1115::NUMBERS[((self.score / 100) % 10) as usize];
        segment_data[1] = as1115::NUMBERS[((self.score / 010) % 10) as usize];
        segment_data[2] = as1115::NUMBERS[((self.score / 001) % 10) as usize];

        self.add_timer_indicators(&mut segment_data);
        DisplayState::Segments(segment_data)
    }
}

impl Default for TimeMode {
    fn default() -> Self {
        TimeMode {
            counter: 0,
            score: 0,
            timers: Vec::new(),
            timer_dots: NUM_DIGITS,
        }
    }
}

impl GameModeHandler for TimeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.counter = 0;
        self.score = 0;
        self.timer_dots = NUM_DIGITS;
        self.timers.clear();

        state.is_over = false;
        state.is_paused = false;
        state.init_trains(Cargo::Empty, 3, 5);
        state.clear_platforms();
        state.display = self.score_display();
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        if state.is_over || state.is_paused {
            self.counter += 1;
            if self.counter == 0 {
                state.display = if state.is_paused {
                    let mut segment_data = DisplayState::PAUSE_BYTES;
                    self.add_timer_indicators(&mut segment_data);
                    DisplayState::Segments(segment_data)
                } else {
                    DisplayState::DED
                }
            } else if self.counter == u8::MAX >> 1 {
                state.display = self.score_display();
            }
            return;
        }

        let mut timer_update = false;
        for timer in self.timers.iter_mut() {
            timer.ticks_left = timer.ticks_left.saturating_sub(1);

            let timer_platform = &mut state.platforms[timer.platform_index as usize];
            if timer.ticks_left == (Self::DEFAULT_TIMER_TICKS >> 1) {
                timer_platform.set_phase_speed(2);
                if self.timer_dots > 2 {
                    self.timer_dots = 2;
                    timer_update = true;
                }
            } else if timer.ticks_left == (Self::DEFAULT_TIMER_TICKS >> 2) {
                timer_platform.set_phase_speed(3);
                if self.timer_dots > 1 {
                    self.timer_dots = 1;
                    timer_update = true;
                }
            } else if timer.ticks_left == (Self::DEFAULT_TIMER_TICKS >> 3) {
                timer_platform.set_phase_speed(6);
                if self.timer_dots > 0 {
                    self.timer_dots = 0;
                    timer_update = true;
                }
            } else if timer.ticks_left == 0 {
                state.display = DisplayState::OVR;
                state.is_over = true;
                return;
            }
        }

        if timer_update {
            state.display = self.score_display();
        }

        // amount of active timers increases with score
        if self.timers.len() < self.max_timer_count() as usize && !self.timers.is_full() {
            for (platform_index, platform) in state.platforms.iter_mut().enumerate() {
                // cargo spawn chance increases with score
                if platform.is_empty() && Rand::default().get_u16() <= self.spawn_chance() {
                    // chances to spawn different cargo increase with score
                    let led_pattern = match Rand::from_range(0, self.led_pattern_options()) {
                        0 | 1 | 2 => LedPattern::Blink1,
                        3 | 4 => LedPattern::Blink2,
                        5 => LedPattern::Blink3,
                        _ => LedPattern::Solid,
                    };
                    let spawned_cargo = Cargo::Full(led_pattern);

                    platform.set_cargo_out(spawned_cargo);
                    self.add_platform_timer(
                        platform_index as u8,
                        self.cargo_ticks_left(spawned_cargo),
                    );
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

                                // increase score based on cargo type
                                match pattern {
                                    LedPattern::Blink1 => self.score += 1,
                                    LedPattern::Blink2 => self.score += 2,
                                    LedPattern::Blink3 => self.score += 3,
                                    _ => {}
                                }
                                state.display = self.score_display();

                                if self.score == 3 || self.score % 10 == 0 {
                                    train.add_car(Cargo::Empty);
                                }
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
                    let rand_index =
                        Rand::from_range(0, available_platform_indices.len() as u8 - 1) as usize;
                    let rand_platform_index = available_platform_indices[rand_index];
                    state.platforms[rand_platform_index].set_cargo_in(cargo);

                    // cargo with more blinks has shorter timer and more points awarded when delivered
                    self.add_platform_timer(
                        rand_platform_index as u8,
                        self.cargo_ticks_left(cargo),
                    );
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
                InputDirection::Up | InputDirection::Down => {
                    state.is_paused = !state.is_paused;
                }
            },
            _ => {}
        }
    }

    fn on_train_advance(&mut self, _: usize, _: &mut GameState) {}
}
