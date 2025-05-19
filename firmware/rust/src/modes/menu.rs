use embedded_hal::i2c::I2c;

use crate::{
    game::GameEntities,
    input::InputEvent,
    modes::{GameModeHandler, SnakeMode},
};

#[derive(Default)]
pub struct MenuMode;

impl GameModeHandler for MenuMode {
    fn short_name(&self) -> &[u8] {
        b"mnu"
    }

    fn num_trains(&self) -> usize {
        2
    }

    fn on_event(&self, event: InputEvent, entities: &mut GameEntities) {
        // if let InputEvent::Select = event {
        //     game.set_mode(GameMode::Snake(SnakeMode::default()));
        // }
        match event {
            // Some(InputEvent::DirectionButtonPressed(direction)) => {
            //     match direction {
            //         InputDirection::Up => game.show_text(b" up"),
            //         InputDirection::Down => game.show_text(b" dn"),
            //         InputDirection::Left => game.show_text(b" lf"),
            //         InputDirection::Right => game.show_text(b" rt"),
            //     }
            //     //self.board_buzzer.tone(4000, 100);
            // }
            InputEvent::SwitchButtonReleased(index) => {}
            InputEvent::DirectionButtonReleased(_) => {}
            _ => {}
        }
    }
}
