use crate::{
    cargo::*,
    game_settings::GameSettings,
    location::{Direction, Location, NUM_PLATFORMS},
};

pub struct Platform {
    location: Location,
    track_location: Location,
    cargo: Cargo,
    is_cargo_in: bool, // is the current cargo going out/shipping, or coming in/receiving?
    last_brightness: u8,
    phase: u8,     // phase of the platform, used for PWM
    phase_inc: u8, // phase increment for speed control
}

impl Platform {
    fn new(location: Location, track_location: Location) -> Self {
        Self {
            location,
            track_location,
            cargo: Cargo::Empty,
            is_cargo_in: false,
            last_brightness: 0,
            phase: 0, // initial phase
            phase_inc: 1,                    // default increment
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

    pub fn update<F>(
        &mut self,
        settings: &GameSettings,
        mut update_callback: F,
        force_update: bool,
    ) -> bool
    where
        F: FnMut(Location, u8),
    {
        self.phase = self.phase.wrapping_add(self.phase_inc);

        // cargo coming in has an inverse pattern of blinking
        let brightness = if self.is_cargo_in {
            self.cargo.platform_brightness(
                self.phase,
                settings.platform_brightness() >> 1,
                0,
            )
        } else {
            self.cargo.platform_brightness(
                self.phase,
                settings.platform_brightness() >> 1,
                settings.platform_brightness(),
            )
        };
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

    pub fn cargo(&self) -> (Cargo, bool) {
        (self.cargo, self.is_cargo_in)
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

    pub fn set_cargo_in(&mut self, cargo: Cargo) {
        self.cargo = cargo;
        self.is_cargo_in = true;
    }

    pub fn set_cargo_out(&mut self, cargo: Cargo) {
        self.cargo = cargo;
        self.is_cargo_in = false;
    }

    pub fn clear_cargo(&mut self) {
        self.cargo = Cargo::Empty;
    }
}
