use core::panic;
use random_trait::Random;

use crate::{
    common::*,
    location::{Direction, Location, NUM_SWITCHES},
    panic_with_error,
    random::Rand,
    train::Train,
};

pub const MAX_UPDATES: usize = 5; // TODO
pub const MIN_BRIGHTNESS: u8 = 50;
pub const MAX_BRIGHTNESS: u8 = 80;

pub struct Switch {
    location: Location,
    brightness: u8,
    brightness_delta: i8,

    // switches only have one active direction (one direction has None values)
    // crosses have two active directions

    // false directs to next_location, true directs to fork_location, none means no switch in that direction
    anode_switched: Option<bool>, 
    anode_last_switched: Option<bool>, // entity update tracking
    anode_next_location: Location,
    anode_fork_location: Option<Location>, // None means no switch in that direction

    // false directs to next_location, true directs to fork_location, none means no switch in that direction
    cathode_switched: Option<bool>, 
    cathode_last_switched: Option<bool>, // entity update tracking
    cathode_next_location: Location,
    cathode_fork_location: Option<Location>, // None means no switch in that direction
}

impl Switch {
    fn new(location: Location) -> Self {
        // set up switch in a direction
        fn setup(
            location: Location,
            direction: Direction,
        ) -> (Option<bool>, Location, Option<Location>) {
            let next = location.next_loc(direction, false);
            let fork = location.next_loc(direction, true);
            if next == fork {
                (None, next, None)
            } else {
                (Some(Rand::default().get_bool()), next, Some(fork))
            }
        }

        let (anode_switched, anode_next_location, anode_fork_location) =
            setup(location, Direction::Anode);
        let (cathode_switched, cathode_next_location, cathode_fork_location) =
            setup(location, Direction::Cathode);

        Self {
            location,
            anode_switched,
            anode_last_switched: None,
            anode_next_location,
            anode_fork_location,
            cathode_switched,
            cathode_last_switched: None,
            cathode_next_location,
            cathode_fork_location,
            brightness: MAX_BRIGHTNESS,
            brightness_delta: 1,
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

    pub fn update<F>(&mut self, trains: &[Train], mut update_callback: F) -> bool
    where
        F: FnMut(EntityUpdate),
    {
        let mut update = false;

        let mut handle_direction = |is_switched: Option<bool>,
                                    last_switched: Option<bool>,
                                    next_location: Location,
                                    fork_location: Option<Location>,
                                    brightness: u8| {
            if let Some(switched) = is_switched {
                // no update if no change
                if let Some(last_switched) = last_switched {
                    if switched == last_switched {
                        return;
                    }
                }

                let (active_loc, inactive_loc) = if switched {
                    (fork_location.unwrap(), next_location)
                } else {
                    (next_location, fork_location.unwrap())
                };

                // Only update inactive_loc if no train is present
                let inactive_occupied = trains.iter().any(|train| train.at_location(inactive_loc));
                if !inactive_occupied {
                    let inactive_loc_update = EntityUpdate::new(inactive_loc, Contents::Empty);
                    update_callback(inactive_loc_update);
                    update = true;
                }

                // Only update active_loc if no train is present
                let active_occupied = trains.iter().any(|train| train.at_location(active_loc));
                if !active_occupied {
                    let active_loc_update =
                        EntityUpdate::new(active_loc, Contents::SwitchIndicator(brightness));
                    update_callback(active_loc_update);
                    update = true;
                }
            }
        };

        handle_direction(
            self.anode_switched,
            self.anode_last_switched,
            self.anode_next_location,
            self.anode_fork_location,
            self.brightness,
        );
        handle_direction(
            self.cathode_switched,
            self.cathode_last_switched,
            self.cathode_next_location,
            self.cathode_fork_location,
            self.brightness,
        );

        update
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

    /// Returns the location a train at this switch will go in the given direction, or None if there is no switch in that direction.
    pub fn active_location(&self, direction: Direction) -> Option<Location> {
        match direction {
            Direction::Anode => {
                match self.anode_switched {
                    Some(switched) => {
                        if switched {
                            self.anode_fork_location
                        } else {
                            Some(self.anode_next_location)
                        }
                    }
                    None => None,
                }
            }
            Direction::Cathode => {
                match self.cathode_switched {
                    Some(switched) => {
                        if switched {
                            self.cathode_fork_location
                        } else {
                            Some(self.cathode_next_location)
                        }
                    }
                    None => None,
                }
            }
        }
    }

    /// Returns the location a train at this switch will go in the given direction if the switch is not switched.
    pub fn next_location(&self, direction: Direction) -> Location {
        match direction {
            Direction::Anode => self.anode_next_location,
            Direction::Cathode => self.cathode_next_location,
        }
    }

    /// Returns the location a train at this switch will go in the given direction if the switch is switched.
    pub fn fork_location(&self, direction: Direction) -> Option<Location> {
        match direction {
            Direction::Anode => self.anode_fork_location,
            Direction::Cathode => self.cathode_fork_location,
        }
    }
}
