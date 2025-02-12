// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use heapless::Vec;
use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location},
    random::Rng,
};

pub const MAX_CARS: usize = 5;
pub const MAX_UPDATES: usize = MAX_CARS + 2; // train length + 1 movement + 1 new car
const MIN_SPEED: u8 = 0;
const MAX_SPEED: u8 = 100;
const DEFAULT_LOCATION: u8 = 0xFF;
const DEFAULT_SPEED: u8 = 10;

#[derive(Debug)]
pub struct Car {
    pub loc: Location,
    pub cargo: Cargo,
}

#[derive(Debug)]
pub struct Train {
    direction: Direction,
    entropy: Rng,
    speed: u8,
    speed_counter: u8,
    cars: Vec<Car, MAX_CARS>,
}

impl Train {
    pub fn new(loc: Location, cargo: Cargo, entropy: Rng) -> Self {
        let mut cars = Vec::new();
        cars.push(Car { loc, cargo }).unwrap();
        Self {
            direction: Direction::Anode, // TODO: random?
            entropy,
            speed: DEFAULT_SPEED,
            speed_counter: 0,
            cars,
        }
    }

    pub fn add_car(&mut self, cargo: Cargo) -> Option<EntityUpdate> {
        if self.cars.len() >= MAX_CARS {
            return None;
        }

        let caboose_loc = self.cars[self.cars.len() - 1].loc;
        let inv_caboose_dir = if self.cars.len() > 1 {
            let next_car_loc = self.cars[self.cars.len() - 2].loc;
            if caboose_loc.next(Direction::Anode, &mut self.entropy).0 == next_car_loc {
                Direction::Cathode
            } else {
                Direction::Anode
            }
        } else {
            self.direction
        };
        let loc = caboose_loc.next(inv_caboose_dir, &mut self.entropy).0;

        self.cars.push(Car { loc, cargo }).unwrap();

        Some(EntityUpdate::new(loc, Contents::Train(cargo)))
    }

    pub fn remove_car(&mut self) -> Option<EntityUpdate> {
        if self.cars.len() <= 1 {
            return None;
        }

        let loc = self.cars.pop().unwrap().loc;

        Some(EntityUpdate::new(loc, Contents::Empty))
    }

    pub fn advance(&mut self) -> Option<Vec<EntityUpdate, MAX_UPDATES>> {
        self.speed_counter += self.speed;

        if self.speed_counter < MAX_SPEED {
            return None;
        }

        self.speed_counter -= MAX_SPEED;

        let mut loc_updates = Vec::new();

        // // randomly add or remove car
        if self.cars.len() < MAX_CARS && self.entropy.get_u8() == 0 {
            let loc_update = self.add_car(Cargo::Empty).unwrap(); // just checked for space
            loc_updates.push(loc_update).unwrap();
        }
        else if self.cars.len() > 1 && self.entropy.get_u8() == 0 {
            let loc_update = self.remove_car().unwrap(); // just checked for space
            loc_updates.push(loc_update).unwrap();
        }

        // move train from the rear, keeping track of location updates
        let last_loc_update = EntityUpdate::new(self.cars.last().unwrap().loc, Contents::Empty);
        loc_updates.push(last_loc_update).unwrap();
        if !self.cars.is_empty() {
            for i in (1..self.cars.len()).rev() {
                self.cars[i].loc = self.cars[i - 1].loc;

                let loc_update =
                    EntityUpdate::new(self.cars[i].loc, Contents::Train(self.cars[i].cargo));
                loc_updates.push(loc_update).unwrap();
            }
        }

        // advance front car to next location, adding final location update
        (self.cars.first_mut().unwrap().loc, self.direction) =
            self.cars.first().unwrap().loc.next(self.direction, &mut self.entropy);
        let loc_update = EntityUpdate::new(
            self.cars.first().unwrap().loc,
            Contents::Train(self.cars.first().unwrap().cargo),
        );
        loc_updates.push(loc_update).unwrap();

        Some(loc_updates)
    }

    pub fn front(&self) -> Location {
        self.cars.first().unwrap().loc
    }
}

pub struct TrainUpdate {
    pub location: Location,
    pub cargo: Cargo,
}
