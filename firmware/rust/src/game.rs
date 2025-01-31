use atmega_hal::port::{
    mode::{Input, PullUp},
    Dynamic, Pin,
};
//use embedded_hal::digital::InputPin;
use crate::as1115::AS1115;
use is31fl3731::IS31FL3731;
use crate::tone::Timer3Tone;

use crate::train::Train;


#[derive(Copy, Clone)]
enum GameMode {
    Animation,
    Freeplay,
    Race,
    Survival,
    Puzzle,
}

const MAX_TRAINS: usize = 5;
const NUM_BUTTONS: usize = 12;
const NUM_DIGITS: usize = 3;
const DIGIT_INTENSITY: u8 = 3;

struct Game<I2C> {
    // board components
    board_buttons: [Pin<Input<PullUp>, Dynamic>; NUM_BUTTONS],
    board_buzzer: Timer3Tone,
    board_digits: AS1115<I2C>,
    board_leds: IS31FL3731<I2C>,

    // game state
    mode: GameMode,
    is_over: bool,
    trains: heapless::Vec<Train, MAX_TRAINS>,
    //platforms: [Platform; PLATFORM_COUNT],
}

impl<I2C> Game<I2C> {
    // do we need singleton enforcement with ownership?
    pub fn new(
        board_buttons: [Pin<Input<PullUp>, Dynamic>; NUM_BUTTONS],
        board_buzzer: Timer3Tone,
        board_digits: AS1115<I2C>,
        board_leds: IS31FL3731<I2C>,
    ) -> Self {
        Self {
            board_buttons,
            board_buzzer,
            board_digits,
            board_leds,
            mode: GameMode::Animation,
            is_over: false,
            trains: heapless::Vec::<Train, MAX_TRAINS>::new(),
        }
    }
}
