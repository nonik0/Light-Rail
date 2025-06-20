use heapless::Vec;

use crate::{
    cargo::*,
    game_settings::GameSettings,
    location::{Direction, NUM_PLATFORMS, NUM_SWITCHES},
    platform::Platform,
    random::Rand,
    switch::Switch,
    train::{Car, Train, DEFAULT_SPEED},
    NUM_DIGITS,
};

pub const MAX_CARS: usize = 128;
pub const MAX_TRAINS: usize = 8;
pub const TRAIN_SIZE: usize = MAX_CARS / MAX_TRAINS;

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayState {
    None,
    Score(u16),
    Segments([u8; NUM_DIGITS as usize]),
    Text([u8; NUM_DIGITS as usize]),
}
impl DisplayState {
    pub const DED: DisplayState = DisplayState::Text(*b"ded");
    pub const GG: DisplayState = DisplayState::Text(*b" gg");
    pub const OVR: DisplayState = DisplayState::Text(*b"ovr");
    pub const PAUSE_BYTES: [u8; NUM_DIGITS as usize] = [0u8, 0x36, 0u8];
    pub const PAUSE: DisplayState = DisplayState::Segments(Self::PAUSE_BYTES);
}

// TODO: move mutable stuff to methods and make stuff private?
pub struct GameState {
    pub target_mode_index: usize, // in state so menu mode can manipulate it
    pub is_over: bool,            // stops entity updates, game is over
    pub is_paused: bool,          // stops entity updates, game is still active
    pub redraw: bool,             // flag to redraw board LEDs
    pub display: DisplayState,
    pub settings: GameSettings,

    // game entities
    pub cars: [Car; MAX_CARS],
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
}

impl GameState {
    pub fn add_train(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8, speed: Option<u8>) {
        if self.trains.is_full() {
            return;
        }

        // TODO: for now, simple allocation method that divides evenly on MAX_TRAINS, only snake allocated single train with max cars
        let cars_ptr = unsafe { self.cars.as_mut_ptr().add(self.trains.len() * TRAIN_SIZE) };
        let loc = self.rand_platform().track_location();
        let mut train = Train::new(cars_ptr, max_cars, loc, cargo, speed);
        for _ in 1..num_cars {
            train.add_car(cargo);
        }
        self.trains.push(train).ok();
        self.redraw = true;
    }

    pub fn remove_train(&mut self) {
        if self.trains.len() > 1 {
            self.trains.pop();
            self.redraw = true;
        }
    }

    /// Initializes the game state with a single train with given parameters.
    pub fn init_trains(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8) {
        // init first train
        if self.trains.len() > 0 {
            while self.trains.len() > 1 {
                self.trains.pop();
            }

            // reuse existing train for smooth transition between modes
            let train = &mut self.trains[0];
            train.init_cars(cargo, num_cars, max_cars);
            train.set_speed(DEFAULT_SPEED);
            self.redraw = true;
        } else {
            self.add_train(cargo, num_cars, max_cars, None);
        }
    }

    pub fn init_platforms(&mut self, cargo: Cargo) {
        for platform in self.platforms.iter_mut() {
            if !platform.is_empty() {
                platform.set_cargo_out(cargo);
                platform.set_phase_speed(1);
            }
        }
    }

    pub fn clear_platforms(&mut self) {
        for platform in self.platforms.iter_mut() {
            platform.clear_cargo();
        }
    }

    pub fn rand_platform(&self) -> &Platform {
        let rand_platform_index = Rand::from_range(0, self.platforms.len() as u8 - 1) as usize;
        &self.platforms[rand_platform_index]
    }

    /// If the train just left a switch, switch it.
    pub fn train_switch(&mut self, train_index: usize) {
        let train = &self.trains[train_index];
        let caboose_loc = train.caboose().loc;
        let last_loc = train.last_loc();

        // If train just left a switch, randomly switch it
        for switch in self.switches.iter_mut() {
            if caboose_loc == switch.location() {
                continue; // Train is entering, not leaving
            }
            for dir in [Direction::Anode, Direction::Cathode] {
                if switch.active_location(dir) == Some(last_loc) {
                    switch.switch();
                    break;
                }
            }
        }
    }
}
