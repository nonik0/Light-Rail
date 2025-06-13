use random_trait::Random;

use crate::{
    cargo::*,
    game_state::*,
    input::{InputDirection, InputEvent},
    modes::GameModeHandler,
    random::Rand,
    NUM_DIGITS,
};

#[derive(Clone, Copy, PartialEq)]
enum Setting {
    DigitBrightness,
    TrainBrightness,
    PlatformBrightness,
    SwitchBrightness,
}

pub struct SettingsMode {
    cur_setting: Setting,
}

impl SettingsMode {
    fn setting_display(&self, settings: &GameSettings) -> DisplayState {
        match self.cur_setting {
            Setting::DigitBrightness => {
                let mut text = [b' '; NUM_DIGITS as usize];
                text[0] = b'D';
                text[1] = b'B';
                text[2] = b'0' + settings.digit_brightness_level();
                DisplayState::Text(text)
            }
            Setting::TrainBrightness => {
                let mut text = [b' '; NUM_DIGITS as usize];
                text[0] = b'T';
                text[1] = b'B';
                text[2] = b'0' + settings.car_brightness_level();
                DisplayState::Text(text)
            }
            Setting::PlatformBrightness => {
                let mut text = [b' '; NUM_DIGITS as usize];
                text[0] = b'P';
                text[1] = b'B';
                text[2] = b'0' + settings.platform_brightness_level();
                DisplayState::Text(text)
            }
            Setting::SwitchBrightness => {
                let mut text = [b' '; NUM_DIGITS as usize];
                text[0] = b'Y';
                text[1] = b'B';
                text[2] = b'0' + settings.switch_brightness_level();
                DisplayState::Text(text)
            }
        }
    }

    fn next_setting(&mut self) {
        self.cur_setting = match self.cur_setting {
            Setting::DigitBrightness => Setting::TrainBrightness,
            Setting::TrainBrightness => Setting::PlatformBrightness,
            Setting::PlatformBrightness => Setting::SwitchBrightness,
            Setting::SwitchBrightness => Setting::DigitBrightness,
        };
    }

    fn prev_setting(&mut self) {
        self.cur_setting = match self.cur_setting {
            Setting::DigitBrightness => Setting::SwitchBrightness,
            Setting::TrainBrightness => Setting::DigitBrightness,
            Setting::PlatformBrightness => Setting::TrainBrightness,
            Setting::SwitchBrightness => Setting::PlatformBrightness,
        };
    }

    fn inc_setting(&mut self, settings: &mut GameSettings) {
        match self.cur_setting {
            Setting::DigitBrightness => {
                settings.inc_digit_brightness_level();
            }
            Setting::TrainBrightness => {
                settings.inc_car_brightness_level();
            }
            Setting::PlatformBrightness => {
                settings.inc_platform_brightness_level();
            }
            Setting::SwitchBrightness => {
                settings.inc_switch_brightness_level();
            }
        }
    }

    fn dec_setting(&mut self, settings: &mut GameSettings) {
        match self.cur_setting {
            Setting::DigitBrightness => {
                settings.dec_digit_brightness_level();
            }
            Setting::TrainBrightness => {
                settings.dec_car_brightness_level();
            }
            Setting::PlatformBrightness => {
                settings.dec_platform_brightness_level();
            }
            Setting::SwitchBrightness => {
                settings.dec_switch_brightness_level();
            }
        }
    }
}

impl Default for SettingsMode {
    fn default() -> Self {
        SettingsMode {
            cur_setting: Setting::DigitBrightness,
        }
    }
}

impl GameModeHandler for SettingsMode {
    fn on_restart(&mut self, state: &mut GameState) {
        state.display = self.setting_display(&state.settings);
        state.is_over = false;

        state.init_trains(
            Cargo::Have(LedPattern::SolidBright),
            3,
            NOMINAL_TRAIN_SIZE as u8,
        );
        state.init_platforms(Cargo::Have(LedPattern::SolidBright));
    }

    fn on_game_tick(&mut self, state: &mut GameState) {
        for platform in state.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                let led_pattern = match Rand::default().get_u8() % 5 {
                    0 => LedPattern::Blink1,
                    1 => LedPattern::Blink2,
                    2 => LedPattern::Blink3,
                    3 => LedPattern::Fade1,
                    _ => LedPattern::SolidBright,
                };
                platform.set_cargo(Cargo::Have(led_pattern));
            }
        }
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        match event {
            InputEvent::DirectionButtonPressed(InputDirection::Up) => self.prev_setting(),
            InputEvent::DirectionButtonPressed(InputDirection::Down) => self.next_setting(),
            InputEvent::DirectionButtonPressed(InputDirection::Left) => {
                self.dec_setting(&mut state.settings)
            }
            InputEvent::DirectionButtonPressed(InputDirection::Right) => {
                self.inc_setting(&mut state.settings)
            }
            _ => return, // don't update display for other events
        }

        state.display = self.setting_display(&state.settings);
    }

    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        let train = &state.trains[train_index];

        // Clear cargo if train front is at a platform with cargo
        for platform in state.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();
            }
        }
    }
}
