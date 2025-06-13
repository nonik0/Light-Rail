use crate::{game_state::*, input::InputEvent};
use enum_dispatch::enum_dispatch;

pub mod freeplay;
pub mod menu;
pub mod settings;
pub mod snake;
pub mod time;

pub use freeplay::*;
pub use menu::*;
pub use settings::*;
pub use snake::*;
pub use time::*;

pub const NUM_MODES: usize = 5;

#[enum_dispatch]
pub trait GameModeHandler {
    // on restart
    fn on_restart(&mut self, state: &mut GameState);

    // on each game tick
    fn on_game_tick(&mut self, entities: &mut GameState);

    // when a button is pressed
    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState);

    // when a train advances
    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState);
}

#[enum_dispatch(GameModeHandler)]
pub enum GameMode {
    Menu(MenuMode),
    Freeplay(FreeplayMode),
    Snake(SnakeMode),
    Time(TimeMode),
    SettingsMode(SettingsMode),
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Menu(MenuMode::default())
    }
}

impl GameMode {
    pub fn from_index(mode_index: usize) -> Self {
        match mode_index {
            1 => GameMode::Freeplay(FreeplayMode::default()),
            2 => GameMode::Snake(SnakeMode::default()),
            3 => GameMode::Time(TimeMode::default()),
            4 => GameMode::SettingsMode(SettingsMode::default()),
            _ => GameMode::Menu(MenuMode::default()),
        }
    }

    pub fn mode_name(mode_index: usize) -> [u8; 3] {
        match mode_index {
            1 => *b"ply", // Play
            2 => *b"snk", // Snake
            3 => *b"tme", // Time (pick up and deliver)
            4 => *b"set", // Settings
            _ => *b"err",
        }
    }
}
