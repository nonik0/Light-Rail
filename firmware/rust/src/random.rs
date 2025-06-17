use avr_device::interrupt::Mutex;
use core::cell::Cell;
use random_trait::Random;

// static struct holding state of the RNG
#[derive(Clone, Copy)]
pub struct RngState {
    value: u32,
    index: usize,
}

static RNG_STATE: Mutex<Cell<RngState>> = Mutex::new(Cell::new(RngState { value: 0, index: 0 }));

// zero-size type in front of the static state
#[derive(Default)]
pub struct Rand;

impl Rand {
    pub fn seed(seed: u32) {
        avr_device::interrupt::free(|cs| {
            RNG_STATE.borrow(cs).set(RngState {
                value: seed,
                index: 0,
            });
        });
    }

    /// Returns a random u8 in the range [min, max] (inclusive), with uniform distribution.
    pub fn from_range(min: u8, max: u8) -> u8 {
        assert!(min <= max, "min must be <= max");
        let mut instance = Self::default();
        let span = max.wrapping_sub(min).wrapping_add(1);

        if span == 0 {
            instance.get_u8();
        }

        // Avoid modulo bias by only accepting values < zone
        let span_max = u8::MAX - (u8::MAX % span);
        loop {
            let value = instance.get_u8();
            if value < span_max {
                return min + (value % span);
            }
        }
    }
}

impl Random for Rand {
    type Error = ();
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        avr_device::interrupt::free(|cs| {
            let mut rng_state = RNG_STATE.borrow(cs).get();
            let mut rand_bytes = rng_state.value.to_le_bytes();
            for e in buf.iter_mut() {
                *e = rand_bytes[rng_state.index];
                rng_state.index += 1;

                if rng_state.index == 4 {
                    rng_state.value = rng_state
                        .value
                        .wrapping_mul(1664525)
                        .wrapping_add(1013904223);
                    rng_state.index = 0;
                    rand_bytes = rng_state.value.to_le_bytes();
                }
            }

            RNG_STATE.borrow(cs).set(rng_state);
        });
        Ok(())
    }
}
