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
pub const DEFAULT_SPEED: u8 = 10;
const MIN_SPEED: u8 = 0;
const MAX_SPEED: u8 = 100;

#[derive(Clone, Copy, Debug, Default)]
pub struct Car {
    pub loc: Location,
    pub cargo: Cargo,
    pub last_brightness: u8,
}

#[derive(Debug)]
pub struct Train {
    direction: Direction,
    speed: u8,
    speed_counter: u8,
    cars_ptr: *mut Car,
    num_cars: u8,
    max_cars: u8,
    last_loc: Location,
    phase: u8, // phase of the train, used for PWM
}

impl Train {
    pub fn new(
        cars_ptr: *mut Car,
        max_cars: u8,
        loc: Location,
        cargo: Cargo,
        speed: Option<u8>,
    ) -> Self {
        let mut new_self = Self {
            direction: if Rand::default().get_bool() {
                Direction::Anode
            } else {
                Direction::Cathode
            },
            speed: speed.unwrap_or(DEFAULT_SPEED),
            speed_counter: 0,
            cars_ptr,
            num_cars: 0,
            max_cars,
            last_loc: loc,
            phase: Rand::default().get_u8(), // initial phase
        };

        new_self.add_car(cargo);
        new_self
    }

    pub fn add_car(&mut self, cargo: Cargo) -> Option<Location> {
        if self.num_cars >= self.max_cars {
            return None;
        }

        let loc = if self.num_cars == 0 {
            self.last_loc
        } else {
            let caboose_loc = self.caboose().loc;
            let inv_caboose_dir = if self.num_cars > 1 {
                let next_car_loc = self.cars()[self.num_cars as usize - 2].loc;
                if caboose_loc.next(Direction::Anode, false).0 == next_car_loc {
                    // TODO: check for switch
                    Direction::Cathode
                } else {
                    Direction::Anode
                }
            } else {
                self.direction
            };
            caboose_loc.next(inv_caboose_dir, false).0 // TODO: is switched
        };

        self.num_cars += 1;
        let new_car = self.cars_mut().last_mut().unwrap();
        new_car.loc = loc;
        new_car.cargo = cargo;

        Some(loc)
    }

    pub fn remove_car(&mut self) -> Option<Location> {
        if self.num_cars <= 1 {
            return None;
        }

        self.num_cars -= 1;
        let loc = self.cars()[self.num_cars as usize].loc;
        Some(loc)
    }

    pub fn set_state(&mut self, num_cars: u8, cargo: Cargo, speed: u8) {
        if num_cars > self.cars().len() as u8 {
            return;
        }

        self.speed = speed.clamp(MIN_SPEED, MAX_SPEED);
        self.speed_counter = 0;

        // Add cars if needed
        while self.cars().len() < num_cars as usize {
            if self.add_car(cargo).is_none() {
                break;
            }
        }

        // Remove cars if needed
        while self.cars().len() > num_cars as usize {
            if self.remove_car().is_none() {
                break;
            }
        }

        // Set cargo for all cars
        for car in self.cars_mut().iter_mut() {
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

        // If not enough speed accumulated, just update brightness and return
        if self.speed_counter < MAX_SPEED {
            for car in self.cars_mut().iter_mut() {
                let brightness = car.cargo.car_brightness(self.phase);
                if car.last_brightness != brightness {
                    car.last_brightness = brightness;
                    update_callback(car.loc, brightness);
                }
            }
            return false;
        }

        self.speed_counter -= MAX_SPEED;

        // Move train cars from rear to front, updating locations and brightness
        self.last_loc = self.caboose().loc;
        update_callback(self.last_loc, 0);

        let cars = self.cars_mut();
        for i in (1..cars.len()).rev() {
            cars[i].loc = cars[i - 1].loc;
            let brightness = cars[i].cargo.car_brightness(self.phase);
            update_callback(cars[i].loc, brightness);
        }

        // Determine if the front of the train is on a switched track
        let front_loc = self.front();
        let mut is_switched = switches.iter().any(|switch| {
            front_loc == switch.location() && switch.is_switched(self.direction)
        });

        // Advance the engine to the next location and update brightness
        let (next_loc, new_dir) = self.front().next(self.direction, is_switched);
        self.direction = new_dir;
        self.engine_mut().loc = next_loc;
        let brightness = self.engine().cargo.car_brightness(self.phase);
        update_callback(self.front(), brightness);
        
        true
    }

    pub fn len(&self) -> usize {
        self.num_cars as usize
    }

    pub fn speed(&self) -> u8 {
        self.speed
    }

    pub fn has_empty_cars(&self) -> bool {
        self.cars().iter().any(|car| car.cargo == Cargo::Empty)
    }

    /// Adds cargo to train, returns true if successful, false is train is full
    /// TODO: add location so cargo can be added to nearest empty car?
    pub fn load_cargo(&mut self, cargo: Cargo) -> bool {
        for car in self.cars_mut().iter_mut() {
            if car.cargo == Cargo::Empty {
                car.cargo = cargo;
                return true;
            }
        }
        false
    }

    pub fn unload_cargo(&mut self, cargo: Cargo) -> bool {
        for car in self.cars_mut().iter_mut() {
            if car.cargo == cargo {
                car.cargo = Cargo::Empty;
                return true;
            }
        }
        false
    }

    /// Returns the vector of cars in the train
    pub fn cars(&self) -> &[Car] {
        unsafe { core::slice::from_raw_parts(self.cars_ptr, self.num_cars as usize) }
    }

    /// Returns the vector of cars in the train
    pub fn cars_mut(&self) -> &mut [Car] {
        unsafe { core::slice::from_raw_parts_mut(self.cars_ptr, self.num_cars as usize) }
    }

    /// Returns reference to the first car of the train (engine)
    pub fn engine(&self) -> &Car {
        self.cars().first().unwrap()
    }

    /// Returns mutable reference to the engine (first car of the train)
    pub fn engine_mut(&mut self) -> &mut Car {
        self.cars_mut().first_mut().unwrap()
    }

    pub fn front(&self) -> Location {
        self.engine().loc
    }

    /// Returns reference to the last car of the train (caboose)
    pub fn caboose(&self) -> &Car {
        self.cars().last().unwrap()
    }

    /// Returns mutable reference to the last car of the train (caboose)
    pub fn caboose_mut(&mut self) -> &mut Car {
        self.cars_mut().last_mut().unwrap()
    }

    /// Returns the previous location of the caboose before the last move
    pub fn last_loc(&self) -> Location {
        self.last_loc
    }

    /// Returns bool if any car is at the given location
    pub fn at_location(&self, loc: Location) -> bool {
        self.cars().iter().any(|car| car.loc == loc)
    }

    /// Return train cargo at the given location, if any
    pub fn cargo_at_location(&self, loc: Location) -> Option<Cargo> {
        self.cars()
            .iter()
            .find(|car| car.loc == loc)
            .map(|car| car.cargo)
    }

    /// Set the cargo of the car at the given location, returns true if successful
    pub fn set_cargo_at_location(&mut self, loc: Location, cargo: Cargo) -> bool {
        for car in self.cars_mut().iter_mut() {
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
        &self.cars()[index]
    }
}

pub struct TrainUpdate {
    pub location: Location,
    pub cargo: Cargo,
}
