use heapless::{Deque, Vec};
use random_trait::Random;

use super::NUM_MODES;
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
use as1115::segments::*;

const SNAKE_LENGTH: usize = 6; // number of segments in the snake
const SNAKE_PERIOD: u8 = 15; // number of ticks between snake movements
const MAX_NEXT_SEGMENTS: usize = 3; // max of 3 options when moving from one segment

#[derive(Clone, Copy, Default)]
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

    /// Returns a vector of the next possible locations to move from the current loc (digit and segment).
    fn next_segment_locations(
        location: &SnakeLocation,
        direction: bool,
    ) -> heapless::Vec<(SnakeLocation, bool), MAX_NEXT_SEGMENTS> {
        let mut next_locations: heapless::Vec<(SnakeLocation, bool), MAX_NEXT_SEGMENTS> =
            heapless::Vec::new();
        if direction {
            match location.segment {
                A => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, A), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, B), false))
                        .ok();
                }
                B => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, A), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, A), false))
                        .ok();
                }
                C => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, G), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, G), false))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(location.digit, B), true))
                        .ok();
                }
                D => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, D), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, C), true))
                        .ok();
                }
                E => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, G), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, F), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(location.digit, G), true))
                        .ok();
                }
                F => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, A), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, A), true))
                        .ok();
                }
                G => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, G), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, B), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(location.digit, C), false))
                        .ok();
                }
                _ => panic_with_error!(600),
            }
        } else {
            match location.segment {
                A => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, A), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, F), false))
                        .ok();
                }
                B => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, G), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, G), false))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(location.digit, C), false))
                        .ok();
                }
                C => {
                    if location.digit < NUM_DIGITS - 1 {
                        next_locations
                            .push((SnakeLocation::new(location.digit + 1, D), true))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, D), false))
                        .ok();
                }
                D => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, D), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, E), true))
                        .ok();
                }
                E => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, D), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, D), true))
                        .ok();
                }
                F => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, G), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, G), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(location.digit, E), false))
                        .ok();
                }
                G => {
                    if location.digit > 0 {
                        next_locations
                            .push((SnakeLocation::new(location.digit - 1, G), false))
                            .ok();
                    }
                    next_locations
                        .push((SnakeLocation::new(location.digit, F), true))
                        .ok();
                    next_locations
                        .push((SnakeLocation::new(location.digit, E), false))
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
            .any(|s| s.digit == loc.digit && s.segment == loc.segment) // TODO: s == loc
    }

    /// Checks if a movement from one segment to another is valid based on the location of the snake's segments
    /// Checks for (hopefully) all relevant cases where the snake would move into or "through itself".
    fn is_valid_movement(&self, from: &SnakeLocation, direction: bool, to: &SnakeLocation) -> bool {
        if self.is_occupied(to) {
            return false; // Can't move to an occupied location
        }

        // Helper to check if both locations are occupied
        let both_occupied =
            |a: &SnakeLocation, b: &SnakeLocation| self.is_occupied(a) && self.is_occupied(b);
        let either_occupied =
            |a: &SnakeLocation, b: &SnakeLocation| self.is_occupied(a) || self.is_occupied(b);
        let any_occupied = |a: &SnakeLocation, b: &SnakeLocation, c: &SnakeLocation| {
            self.is_occupied(a) || self.is_occupied(b) || self.is_occupied(c)
        };

        match (from.segment, direction, to.segment) {
            // up/right from B
            (B, true, A) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit - 1 => {
                !both_occupied(
                    &SnakeLocation::new(from.digit, A),
                    &SnakeLocation::new(to.digit, F),
                )
            }
            // down from B and up from C
            (B, false, C) | (C, true, B)
                if from.digit < NUM_DIGITS - 1 && from.digit == to.digit =>
            {
                !self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && any_occupied(
                        &SnakeLocation::new(from.digit + 1, F),
                        &SnakeLocation::new(from.digit + 1, G),
                        &SnakeLocation::new(from.digit + 1, E),
                    )
            }
            // down/right from B and  up/right from C
            (B, false, G) | (C, true, G)
                if from.digit < NUM_DIGITS - 1 && from.digit == to.digit - 1 =>
            {
                !self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && either_occupied(
                        &SnakeLocation::new(to.digit, F),
                        &SnakeLocation::new(to.digit, E),
                    )
            }
            // down/right from C
            (C, false, D) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit - 1 => {
                !both_occupied(
                    &SnakeLocation::new(from.digit, D),
                    &SnakeLocation::new(to.digit, E),
                )
            }
            // down/left from E
            (E, false, D) if from.digit > 0 && from.digit == to.digit + 1 => !both_occupied(
                &SnakeLocation::new(from.digit, D),
                &SnakeLocation::new(to.digit, C),
            ),
            // down from E
            (E, true, F) if from.digit > 0 && from.digit == to.digit - 1 => {
                !(both_occupied(
                    &SnakeLocation::new(from.digit, G),
                    &SnakeLocation::new(to.digit, B),
                ) || both_occupied(
                    &SnakeLocation::new(from.digit, C),
                    &SnakeLocation::new(to.digit, D),
                ))
            }
            // up from E and down from F
            (E, true, F) | (F, false, E) if from.digit > 0 && from.digit == to.digit => {
                !self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && any_occupied(
                        &SnakeLocation::new(from.digit - 1, B),
                        &SnakeLocation::new(from.digit - 1, G),
                        &SnakeLocation::new(from.digit - 1, C),
                    )
            }
            // left from E and F
            (E, true, G) | (F, false, G) if from.digit > 0 && from.digit == to.digit + 1 => {
                !self.is_occupied(&SnakeLocation::new(from.digit, G))
                    && either_occupied(
                        &SnakeLocation::new(to.digit, B),
                        &SnakeLocation::new(to.digit, C),
                    )
            }
            // up/left from F
            (F, true, A) if from.digit > 0 && from.digit == to.digit + 1 => !both_occupied(
                &SnakeLocation::new(from.digit, A),
                &SnakeLocation::new(to.digit, B),
            ),
            // left from G
            (G, false, G) if from.digit > 0 && from.digit == to.digit + 1 => {
                !(both_occupied(
                    &SnakeLocation::new(from.digit, E),
                    &SnakeLocation::new(from.digit, F),
                ) || both_occupied(
                    &SnakeLocation::new(to.digit, B),
                    &SnakeLocation::new(to.digit, C),
                ))
            }
            // right from G
            (G, true, G) if from.digit < NUM_DIGITS - 1 && from.digit == to.digit - 1 => {
                !(both_occupied(
                    &SnakeLocation::new(from.digit, B),
                    &SnakeLocation::new(from.digit, C),
                ) || both_occupied(
                    &SnakeLocation::new(to.digit, E),
                    &SnakeLocation::new(to.digit, F),
                ))
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

        let mut next_locations = Self::next_segment_locations(loc, dir);
        Self::shuffle(&mut next_locations);
        for (next_loc, next_dir) in next_locations.iter() {
            if self.is_valid_movement(loc, dir, next_loc) {
                if let Some(_) = self.find_path(next_loc, *next_dir, path_left - 1) {
                    return Some((next_loc.clone(), *next_dir));
                }
            }
        }
        None
    }

    fn snake_slither(&mut self) {
        let snake_head = self.snake_segments.front().unwrap();

        // if let Some((next_loc, next_dir)) = self.find_path(snake_head, self.snake_direction, 3) {
        //     // pop old tail and push new head
        //     self.snake_segments.pop_back().unwrap();
        //     self.snake_direction = next_dir;
        //     self.snake_segments.push_front(next_loc).unwrap();
        // }

        // look for a path to slither
        let mut new_head = None;
        let mut new_direction = self.snake_direction;

        // search for a path of at least half the snake length at first, then decreasing length until one is found
        for path_length in (1..=(SNAKE_LENGTH as u8 / 2)).rev() {
            if let Some((next_loc, next_dir)) =
                self.find_path(snake_head, self.snake_direction, path_length)
            {
                new_head = Some(next_loc);
                new_direction = next_dir;
                break;
            }
        }

        // if still no path found, return and be stuck (does it happen?)
        if new_head.is_none() {
            return;
            // let next_locs = Self::next_segment_locations(
            //     self.snake_segments.front().unwrap(),
            //     self.snake_direction,
            // );
            // let (next_loc, next_direction) =
            //     next_locs[Rand::default().get_u8() as usize % next_locs.len()];
            // new_head = Some(next_loc);
            // new_direction = next_direction;
        }

        // pop old tail and push new head
        let new_head = new_head.unwrap();
        self.snake_segments.pop_back().unwrap();
        self.snake_direction = new_direction;
        self.snake_segments.push_front(new_head).ok();
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

        state.init_trains(Cargo::Full(LedPattern::Solid), 5, 5);
        state.add_train(
            Cargo::Full(LedPattern::Solid),
            3,
            5,
            Some(DEFAULT_SPEED + 5),
        );
        state.init_platforms(Cargo::Full(LedPattern::Solid));

        // kill snake
        self.snake_counter = 0;
        self.snake_segments.clear();

        // birth snake
        self.snake_direction = Rand::default().get_bool();
        let snake_segment = SnakeLocation::new(
            Rand::default().get_u8() % NUM_DIGITS,
            1u8 << (Rand::default().get_u8() % 7),
        );
        for _ in 0..SNAKE_LENGTH {
            self.snake_segments.push_back(snake_segment.clone()).ok();
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
                platform.set_cargo_out(Cargo::Full(LedPattern::Solid));
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
