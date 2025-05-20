// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::IS31FL3731;
use static_cell::make_static;

// use embedded_hal::delay::DelayNs;
use random_trait::Random;

use crate::{
    common::*, input::{BoardInput, InputDirection, InputEvent}, location::{Direction, Location, NUM_PLATFORMS, NUM_SWITCHES}, modes::*, panic::trace, platform::Platform, switch::Switch, tone::TimerTone, train::Train, Rand
};

const MAX_TRAINS: usize = 5;
const MAX_LOC_UPDATES: usize = crate::train::MAX_UPDATES * MAX_TRAINS + NUM_PLATFORMS;

pub struct GameEntities {
    pub trains: Vec<Train, MAX_TRAINS>,
    pub platforms: [Platform; NUM_PLATFORMS],
    pub switches: [Switch; NUM_SWITCHES],
}

pub struct Game<I2C>
where
    I2C: I2c + 'static,
{
    // board components
    board_buzzer: TimerTone,
    board_digits: AS1115<I2C>,
    board_input: BoardInput,
    board_leds: IS31FL3731<I2C>,

    // game mode state
    active_mode_index: usize,
    modes: [&'static mut dyn GameModeHandler; 2], // TODO refine?
    is_over: bool,
    score: u16,

    // game entities, are state passed to game modes and create update events rendered by game
    entities: GameEntities,
}

impl<I2C> Game<I2C>
where
    I2C: I2c + 'static,
{
    pub fn new(
        board_buzzer: TimerTone,
        board_digits: AS1115<I2C>,
        board_input: BoardInput,
        board_leds: IS31FL3731<I2C>,
    ) -> Self {
        let menu_mode = make_static!(MenuMode::default());
        let snake_mode = make_static!(SnakeMode::default());

        let entities = GameEntities {
            trains: Vec::<Train, MAX_TRAINS>::new(),
            platforms: Platform::take(),
            switches: Switch::take(),
        };

        let mut self_ = Self {
            board_buzzer,
            board_digits,
            board_input,
            board_leds,
            active_mode_index: 0,
            modes: [menu_mode, snake_mode],
            is_over: false,
            score: 0,
            entities,
        };

        self_.restart();
        self_
    }

    fn mode(&self) -> &dyn GameModeHandler {
        self.modes[self.active_mode_index]
    }

    fn restart(&mut self) {
        self.is_over = false;
        self.board_digits.clear().ok();
        self.board_leds.clear_blocking().unwrap();
        self.entities.trains.clear();

        for _ in 0..self.mode().num_trains() {
            let rand_platform_index = Rand::default().get_usize() % self.entities.platforms.len();
            let rand_platform = &self.entities.platforms[rand_platform_index];
            let rand_speed = 5 + Rand::default().get_u8() % 10;
            let mut train = Train::new(rand_platform.track_location(), Cargo::Full, Some(rand_speed));
            let num_cars = 1 + Rand::default().get_usize() % 3;
            for _ in 0..num_cars {
                train.add_car(Cargo::Full);
            }
            self.entities.trains.push(train).unwrap();
        }
    }

    pub fn tick(&mut self) {
        let mode = &self.modes[self.active_mode_index];

        if let Some(event) = self.board_input.update() {
            // shared events for all game modes
            match event {
                // toggle switches (TODO: maybe specialized for specific modes?)
                InputEvent::SwitchButtonPressed(index) => {
                    self.board_buzzer.tone(4000, 100);
                    let index = index as usize;
                    if index < self.entities.switches.len() {
                        self.entities.switches[index].switch();
                    }
                },
                // go to menu
                InputEvent::DirectionButtonHeld(InputDirection::Left) => {
                    // TODO: to menu
                },
                _ => {}
            }

            // mode specific events
            mode.on_input_event(event, &mut self.entities);
        }

        mode.on_game_tick(&mut self.entities);

        let mut updates = Vec::<EntityUpdate, MAX_LOC_UPDATES>::new();
        self.advance_trains(&mut updates);
        self.render_updates(&mut updates);
    }

    fn advance_trains(&mut self, updates: &mut Vec<EntityUpdate, MAX_LOC_UPDATES>) {
        let mut event_indices = heapless::Vec::<usize, MAX_TRAINS>::new();
        for (train_index, train) in self.entities.trains.iter_mut().enumerate() {
            if let Some(u) = train.advance(&self.entities.switches) {
                event_indices.push(train_index).ok();
                updates.extend(u.into_iter());
            }
        }

        let mode = &self.modes[self.active_mode_index];
        for &train_index in event_indices.iter() {
            mode.on_train_event(train_index, &mut self.entities);
        }
    }

    fn render_updates(&mut self, updates: &mut Vec<EntityUpdate, MAX_LOC_UPDATES>) {
        for platform in self.entities.platforms.iter_mut() {
            if let Some(u) = platform.get_update() {
                updates.push(u).ok();
            }
        }

        for switch in self.entities.switches.iter_mut() {
            if let Some(u) = switch.get_updates(&self.entities.trains) {
                updates.extend(u.into_iter());
            }
        }

        for update in updates {
            self.board_leds
                .pixel_blocking(update.location.index(), update.contents.to_pwm_value())
                .ok();
        }
    }
}

