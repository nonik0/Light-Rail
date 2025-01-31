use crate::{location::{Cargo, Location}, random::Rng};

struct Platform {
    pub location: Location,
    pub track_location: Location,
    pub cargo: Option<Cargo>,
    random: Rng,
}

impl Platform {
    pub fn new(location: Location, track_location: Location) -> Self {
        if !location.is_platform()  {
            panic!("LOC NOT PLAT");
        }

        if !track_location.is_track() {
            panic!("LOC NOT TRACK");
        }

        Self {
            location,
            track_location,
            cargo: None,
            random: Rng::default(),
        }
    }

    pub fn tick(&mut self) {
        if self.cargo.is_none() {
            return;
        }
    }

    pub fn set_cargo(&mut self, cargo: Cargo) {
        self.cargo = Some(cargo);
    }

    pub fn clear_cargo(&mut self) {
        self.cargo = None;
    }
}