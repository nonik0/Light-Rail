use core::u8;

use crate::{location::Location, NUM_DIGITS};
use is31fl3731::gamma;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LedPattern {
    SolidBright,
    SolidDim,
    Blink1,
    Blink2,
    Blink3,
    Fade1
}

impl LedPattern {
    pub fn get_pwm(&self, phase: u8, min_b: u8, max_b: u8) -> u8 {
        match self {
            LedPattern::SolidBright => gamma(max_b),
            LedPattern::SolidDim => gamma(min_b),
            LedPattern::Blink1 => match phase % 64 {
                0..=7 => gamma(max_b), // 8 ticks on
                8..=u8::MAX => gamma(min_b), // 56 ticks off
            }
            LedPattern::Blink2 => match phase % 64 {
                0..=7 => gamma(max_b), // 8 ticks on
                8..=15 => gamma(min_b), // 8 ticks off
                16..=23 => gamma(max_b), // 8 ticks on
                24..=u8::MAX => gamma(min_b), // 40 ticks off
            }
            LedPattern::Blink3 => match phase % 64 {
                0..=7 => gamma(max_b), // 8 ticks on
                8..=15 => gamma(min_b), // 8 ticks off
                16..=23 => gamma(max_b), // 8 ticks on
                24..=31 => gamma(min_b), // 8 ticks off
                32..=39 => gamma(max_b), // 8 ticks on
                40..=u8::MAX => gamma(min_b), // 24 ticks off
            },
            LedPattern::Fade1 => {
                // Fade up for phase 0..127, fade down for 128..255
                let half_phase = if phase < 128 {
                    phase as u16
                } else {
                    255 - phase as u16
                };
                let value = ((half_phase * (max_b as u16 - min_b as u16)) / 127 + min_b as u16) as u8;
                gamma(value)
            }
        }
    }
}

pub const RED_LED_MIN_B: u8 = 30;
pub const RED_LED_MAX_B: u8 = 90;
pub const YELLOW_LED_MIN_B: u8 = 64;
pub const YELLOW_LED_MAX_B: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cargo {
    Empty,
    Have(LedPattern),
    // Want(LedPattern),
}

impl Cargo {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cargo::Empty)
    }

    pub fn get_platform_pwm(&self, phase: u8) -> u8 {
        match self {
            Cargo::Empty => 0,
            Cargo::Have(pattern) => pattern.get_pwm(phase, RED_LED_MIN_B, RED_LED_MAX_B),
            //Cargo::Want(pattern) => pattern.get_pwm(phase, RED_LED_MAX_B, RED_LED_MIN_B),
        }
    }

    pub fn get_track_pwm(&self, phase: u8) -> u8 {
        match self {
            Cargo::Empty => 30,
            Cargo::Have(pattern) => pattern.get_pwm(phase, YELLOW_LED_MIN_B, YELLOW_LED_MAX_B),
            //Cargo::Want(pattern) => pattern.get_pwm(phase, YELLOW_LED_MAX_B, YELLOW_LED_MIN_B),
        }
    }
}

#[derive(Debug)]
pub struct LedUpdate {
    pub location: Location,
    pub pwm: u8,
}

impl LedUpdate {
    pub fn new(location: Location, pwm: u8) -> Self {
        Self { location, pwm }
    }
}