use core::cell::Cell;
use random_trait::{GenerateRand, Random};

// TODO: different seed, maybe use option before setting seed?
static RNG: avr_device::interrupt::Mutex<Cell<Rng>> =
    avr_device::interrupt::Mutex::new(Cell::new(Rng(0)));

// struct LCG(u32);

// impl LCG {
//     pub fn new(seed: u32) -> Self {
//         LCG { seed }
//     }

//     pub fn next(&mut self) -> u32 {
//         self.0 = self.0.wrapping_mul(1664525).wrapping_add(1013904223);
//         self.0
//     }
// }

#[derive(Clone, Copy)]
struct Rng(u32);

impl Random for Rng {
    type Error = ();
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        avr_device::interrupt::free(|cs| {
            let mut rng = RNG.borrow(cs).get();
            let mut rng_value = rng.0;
            let mut rng_bytes = rng_value.to_le_bytes();
            let mut byte_index = 0;

            for e in buf.iter_mut() {
                if byte_index == 4 {
                    rng_value = rng_value.wrapping_mul(1664525).wrapping_add(1013904223);
                    rng_bytes = rng_value.to_le_bytes();
                    byte_index = 0;
                }
                *e = rng_bytes[byte_index];
                byte_index += 1;
            }

            rng.0 = rng_value;
            RNG.borrow(cs).set(rng);
        }); 

        Ok(())
    }
}
