use spirv_std::glam::*;
use spirv_std::*;

#[spirv(compute(threads(64)))]
pub fn put(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] sample: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sample_pos: &[Vec2],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 2)] image: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
) {
    let idx = (pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    // let wavefront_size = size.x * size.y;
    // let img_size: UVec2 = image.query_size();

    unsafe { image.write((sample_pos[idx]).as_uvec2(), sample[idx].xyz().extend(1.)) };
}
