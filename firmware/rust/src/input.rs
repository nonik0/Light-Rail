
#![allow(unused)]
use atmega_hal::port::{
    mode::{Input, PullUp},
    Dynamic, Pin,
};
use embedded_hal::digital::InputPin;
use crate::NUM_BUTTONS;

const PRESS_CYCLES: u8 = 1;
const HOLD_CYCLES: u8 = 20;
const DEBOUNCE_CYCLES: u8 = 3;

pub enum InputDirection {
    Up,
    Down,
    Left,
    Right,
}

pub enum InputEvent {
    SwitchButtonPressed(u8),
    SwitchButtonHeld(u8),
    SwitchButtonReleased(u8),
    DirectionButtonPressed(InputDirection),
    DirectionButtonHeld(InputDirection),
    DirectionButtonReleased(InputDirection),
}

pub struct BoardInput {
    button_pins: [Pin<Input<PullUp>, Dynamic>; NUM_BUTTONS],
    held_cycles: [u8; NUM_BUTTONS],
    debounce_cycles: [u8; NUM_BUTTONS],
}

impl BoardInput {
    pub fn new(button_pins: [Pin<Input<PullUp>, Dynamic>; 12]) -> Self {
        BoardInput {
            button_pins,
            held_cycles: [0; NUM_BUTTONS],
            debounce_cycles: [0; NUM_BUTTONS],
        }
    }

    fn index_to_hold_event(index: usize) -> InputEvent {
        match index {
            0..=7 => InputEvent::SwitchButtonHeld(index as u8),
            8 => InputEvent::DirectionButtonHeld(InputDirection::Up),
            9 => InputEvent::DirectionButtonHeld(InputDirection::Down),
            10 => InputEvent::DirectionButtonHeld(InputDirection::Left),
            11 => InputEvent::DirectionButtonHeld(InputDirection::Right),
            _ => unreachable!(),
        }
    }

    fn index_to_press_event(index: usize) -> InputEvent {
        match index {
            0..=7 => InputEvent::SwitchButtonPressed(index as u8),
            8 => InputEvent::DirectionButtonPressed(InputDirection::Up),
            9 => InputEvent::DirectionButtonPressed(InputDirection::Down),
            10 => InputEvent::DirectionButtonPressed(InputDirection::Left),
            11 => InputEvent::DirectionButtonPressed(InputDirection::Right),
            _ => unreachable!(),
        }
    }

    fn index_to_release_event(index: usize) -> InputEvent {
        match index {
            0..=7 => InputEvent::SwitchButtonReleased(index as u8),
            8 => InputEvent::DirectionButtonReleased(InputDirection::Up),
            9 => InputEvent::DirectionButtonReleased(InputDirection::Down),
            10 => InputEvent::DirectionButtonReleased(InputDirection::Left),
            11 => InputEvent::DirectionButtonReleased(InputDirection::Right),
            _ => unreachable!(),
        }
    }

    pub fn update(&mut self) -> Option<InputEvent> {
        for (i, button_pin) in self.button_pins.iter_mut().enumerate() {
            if button_pin.is_low().unwrap_or(false) {
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
                        return Some(Self::index_to_hold_event(i))
                    }
                } else if self.held_cycles[i] > 0 {
                    //let was_held = self.held_cycles[i] >= HOLD_CYCLES;

                    self.debounce_cycles[i] = DEBOUNCE_CYCLES;
                    self.held_cycles[i] = 0;

                    // only one release for now
                    // if was_held {
                    //     return Some(Self::index_to_holdrelease_event(i))
                    // } else {
                    //     return Some(Self::index_to_release_event(i))
                    // }
                    return Some(Self::index_to_release_event(i))
                }
            }
        }

        None
    }
}
