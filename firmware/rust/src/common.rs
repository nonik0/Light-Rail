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
    Platform(Cargo),
}

impl Contents {
    pub fn to_pwm_value(&self) -> u8 {
        match self {
            Contents::Empty => 0,
            Contents::Train(cargo) => match cargo {
                Cargo::Empty => 1,
                Cargo::Full => 2,
            },
            Contents::Platform(cargo) => match cargo {
                Cargo::Empty => 3,
                Cargo::Full => 4,
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