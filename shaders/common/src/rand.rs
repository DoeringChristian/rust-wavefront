use core::num::Wrapping;

pub fn sample_tea_32(v0: u32, v1: u32, rounds: usize) -> (u32, u32) {
    let rounds = if rounds <= 0 { 4 } else { rounds };
    let mut v0 = v0;
    let mut v1 = v0;
    let mut sum = 0;
    for _ in 0..rounds {
        sum += 0x9e3779b9;

        v0 += v1.wrapping_shl(4).wrapping_add(0xad90777du32)
            ^ v1.wrapping_add(sum)
            ^ v1.wrapping_shr(5).wrapping_add(0x7e95761eu32);

        v1 += v0.wrapping_shl(4).wrapping_add(0xad90777du32)
            ^ v0.wrapping_add(sum)
            ^ v0.wrapping_shr(5).wrapping_add(0x7e95761eu32);
    }
    (v0, v1)
}

pub fn sample_tea_64(v0: u64, v1: u64, rounds: usize) -> (u64, u64) {
    let rounds = if rounds <= 0 { 4 } else { rounds };
    todo!()
}
