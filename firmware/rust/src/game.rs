use as1115::AS1115;
use embedded_hal::i2c::I2c;
use heapless::Vec;
use is31fl3731::{gamma, IS31FL3731};

use crate::{
    game_state::*,
    input::{BoardInput, InputDirection, InputEvent},
    location::Location,
    modes::*,
    platform::Platform,
    switch::Switch,
    tone::TimerTone,
    train::{Car, Train},
};

pub struct Game<I2C>
where
    I2C: I2c,
{
    // board components
    board_buzzer: TimerTone,
    board_digits: AS1115<I2C>,
    board_input: BoardInput,
    board_leds: IS31FL3731<I2C>,

    // game mode state
    mode_index: usize,
    mode: GameMode,
    last_display: DisplayState,
    buzzer_enabled: bool,

    // state passed to game modes, changes to state entities are rendered into updates for digits and LEDs
    state: GameState,
}

impl<I2C> Game<I2C>
where
    I2C: I2c,
{
    pub fn new(
        board_buzzer: TimerTone,
        board_digits: AS1115<I2C>,
        board_input: BoardInput,
        board_leds: IS31FL3731<I2C>,
        cars: [Car; MAX_CARS],
        settings: GameSettings,
    ) -> Self {
        let platforms = Platform::take();
        let switches = Switch::take();
        let trains = Vec::<Train, MAX_TRAINS>::new();

        let state = GameState {
            target_mode_index: 0,
            is_over: false,
            redraw: false,
            display: DisplayState::None,
            settings,
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
            mode_index: 0,
            mode: GameMode::default(),
            last_display: DisplayState::None,
            buzzer_enabled: true,
            state,
        }
    }

    pub fn restart(&mut self) {
        self.mode = GameMode::from_index(self.mode_index);
        self.mode.on_restart(&mut self.state);
        self.state.redraw = true;
    }

    pub fn tick(&mut self) {
        // handle input events, some events are shared betweens all modes
        if let Some(event) = self.board_input.update() {
            match event {
                // toggle switches
                InputEvent::SwitchButtonPressed(index) => {
                    if self.buzzer_enabled {
                        self.board_buzzer.tone(3000, 15);
                    }
                    let index = index as usize;
                    if index < self.state.switches.len() {
                        self.state.switches[index].switch();
                    }
                }
                // tones on button presses
                InputEvent::DirectionButtonPressed(InputDirection::Up)
                | InputEvent::DirectionButtonPressed(InputDirection::Right) => {
                    if self.buzzer_enabled {
                        self.board_buzzer.tone(3500, 10);
                    }
                }
                InputEvent::DirectionButtonPressed(InputDirection::Down)
                | InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                    if self.buzzer_enabled {
                        self.board_buzzer.tone(3000, 10);
                    }
                }
                // enable/disable tones on buttons
                InputEvent::DirectionButtonHeld(InputDirection::Up) => {
                    self.buzzer_enabled = true;
                }
                InputEvent::DirectionButtonHeld(InputDirection::Down) => {
                    self.buzzer_enabled = false;
                }
                // exit to menu mode
                InputEvent::DirectionButtonHeld(InputDirection::Left) => {
                    if self.buzzer_enabled {
                        self.board_buzzer.tone(3000, 10);
                    }
                    self.state.target_mode_index = 0;
                    self.mode_index = 0;
                    self.state.is_over = false;
                    self.restart();
                    //return;
                }
                _ => {}
            }

            self.mode.on_input_event(event, &mut self.state);
        }

        // change mode when current mode requests it (mostly from menu mode)
        if self.state.target_mode_index != self.mode_index {
            self.mode_index = self.state.target_mode_index;
            self.restart();
        }

        self.mode.on_game_tick(&mut self.state);

        // update board digits/score display
        if self.last_display != self.state.display {
            self.last_display = self.state.display;

            // TODO: flag or something to avoid calling unnecessarily
            self.board_digits
                .set_intensity(self.state.settings.digit_brightness_level())
                .ok();

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

        // clear board LEDs and force update all entities when requested
        if self.state.redraw {
            self.board_leds.clear_blocking().ok();
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
        let mut event_indices = heapless::Vec::<usize, MAX_TRAINS>::new();
        for (train_index, train) in self.state.trains.iter_mut().enumerate() {
            if train.advance(&self.state.settings, &self.state.switches, &mut do_led_update, self.state.redraw) {
                event_indices.push(train_index).ok();
            }
        }
        for &train_index in event_indices.iter() {
            self.mode.on_train_advance(train_index, &mut self.state);
        }

        for platform in self.state.platforms.iter_mut() {
            platform.update(&self.state.settings, &mut do_led_update, self.state.redraw);
        }
        for switch in self.state.switches.iter_mut() {
            switch.update(&self.state.settings, &self.state.trains, &mut do_led_update, self.state.redraw);
        }

        self.state.redraw = false;
    }
}
