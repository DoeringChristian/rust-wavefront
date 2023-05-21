use common::pcg::PCG;
use spirv_std::glam::*;
use spirv_std::*;

pub struct IndependentSampler {
    pcg: PCG,
}

impl IndependentSampler {
    #[spirv(compute(threads(64)))]
    pub fn next_1d(
        #[spirv(global_invocation_id)] pos: glam::UVec3,
        #[spirv(num_workgroups)] size: glam::UVec3,
        #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] this: &mut [Self],
        #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sample: &mut [f32],
    ) {
        let idx = (pos.x) as usize;
        assert!(pos.x < size.x);
        assert!(pos.y < size.y);
        assert!(pos.z < size.z);
        sample[idx] = this[idx].pcg.next_f32();
    }
    #[spirv(compute(threads(64)))]
    pub fn next_2d(
        #[spirv(global_invocation_id)] pos: glam::UVec3,
        #[spirv(num_workgroups)] size: glam::UVec3,
        #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] this: &[Self],
        #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sample: &[Vec2],
    ) {
        let idx = (size.x * pos.y + pos.x) as usize;
        assert!(pos.x < size.x);
        assert!(pos.y < size.y);
        assert!(pos.z < size.z);
    }
}
