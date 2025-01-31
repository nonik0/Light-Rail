use heapless::Vec;

use crate::location::{Direction, Location};

const MAX_CARS: usize = 5;
const MIN_SPEED: u8 = 0;
const MAX_SPEED: u8 = 100;
const DEFAULT_LOCATION: u8 = 0xFF;
const DEFAULT_SPEED: u8 = 10;

#[derive(Clone, Copy, Debug)]
pub enum Cargo {
    Empty = 0,
    Full = 1,
}

#[derive(Debug)]
pub struct Car {
    pub loc: Location,
    pub cargo: Cargo,
}

// impl Default for Engine {
//     fn default() -> Self {
//         Self {
//             direction: Direction::Anode, // TODO: random?
//             speed: DEFAULT_SPEED,
//             counter: 0,
//         }
//     }
// }

pub type UpdateLocationCallback = fn(Location, Option<Cargo>);

#[derive(Debug)]
pub struct Train {
    pub direction: Direction,
    pub speed: u8,
    pub speed_counter: u8,
    pub cars: Vec<Car, MAX_CARS>,
    pub set_led: UpdateLocationCallback,
}

impl Train {
    pub fn new(loc: Location, cargo: Cargo, set_led: UpdateLocationCallback) -> Self {
        let mut cars = Vec::new();
        cars.push(Car { loc, cargo }).unwrap();
        Self {
            direction: Direction::Anode, // TODO: random?
            speed: DEFAULT_SPEED,
            speed_counter: 0,
            cars,
            set_led,
        }
    }

    pub fn add_car(&mut self, cargo: Cargo) -> bool {
        if self.cars.len() >= MAX_CARS {
            return false;
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
        true
    }

    pub fn advance(&mut self) -> bool {
        self.speed_counter += self.speed;

        if self.speed_counter < MAX_SPEED {
            return false;
        }

        self.speed_counter -= MAX_SPEED;

        // move train from the rear, setting each LED accordingly
        if !self.cars.is_empty() {
            for i in (1..self.cars.len()).rev() {
                (self.set_led)(self.cars[i].loc, None);

                self.cars[i].loc = self.cars[i - 1].loc;

                (self.set_led)(self.cars[i].loc, Some(self.cars[i].cargo));
            }
        }

        // advance front car to next location, setting LED accordingly
        (self.cars[0].loc, self.direction) = self.cars[0].loc.next(self.direction);
        (self.set_led)(self.cars[0].loc, Some(self.cars[0].cargo));

        true
    }
}
