// Cargo/LedPattern manage the abstractions for indicating state on LEDs

pub const RED_LED_MIN_B: u8 = 30;
pub const RED_LED_MAX_B: u8 = 80;
pub const YELLOW_LED_MIN_B: u8 = 80;
pub const YELLOW_LED_MAX_B: u8 = 255;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Cargo {
    Empty,
    Have(LedPattern),
    Want(LedPattern),
}

impl Default for Cargo {
    fn default() -> Self {
        Cargo::Empty
    }
}

impl Cargo {
    // pub fn is_empty(&self) -> bool {
    //     matches!(self, Cargo::Empty)
    // }

    pub fn platform_brightness(&self, phase: u8) -> u8 {
        match self {
            Cargo::Empty => 0,
            Cargo::Have(pattern) => pattern.get_pwm(phase, RED_LED_MIN_B, RED_LED_MAX_B),
            Cargo::Want(pattern) => pattern.get_pwm(phase, RED_LED_MAX_B / 2, RED_LED_MIN_B / 2),
        }
    }

    pub fn car_brightness(&self, phase: u8) -> u8 {
        match self {
            Cargo::Empty => YELLOW_LED_MAX_B - 92, // slightly dimmer
            Cargo::Have(pattern) => pattern.get_pwm(phase, YELLOW_LED_MIN_B, YELLOW_LED_MAX_B),
            _ => YELLOW_LED_MIN_B,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum LedPattern {
    SolidBright,
    //SolidDim,
    Blink1,
    Blink2,
    Blink3,
    Fade1,
}

impl LedPattern {
    pub fn get_pwm(&self, phase: u8, min_b: u8, max_b: u8) -> u8 {
        match self {
            LedPattern::SolidBright => max_b,
            //LedPattern::SolidDim => min_b,
            LedPattern::Blink1 => match phase % 64 {
                0..=11 => min_b, // 12 ticks off
                _ => max_b,      // 52 ticks on
            },
            LedPattern::Blink2 => match phase % 64 {
                0..=7 => min_b,   // 8 ticks off
                8..=15 => max_b,  // 8 ticks on
                16..=23 => min_b, // 8 ticks off
                _ => max_b,       // 40 ticks on
            },
            LedPattern::Blink3 => match phase % 64 {
                0..=5 => min_b,   // 6 ticks off
                6..=11 => max_b,  // 6 ticks on
                12..=17 => min_b, // 6 ticks off
                18..=23 => max_b, // 6 ticks on
                24..=29 => min_b, // 6 ticks off
                _ => max_b,       // 34 ticks on
            },
            LedPattern::Fade1 => {
                // Fade up for phase 0..127, fade down for 128..255
                let half_phase = if phase < 128 {
                    phase as u16
                } else {
                    255 - phase as u16
                };
                ((half_phase * (min_b as u16 - max_b as u16)) / 127 + max_b as u16) as u8
            }
        }
    }
}