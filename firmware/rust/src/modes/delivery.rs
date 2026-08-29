use heapless::Vec;
use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    location::NUM_PLATFORMS,
    modes::GameModeHandler,
    platform::Platform,
    random::Rand,
    train::Train,
    NUM_DIGITS,
};

// macro for newtypes for delivery mode variants (needed for enum_dispatch that needs unique types for each variant)
macro_rules! impl_delivery_mode_wrapper {
    ($wrapper:ident,$timer_enabled:expr) => {
        pub struct $wrapper(pub crate::modes::delivery::DeliveryMode);

        impl Default for $wrapper {
            fn default() -> Self {
                Self(crate::modes::delivery::DeliveryMode::new($timer_enabled))
            }
        }

        impl GameModeHandler for $wrapper {
            fn on_restart(&mut self, state: &mut GameState) {
                self.0.on_restart(state);
            }

            fn on_game_tick(&mut self, entities: &mut GameState) {
                self.0.on_game_tick(entities);
            }

            fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
                self.0.on_input_event(event, state);
            }

            fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
                self.0.on_train_advance(train_index, state);
            }
        }
    };
}

impl_delivery_mode_wrapper!(FreeplayDeliveryMode, false);
impl_delivery_mode_wrapper!(TimedDeliveryMode, true);

pub struct CargoTimer {
    platform_index: u8,
    ticks_left: u16,
}

pub struct DeliveryMode {
    is_alt_display: bool,
    autostop_active: i8,
    cooldown_ticks_left: u8, // event cooldown timer
    score: u16,
    timers: Vec<CargoTimer, { DeliveryMode::CARGO_TIMERS_MAX_COUNT as usize }>,
    timer_dots: u8,      // indicate time left with the 3 decimal points on display
    timer_enabled: bool, // whether expired timers cause game over or not
}

impl DeliveryMode {
    const CARGO_TIMER_TICKS: u16 = 8000; // ~ 120 seconds with current runtime at 10ms base delay
    const CARGO_TIMERS_MAX_COUNT: u8 = 8;
    const COOLDOWN_TICKS: u8 = 75;
    const TRAIN_INDEX: usize = 0;
    const TRAIN_LOOKAHEAD: u8 = 3;
    const TRAIN_MAX_SPEED: u8 = 15;
    const TRAIN_SPEED_INC: u8 = 5;

    pub fn new(timer_enabled: bool) -> Self {
        DeliveryMode {
            is_alt_display: false,
            autostop_active: 0,
            cooldown_ticks_left: 0,
            score: 0,
            timers: Vec::new(),
            timer_dots: NUM_DIGITS,
            timer_enabled,
        }
    }

    #[inline(always)]
    fn get_timer_count(&self) -> u8 {
        (3 + self.score / 7) as u8
    }

    // difficulty calc functions
    #[inline(always)]
    fn get_timer_score(&self, cargo: Cargo) -> u16 {
        match cargo {
            Cargo::Full(led_pattern) => {
                if self.timer_enabled {
                    match led_pattern {
                        LedPattern::Blink1 => 1,
                        LedPattern::Blink2 => 2,
                        LedPattern::Blink3 => 3,
                        _ => 0,
                    }
                } else {
                    1
                }
            }
            _ => 0,
        }
    }

    #[inline(always)]
    fn get_timer_ticks(&self, cargo: Cargo) -> u16 {
        if self.timer_enabled {
            match cargo {
                Cargo::Full(LedPattern::Blink1) => Self::CARGO_TIMER_TICKS >> 0,
                Cargo::Full(LedPattern::Blink2) => Self::CARGO_TIMER_TICKS >> 1,
                Cargo::Full(LedPattern::Blink3) => Self::CARGO_TIMER_TICKS >> 2,
                _ => Self::CARGO_TIMER_TICKS,
            }
        } else {
            Self::CARGO_TIMER_TICKS
        }
    }

    #[inline(always)]
    fn spawn_cargo(&self) -> Cargo {
        let divisor = if self.timer_enabled { 3 } else { 10 };
        let count = (self.score / divisor) as u8;

        if self.timer_enabled {
            let count = if count > 5 { 5 } else { count };
            let led_pattern = match Rand::from_range(0, count) {
                0 | 1 | 2 => LedPattern::Blink1,
                3 | 4 => LedPattern::Blink2,
                5 => LedPattern::Blink3,
                _ => LedPattern::Solid,
            };

            Cargo::Full(led_pattern)
        } else {
            let count = if count > 3 { 3 } else { count };
            let led_pattern = match Rand::from_range(0, count) {
                0 => LedPattern::Blink1,
                1 => LedPattern::Blink2,
                2 => LedPattern::Blink3,
                _ => LedPattern::Solid,
            };

            Cargo::Full(led_pattern)
        }
    }

    #[inline(always)]
    fn spawn_chance(&self) -> u16 {
        20 + self.score / 10
    }

    fn add_platform_timer(&mut self, platform_index: u8, cargo: Cargo) {
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
            ticks_left: self.get_timer_ticks(cargo),
        };
        self.timers.push(timer).ok();
    }

    /// Removes timer from a platform and returns the ticks left for that timer.
    fn remove_platform_timer(&mut self, platform_index: u8) -> u16 {
        if let Some(index) = self
            .timers
            .iter()
            .position(|t| t.platform_index == platform_index)
        {
            let timer = self.timers.remove(index);
            timer.ticks_left
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

    fn update_timer_dots(&mut self, ticks_left: u16) -> bool {
        let new_timer_dots = if ticks_left <= (Self::CARGO_TIMER_TICKS >> 3) {
            0
        } else if ticks_left <= (Self::CARGO_TIMER_TICKS >> 2) {
            1
        } else if ticks_left <= (Self::CARGO_TIMER_TICKS >> 1) {
            2
        } else {
            3
        };

        if new_timer_dots != self.timer_dots {
            self.timer_dots = new_timer_dots;
            true
        } else {
            false
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

    fn set_display(&mut self, state: &mut GameState, alt: bool) {
        state.display = match (alt, state.is_paused) {
            (false, _) => self.score_display(),
            (true, false) => DisplayState::OVR,
            (true, true) => {
                let mut segment_data = DisplayState::PAUSE_BYTES;
                self.add_timer_indicators(&mut segment_data);
                DisplayState::Segments(segment_data)
            }
        };
        self.is_alt_display = alt;
        self.cooldown_ticks_left = Self::COOLDOWN_TICKS;
    }

    // TODO: below helpers might be more better off in game_state.rs, platform state is
    // implicit here and can simplify the calls, even train state can be as well

    /// Places a dropoff location for a cargo on a random empty platform that the train is not currently at and
    /// is not the "source" platform location for the dropoff cargo location being placed.
    fn place_dropoff_for_cargo(
        &mut self,
        train: &Train,
        platforms: &mut [Platform; NUM_PLATFORMS],
        source_index: usize,
        cargo: Cargo,
    ) {
        let mut available: Vec<usize, NUM_PLATFORMS> = Vec::new();
        for (i, platform) in platforms.iter().enumerate() {
            if i != source_index
                && platform.is_empty()
                && !train.at_location(platform.track_location())
            {
                available.push(i).ok();
            }
        }

        if !available.is_empty() {
            let rand_index = Rand::from_range(0, available.len() as u8 - 1) as usize;
            let dest_index = available[rand_index];
            platforms[dest_index].set_cargo_in(cargo);
            self.add_platform_timer(dest_index as u8, cargo);
        }
    }

    /// Try to load or unload cargo from the train to an adjacent platform, returns true if a transfer was made
    fn try_transfer_one(
        &mut self,
        train: &mut Train,
        platforms: &mut [Platform; NUM_PLATFORMS],
    ) -> bool {
        for (platform_index, platform) in platforms.iter_mut().enumerate() {
            // skip empty platforms the train is not at
            if platform.is_empty() || !train.at_location(platform.track_location()) {
                continue;
            }

            let (platform_cargo, is_receiving) = platform.cargo();

            if platform_cargo == Cargo::Empty {
                crate::panic_with_error!(401); // should not happen, platform is not empty but cargo is empty
            }

            if is_receiving {
                if train.unload_cargo(platform_cargo) {
                    platform.clear_cargo();
                    self.remove_platform_timer(platform_index as u8);
                    self.score += self.get_timer_score(platform_cargo);

                    // bonus car for score milestones
                    if self.score == 3 || self.score == 10 || self.score == 20 || self.score == 30 {
                        train.add_car(Cargo::Empty);
                    }
                    return true;
                }
            } else if train.load_cargo(platform_cargo) {
                platform.clear_cargo();
                self.remove_platform_timer(platform_index as u8);
                self.place_dropoff_for_cargo(train, platforms, platform_index, platform_cargo);
                return true;
            }
        }

        false
    }
}

impl GameModeHandler for DeliveryMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.is_alt_display = false;
        self.autostop_active = 0;
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
        if self.cooldown_ticks_left > 0 {
            self.cooldown_ticks_left -= 1;
            return;
        }

        // toggle score and gameover/pause screens
        if state.is_over || state.is_paused {
            if self.cooldown_ticks_left == 0 {
                self.is_alt_display = !self.is_alt_display;
                self.set_display(state, self.is_alt_display);
            }
            return;
        }

        // decrement cargo timers and update blink period
        let mut min_ticks = Self::CARGO_TIMER_TICKS;
        for timer in self.timers.iter_mut() {
            timer.ticks_left = timer.ticks_left.saturating_sub(1);

            let timer_platform = &mut state.platforms[timer.platform_index as usize];

            if timer.ticks_left == (Self::CARGO_TIMER_TICKS >> 1) {
                timer_platform.set_phase_speed(2);
            } else if timer.ticks_left == (Self::CARGO_TIMER_TICKS >> 2) {
                timer_platform.set_phase_speed(3);
            } else if timer.ticks_left == (Self::CARGO_TIMER_TICKS >> 3) {
                timer_platform.set_phase_speed(6);
            } else if self.timer_enabled && timer.ticks_left == 0 {
                state.is_over = true;
                self.set_display(state, true);
                return;
            }

            min_ticks = min_ticks.min(timer.ticks_left);
        }

        // update display with new timer dots if needed
        if self.update_timer_dots(min_ticks) {
            state.display = self.score_display();
        }

        // spawn a new cargo timer if conditions are all met
        if self.timers.len() < self.get_timer_count() as usize && !self.timers.is_full() {
            for (platform_index, platform) in state.platforms.iter_mut().enumerate() {
                // cargo spawn chance increases with score
                if platform.is_empty() && Rand::default().get_u16() <= self.spawn_chance() {
                    let spawned_cargo = self.spawn_cargo();
                    platform.set_cargo_out(spawned_cargo);
                    self.add_platform_timer(platform_index as u8, spawned_cargo);

                    // we're done now if we hit max timers after adding this one
                    if self.timers.len() as u8 > self.get_timer_count() {
                        break;
                    }
                }
            }
        }

        let autostop_level = state.settings.autostop_level();
        if autostop_level > 0 {
            if self.autostop_active == 0
                && autostop_level >= 2
                && state.trains[Self::TRAIN_INDEX].speed() == Self::TRAIN_SPEED_INC * 2
                && state.train_approaching_platform(Self::TRAIN_INDEX, Self::TRAIN_LOOKAHEAD)
            {
                state.trains[Self::TRAIN_INDEX].set_speed(Self::TRAIN_SPEED_INC);
                self.autostop_active = -2;
                // no cooldown needed, train will stop at end of platform
            } else if state.trains[Self::TRAIN_INDEX].speed() == Self::TRAIN_SPEED_INC
                && self.autostop_active <= 0 // not currently resuming
                && state.train_ready_at_platform(Self::TRAIN_INDEX)
            {
                state.trains[Self::TRAIN_INDEX].set_speed(0);
                if self.autostop_active == 0 {
                    self.autostop_active = -1;
                }
                self.cooldown_ticks_left = Self::COOLDOWN_TICKS;
                return; // do not do stopped behavior below yet!
            }
        }

        let speed = state.trains[Self::TRAIN_INDEX].speed();
        if speed == 0 {
            if self.autostop_active <= 0 {
                let acted = self
                    .try_transfer_one(&mut state.trains[Self::TRAIN_INDEX], &mut state.platforms);

                if acted {
                    state.display = self.score_display();
                    self.cooldown_ticks_left = Self::COOLDOWN_TICKS;
                } else if self.autostop_active < 0 {
                    self.autostop_active = -self.autostop_active;
                    self.cooldown_ticks_left = Self::COOLDOWN_TICKS;
                }
            } else {
                state.trains[Self::TRAIN_INDEX].set_speed(Self::TRAIN_SPEED_INC);
                state.trains[Self::TRAIN_INDEX].force_advance_next_tick();
                self.cooldown_ticks_left = Self::COOLDOWN_TICKS;
            }
        } else if self.autostop_active > 0 {
            let target_speed = Self::TRAIN_SPEED_INC * self.autostop_active as u8;
            if speed < target_speed {
                let new_speed = speed + Self::TRAIN_SPEED_INC;
                state.trains[Self::TRAIN_INDEX].set_speed(new_speed);
                if new_speed < target_speed {
                    self.cooldown_ticks_left = Self::COOLDOWN_TICKS >> 2;
                }
            } else {
                self.autostop_active = 0;
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        if state.is_over {
            self.on_restart(state);
        }

        match event {
            InputEvent::DirectionButtonPressed(direction) => {
                // any direction input cancels current autostop behavior
                self.autostop_active = 0;
                self.cooldown_ticks_left = Self::COOLDOWN_TICKS;

                match direction {
                    InputDirection::Left => {
                        let speed = state.trains[0].speed();
                        state.trains[0].set_speed(speed.saturating_sub(Self::TRAIN_SPEED_INC));
                    }
                    InputDirection::Right => {
                        let speed = state.trains[0].speed();
                        let new_speed = speed
                            .saturating_add(Self::TRAIN_SPEED_INC)
                            .min(Self::TRAIN_MAX_SPEED);
                        state.trains[0].set_speed(new_speed);
                    }
                    InputDirection::Up | InputDirection::Down => {
                        state.is_paused = !state.is_paused;
                        self.set_display(state, state.is_paused);
                    }
                }
            }
            _ => {}
        }
    }

    fn on_train_advance(&mut self, _: usize, _: &mut GameState) {}
}
