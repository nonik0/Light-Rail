use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location, NUM_PLATFORMS},
    panic_with_error,
    random::Rand,
    train::Train,
};

// TODO: update so it only tracks what updates it has sent based on its state that is controlled elsewhere

pub struct Platform {
    location: Location,
    track_location: Location,
    cargo: Cargo,
    last_pwm: u8,
    phase: u8, // phase of the platform, used for PWM
}

impl Platform {
    fn new(location: Location, track_location: Location) -> Self {
        Self {
            location,
            track_location,
            cargo: Cargo::Empty,
            last_pwm: 0xFF,
            phase: Rand::default().get_u8(), // initial phase
        }
    }

    pub fn take() -> [Platform; NUM_PLATFORMS] {
        static mut TAKEN: bool = false;
        unsafe {
            if TAKEN {
                panic_with_error!(200);
            }
            TAKEN = true;
        }

        let platforms = Location::platform_locs().map(|location| {
            let track_location = location.next_loc(Direction::Anode, false); // args are ignored
            Platform::new(location, track_location)
        });
        platforms
    }

    pub fn update<F>(&mut self, mut update_callback: F) -> bool
    where
        F: FnMut(LedUpdate),
    {
        self.phase = self.phase.wrapping_add(1);

        let pwm = self.cargo.get_platform_pwm(self.phase);
        if pwm != self.last_pwm {
            self.last_pwm = pwm;
            update_callback(LedUpdate::new(self.location, pwm));
            true
        } else {
            false
        }
    }

    pub fn cargo(&self) -> Cargo {
        self.cargo
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn track_location(&self) -> Location {
        self.track_location
    }

    pub fn is_empty(&self) -> bool {
        self.cargo == Cargo::Empty
    }

    pub fn set_cargo(&mut self, cargo: Cargo) {
        self.cargo = cargo;
    }

    pub fn clear_cargo(&mut self) {
        self.cargo = Cargo::Empty;
    }
}
