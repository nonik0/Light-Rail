use random_trait::Random;

use crate::{
    common::*,
    location::{Location, NUM_PLATFORMS},
    panic::set_panic_msg,
    panic_to_digits,
    random::Rng,
    train::Train,
};

pub struct Platform {
    location: Location,
    track_location: Location,
    cargo: Cargo,
}

impl Platform {
    fn new(location: Location, track_location: Location) -> Self {
        Self {
            location,
            track_location,
            cargo: Cargo::Empty,
        }
    }

    pub fn take() -> [Platform; NUM_PLATFORMS] {
        static mut TAKEN: bool = false;
        unsafe {
            if TAKEN {
                panic_to_digits!("take() called more than once");
            }
            TAKEN = true;
        }

        let platforms = Location::platform_locs().map(|location| {
            let track_location = location.adjacent_track();
            Platform::new(location, track_location)
        });
        platforms
    }

    pub fn tick(&mut self, trains: &[Train]) -> Option<EntityUpdate> {
        if self.cargo == Cargo::Full {
            for train in trains {
                if train.front() == self.track_location {
                    self.clear_cargo();
                    return Some(EntityUpdate::new(
                        self.location,
                        Contents::Platform(Cargo::Empty),
                    ));
                }
            }
        } else {
            if Rng::default().get_u16() <= 100 {
                self.cargo = Cargo::Full;
                return Some(EntityUpdate::new(
                    self.location,
                    Contents::Platform(Cargo::Full),
                ));
            }
        }

        None
    }

    pub fn location(&self) -> Location {
        self.location
    }

    pub fn track_location(&self) -> Location {
        self.track_location
    }

    pub fn set_cargo(&mut self) {
        self.cargo = Cargo::Full;
    }

    pub fn clear_cargo(&mut self) {
        self.cargo = Cargo::Empty;
    }
}
