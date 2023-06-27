use common::render::pcg::PCG;
use common::sampler::independent::IndependentSampler;
use spirv_std::glam::*;
use spirv_std::*;

#[spirv(compute(threads(64)))]
pub fn next_1d(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] this: &mut [IndependentSampler],
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
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] this: &mut [IndependentSampler],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sample: &mut [Vec2],
) {
    let idx = (pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);
    sample[idx] = vec2(this[idx].pcg.next_f32(), this[idx].pcg.next_f32());
}

#[spirv(compute(threads(64)))]
pub fn seed(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] this: &mut [IndependentSampler],
    #[spirv(push_constant)] seed: &u64,
) {
    let idx = (pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);
    this[idx].pcg = PCG::new(*seed, idx as _);
}
