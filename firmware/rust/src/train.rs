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

        let loc = self.caboose().loc;
        self.num_cars -= 1;
        Some(loc)
    }

    /// Unsafe function if not called properly, should only be called when first train is initialized
    pub fn init_cars(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8) {
        self.num_cars = num_cars;
        self.max_cars = max_cars;

        for car in self.cars_mut().iter_mut() {
            car.cargo = cargo;
            car.last_brightness = 0;
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

    /// Returns the vector of cars in the train
    pub fn cars(&self) -> &[Car] {
        unsafe { core::slice::from_raw_parts(self.cars_ptr, self.num_cars as usize) }
    }

    /// Returns the number of cars in the train
    pub fn len(&self) -> usize {
        self.num_cars as usize
    }

    /// Returns speed of the train
    pub fn speed(&self) -> u8 {
        self.speed
    }

    /// Sets the speed of the train, clamping it between MIN_SPEED and MAX_SPEED
    pub fn set_speed(&mut self, speed: u8) {
        self.speed = speed.clamp(MIN_SPEED, MAX_SPEED);
        self.speed_counter = 0;
    }

    /// Adds cargo to train, returns true if train loads cargo into an available empty car
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

    /// Unloads cargo from the train, returns true if train removes cargo from a car that has it
    pub fn unload_cargo(&mut self, cargo: Cargo) -> bool {
        for car in self.cars_mut().iter_mut() {
            if car.cargo == cargo {
                car.cargo = Cargo::Empty;
                return true;
            }
        }
        false
    }

    /// Returns reference to the first car of the train (engine)
    pub fn engine(&self) -> &Car {
        self.cars().first().unwrap()
    }

    /// Returns the current location of the train engine
    pub fn front(&self) -> Location {
        self.engine().loc
    }

    /// Returns reference to the last car of the train (caboose)
    pub fn caboose(&self) -> &Car {
        self.cars().last().unwrap()
    }

    /// Returns the previous location of the caboose before the last move
    pub fn last_loc(&self) -> Location {
        self.last_loc
    }

    /// Returns bool if any car is at the given location
    pub fn at_location(&self, loc: Location) -> bool {
        self.cars().iter().any(|car| car.loc == loc)
    }

    // private mutable functions

    /// Returns mutable reference to the engine (first car of the train)
    fn engine_mut(&mut self) -> &mut Car {
        self.cars_mut().first_mut().unwrap()
    }

    /// Returns mutable reference to the last car of the train (caboose)
    fn caboose_mut(&mut self) -> &mut Car {
        self.cars_mut().last_mut().unwrap()
    }

    /// Returns the vector of cars in the train
    fn cars_mut(&self) -> &mut [Car] {
        unsafe { core::slice::from_raw_parts_mut(self.cars_ptr, self.num_cars as usize) }
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
