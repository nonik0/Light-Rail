use core::panic;

use heapless::Deque;
use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    location::Direction,
    modes::{GameMode, GameModeHandler},
    random::Rand,
    train::DEFAULT_SPEED,
    NUM_DIGITS,
};

use super::NUM_MODES;
use as1115::segments::*;

const SNAKE_LENGTH: usize = 5;
const SNAKE_PERIOD: u8 = 30;

#[derive(Clone, Copy, Debug, Default)]
struct SnakeLocation {
    digit: u8,
    segment: u8,
}

impl SnakeLocation {
    fn new(digit: u8, segment: u8) -> Self {
        Self { digit, segment }
    }
}

#[derive(Default)]
pub struct MenuMode {
    index: usize,
    snake_counter: u8,
    snake_direction: bool,
    snake_segments: Deque<SnakeLocation, SNAKE_LENGTH>,
}

impl MenuMode {
    // direction is true if up or right, false if down or left
    fn candidate_snake_locations(
        &self,
        loc: &SnakeLocation,
        direction: bool,
    ) -> heapless::Vec<(SnakeLocation, bool), 3> {
        let mut next_locations: heapless::Vec<(SnakeLocation, bool), 3> = heapless::Vec::new();
        if direction {
            match loc.segment {
                A => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, A), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, B), false))
                        .ok();
                }
                B => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, A), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, A), false))
                        .ok();
                }
                C => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, G), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, G), false))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(loc.digit, B), true))
                        .ok();
                }
                D => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, D), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, C), true))
                        .ok();
                }
                E => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, G), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, F), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(loc.digit, G), true))
                        .ok();
                }
                F => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, A), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, A), true))
                        .ok();
                }
                G => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, G), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, B), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(loc.digit, C), false))
                        .ok();
                }
                _ => panic_with_error!(600),
            }
        } else {
            match loc.segment {
                A => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, A), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, F), false))
                        .ok();
                }
                B => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, G), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, G), false))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(loc.digit, C), false))
                        .ok();
                }
                C => {
                    if loc.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit + 1, D), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, D), false))
                        .ok();
                }
                D => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, D), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, E), true))
                        .ok();
                }
                E => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, D), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, D), true))
                        .ok();
                }
                F => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, G), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, G), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(loc.digit, E), false))
                        .ok();
                }
                G => {
                    if loc.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(loc.digit - 1, G), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(loc.digit, F), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(loc.digit, E), false))
                        .ok();
                }
                _ => panic_with_error!(600),
            }
        }
        next_locations
    }

    fn is_occupied(&self, loc: &SnakeLocation) -> bool {
        self.snake_segments
            .iter()
            .any(|s| s.digit == loc.digit && s.segment == loc.segment)
    }

    fn is_valid_movement(&self, from: &SnakeLocation, direction: bool, to: &SnakeLocation) -> bool {
        if self.is_occupied(to) {
            return false; // Can't move to an occupied location
        }

        // Helper to check if both locations are occupied
        let both_occupied =
            |a: SnakeLocation, b: SnakeLocation| self.is_occupied(&a) && self.is_occupied(&b);

        match (from.segment, direction, to.segment) {
            // G segment: can't move right through itself
            (G, true, G) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit - 1 => {
                !(both_occupied(
                    SnakeLocation::new(from.digit, B),
                    SnakeLocation::new(from.digit, C),
                ) || both_occupied(
                    SnakeLocation::new(to.digit, E),
                    SnakeLocation::new(to.digit, F),
                ))
            }
            // G segment: can't move left through itself
            (G, false, G) if from.digit > 0 && from.digit == to.digit + 1 => {
                !(both_occupied(
                    SnakeLocation::new(from.digit, E),
                    SnakeLocation::new(from.digit, F),
                ) || both_occupied(
                    SnakeLocation::new(to.digit, B),
                    SnakeLocation::new(to.digit, C),
                ))
            }
            // Down from B: can't move down through itself
            (B, false, C) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit => {
                !both_occupied(
                    SnakeLocation::new(from.digit, G),
                    SnakeLocation::new(from.digit + 1, G),
                )
            }
            // Down from F: can't move down through itself
            (F, false, E) if from.digit > 0 && from.digit == to.digit => !both_occupied(
                SnakeLocation::new(from.digit, G),
                SnakeLocation::new(from.digit - 1, G),
            ),
            // Up from C: can't move up through itself
            (C, true, B) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit => {
                !both_occupied(
                    SnakeLocation::new(from.digit, G),
                    SnakeLocation::new(from.digit + 1, G),
                )
            }
            // Up from E: can't move up through itself
            (E, true, F) if from.digit > 0 && from.digit == to.digit => !both_occupied(
                SnakeLocation::new(from.digit, G),
                SnakeLocation::new(from.digit - 1, G),
            ),
            _ => true,
        }
    }

    fn snake_slither(&mut self) {
        let snake_head = self.snake_segments.front().unwrap();
        let next_locations = self.candidate_snake_locations(snake_head, self.snake_direction);

        // Helper to filter valid movements
        let filter_valid = |locations: &heapless::Vec<(SnakeLocation, bool), 3>| {
            let mut out: heapless::Vec<(SnakeLocation, bool), 3> = heapless::Vec::new();
            for (loc, dir) in locations.iter() {
                if self.is_valid_movement(snake_head, self.snake_direction, loc) {
                    out.push((loc.clone(), *dir)).ok();
                }
            }
            out
        };

        let valid_next_locations = filter_valid(&next_locations);

        // For each valid next location, count how many of its onward moves are occupied
        let mut best_locations: heapless::Vec<(SnakeLocation, bool), 3> = heapless::Vec::new();
        let mut min_occupied = u8::MAX;

        for (loc, dir) in valid_next_locations.iter() {
            let next_next_locs = self.candidate_snake_locations(loc, *dir);
            let mut occupied_count = 0u8;
            for (onward_loc, _) in next_next_locs.iter() {
                if self.is_occupied(onward_loc) {
                    occupied_count += 1;
                }
            }
            if occupied_count < min_occupied {
                min_occupied = occupied_count;
                best_locations.clear();
                best_locations.push((loc.clone(), *dir)).ok();
            } else if occupied_count == min_occupied {
                best_locations.push((loc.clone(), *dir)).ok();
            }
        }

        let next_location_options = if !best_locations.is_empty() {
            &best_locations
        } else if !valid_next_locations.is_empty() {
            &valid_next_locations
        } else {
            &next_locations
        };

        let index = Rand::default().get_u8() as usize % next_location_options.len();
        let (new_location, new_direction) = next_location_options[index];
        let new_head = SnakeLocation::new(new_location.digit, new_location.segment);

        self.snake_direction = new_direction;
        self.snake_segments.pop_back().unwrap();
        self.snake_segments.push_front(new_head).unwrap();
    }

    fn snake_segment_data(&self) -> [u8; NUM_DIGITS as usize] {
        let mut segment_data = [0; NUM_DIGITS as usize];
        for segment in self.snake_segments.iter() {
            segment_data[segment.digit as usize] |= segment.segment;
        }
        segment_data
    }

    fn next_game_mode(&mut self, inc: bool) -> [u8; NUM_DIGITS as usize] {
        let delta = if inc { 1 } else { NUM_MODES - 1 };
        self.index = (self.index + delta) % NUM_MODES;
        if self.index == 0 {
            self.index = if inc { 1 } else { NUM_MODES - 1 };
        }
        GameMode::mode_name(self.index)
    }
}

impl GameModeHandler for MenuMode {
    fn on_restart(&mut self, state: &mut GameState) {
        state.is_over = false;

        state.init_trains(Cargo::Have(LedPattern::SolidBright), 3, 5);
        state.add_train(
            Cargo::Have(LedPattern::SolidBright),
            5,
            5,
            Some(DEFAULT_SPEED / 2),
        );
        state.init_platforms(Cargo::Have(LedPattern::SolidBright));

        // push snake head onto segments
        self.snake_counter = 0;
        self.snake_direction = Rand::default().get_bool();
        self.snake_segments.clear();

        let snake_segment = SnakeLocation::new(
            Rand::default().get_u8() % NUM_DIGITS,
            1u8 << (Rand::default().get_u8() % 7),
        );

        // push snake body segments
        for _ in 0..SNAKE_LENGTH {
            self.snake_segments
                .push_back(snake_segment.clone())
                .unwrap();
            self.snake_slither();
        }

        state.display = if self.index == 0 {
            DisplayState::Segments(self.snake_segment_data())
        } else {
            DisplayState::Text(GameMode::mode_name(self.index))
        };
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        // snake animation only on menu index 0
        if self.index == 0 {
            self.snake_counter = (self.snake_counter + 1) % SNAKE_PERIOD;
            if self.snake_counter == 0 {
                self.snake_slither();
                state.display = DisplayState::Segments(self.snake_segment_data())
            }
        }

        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo(Cargo::Have(LedPattern::SolidBright));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        match event {
            InputEvent::DirectionButtonPressed(direction) => match direction {
                InputDirection::Up => {
                    state.display = DisplayState::Text(self.next_game_mode(false));
                }
                InputDirection::Down => {
                    state.display = DisplayState::Text(self.next_game_mode(true));
                }
                InputDirection::Right => {
                    state.target_mode_index = self.index; // no-op if index is 0, so user needs to press up/down to select a game mode
                }
                _ => {}
            },
            _ => {}
        }
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &state.trains[train_index];
        let caboose_loc = train.caboose().loc;
        let last_loc = train.last_loc();

        // If train just left a switch, randomly switch it
        for switch in state.switches.iter_mut() {
            if caboose_loc == switch.location() {
                continue; // Train is entering, not leaving
            }
            for dir in [Direction::Anode, Direction::Cathode] {
                if switch.active_location(dir) == Some(last_loc) && Rand::default().get_bool() {
                    switch.switch();
                    break;
                }
            }
        }

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();
            }
        }
    }
}
