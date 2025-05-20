use crate::{
    game::{Game, GameState},
    input::InputEvent,
    train::Train,
};

pub mod menu;
pub mod freeplay;
pub mod snake;

pub use menu::*;
pub use freeplay::*;
pub use snake::*;

pub const NUM_GAME_MODES: usize = 2;

pub trait GameModeHandler
{
    fn short_name(&self) -> &[u8];
    fn num_trains(&self) -> usize { 1 }

    // on each game tick
    fn on_game_tick(&mut self, entities: &mut GameState) {}

    // when a button is pressed
    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {}

    // when a train advances
    fn on_train_event(&mut self, train_index: usize, state: &mut GameState) {}
}
