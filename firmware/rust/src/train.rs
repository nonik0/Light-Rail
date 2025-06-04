// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use heapless::Vec;
use is31fl3731::gamma;
use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location},
    random::Rand,
    switch::Switch,
};

// TODO: refactor train for more efficient SRAM storage.
// train can take a slice of cars, and the game can manage a single array of cars
// and then pass a slice to the train as needed when initialized or reinitialized
pub const MAX_CARS: usize = 30;
pub const MAX_UPDATES: usize = MAX_CARS + 2; // train length + 1 movement + 1 new car
pub const DEFAULT_SPEED: u8 = 10;
const MIN_SPEED: u8 = 0;
const MAX_SPEED: u8 = 100;

#[derive(Debug)]
pub struct Car {
    pub loc: Location,
    pub cargo: Cargo,
    pub state: u8,
}

#[derive(Debug)]
pub struct Train {
    direction: Direction,
    speed: u8,
    speed_counter: u8,
    cars: Vec<Car, MAX_CARS>,
    last_caboose_loc: Location,
    phase: u8, // phase of the train, used for PWM
}

impl Train {
    pub fn new(loc: Location, cargo: Cargo, speed: Option<u8>) -> Self {
        let mut cars = Vec::new();
        cars.push(Car {
            loc,
            cargo,
            state: 0,
        })
        .unwrap();
        Self {
            direction: if Rand::default().get_bool() {
                Direction::Anode
            } else {
                Direction::Cathode
            },
            speed: speed.unwrap_or(DEFAULT_SPEED),
            speed_counter: 0,
            cars,
            last_caboose_loc: loc,
            phase: Rand::default().get_u8(), // initial phase
        }
    }

    pub fn add_car(&mut self, cargo: Cargo) -> Option<LedUpdate> {
        if self.cars.len() >= MAX_CARS {
            return None;
        }

        let caboose_loc = self.cars[self.cars.len() - 1].loc;
        let inv_caboose_dir = if self.cars.len() > 1 {
            let next_car_loc = self.cars[self.cars.len() - 2].loc;
            if caboose_loc.next(Direction::Anode, false).0 == next_car_loc {
                // TODO: check for switch
                Direction::Cathode
            } else {
                Direction::Anode
            }
        } else {
            self.direction
        };
        let loc = caboose_loc.next(inv_caboose_dir, false).0;

        self.cars
            .push(Car {
                loc,
                cargo,
                state: 0,
            })
            .unwrap();

        Some(LedUpdate::new(loc, cargo.car_brightness(0)))
    }

    pub fn remove_car(&mut self) -> Option<LedUpdate> {
        if self.cars.len() <= 1 {
            return None;
        }

        let loc = self.cars.pop().unwrap().loc;

        Some(LedUpdate::new(loc, 0))
    }

    pub fn set_state(&mut self, num_cars: u8, cargo: Cargo, speed: u8) {
        if num_cars > MAX_CARS as u8 {
            return;
        }

        self.speed = speed.clamp(MIN_SPEED, MAX_SPEED);
        self.speed_counter = 0;

        // Add cars if needed
        while self.cars.len() < num_cars as usize {
            if self.add_car(cargo).is_none() {
                break;
            }
        }

        // Remove cars if needed
        while self.cars.len() > num_cars as usize {
            if self.remove_car().is_none() {
                break;
            }
        }

        // Set cargo for all cars
        for car in self.cars.iter_mut() {
            car.cargo = cargo;
        }
    }

    /// Game tick for train, returns location updates as cars move along track
    pub fn advance<F>(&mut self, switches: &[Switch], mut update_callback: F) -> bool
    where
        F: FnMut(Location, u8),
    {
        self.phase = self.phase.wrapping_add(1);
        self.speed_counter += self.speed;

        // if speed not past threshold, update car brightnesses and return
        if self.speed_counter < MAX_SPEED {
            for car in self.cars.iter_mut() {
                let car_brightness = car.cargo.car_brightness(self.phase);
                if car.state != car_brightness {
                    car.state = car_brightness;
                    update_callback(car.loc, car_brightness);
                }
            }
            return false;
        }

        self.speed_counter -= MAX_SPEED;

        // move train from the rear, keeping track of location updates
        self.last_caboose_loc = self.cars.last().unwrap().loc;
        update_callback(self.last_caboose_loc, 0);
        if !self.cars.is_empty() {
            for i in (1..self.cars.len()).rev() {
                self.cars[i].loc = self.cars[i - 1].loc;
                update_callback(self.cars[i].loc, self.cars[i].cargo.car_brightness(self.phase));
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
        (self.cars.first_mut().unwrap().loc, self.direction) = self
            .cars
            .first()
            .unwrap()
            .loc
            .next(self.direction, is_switched);
        update_callback(
            self.cars.first().unwrap().loc,
            self.cars.first().unwrap().cargo.car_brightness(self.phase),
        );

        true
    }

    pub fn len(&self) -> usize {
        self.cars.len()
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn has_empty_cars(&self) -> bool {
        self.cars.iter().any(|car| car.cargo == Cargo::Empty)
    }

    /// Adds cargo to train, returns true if successful, false is train is full
    /// TODO: add location so cargo can be added to nearest empty car?
    pub fn load_cargo(&mut self, cargo: Cargo) -> bool {
        for car in self.cars.iter_mut() {
            if car.cargo == Cargo::Empty {
                car.cargo = cargo;
                return true;
            }
        }
        false
    }

    pub fn unload_cargo(&mut self, cargo: Cargo) -> bool {
        for car in self.cars.iter_mut() {
            if car.cargo == cargo {
                car.cargo = Cargo::Empty;
                return true;
            }
        }
        false
    }

    /// Returns the vector of cars in the train
    pub fn cars(&self) -> &[Car] {
        &self.cars
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

    /// Return train cargo at the given location, if any
    pub fn cargo_at_location(&self, loc: Location) -> Option<Cargo> {
        self.cars
            .iter()
            .find(|car| car.loc == loc)
            .map(|car| car.cargo)
    }

    /// Set the cargo of the car at the given location, returns true if successful
    pub fn set_cargo_at_location(&mut self, loc: Location, cargo: Cargo) -> bool {
        for car in self.cars.iter_mut() {
            if car.loc == loc {
                car.cargo = cargo;
                return true;
            }
        }
        false
    }

    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed.clamp(MIN_SPEED, MAX_SPEED);
        self.speed_counter = 0;
    }
}

impl core::ops::Index<usize> for Train {
    type Output = Car;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cars[index]
    }
}

pub struct TrainUpdate {
    pub location: Location,
    pub cargo: Cargo,
}
