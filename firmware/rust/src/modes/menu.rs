use heapless::{Deque, Vec};
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
const SNAKE_PERIOD: u8 = 100;

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
    fn shuffle<T, const N: usize>(vec: &mut Vec<T, N>) {
        let len = vec.len();
        for i in (1..len).rev() {
            let j = Rand::default().get_usize() % (i + 1);
            vec.swap(i, j);
        }
    }

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

    /// Checks if a movement from `from` to `to` is valid based on the current snake segments, checks for both
    /// occupied locations and also ensures that the snake does not move through itself in invalid ways.
    fn is_valid_movement(&self, from: &SnakeLocation, direction: bool, to: &SnakeLocation) -> bool {
        if self.is_occupied(to) {
            return false; // Can't move to an occupied location
        }

        // Helper to check if both locations are occupied
        let both_occupied =
            |a: SnakeLocation, b: SnakeLocation| self.is_occupied(&a) && self.is_occupied(&b);
        let any_occupied = |a: SnakeLocation, b: SnakeLocation, c: SnakeLocation| {
            self.is_occupied(&a) || self.is_occupied(&b) || self.is_occupied(&c)
        };

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
                self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && any_occupied(
                        SnakeLocation::new(from.digit + 1, F),
                        SnakeLocation::new(from.digit + 1, G),
                        SnakeLocation::new(from.digit + 1, E),
                    )
            }
            // Down from F: can't move down through itself
            (F, false, E) if from.digit > 0 && from.digit == to.digit => {
                self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && any_occupied(
                        SnakeLocation::new(from.digit - 1, B),
                        SnakeLocation::new(from.digit - 1, G),
                        SnakeLocation::new(from.digit - 1, C),
                    )
            }
            // Up from C: can't move up through itself
            (C, true, B) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit => {
                self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && any_occupied(
                        SnakeLocation::new(from.digit + 1, F),
                        SnakeLocation::new(from.digit + 1, G),
                        SnakeLocation::new(from.digit + 1, E),
                    )
            }
            // Up from E: can't move up through itself
            (E, true, F) if from.digit > 0 && from.digit == to.digit => {
                self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && any_occupied(
                        SnakeLocation::new(from.digit - 1, B),
                        SnakeLocation::new(from.digit - 1, G),
                        SnakeLocation::new(from.digit - 1, C),
                    )
            }
            _ => true,
        }
    }

    // Finds a path from loc of length path_left, and returns the first location and new direction in that path.
    fn find_path(
        &self,
        loc: &SnakeLocation,
        dir: bool,
        path_left: u8,
    ) -> Option<(SnakeLocation, bool)> {
        if path_left == 0 {
            return Some((loc.clone(), false));
        }

        let mut next_locations = self.candidate_snake_locations(loc, dir);
        Self::shuffle(&mut next_locations);
        for (next_loc, next_dir) in next_locations.iter() {
            if self.is_valid_movement(loc, dir, next_loc) {
                if let Some(_) = self.find_path(next_loc, *next_dir, path_left - 1) {
                    return Some((next_loc.clone(), next_dir));
                }
            }
        }
        None
    }

    fn snake_slither(&mut self) {
        let snake_head = self.snake_segments.front().unwrap();

        // look for a path to slither
        let mut new_head = None;
        let mut new_direction = self.snake_direction;

        // search for a path of length 3, 2, or 1
        for path_length in (1..=3).rev() {
            if let Some((next_loc, next_dir)) =
                self.find_path(snake_head, self.snake_direction, path_length)
            {
                new_head = Some(next_loc);
                new_direction = next_dir;
                break;
            }
        }

        // if still no path found, just move to random candidate location
        if new_head.is_none() {
            let next_locs = self.candidate_snake_locations(
                self.snake_segments.front().unwrap(),
                self.snake_direction,
            );
            let (next_loc, next_direction) =
                next_locs[Rand::default().get_u8() as usize % next_locs.len()];
            new_head = Some(next_loc);
            new_direction = next_direction;
        }

        // pop old tail and push new head
        let new_head = new_head.unwrap();
        self.snake_segments.pop_back().unwrap();
        self.snake_direction = new_direction;
        self.snake_segments
            .push_front(SnakeLocation::new(new_head.digit, new_head.segment))
            .unwrap();
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
