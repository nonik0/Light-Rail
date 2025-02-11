use random_trait::Random;

// Clone/Copy isn't the best but also avoiding lifetime issue here for now
#[derive(Clone, Copy, Debug)]
pub struct Rng {
    value: u32,
    index: usize,
}

// TODO: try to figure out static implementation
impl Rng {
    pub fn seed(seed: u32) -> Self {
        Self {
            value: seed,
            index: 0,
        }
    }
}

impl Random for Rng {
    type Error = ();
    fn try_fill_bytes(&mut self, buf: &mut [u8]) -> Result<(), Self::Error> {
        let mut rand_bytes = self.value.to_le_bytes();
        for e in buf.iter_mut() {
            *e = rand_bytes[self.index];
            self.index += 1;

            if self.index == 4 {
                self.value = self.value.wrapping_mul(1664525).wrapping_add(1013904223);
                self.index = 0;
                rand_bytes = self.value.to_le_bytes();
            }
        }
        Ok(())
    }
}
