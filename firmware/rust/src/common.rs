use crate::{location::Location, NUM_DIGITS};
use is31fl3731::gamma;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cargo {
    Empty,
    Full,
}

// TODO: OK for now but revisit
#[derive(Debug)]
pub enum Contents {
    Empty,
    SwitchIndicator(u8),
    Train(Cargo),
    Platform(Cargo),
}

impl Contents {
    pub fn to_pwm_value(&self) -> u8 {
        match self {
            Contents::Empty => 0,
            Contents::SwitchIndicator(brightness) => gamma(*brightness),
            Contents::Train(cargo) => match cargo {
                Cargo::Empty => 30,
                Cargo::Full => 255,
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