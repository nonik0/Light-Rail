use embedded_hal::i2c::I2c;
use heapless::Vec;
use random_trait::Random;

use crate::{
    common::*,
    game::{DisplayState, GameState},
    input::{InputDirection, InputEvent},
    location::NUM_PLATFORMS,
    modes::GameModeHandler,
    platform,
    random::Rand,
    train::DEFAULT_SPEED,
};

pub struct TimeMode {
    score: u16,
    num_active_cargo: u8,
}

impl Default for TimeMode {
    fn default() -> Self {
        TimeMode {
            score: 0,
            num_active_cargo: 0,
        }
    }
}

impl GameModeHandler for TimeMode {
    fn on_restart(&mut self, state: &mut GameState) {
        self.score = 1;
        state.is_over = false;
        state.display = DisplayState::Score(self.score);
        state.redraw = true;

        // set up starter train, length 3
        while state.trains.len() > 1 {
            state.trains.pop();
        }
        state.trains[0].set_state(3, Cargo::Empty, DEFAULT_SPEED);

        // clear cargo on all platforms
        for platform in state.platforms.iter_mut() {
            platform.clear_cargo();
        }
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        if self.num_active_cargo < 3 {
            for platform in state.platforms.iter_mut() {
                if platform.is_empty() && Rand::default().get_u16() <= 20 {
                    let led_pattern = match Rand::default().get_u8() % 3 {
                        0 => LedPattern::Blink1,
                        1 => LedPattern::Blink2,
                        2 => LedPattern::Blink3,
                        _ => LedPattern::SolidBright,
                    };
                    platform.set_cargo(Cargo::Have(led_pattern));
                    self.num_active_cargo += 1;
                }
            }
        }

        // if train is stopped
        let train = &mut state.trains[0];
        if train.speed() == 0 {
            let mut cargo_spawn: Vec<Cargo, NUM_PLATFORMS> = Vec::new();

            for platform in state.platforms.iter_mut() {
                // if train is at platform and platform has cargo
                if train.at_location(platform.track_location()) && !platform.is_empty() {
                    let platform_cargo = platform.cargo();
                    match platform_cargo {
                        // pick up cargo if train has space, then add cargo drop off to random other empty platform
                        Cargo::Have(pattern) => {
                            if train.load_cargo(platform.cargo()) {
                                platform.clear_cargo();
                                //cargo_spawn.push(Cargo::Want(pattern));
                                cargo_spawn.push(Cargo::Have(pattern));
                            }
                        }
                        // // drop off cargo if train has it
                        // Cargo::Want(pattern) => {
                        //     if train.unload_cargo(platform_cargo) {
                        //         platform.clear_cargo();
                        //         self.num_active_cargo -= 1;

                        //         self.score += 1;
                        //         state.display = DisplayState::Score(self.score);
                        //     }
                        // }
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

            for cargo in cargo_spawn {
                if !available_platform_indices.is_empty() {
                    let rand_index = Rand::default().get_usize() % available_platform_indices.len();
                    let rand_platform_index = available_platform_indices[rand_index];
                    state.platforms[rand_platform_index].set_cargo(cargo);
                    available_platform_indices.remove(rand_index);
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
}
