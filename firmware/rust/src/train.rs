

const MAX_CARS: usize = 5;
const MIN_SPEED: u8 = 0;
const MAX_SPEED: u8 = 100;
const DEFAULT_SPEED: u8 = 10;
const CAR_FULL_PWM: u8 = 200;
const CAR_EMPTY_PWM: u8 = 50;

pub struct Car {
    pub location: u8,
    pub cargo: u8,
}

pub struct Train {
    pub engine_direction: bool,
    pub cars: [Car; MAX_CARS],
    pub num_cars: u8,
    pub speed: u8,
    pub speed_counter: u8,
}
