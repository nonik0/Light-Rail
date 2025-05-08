use atmega_hal::port::{
    mode::{Input, PullUp},
    Dynamic, Pin,
};
use embedded_hal::digital::InputPin;
use crate::NUM_BUTTONS;

const PRESS_CYCLES: u8 = 1;
const HOLD_CYCLES: u8 = 20;
const DEBOUNCE_CYCLES: u8 = 3;

pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub enum InputEvent {
    TrackButtonPressed(u8),
    TrackButtonReleased(u8),
    DirectionButtonPressed(Direction),
    DirectionButtonReleased(Direction),
}

pub struct Buttons {
    button_pins: [Pin<Input<PullUp>, Dynamic>; NUM_BUTTONS],
    held_cycles: [u8; NUM_BUTTONS],
    debounce_cycles: [u8; NUM_BUTTONS],
}

impl Buttons {
    pub fn new(button_pins: [Pin<Input<PullUp>, Dynamic>; 12]) -> Self {
        Buttons {
            button_pins,
            held_cycles: [0; NUM_BUTTONS],
            debounce_cycles: [0; NUM_BUTTONS],
        }
    }

    fn index_to_press_event(index: usize) -> InputEvent {
        match index {
            0..=7 => InputEvent::TrackButtonPressed(index as u8),
            8 => InputEvent::DirectionButtonPressed(Direction::Up),
            9 => InputEvent::DirectionButtonPressed(Direction::Down),
            10 => InputEvent::DirectionButtonPressed(Direction::Left),
            11 => InputEvent::DirectionButtonPressed(Direction::Right),
            _ => unreachable!(),
        }
    }

    fn index_to_release_event(index: usize) -> InputEvent {
        match index {
            0..=7 => InputEvent::TrackButtonReleased(index as u8),
            8 => InputEvent::DirectionButtonReleased(Direction::Up),
            9 => InputEvent::DirectionButtonReleased(Direction::Down),
            10 => InputEvent::DirectionButtonReleased(Direction::Left),
            11 => InputEvent::DirectionButtonReleased(Direction::Right),
            _ => unreachable!(),
        }
    }

    pub fn update(&mut self) -> Option<InputEvent> {
        for (i, button_pin) in self.button_pins.iter_mut().enumerate() {
            if button_pin.is_low().unwrap() {
                let mut pressed = button_pin.is_low().unwrap();

                if self.debounce_cycles[i] > 0 {
                    self.debounce_cycles[i] -= 1;
                    pressed = false;
                }

                if pressed {
                    self.held_cycles[i] += 1;
                    if self.held_cycles[i] == PRESS_CYCLES {
                        return Some(Self::index_to_press_event(i))
                    } else if self.held_cycles[i] == HOLD_CYCLES {
                        //return Some(Self::index_to_hold_event(i))
                    }
                } else if self.held_cycles[i] > 0 {
                    let was_held = self.held_cycles[i] >= HOLD_CYCLES;

                    self.debounce_cycles[i] = DEBOUNCE_CYCLES;
                    self.held_cycles[i] = 0;

                    if was_held {
                        //return Some(Self::index_to_holdrelease_event(i))
                    } else {
                        return Some(Self::index_to_release_event(i))
                    }
                }
            }
        }

        None
    }
}
