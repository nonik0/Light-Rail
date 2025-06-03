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

pub const MAX_TRAINS: usize = 3;

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
    pub redraw: bool, // flag to redraw board LEDs
    pub display: DisplayState,

    // game entities
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
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
        modes: &'a mut [&'a mut dyn GameModeHandler],
    ) -> Self {
        let state = GameState {
            target_mode_index: 0,
            is_over: false,
            redraw: false,
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

        for train in self.state.trains.iter() {
            for car in train.cars().iter() {
                self.board_leds
                    .pixel_blocking(
                        car.loc.index(),
                        Contents::Train(car.cargo).to_pwm_value(),
                    )
                    .unwrap();
            }
        }
        for platform in self.state.platforms.iter() {
            self.board_leds
                .pixel_blocking(
                    platform.location().index(),
                    Contents::Platform(platform.cargo()).to_pwm_value(),
                )
                .unwrap();
        }
        for switch in self.state.switches.iter() {
            if let Some(active_anode_location) = switch.active_location(Direction::Anode) {
                self.board_leds
                    .pixel_blocking(
                        active_anode_location.index(),
                        Contents::SwitchIndicator(100).to_pwm_value(),
                    )
                    .unwrap();
            }
            if let Some(active_cathode_location) =
                switch.active_location(Direction::Cathode)
            {
                self.board_leds
                    .pixel_blocking(
                        active_cathode_location.index(),
                        Contents::SwitchIndicator(100).to_pwm_value(),
                    )
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
                    self.board_buzzer.tone(3500, 100);
                }
                InputEvent::DirectionButtonPressed(InputDirection::Down)
                | InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                    self.board_buzzer.tone(3000, 100);
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
        if self.state.redraw{
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
        let mut do_entity_update = |update: EntityUpdate| {
            self.board_leds
                .pixel_blocking(update.location.index(), update.contents.to_pwm_value())
                .ok();
        };

        // update train, platform, and switch entities
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
