// TEMP: quiet unused warnings
#![allow(dead_code)]
#![allow(unused_variables)]

/// A simple random number generator based on the Lehmer LCG algorithm. TODO: verify algorithm is correct
use core::cell::Cell;
use random_trait::Random;

static RNG_STATE: avr_device::interrupt::Mutex<Cell<RngState>> =
    avr_device::interrupt::Mutex::new(Cell::new(RngState { value: 0, index: 0 }));

#[derive(Clone, Copy, Default)]
struct RngState {
    value: u32,
    index: usize,
}

// zero-sized type to represent the RNG
#[derive(Clone, Copy, Default)]
pub struct Rng;

impl Rng {
    pub fn seed(&mut self, seed: u32) {
        avr_device::interrupt::free(|cs| {
            let cell = RNG_STATE.borrow(cs);
            let mut state = cell.get();
            state.value = seed;
            state.value = state.value.wrapping_mul(1664525).wrapping_add(1013904223);
            cell.set(state);
        });
    }
}

impl Random for Rng {
    type Error = ();
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        avr_device::interrupt::free(|cs| {
            let state_cell = RNG_STATE.borrow(cs);
            let mut state = state_cell.get();
            let mut rand_bytes = state.value.to_le_bytes();

            for e in buf.iter_mut() {
                if state.index == 4 {
                    state.value = state.value.wrapping_mul(1664525).wrapping_add(1013904223);
                    rand_bytes = state.value.to_le_bytes();
                    state.index = 0;
                }
                *e = rand_bytes[state.index];
                state.index += 1;
            }

            state_cell.set(state);
        });

        Ok(())
    }
}
