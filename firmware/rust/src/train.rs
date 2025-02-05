// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use heapless::Vec;

use crate::common::*;
use crate::location::{Direction, Location};

pub const MAX_CARS: usize = 5;
pub const MAX_UPDATES: usize = MAX_CARS + 1; // train length + 1 movement
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
    pub direction: Direction,
    pub speed: u8,
    pub speed_counter: u8,
    pub cars: Vec<Car, MAX_CARS>,
}

impl Train {
    pub fn new(loc: Location, cargo: Cargo) -> Self {
        let mut cars = Vec::new();
        cars.push(Car { loc, cargo }).unwrap();
        Self {
            direction: Direction::Anode, // TODO: random?
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
            if caboose_loc.next(Direction::Anode).0 == next_car_loc {
                Direction::Cathode
            } else {
                Direction::Anode
            }
        } else {
            self.direction
        };
        let loc = caboose_loc.next(inv_caboose_dir).0;

        self.cars.push(Car { loc, cargo }).unwrap();

        Some(EntityUpdate::new(loc, Contents::Train(cargo)))
    }

    pub fn advance(&mut self) -> Option<Vec<EntityUpdate, MAX_UPDATES>> {
        self.speed_counter += self.speed;

        if self.speed_counter < MAX_SPEED {
            return None;
        }

        self.speed_counter -= MAX_SPEED;

        let mut loc_updates = Vec::new();

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
            self.cars.first().unwrap().loc.next(self.direction);
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
