use rand_core::{CryptoRng, RngCore};
use std::vec::Vec;

/// This is a dummy RNG for cases when we don't actually want RNG and just need to set a specific value for testing.
#[derive(Clone)]
pub struct SetSeed {
    seed: Vec<u8>,
}

impl SetSeed {

    /// Initialize 
    pub fn new(seed: Vec<u8>) -> Self {
        Self { seed }
    }
}

impl RngCore for SetSeed {
    fn next_u32(&mut self) -> u32 {
        unimplemented!();
    }

    fn next_u64(&mut self) -> u64 {
        unimplemented!();
    }

    fn fill_bytes(&mut self, target: &mut [u8]) {
        target.copy_from_slice(&self.seed.as_slice());
    }

    fn try_fill_bytes(&mut self, _: &mut [u8]) -> core::result::Result<(), rand_core::Error> {
        unimplemented!();
    }
}

impl CryptoRng for SetSeed {}