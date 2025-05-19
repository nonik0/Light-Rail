use crate::{
    game::{Game, GameEntities},
    input::InputEvent,
};

pub mod menu;
pub mod snake;

pub use menu::*;
pub use snake::*;

pub trait GameModeHandler
{
    fn short_name(&self) -> &[u8];
    fn num_trains(&self) -> usize;
    fn on_event(&self, event: InputEvent, entities: &mut GameEntities);
}
