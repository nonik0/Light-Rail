use random_trait::Random;

use crate::{
    cargo::*,
    game_settings::GameSettings,
    location::{Direction, Location, NUM_SWITCHES},
    random::Rand,
    train::Train,
};

pub struct Switch {
    location: Location,
    phase: u8, // phase of the switch, used for PWM

    // switches only have one active direction (one direction has None values)
    // crosses have two active directions

    // false directs to next_location, true directs to fork_location, none means no switch in that direction
    anode_switched: Option<bool>,
    //anode_last_switched: Option<bool>, // entity update tracking
    anode_next_location: Location,
    anode_fork_location: Option<Location>, // None means no switch in that direction
    anode_last_brightness: u8,             // last brightness for active location

    // false directs to next_location, true directs to fork_location, none means no switch in that direction
    cathode_switched: Option<bool>,
    //cathode_last_switched: Option<bool>, // entity update tracking
    cathode_next_location: Location,
    cathode_fork_location: Option<Location>, // None means no switch in that direction
    cathode_last_brightness: u8,             // last brightness for active location
}

impl Switch {
    fn new(location: Location) -> Self {
        // set up switch in a direction
        fn setup_direction(
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
            setup_direction(location, Direction::Anode);
        let (cathode_switched, cathode_next_location, cathode_fork_location) =
            setup_direction(location, Direction::Cathode);

        Self {
            location,
            //phase: Rand::default().get_u8(), // initial phase
            phase: 0,
            anode_switched,
            //anode_last_switched: None,
            anode_next_location,
            anode_fork_location,
            anode_last_brightness: 0,
            cathode_switched,
            //cathode_last_switched: None,
            cathode_next_location,
            cathode_fork_location,
            cathode_last_brightness: 0,
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

    pub fn update<F>(
        &mut self,
        settings: &GameSettings,
        trains: &[Train],
        mut update_callback: F,
        force_update: bool,
    ) -> bool
    where
        F: FnMut(Location, u8),
    {
        let mut update = false;
        self.phase = self.phase.wrapping_add(1);

        let mut handle_direction =
            |is_switched: Option<bool>,
             //last_switched: Option<bool>,
             last_brightness: &mut u8,
             next_location: Location,
             fork_location: Option<Location>| {
                if let Some(switched) = is_switched {
                    // // no update if no change
                    // if let Some(last_switched) = last_switched {
                    //     if switched == last_switched {
                    //         return;
                    //     }
                    // }

                    let (active_loc, inactive_loc) = if switched {
                        (fork_location.unwrap(), next_location)
                    } else {
                        (next_location, fork_location.unwrap())
                    };

                    // Only update inactive_loc if no train is present
                    let inactive_occupied =
                        trains.iter().any(|train| train.at_location(inactive_loc));
                    if !inactive_occupied {
                        update_callback(inactive_loc, 0);
                        update = true;
                    }

                    // Only update active_loc if no train is present
                    let active_occupied = trains.iter().any(|train| train.at_location(active_loc));
                    if !active_occupied {
                        let brightness = LedPattern::Fade.get_pwm(
                            self.phase,
                            settings.switch_brightness() >> 1,
                            settings.switch_brightness(),
                        );

                        if force_update || brightness != *last_brightness {
                            *last_brightness = brightness;
                            update_callback(active_loc, brightness);
                            update = true;
                        }
                    }
                }
            };

        handle_direction(
            self.anode_switched,
            //self.anode_last_switched,
            &mut self.anode_last_brightness,
            self.anode_next_location,
            self.anode_fork_location,
        );
        handle_direction(
            self.cathode_switched,
            //self.cathode_last_switched,
            &mut self.cathode_last_brightness,
            self.cathode_next_location,
            self.cathode_fork_location,
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
            Direction::Anode => match self.anode_switched {
                Some(switched) => {
                    if switched {
                        self.anode_fork_location
                    } else {
                        Some(self.anode_next_location)
                    }
                }
                None => None,
            },
            Direction::Cathode => match self.cathode_switched {
                Some(switched) => {
                    if switched {
                        self.cathode_fork_location
                    } else {
                        Some(self.cathode_next_location)
                    }
                }
                None => None,
            },
        }
    }

    // /// Returns the location a train at this switch will go in the given direction if the switch is not switched.
    // pub fn next_location(&self, direction: Direction) -> Location {
    //     match direction {
    //         Direction::Anode => self.anode_next_location,
    //         Direction::Cathode => self.cathode_next_location,
    //     }
    // }

    // /// Returns the location a train at this switch will go in the given direction if the switch is switched.
    // pub fn fork_location(&self, direction: Direction) -> Option<Location> {
    //     match direction {
    //         Direction::Anode => self.anode_fork_location,
    //         Direction::Cathode => self.cathode_fork_location,
    //     }
    // }
}
