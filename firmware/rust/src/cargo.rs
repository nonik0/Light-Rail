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
            LedPattern::Blink1 => match phase % 128 {
                0..=23 => min_b, // 24 ticks off
                _ => max_b,
            },
            LedPattern::Blink2 => match phase % 128 {
                0..=15 => min_b,   // 16 ticks off
                16..=31 => max_b,  // 16 ticks on
                32..=47 => min_b, // 16 ticks off
                _ => max_b,
            },
            LedPattern::Blink3 => match phase % 128 {
                0..=11 => min_b,   // 12 ticks off
                12..=23 => max_b,  // 12 ticks on
                24..=35 => min_b, // 12 ticks off
                36..=47 => max_b, // 12 ticks on
                48..=59 => min_b, // 12 ticks off
                _ => max_b,
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
