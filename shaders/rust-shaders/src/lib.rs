#![no_std]

use common::*;
use spirv_std::glam::*;
use spirv_std::ray_tracing::AccelerationStructure;
use spirv_std::*;

#[repr(C)]
pub struct RayPayload {
    p: Vec3,
}

#[spirv(ray_generation)]
pub fn path_trace(
    #[spirv(launch_id)] pos: UVec3,
    #[spirv(launch_size)] size: UVec3,
    // #[spirv(push_constant)] push_constant: &PathTracePushConstant,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] indices: &[u32],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] positions: &[Vec3],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] normals: &[Vec3],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] uvs: &[Vec2],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] instances: &[Instance],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] meshes: &[Mesh],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] emitters: &[Emitter],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] materials: &[Material],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] cammeras: &[Camera],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 10)] accel: &AccelerationStructure,
    // #[spirv(uniform_constant, descriptor_set = 0, binding = 9)] textures: &RuntimeArray<
    //     Image!(2D, format = rgba32f, sampled = false),
    // >,
    #[spirv(uniform_constant, descriptor_set = 1, binding = 0)] color: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
    #[spirv(uniform_constant, descriptor_set = 1, binding = 1)] normal: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
    #[spirv(uniform_constant, descriptor_set = 1, binding = 2)] position: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
) {
    let idx = size.x * pos.y + pos.x;
    unsafe { color.write(pos.xy().as_ivec2().as_uvec2(), vec4(1., 0., 0., 0.)) };
}

#[spirv(closest_hit)]
#[allow(unused_variables)]
pub fn rchit(
    #[spirv(incoming_ray_payload)] payload: &mut RayPayload,
    #[spirv(hit_attribute)] hit_co: &mut Vec2,
) {
}

#[spirv(miss)]
pub fn rmiss(#[spirv(incoming_ray_payload)] payload: &mut RayPayload) {}

#[spirv(miss)]
pub fn rmiss_shadow(#[spirv(incoming_ray_payload)] payload: &mut RayPayload) {}
