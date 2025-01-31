use heapless::Vec;

use crate::location::{Cargo, Direction, Location, LocationUpdate};

const MAX_CARS: usize = 5;
const MAX_LOC_UPDATES: usize = MAX_CARS + 1; // train length + 1 movement
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

    pub fn add_car(&mut self, cargo: Cargo) -> Option<LocationUpdate> {
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

        Some(LocationUpdate { loc, cargo })
    }

    pub fn advance(&mut self) -> Option<Vec<LocationUpdate, MAX_LOC_UPDATES>> {
        self.speed_counter += self.speed;

        if self.speed_counter < MAX_SPEED {
            return None;
        }

        self.speed_counter -= MAX_SPEED;

        // let mut loc_updates = Vec::new();

        // // move train from the rear, setting each LED accordingly
        // let loc_update = LocationUpdate {
        //     loc: self.cars[self.cars.len() - 1].loc,
        //     cargo: None,
        // };
        //loc_updates.push(Locaself.cars[i].loc, None);
        if !self.cars.is_empty() {
            for i in (1..self.cars.len()).rev() {

                self.cars[i].loc = self.cars[i - 1].loc;

                //(self.set_led)(self.cars[i].loc, Some(self.cars[i].cargo));
            }
        }

        // advance front car to next location, setting LED accordingly
        (self.cars[0].loc, self.direction) = self.cars[0].loc.next(self.direction);
        //(self.set_led)(self.cars[0].loc, Some(self.cars[0].cargo));
        //loc_updates
        None
    }
}
