use core::num::Wrapping;

/// The initial/default state to initialize the Pcg struct with
pub const INIT_STATE: u64 = 0x853c_49e6_748f_ea9b;

/// The initial/default incrementing value to initialize the Pcg struct with
pub const INIT_INC: u64 = 0xda3e_39cb_94b9_5bdb;

/// The value to multiply the state with when a random number is generated in order to
/// alter the random number generator's state
pub const INCREMENTOR: u64 = 6_364_136_223_846_793_005;

#[derive(Clone, Copy)]
pub struct PCG {
    state: u64,
    inc: u64,
}

impl PCG {
    pub fn new(seed: u64, seq: u64) -> Self {
        Self {
            state: seed,
            inc: (seq << 1) | 1,
        }
    }

    pub fn next_u64(&mut self) -> u64 {
        let old_state = self.state;
        self.state = (Wrapping(old_state) * Wrapping(INCREMENTOR) + Wrapping(self.inc)).0;
        let xor_shifted = (old_state >> 18) ^ old_state >> 27;

        let rot = (old_state >> 59) as i64;
        (xor_shifted >> rot as u64) | (xor_shifted << ((-rot) & 31))
    }

    pub fn next_u32(&mut self) -> u32 {
        self.next_u64() as u32
    }

    pub fn next_f32(&mut self) -> f32 {
        // let sample = self.next_u32();
        // let sample = sample.wrapping_shr(9) | 0x3f800000u32;
        // unsafe { *(&sample as *const u32 as *const f32) }
        (self.next_u32() & 0xffffff) as f32 / 16777216.0
    }
}

impl Default for PCG {
    fn default() -> Self {
        Self {
            state: INIT_STATE,
            inc: INIT_INC,
        }
    }
}
