// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use core::num;

use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::{gamma, IS31FL3731};

// use embedded_hal::delay::DelayNs;
use random_trait::Random;

use crate::{
    common::*,
    input::{BoardInput, InputDirection, InputEvent},
    location::{Direction, Location, NUM_PLATFORMS, NUM_SWITCHES},
    modes::*,
    panic,
    platform::{self, Platform},
    switch::{self, Switch},
    tone::TimerTone,
    train::{Car, Train, DEFAULT_SPEED},
    Rand,
};

pub const MAX_CARS: usize = 50;
pub const MAX_TRAINS: usize = 3;
pub const NOMINAL_TRAIN_SIZE: usize = MAX_CARS / MAX_TRAINS;

#[derive(Clone, Copy, PartialEq)]
pub enum DisplayState {
    None,
    Score(u16),
    Text([u8; crate::NUM_DIGITS as usize]),
    //ScrollingText
}

// TOOD: rename to GameState?
pub struct GameState {
    pub target_mode_index: usize, // in state so menu mode can manipulate it
    pub is_over: bool,            // stops entity updates
    pub redraw: bool,             // flag to redraw board LEDs
    pub display: DisplayState,

    // game entities
    pub cars: [Car; MAX_CARS],
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
}

impl GameState {
    pub fn add_train(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8) {
        if self.trains.is_full() {
            return;
        }

        // TODO: for now, simple allocation method that divides evenly on MAX_TRAINS, only snake allocated single train with max cars
        let cars_ptr = unsafe {
            self.cars
                .as_mut_ptr()
                .add(self.trains.len() * NOMINAL_TRAIN_SIZE)
        };
        //let cars_ptr = unsafe { self.cars.as_mut_ptr() };
        let loc = self.rand_platform().track_location();
        let speed = Some(DEFAULT_SPEED);
        let mut train = Train::new(cars_ptr, max_cars, loc, cargo, speed);
        for _ in 1..num_cars {
            train.add_car(cargo);
        }
        self.trains.push(train).unwrap();
        self.redraw = true;
    }

    pub fn remove_train(&mut self) {
        if !self.trains.is_empty() {
            self.trains.pop();
        }
        self.redraw = true;
    }

    /// Initializes the game state with a single train with given parameters.
    pub fn init_trains(&mut self, cargo: Cargo, num_cars: u8, max_cars: u8) {
        // init first train
        if self.trains.len() > 0 {
            while self.trains.len() > 1 {
                self.trains.pop();
            }

            // reuse existing train for smooth transition between modes
            let train = &mut self.trains[0];
            train.init_cars(cargo, num_cars, max_cars);
            train.set_speed(DEFAULT_SPEED);
            self.redraw = true;
        } else {
            self.add_train(cargo, num_cars, max_cars);
        }
    }

    pub fn init_platforms(&mut self, cargo: Cargo) {
        for platform in self.platforms.iter_mut() {
            if !platform.is_empty() {
                platform.set_cargo(cargo);
            }
        }
    }

    pub fn clear_platforms(&mut self) {
        for platform in self.platforms.iter_mut() {
            platform.clear_cargo();
        }
    }

    pub fn rand_platform(&self) -> &Platform {
        let rand_platform_index = Rand::default().get_usize() % self.platforms.len();
        &self.platforms[rand_platform_index]
    }
}

pub struct Game<'a, I2C>
where
    I2C: I2c,
{
    // board components
    board_buzzer: TimerTone,
    board_digits: AS1115<I2C>,
    board_input: BoardInput,
    board_leds: IS31FL3731<I2C>,

    // game mode state
    active_mode_index: usize,
    last_display: DisplayState,
    last_over: bool,
    modes: &'a mut [&'a mut (dyn GameModeHandler + 'a)],

    // state passed to game modes, changes to state entities are rendered into updates for digits and LEDs
    state: GameState,
}

impl<'a, I2C> Game<'a, I2C>
where
    I2C: I2c,
{
    pub fn new(
        board_buzzer: TimerTone,
        board_digits: AS1115<I2C>,
        board_input: BoardInput,
        board_leds: IS31FL3731<I2C>,
        cars: [Car; MAX_CARS],
        modes: &'a mut [&'a mut dyn GameModeHandler],
    ) -> Self {
        let platforms = Platform::take();
        let switches = Switch::take();
        let trains = Vec::<Train, MAX_TRAINS>::new();

        let state = GameState {
            target_mode_index: 0,
            is_over: false,
            redraw: false,
            display: DisplayState::None,
            cars,
            trains,
            platforms,
            switches,
        };

        Self {
            board_buzzer,
            board_digits,
            board_input,
            board_leds,
            active_mode_index: 0,
            last_display: DisplayState::None,
            last_over: true,
            modes,
            state,
        }
    }

    fn mode(&self) -> &dyn GameModeHandler {
        self.modes[self.active_mode_index]
    }

    pub fn restart(&mut self) {
        // reset game state, on_restart should update state.display and entities will be updates by self.refresh_board_leds()
        let mode = &mut self.modes[self.active_mode_index];
        mode.on_restart(&mut self.state);
        self.redraw_board_leds();
        self.state.redraw = false;
    }

    pub fn redraw_board_leds(&mut self) {
        self.board_leds.clear_blocking().unwrap();

        // TODO: handle or reset phases for entities?

        for train in self.state.trains.iter() {
            for car in train.cars().iter() {
                self.board_leds
                    .pixel_blocking(car.loc.index(), gamma(car.cargo.car_brightness(0)))
                    .unwrap();
            }
        }
        for platform in self.state.platforms.iter() {
            self.board_leds
                .pixel_blocking(
                    platform.location().index(),
                    platform.cargo().platform_brightness(0),
                )
                .unwrap();
        }
        for switch in self.state.switches.iter() {
            if let Some(active_anode_location) = switch.active_location(Direction::Anode) {
                self.board_leds
                    .pixel_blocking(active_anode_location.index(), gamma(100))
                    .unwrap();
            }
            if let Some(active_cathode_location) = switch.active_location(Direction::Cathode) {
                self.board_leds
                    .pixel_blocking(active_cathode_location.index(), gamma(100))
                    .unwrap();
            }
        }
    }

    pub fn tick(&mut self) {
        // handle input events, some events are shared betweens all modes
        if let Some(event) = self.board_input.update() {
            match event {
                // toggle switches
                InputEvent::SwitchButtonPressed(index) => {
                    self.board_buzzer.tone(3000, 100);
                    let index = index as usize;
                    if index < self.state.switches.len() {
                        self.state.switches[index].switch();
                    }
                }
                // tones on button presses
                InputEvent::DirectionButtonPressed(InputDirection::Up)
                | InputEvent::DirectionButtonPressed(InputDirection::Right) => {
                    self.board_buzzer.tone(3500, 50);
                }
                InputEvent::DirectionButtonPressed(InputDirection::Down)
                | InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                    self.board_buzzer.tone(3000, 50);
                }
                // exit to menu mode
                InputEvent::DirectionButtonHeld(InputDirection::Left) => {
                    self.state.target_mode_index = 0;
                    self.active_mode_index = 0;
                    self.state.is_over = false;
                    self.restart();
                    //return;
                }
                _ => {}
            }

            let mode = &mut self.modes[self.active_mode_index];
            mode.on_input_event(event, &mut self.state);
        }

        // redraw board LEDs when current mode requests it
        if self.state.redraw {
            self.state.redraw = false;
            self.redraw_board_leds();
        }

        // change mode when current mode requests it (mostly from menu mode)
        if self.state.target_mode_index != self.active_mode_index {
            self.active_mode_index = self.state.target_mode_index;
            self.restart();
        }

        // update board digits/score display
        if self.last_display != self.state.display {
            self.last_display = self.state.display;
            match self.state.display {
                DisplayState::None => {
                    self.board_digits.clear().ok();
                }
                DisplayState::Score(score) => {
                    self.board_digits.display_number(score).ok();
                }
                DisplayState::Text(ref text) => {
                    self.board_digits.display_ascii(text).ok();
                }
            }
        }

        // skip updating game entities if game is over
        if self.state.is_over {
            return;
        }

        // helper closure to update entity LEDs
        let mut do_led_update = |location: Location, brightness: u8| {
            self.board_leds
                .pixel_blocking(location.index(), gamma(brightness))
                .ok();
        };

        // update train, platform, and switch entities
        let mode = &mut self.modes[self.active_mode_index];
        mode.on_game_tick(&mut self.state);

        let mut event_indices = heapless::Vec::<usize, MAX_TRAINS>::new();
        for (train_index, train) in self.state.trains.iter_mut().enumerate() {
            if train.advance(&self.state.switches, &mut do_led_update) {
                event_indices.push(train_index).ok();
            }
        }
        for &train_index in event_indices.iter() {
            mode.on_train_advance(train_index, &mut self.state);
        }

        for platform in self.state.platforms.iter_mut() {
            platform.update(&mut do_led_update);
        }
        for switch in self.state.switches.iter_mut() {
            switch.update(&self.state.trains, &mut do_led_update);
        }
    }
}
