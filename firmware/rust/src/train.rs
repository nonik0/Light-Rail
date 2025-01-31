use crate::location::Location;
use heapless::Vec;

const MAX_CARS: usize = 5;
const MIN_SPEED: u8 = 0;
const MAX_SPEED: u8 = 100;
const DEFAULT_LOCATION: u8 = 0xFF;
const DEFAULT_SPEED: u8 = 10;
const CAR_FULL_PWM: u8 = 200;
const CAR_EMPTY_PWM: u8 = 50;

#[derive(Debug)]
pub enum Cargo {
    Empty = 0,
    Full = 1,
}

#[derive(Debug)]
pub struct Car {
    pub loc: Location,
    pub cargo: Cargo,
}

pub struct Engine {
    pub speed: u8,
    pub counter: u8,
}

impl Default for Engine {
    fn default() -> Self {
        Self {
            speed: DEFAULT_SPEED,
            counter: 0,
        }
    }
}

pub struct Train {
    pub engine: Engine,
    pub cars: Vec<Car, MAX_CARS>,
}

impl Train {
    
    pub fn new(loc: Location, cargo: Cargo) -> Self {
        let mut cars = Vec::new();
        cars.push(Car { loc, cargo }).unwrap();
        Self {
            engine: Engine::default(),
            cars,
        }
    }

    pub fn advance(&mut self) -> bool {
        self.engine.counter += self.engine.speed;

        if self.engine.counter < MAX_SPEED {
            return false;
        }

        self.engine.counter -= MAX_SPEED;

        // move train from the rear, setting each LED accordingly
        if !self.cars.is_empty() {
            //set_led(self.cars[self.cars.len() - 1].loc, 0);
            for i in (1..self.cars.len()).rev() {
                self.cars[i].loc = self.cars[i - 1].loc;
                //set_led(self.cars[i].loc, TODO);
            }
        }

        // advance front car to next location, setting LED accordingly
        //self.cars[0].loc = self.cars[0].loc.next();

        true
    }
}
