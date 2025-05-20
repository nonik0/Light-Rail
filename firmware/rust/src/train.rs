// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use heapless::Vec;
use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location},
    panic::trace,
    random::Rand,
    switch::Switch,
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
    speed: u8,
    speed_counter: u8,
    cars: Vec<Car, MAX_CARS>,
    last_caboose_loc: Location,
}

impl Train {
    pub fn new(loc: Location, cargo: Cargo, speed: Option<u8>) -> Self {
        let mut cars = Vec::new();
        cars.push(Car { loc, cargo }).unwrap();
        Self {
            direction: Direction::Anode, // TODO: random?
            speed: speed.unwrap_or(DEFAULT_SPEED),
            speed_counter: 0,
            cars,
            last_caboose_loc: loc,
        }
    }

    pub fn add_car(&mut self, cargo: Cargo) -> Option<EntityUpdate> {
        if self.cars.len() >= MAX_CARS {
            return None;
        }

        let caboose_loc = self.cars[self.cars.len() - 1].loc;
        let inv_caboose_dir = if self.cars.len() > 1 {
            let next_car_loc = self.cars[self.cars.len() - 2].loc;
            if caboose_loc.next(Direction::Anode, false).0 == next_car_loc { // TODO: check for switch
                Direction::Cathode
            } else {
                Direction::Anode
            }
        } else {
            self.direction
        };
        let loc = caboose_loc.next(inv_caboose_dir, false).0;

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

    /// Game tick for train, returns location updates as cars move along track
    pub fn advance(&mut self, switches: &[Switch]) -> Option<Vec<EntityUpdate, MAX_UPDATES>> {
        trace(b"advance");
        self.speed_counter += self.speed;

        if self.speed_counter < MAX_SPEED {
            return None;
        }

        self.speed_counter -= MAX_SPEED;

        let mut loc_updates = Vec::new();

        // move train from the rear, keeping track of location updates
        self.last_caboose_loc = self.cars.last().unwrap().loc;
        let last_loc_update = EntityUpdate::new(self.last_caboose_loc, Contents::Empty);
        loc_updates.push(last_loc_update).unwrap();
        if !self.cars.is_empty() {
            for i in (1..self.cars.len()).rev() {
                self.cars[i].loc = self.cars[i - 1].loc;

                let loc_update =
                    EntityUpdate::new(self.cars[i].loc, Contents::Train(self.cars[i].cargo));
                loc_updates.push(loc_update).unwrap();
            }
        }

        // check switch state, if front car is on a switch
        let front_loc = self.cars.first().unwrap().loc;
        let mut is_switched = false;
        for switch in switches {
            if front_loc == switch.location() {
                is_switched = switch.is_switched(self.direction);
                break;
            }
        }

        // advance front car to next location, adding final location update
        (self.cars.first_mut().unwrap().loc, self.direction) =
            self.cars.first().unwrap().loc.next(self.direction, is_switched);
        let loc_update = EntityUpdate::new(
            self.cars.first().unwrap().loc,
            Contents::Train(self.cars.first().unwrap().cargo),
        );
        loc_updates.push(loc_update).unwrap();

        Some(loc_updates)
    }

    /// Returns the location of the front car
    pub fn front(&self) -> Location {
        self.cars.first().unwrap().loc
    }

    /// Returns the location of the engine (first car of the train)
    pub fn engine(&self) -> Location {
        self.cars.first().unwrap().loc
    }

    /// Returns the location of the caboose (last car of the train)
    pub fn caboose(&self) -> Location {
        self.cars.last().unwrap().loc
    }

    /// Returns the previous location of the caboose before the last move
    pub fn last_loc(&self) -> Location {
        self.last_caboose_loc
    }

    /// Returns bool if any car is at the given location
    pub fn at_location(&self, loc: Location) -> bool {
        self.cars.iter().any(|car| car.loc == loc)
    }
}

pub struct TrainUpdate {
    pub location: Location,
    pub cargo: Cargo,
}
