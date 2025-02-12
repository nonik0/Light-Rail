use crate::location::Location;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cargo {
    Empty,
    Full,
}

// TODO: OK for now but revisit
#[derive(Debug)]
pub enum Contents {
    Empty,
    Train(Cargo),
    // platforms never move so will never be Contents::Empty like tracks/train locations
    Platform(Cargo),
}

impl Contents {
    pub fn to_pwm_value(&self) -> u8 {
        match self {
            Contents::Empty => 0,
            Contents::Train(cargo) => match cargo {
                Cargo::Empty => 50,
                Cargo::Full => 200,
            },
            Contents::Platform(cargo) => match cargo {
                Cargo::Empty => 0,
                Cargo::Full => 16,
            },
        }
    }
}

#[derive(Debug)]
pub struct EntityUpdate {
    pub location: Location,
    pub contents: Contents,
}

impl EntityUpdate {
    pub fn new(location: Location, contents: Contents) -> Self {
        Self { location, contents }
    }
}