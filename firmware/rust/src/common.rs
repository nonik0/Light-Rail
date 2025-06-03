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
            LedPattern::Blink1 => if phase % 2 == 0 { max_b } else { min_b },
            LedPattern::Blink2 => if phase % 4 < 2 { max_b } else { min_b },
            LedPattern::Blink3 => if phase % 6 < 3 { max_b } else { min_b },
            LedPattern::Fade1 => {
                // match phase to linear increase and decrease
                if phase < 128 {
                    (phase * max_b) / 128
                } else {
                    ((255 - phase) * max_b) / 128
                }
            }
        }
    }
}

pub const RED_LED_MIN_B: u8 = 31;
pub const RED_LED_MAX_B: u8 = 63;
pub const YELLOW_LED_MIN_B: u8 = 64;
pub const YELLOW_LED_MAX_B: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cargo {
    Empty,
    Full(LedPattern),
}

impl Cargo {
    pub fn is_empty(&self) -> bool {
        matches!(self, Cargo::Empty)
    }

    pub fn get_platform_pwm(&self, phase: u8) -> u8 {
        match self {
            Cargo::Empty => 0,
            Cargo::Full(pattern) => pattern.get_pwm(phase, RED_LED_MIN_B, RED_LED_MAX_B),
        }
    }

    pub fn get_track_pwm(&self, phase: u8) -> u8 {
        match self {
            Cargo::Empty => 30,
            Cargo::Full(pattern) => pattern.get_pwm(phase, YELLOW_LED_MIN_B, YELLOW_LED_MAX_B),
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