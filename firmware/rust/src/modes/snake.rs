use embedded_hal::i2c::I2c;

use crate::{game::GameEntities, input::InputEvent, modes::GameModeHandler};

#[derive(Default)]
pub struct SnakeMode;

impl GameModeHandler for SnakeMode
{
    fn short_name(&self) -> &[u8] {
        b"snk"
    }

    fn num_trains(&self) -> usize {
        1
    }

    fn on_event(&self, event: InputEvent, entities: &mut GameEntities) {
        // Implement Snake behavior here
    }
}
