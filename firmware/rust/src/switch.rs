
use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location, NUM_SWITCHES},
    panic::trace,
    panic_with_error,
    random::Rand,
    train::Train,
};

pub struct Switch {
    is_switched: bool, // false directs to next_location, true directs to fork_location
    location: Location,
    next_location: Location,
    fork_location: Location,
    // TODO cross has opposite fork locations
}

impl Switch {
    fn new(location: Location, next_location: Location, fork_location: Location) -> Self {
        Self {
            is_switched: Rand::default().get_bool(),
            location,
            next_location,
            fork_location,
        }
    }

    pub fn switch(&mut self) -> bool {
        self.is_switched = !self.is_switched;
        self.is_switched
    }

    pub fn take() -> [Switch; NUM_SWITCHES] {
        static mut TAKEN: bool = false;
        unsafe {
            if TAKEN {
                panic_with_error!(300);
            }
            TAKEN = true;
        }

        let switches = Location::switch_locs().map(|location| {
            let next_anode = location.next_loc(Direction::Anode, false);
            let next_anode_2 = location.next_loc(Direction::Anode, true);
            let next_cathode = location.next_loc(Direction::Cathode, false);
            let next_cathode_2 = location.next_loc(Direction::Cathode, true);

            // TODO: crosses have both, need to check
            let (next_location, fork_location) = if next_anode_2 != next_anode {
                (next_anode, next_anode_2)
            } else if next_cathode_2 != next_cathode {
                (next_cathode, next_cathode_2)
            } else {
                panic_with_error!(301);
            };

            Switch::new(location, next_location, fork_location)
        });
        switches
    }

    pub fn tick(&mut self) -> Option<EntityUpdate> {
        trace(b"switch tick");
        // TODO: switch lighting to indicate switch state
        None
    }

    pub fn location(&self) -> Location {
        self.location
    }
 
    pub fn next_location(&self) -> Location {
        self.next_location
    }

    pub fn fork_location(&self) -> Location {
        self.fork_location
    }
}
