
use core::panic;
use heapless::Vec;
use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location, NUM_SWITCHES},
    panic::trace,
    panic_with_error,
    random::Rand,
    train::Train,
};

pub const MAX_UPDATES: usize = 5; // TODO
pub const MIN_BRIGHTNESS: u8 = 25;
pub const MAX_BRIGHTNESS: u8 = 50;

pub struct Switch {
    location: Location,
    brightness: u8,
    brightness_delta: i8,

    // switches only have one active direction at a time
    // crosses have two active directions
    anode_switched: Option<bool>, // false directs to next_location, true directs to fork_location
    anode_next_location: Location,
    anode_fork_location: Location,
    cathode_switched: Option<bool>, // false directs to next_location, true directs to fork_location
    cathode_next_location: Location,
    cathode_fork_location: Location,
}

impl Switch {
    fn new(location: Location) -> Self {
        let anode_next_location = location.next_loc(Direction::Anode, false);
        let anode_fork_location = location.next_loc(Direction::Anode, true);
        let anode_switched = if anode_fork_location == anode_next_location {
            None
        } else {
            Some(Rand::default().get_bool())
        };

        let cathode_next_location = location.next_loc(Direction::Cathode, false);
        let cathode_fork_location = location.next_loc(Direction::Cathode, true);
        let cathode_switched = if cathode_fork_location == cathode_next_location {
            None
        } else {
            Some(Rand::default().get_bool())
        };

        Self {
            location,
            brightness: MAX_BRIGHTNESS,
            brightness_delta: 1,
            anode_switched,
            anode_next_location,
            anode_fork_location,
            cathode_switched,
            cathode_next_location,
            cathode_fork_location,
        }
    }

    pub fn switch(&mut self) {
        match (self.anode_switched, self.cathode_switched) {
            (Some(a), None) => {
                self.anode_switched = Some(!a);
            }
            (None, Some(c)) => {
                self.cathode_switched = Some(!c);
            }
            (Some(a), Some(c)) => {
                // TODO: different switch control options for cross switches
                if a && c {
                    self.anode_switched = Some(false);
                    self.cathode_switched = Some(true);
                } else if a {
                    self.anode_switched = Some(true);
                    self.cathode_switched = Some(true);
                } else if c {
                    self.anode_switched = Some(false);
                    self.cathode_switched = Some(false);
                } else {
                    self.anode_switched = Some(true);
                    self.cathode_switched = Some(false);
                }
            }
            _ => {
                panic_with_error!(301);
            }
        }
    }

    pub fn take() -> [Switch; NUM_SWITCHES] {
        static mut TAKEN: bool = false;
        unsafe {
            if TAKEN {
                panic_with_error!(300);
            }
            TAKEN = true;
        }

        Location::switch_locs().map(|location| Switch::new(location))
    }

    pub fn tick(&mut self, trains: &[Train]) -> Option<Vec<EntityUpdate, MAX_UPDATES>> {
        trace(b"switch tick");

        let mut loc_updates = Vec::new();

        let mut handle_direction = |is_switched: Option<bool>, next_location: Location, fork_location: Location, brightness: u8| {
            if let Some(switched) = is_switched {
                let (active_loc, inactive_loc) = if switched {
                    (fork_location, next_location)
                } else {
                    (next_location, fork_location)
                };

                // Only update inactive_loc if no train is present
                let inactive_occupied = trains.iter().any(|train| train.at_location(inactive_loc));
                if !inactive_occupied {
                    let inactive_loc_update = EntityUpdate::new(inactive_loc, Contents::Empty);
                    loc_updates.push(inactive_loc_update).ok();
                }

                // Only update active_loc if no train is present
                let active_occupied = trains.iter().any(|train| train.at_location(active_loc));
                if !active_occupied {
                    let active_loc_update = EntityUpdate::new(active_loc, Contents::SwitchIndicator(brightness));
                    loc_updates.push(active_loc_update).ok();
                }
            }
        };

        handle_direction(self.anode_switched, self.anode_next_location, self.anode_fork_location, self.brightness);
        handle_direction(self.cathode_switched, self.cathode_next_location, self.cathode_fork_location, self.brightness);

        if loc_updates.is_empty() {
            None
        } else {
            Some(loc_updates)
        }
    }

    pub fn is_switched(&self, direction: Direction) -> bool {
        match direction {
            Direction::Anode => self.anode_switched.unwrap_or(false),
            Direction::Cathode => self.cathode_switched.unwrap_or(false),
        }
    }

    pub fn location(&self) -> Location {
        self.location
    }
 
    pub fn next_location(&self, direction: Direction) -> Location {
        match direction {
            Direction::Anode => self.anode_next_location,
            Direction::Cathode => self.cathode_next_location,
        }
    }

    pub fn fork_location(&self, direction: Direction) -> Location {
        match direction {
            Direction::Anode => self.anode_fork_location,
            Direction::Cathode => self.cathode_fork_location,
        }
    }
}
