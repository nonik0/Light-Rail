use heapless::Vec;
use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    location::NUM_PLATFORMS,
    modes::GameModeHandler,
    random::Rand,
};

pub struct CargoTimer {
    platform_index: u8,
    ticks_left: u16,
}

pub struct TimeMode {
    score: u16,
    timers: Vec<CargoTimer, { TimeMode::MAX_CARGO as usize }>,
}

impl TimeMode {
    const MAX_CARGO: u8 = 3;
    const MAX_SPEED: u8 = 15;
    const SPEED_INC: u8 = 5;
    const TIMER_TICKS: u16 = 8000;

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
        }
        else {
            crate::panic_with_error!(403);
        }
    }
}

impl Default for TimeMode {
    fn default() -> Self {
        TimeMode {
            score: 0,
            timers: Vec::new(),
        }
    }
}

impl GameModeHandler for TimeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 0;
        self.timers.clear();

        state.is_over = false;
        state.display = DisplayState::Score(self.score);
        state.init_trains(Cargo::Empty, 3, 5);
        state.clear_platforms();
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for timer in self.timers.iter_mut() {
            timer.ticks_left = timer.ticks_left.saturating_sub(1);

            let timer_platform = &mut state.platforms[timer.platform_index as usize];
            match timer.ticks_left {
                3000 => timer_platform.set_phase_speed(2),
                1000 => timer_platform.set_phase_speed(3),
                0 => {
                    state.display = DisplayState::Text(*b"ovr");
                    state.is_over = true;
                    return;
                }
                _ => {}
            }
        }

        if !self.timers.is_full() {
            for (platform_index, platform) in state.platforms.iter_mut().enumerate() {
                if platform.is_empty() && Rand::default().get_u16() <= 20 {
                    let led_pattern = match Rand::default().get_u8() % 3 {
                        0 => LedPattern::Blink1,
                        1 => LedPattern::Blink2,
                        2 => LedPattern::Blink3,
                        _ => LedPattern::SolidBright,
                    };
                    platform.set_cargo(Cargo::Have(led_pattern));
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
                    let platform_cargo = platform.cargo();
                    match platform_cargo {
                        // pick up cargo if train has space, add a platform with cargo to dropoff later
                        Cargo::Have(pattern) => {
                            if train.load_cargo(platform_cargo) {
                                platform.clear_cargo();
                                self.remove_platform_timer(platform_index as u8);
                                cargo_to_place.push(Cargo::Want(pattern)).ok();
                            }
                        }
                        // drop off cargo if train has what platform wants
                        Cargo::Want(pattern) => {
                            if train.unload_cargo(Cargo::Have(pattern)) {
                                platform.clear_cargo();
                                self.remove_platform_timer(platform_index as u8);
                                self.score += 1;
                                state.display = DisplayState::Score(self.score);

                                if self.score == 3 || self.score == 10 || self.score == 20 {
                                    train.add_car(Cargo::Empty);
                                }
                            }
                        }
                        _ => {}
                    }
                }
            }

            // find a random empty platform to drop off cargo
            let mut available_platform_indices: Vec<usize, NUM_PLATFORMS> = Vec::new();
            for (i, platform) in state.platforms.iter().enumerate() {
                if platform.is_empty() && !train.at_location(platform.track_location()) {
                    available_platform_indices.push(i).unwrap();
                }
            }

            for cargo in cargo_to_place {
                if !available_platform_indices.is_empty() {
                    let rand_index = Rand::default().get_usize() % available_platform_indices.len();
                    let rand_platform_index = available_platform_indices[rand_index];
                    state.platforms[rand_platform_index].set_cargo(cargo);
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
