use random_trait::Random;

use crate::{
    cargo::*,
    game_settings::GameSettings,
    location::{Direction, Location, NUM_PLATFORMS},
    random::Rand,
};

pub struct Platform {
    location: Location,
    track_location: Location, // remove and use location.next_loc() to save SRAM?
    cargo: Cargo,
    last_brightness: u8,
    phase: u8, // phase of the platform, used for PWM
    phase_inc: u8, // phase increment for speed control
}

impl Platform {
    fn new(location: Location, track_location: Location) -> Self {
        Self {
            location,
            track_location,
            cargo: Cargo::Empty,
            last_brightness: 0,
            phase: Rand::default().get_u8(), // initial phase
            phase_inc: 1, // default increment
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

    pub fn update<F>(&mut self, settings: &GameSettings, mut update_callback: F, force_update: bool) -> bool
    where
        F: FnMut(Location, u8),
    {
        self.phase = self.phase.wrapping_add(self.phase_inc);

        let brightness = self.cargo.platform_brightness(self.phase, settings.platform_brightness());
        if force_update || brightness != self.last_brightness {
            self.last_brightness = brightness;
            update_callback(self.location, brightness);
            true
        } else {
            false
        }
    }

    pub fn set_phase_speed(&mut self, speed: u8) {
        self.phase_inc = speed;
    }

    pub fn cargo(&self) -> Cargo {
        self.cargo
    }

    // pub fn location(&self) -> Location {
    //     self.location
    // }

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
