use enum_dispatch::enum_dispatch;
use crate::{game_state::*, input::InputEvent};

pub mod freeplay;
pub mod menu;
pub mod snake;
pub mod time;

pub use freeplay::*;
pub use menu::*;
pub use snake::*;
pub use time::*;

pub const NUM_MODES: usize = 4;

#[enum_dispatch]
pub trait GameModeHandler
{
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
    Time(TimeMode),
    Snake(SnakeMode),
}

impl Default for GameMode {
    fn default() -> Self {
        GameMode::Menu(MenuMode::default())
    }
}
