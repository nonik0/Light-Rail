use crate::train;

#[derive(Copy, Clone)]
enum GameMode {
    Animation,
    Freeplay,
    Race,
    Survival,
    Puzzle,
}

struct Game {
    //track_graph: &'static Track,
    // board_leds: AdafruitIS31FL3731,
    // board_digits: AS1115,
    mode: GameMode,
    is_over: bool,

    num_trains: u8,
    trains: [train::Train; Game::MAX_TRAINS],
    //platforms: [Platform; PLATFORM_COUNT],
}

impl Game {
    const MAX_TRAINS: usize = 5;
    const NUM_DIGITS: usize = 3;
    const DIGIT_INTENSITY: u8 = 3;

    // fn new() -> Self {
    //     Game {
    //         track_graph: unsafe { &*(TRACK_DATA as *const Track) },
    //         board_leds: AdafruitIS31FL3731::new(),
    //         board_digits: AS1115::new(0x00),
    //         mode: GameMode::Animation,
    //         is_over: false,
    //         num_trains: 0,
    //         trains: [Train::default(); Game::MAX_TRAINS],
    //         platforms: [Platform::default(); PLATFORM_COUNT],
    //     }
    // }
}