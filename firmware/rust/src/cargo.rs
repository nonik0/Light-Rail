#[derive(Clone, Copy, PartialEq)]
pub enum Cargo {
    Empty,
    Full(LedPattern),
}

impl Default for Cargo {
    fn default() -> Self {
        Cargo::Empty
    }
}

impl Cargo {
    pub fn platform_brightness(&self, phase: u8, min: u8, max: u8) -> u8 {
        match self {
            Cargo::Empty => 0,
            Cargo::Full(pattern) =>  pattern.get_pwm(phase, min, max),
        }
    }

    pub fn car_brightness(&self, phase: u8, max: u8) -> u8 {
        match self {
            Cargo::Empty => max >> 1,
            Cargo::Full(pattern) => pattern.get_pwm(phase, max >> 1, max),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum LedPattern {
    Solid,
    Blink1,
    Blink2,
    Blink3,
    Fade,
}

impl LedPattern {
    pub fn get_pwm(&self, phase: u8, min_b: u8, max_b: u8) -> u8 {
        match self {
            LedPattern::Solid => max_b,
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
            LedPattern::Fade => {
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
