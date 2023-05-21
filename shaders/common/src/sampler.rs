use crate::pcg::PCG;
use crate::rand::sample_tea_32;
use spirv_std::glam;

#[derive(Clone, Copy, Default)]
pub struct IndependentSampler {
    pcg: PCG,
}

impl IndependentSampler {
    pub fn new(seed: u32, idx: u32) -> Self {
        let (v0, v1) = sample_tea_32(seed, idx, 4);
        Self {
            pcg: PCG::new(v0 as _, v1 as _),
        }
    }
    pub fn seed(&mut self, seed: u32, idx: u32) {
        let (v0, v1) = sample_tea_32(seed, idx, 4);
        self.pcg = PCG::new(v0 as _, v1 as _);
    }

    pub fn next_1d(&mut self) -> f32 {
        self.pcg.next_f32()
    }
    pub fn next_2d(&mut self) -> glam::Vec2 {
        glam::vec2(self.next_1d(), self.next_1d())
    }
}
