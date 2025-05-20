use embedded_hal::i2c::I2c;
use random_trait::Random;

use crate::{
    game::GameEntities, input::InputEvent, location::Direction, modes::{GameModeHandler, SnakeMode}, platform, random::Rand, switch, train::Train
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

    fn on_game_tick(&self, entities: &mut GameEntities) {
        for platform in entities.platforms.iter_mut() {
            if platform.is_empty() && Rand::default().get_u16() <= 50 {
                platform.set_cargo();
                // TODO: score?
            }
        }   
    }

    fn on_input_event(&self, event: InputEvent, entities: &mut GameEntities) {
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



    fn on_train_event(&self, train_index: usize, entities: &mut GameEntities) {
        // after a train moves away from a switch, randomly switch the switch
        let train = &entities.trains[train_index];
        let last_loc = train.last_loc();
        for switch in entities.switches.iter_mut() {
            if last_loc == switch.next_location(Direction::Anode)
                || last_loc == switch.fork_location(Direction::Anode)
                || last_loc == switch.next_location(Direction::Cathode)
                || last_loc == switch.fork_location(Direction::Cathode)
            {
                if Rand::default().get_bool() {
                    switch.switch();
                }
            }
        }

        for platform in entities.platforms.iter_mut() {
            if !platform.is_empty() && train.front() == platform.track_location() {
                platform.clear_cargo();
                //self.score += 1;
                //self.board_digits.display_number(self.score).unwrap();
            }
        }
    }


}
