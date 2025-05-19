use crate::{
    game::{Game, GameEntities},
    input::InputEvent,
    train::Train,
};

pub mod menu;
pub mod snake;

pub use menu::*;
pub use snake::*;

pub trait GameModeHandler
{
    fn short_name(&self) -> &[u8];
    fn num_trains(&self) -> usize { 1 }
    fn on_input_event(&self, event: InputEvent, entities: &mut GameEntities) {}
    fn on_train_event(&self, train_index: usize, entities: &mut GameEntities) {}
}
