// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::IS31FL3731;

// use embedded_hal::delay::DelayNs;
use random_trait::Random;

use crate::{
    common::*,
    input::{BoardInput, InputDirection, InputEvent},
    location::{Direction, Location, NUM_PLATFORMS, NUM_SWITCHES},
    modes::*,
    platform::Platform,
    switch::Switch,
    tone::TimerTone,
    train::Train,
    Rand,
};

const MAX_TRAINS: usize = 1;

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
    pub display: DisplayState,

    // game entities
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
}

impl GameState {
    fn clear_platforms(&mut self) {
        for platform in self.platforms.iter_mut() {
            platform.clear_cargo();
        }
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
        modes: &'a mut [&'a mut dyn GameModeHandler],
    ) -> Self {
        let state = GameState {
            target_mode_index: 0,
            is_over: false,
            display: DisplayState::None,
            trains: Vec::<Train, MAX_TRAINS>::new(),
            platforms: Platform::take(),
            switches: Switch::take(),
        };

        Self {
            board_buzzer,
            board_digits,
            board_input,
            board_leds,
            active_mode_index: 0,
            modes,
            state,
        }
    }

    fn mode(&self) -> &dyn GameModeHandler {
        self.modes[self.active_mode_index]
    }

    pub fn restart(&mut self) {
        let mode = &mut self.modes[self.active_mode_index];
        mode.on_restart(&mut self.state);
    }

    pub fn tick(&mut self) {
        if let Some(event) = self.board_input.update() {
            self.board_buzzer.tone(2000, 20);
            match event {
                // all modes share switch toggle
                InputEvent::SwitchButtonPressed(index) => {
                    let index = index as usize;
                    if index < self.state.switches.len() {
                        self.state.switches[index].switch();
                    }
                }
                // shared exit to menu
                InputEvent::DirectionButtonHeld(InputDirection::Left) => {
                    self.state.target_mode_index = 0;
                    self.active_mode_index = 0;
                    self.state.is_over = false;
                    self.restart();
                }
                _ => {}
            }

            let mode = &mut self.modes[self.active_mode_index];
            mode.on_input_event(event, &mut self.state);
        }

        if self.state.target_mode_index != self.active_mode_index {
            self.active_mode_index = self.state.target_mode_index;
            self.restart();
        }

        match self.state.display {
            DisplayState::None => { self.board_digits.clear().ok(); }
            DisplayState::Score(score) => { self.board_digits.display_number(score).ok(); }
            DisplayState::Text(ref text) => { self.board_digits.display_ascii(text).ok(); }
        }

        if self.state.is_over {
            return;
        }

        let mut do_entity_update = |update: EntityUpdate| {
            self.board_leds
                .pixel_blocking(update.location.index(), update.contents.to_pwm_value())
                .ok();
        };
        let mode = &mut self.modes[self.active_mode_index];
        
        mode.on_game_tick(&mut self.state);

        let mut event_indices = heapless::Vec::<usize, MAX_TRAINS>::new();
        for (train_index, train) in self.state.trains.iter_mut().enumerate() {
            if train.advance(&self.state.switches, &mut do_entity_update) {
                event_indices.push(train_index).ok();
            }
        }
        for &train_index in event_indices.iter() {
            mode.on_train_advance(train_index, &mut self.state);
        }

        for platform in self.state.platforms.iter_mut() {
            platform.update(&mut do_entity_update);
        }
        for switch in self.state.switches.iter_mut() {
            switch.update(&self.state.trains, &mut do_entity_update);
        }
    }
}
