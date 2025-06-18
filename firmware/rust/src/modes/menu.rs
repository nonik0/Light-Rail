use heapless::{Deque, Vec};
use random_trait::Random;

use super::NUM_MODES;
use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    modes::{GameMode, GameModeHandler},
    random::Rand,
    train::DEFAULT_SPEED,
    NUM_DIGITS,
};
use as1115::segments::*;

const IDLE_CYCLES: u16 = 400; // number of cycles before switching back to animation
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
    counter: u16,
    index: usize,
    snake_grow: bool,
    snake_counter: u8,
    snake_hunger: u8, // hunger counter that reduces length of snake
    snake_direction: bool,
    snake_segments: Deque<SnakeLocation, SNAKE_LENGTH>,
    snake_food: u8, // bitmask of digits that have food (decimal point)
}

impl MenuMode {
    fn shuffle<T, const N: usize>(vec: &mut Vec<T, N>) {
        let len = vec.len();
        for i in (1..len).rev() {
            let j = Rand::from_range(0, i as u8);
            vec.swap(i, j as usize);
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

    fn snake_eat(&mut self) {
        if let Some(snake_head) = self.snake_segments.front() {
            let head_digit = snake_head.digit;
            let food_mask = 1 << head_digit;

            match (snake_head.segment, self.snake_direction) {
                // Eats food on current digit if moving down from C or right from D
                (C, false) | (D, true) if (self.snake_food & food_mask) != 0 => {
                    self.snake_food &= !food_mask;
                    self.snake_grow = true; // grow snake
                }
                // Eats food on previous digit if moving left from D
                (D, false) if head_digit > 0 && (self.snake_food & (food_mask >> 1)) != 0 => {
                    self.snake_food &= !(food_mask >> 1);
                    self.snake_grow = true; // grow snake
                }
                _ => {}
            }
        }
    }

    fn snake_slither(&mut self) {
        let snake_head = self.snake_segments.front().unwrap();

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
        }

        // pop old tail and push new head
        let new_head = new_head.unwrap();
        // grow by not popping tail when slithering
        if !self.snake_grow || self.snake_segments.is_full() {
            self.snake_segments.pop_back();
        }
        self.snake_grow = false;
        self.snake_direction = new_direction;
        self.snake_segments.push_front(new_head).ok();
    }

    fn snake_segment_data(&self) -> [u8; NUM_DIGITS as usize] {
        let mut segment_data = [0; NUM_DIGITS as usize];
        for segment in self.snake_segments.iter() {
            segment_data[segment.digit as usize] |= segment.segment;
        }

        for digit in 0..NUM_DIGITS {
            if (self.snake_food & (1 << digit)) != 0 {
                segment_data[digit as usize] |= DP;
            }
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
        self.snake_food = 0;
        self.snake_segments.clear();

        // birth snake
        self.snake_direction = Rand::default().get_bool();
        let snake_segment = SnakeLocation::new(
            Rand::from_range(0, NUM_DIGITS - 1),
            1u8 << Rand::from_range(0, 6), // random segment A-G
        );
        for _ in 0..1 {
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
                self.snake_eat();

                // the longer the snake, the faster it gets hungry
                let snake_length = self.snake_segments.len() as u8;
                if snake_length > 1 {
                    self.snake_hunger += 1;

                    let hunger_threshold = 0xFF >> snake_length - 1;
                    if self.snake_hunger >= hunger_threshold {
                        self.snake_hunger = 0;
                        self.snake_segments.pop_back();
                    }
                }

                state.display = DisplayState::Segments(self.snake_segment_data())
            }

            // randomly spawn food if snake is not occupying the vicinity and the head is not a neighbor
            if self.snake_food < 0b111 && Rand::from_range(0, 100) == 0 {
                let food_digit = Rand::from_range(0, NUM_DIGITS - 1);
                let mut neighbors: Vec<SnakeLocation, MAX_NEXT_SEGMENTS> = Vec::new();
                neighbors.push(SnakeLocation::new(food_digit, C)).ok();
                neighbors.push(SnakeLocation::new(food_digit, D)).ok();
                if food_digit < NUM_DIGITS - 1 {
                    neighbors.push(SnakeLocation::new(food_digit + 1, D)).ok();
                }

                let occupied_neighbors = self
                    .snake_segments
                    .iter()
                    .filter(|s| {
                        neighbors
                            .iter()
                            .any(|n| n.digit == s.digit && n.segment == s.segment)
                    })
                    .count();
                let head_is_neighbor = self
                    .snake_segments
                    .front()
                    .map_or(false, |s| s.digit == food_digit);
                if occupied_neighbors < 2 && !head_is_neighbor {
                    self.snake_food |= 1 << food_digit;
                    state.display = DisplayState::Segments(self.snake_segment_data());
                }
            } // end food spawning
        }
        // end snake animation
        else {
            self.counter += 1;
            if self.counter > IDLE_CYCLES {
                self.counter = 0;
                self.index = 0;
            }
        }

        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo_out(Cargo::Full(LedPattern::Solid));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        self.counter = 0;
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
        if Rand::default().get_bool() {
            state.train_switch(train_index);
        }

        // Clear cargo if train front is at a platform with cargo
        let train = &state.trains[train_index];
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();
            }
        }
    }
}
