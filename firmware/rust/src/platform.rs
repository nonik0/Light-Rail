// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use random_trait::Random;

use crate::{
    location::{Cargo, Location, LocationUpdate, NUM_PLATFORMS, PLATFORM_INDICES},
    random::Rng,
};

pub struct Platform {
    location: Location,
    track_location: Location,
    opt_cargo: Option<Cargo>,
}

impl Platform {
    fn new(location: Location, track_location: Location) -> Self {
        Self {
            location,
            track_location,
            opt_cargo: None,
        }
    }

    pub fn take() -> [Platform; NUM_PLATFORMS] {
        // TODO: panic if called more than once
        PLATFORM_INDICES.map(|index| {
            let location = Location { index: index as u8 };
            let track_location = location.adjacent_track();
            Platform::new(location, track_location)
        })
    }
    pub fn tick(&mut self) -> Option<LocationUpdate> {
        if self.opt_cargo.is_none() || Rng::default().get_u16() > 100 {
            return None;
        }

        Some(LocationUpdate::new(self.location, Some(Cargo::Full)))
    }

    pub fn set_cargo(&mut self, cargo: Cargo) {
        self.opt_cargo = Some(cargo);
    }

    pub fn clear_cargo(&mut self) {
        self.opt_cargo = None;
    }
}
