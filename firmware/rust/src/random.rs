// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

/// A simple random number generator based on the Lehmer LCG algorithm. TODO: verify algorithm is correct

use core::cell::Cell;
use random_trait::Random;

static RNG_STATE: avr_device::interrupt::Mutex<Cell<u32>> =
    avr_device::interrupt::Mutex::new(Cell::new(0));

// zero-sized type to represent the RNG
#[derive(Clone, Copy, Default)]
pub struct Rng;

impl Rng {
    pub fn seed(&mut self, seed: u32) {
        avr_device::interrupt::free(|cs| {
            let cell = RNG_STATE.borrow(cs);
            cell.set(seed);
        });
    }
}

impl Random for Rng {
    type Error = ();
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        avr_device::interrupt::free(|cs| {
            let cell = RNG_STATE.borrow(cs);
            let mut state = cell.get();
            let mut rand_bytes = state.to_le_bytes();
            let mut index = 0;

            state = state.wrapping_mul(1664525).wrapping_add(1013904223);
            for e in buf.iter_mut() {
                if index == 4 {
                    state = state.wrapping_mul(1664525).wrapping_add(1013904223);
                    rand_bytes = state.to_le_bytes();
                    index = 0;
                }
                *e = rand_bytes[index];
                index += 1;
            }

            cell.set(state);
        }); 

        Ok(())
    }
}
