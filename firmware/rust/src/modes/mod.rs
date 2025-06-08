use crate::{
    game_state::*,
    input::InputEvent,
};

pub mod menu;
pub mod freeplay;
pub mod snake;
pub mod time;

pub use menu::*;
pub use freeplay::*;
pub use snake::*;
pub use time::*;

pub const NUM_MODES: usize = 4;

pub trait GameModeHandler
{
    // on restart
    fn on_restart(&mut self, _state: &mut GameState) {}

    // on each game tick
    fn on_game_tick(&mut self, _entities: &mut GameState) {}

    // when a button is pressed
    fn on_input_event(&mut self, _event: InputEvent, _state: &mut GameState) {}

    // when a train advances
    fn on_train_advance(&mut self, _train_index: usize, _tate: &mut GameState) {}
}

pub enum GameMode {
    Menu(MenuMode),
    Freeplay(FreeplayMode),
    Time(TimeMode),
    Snake(SnakeMode),
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Menu(MenuMode::default())
    }
}

impl GameModeHandler for GameMode {
    fn on_restart(&mut self, state: &mut GameState) {
        match self {
            GameMode::Menu(m) => m.on_restart(state),
            GameMode::Freeplay(m) => m.on_restart(state),
            GameMode::Time(m) => m.on_restart(state),
            GameMode::Snake(m) => m.on_restart(state),
        }
    }
    fn on_game_tick(&mut self, state: &mut GameState) {
        match self {
            GameMode::Menu(m) => m.on_game_tick(state),
            GameMode::Freeplay(m) => m.on_game_tick(state),
            GameMode::Time(m) => m.on_game_tick(state),
            GameMode::Snake(m) => m.on_game_tick(state),
        }
    }
    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        match self {
            GameMode::Menu(m) => m.on_input_event(event, state),
            GameMode::Freeplay(m) => m.on_input_event(event, state),
            GameMode::Time(m) => m.on_input_event(event, state),
            GameMode::Snake(m) => m.on_input_event(event, state),
        }
    }
    fn on_train_advance(&mut self, train_index: usize, state: &mut GameState) {
        match self {
            GameMode::Menu(m) => m.on_train_advance(train_index, state),
            GameMode::Freeplay(m) => m.on_train_advance(train_index, state),
            GameMode::Time(m) => m.on_train_advance(train_index, state),
            GameMode::Snake(m) => m.on_train_advance(train_index, state),
        }
    }
}