use embedded_hal::i2c::I2c;

use crate::{game::GameState, input::InputEvent, modes::GameModeHandler};

#[derive(Default)]
pub struct FreeplayMode;

impl GameModeHandler for FreeplayMode
{
    fn short_name(&self) -> &[u8] {
        b"ply"
    }

    fn num_trains(&self) -> usize {
        1
    }

    fn on_input_event(&mut self, event: InputEvent, state: &mut GameState) {
        // Implement freeplay behavior here
        // create/destroy trains, add cars, etc.
    }
}
