// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use random_trait::Random;

use crate::{
    location::{Cargo, Location, LocationUpdate},
    random::Rng,
};

pub struct Platform {
    pub location: Location,
    pub track_location: Location,
    pub opt_cargo: Option<Cargo>,
}

impl Platform {
    pub fn new(location: Location, track_location: Location) -> Self {
        if !location.is_platform() {
            panic!("LOC NOT PLAT");
        }

        if !track_location.is_track() {
            panic!("LOC NOT TRACK");
        }

        Self {
            location,
            track_location,
            opt_cargo: None,
        }
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
