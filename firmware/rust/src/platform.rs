// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use random_trait::Random;

use crate::{
    common::*,
    location::{Location, NUM_PLATFORMS},
    random::Rng,
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
        // TODO: panic if called more than once
        Location::platform_locs().map(|location| {
            let track_location = location.adjacent_track();
            Platform::new(location, track_location)
        })
    }
    pub fn tick(&mut self) -> Option<EntityUpdate> {
        if self.cargo == Cargo::Empty || Rng::default().get_u16() > 100 {
            return None;
        }

        self.cargo = Cargo::Full;
        Some(EntityUpdate::new(
            self.location,
            Contents::Platform(Cargo::Full),
        ))
    }

    pub fn set_cargo(&mut self) {
        self.cargo = Cargo::Full;
    }

    pub fn clear_cargo(&mut self) {
        self.cargo = Cargo::Empty;
    }
}
