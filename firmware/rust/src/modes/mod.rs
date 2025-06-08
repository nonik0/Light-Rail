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
