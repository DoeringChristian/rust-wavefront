#![no_std]

use common::workqueue::WorkQueue;
use common::*;
use spirv_std::arch::atomic_i_add;
use spirv_std::glam::*;
use spirv_std::ray_tracing::AccelerationStructure;
use spirv_std::*;

#[repr(C)]
pub struct RayPayload {
    p: Vec3,
}

#[spirv(compute(threads(64)))]
pub fn generate_camera_rays(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] cameras: &[Camera],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] rays: &mut WorkQueue<WorkItem<Ray3f>>,
) {
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    let idx = size.x * pos.y + pos.x;
    let wavefront_size = size.x * size.y;

    let sample_pos = pos.as_vec3().xy() / size.as_vec3().xy();

    // cameras[0].near_clip = 0.5;
    let camera = cameras[0];

    let view2camera = camera.to_view().inverse();

    let near_p = (view2camera * sample_pos.extend(0.).extend(1.)).xyz();

    let o = Vec4::from(camera.to_world[3]).xyz();
    let d = near_p.normalize();

    let near_t = camera.near_clip / -d.z;
    let far_t = camera.far_clip / -d.z;

    let d = -(camera.to_world() * d.extend(0.)).xyz().normalize();

    let ray = Ray3f {
        o: o.extend(1.),
        d: d.extend(1.),
        tmin: 0.001,
        tmax: 10000.,
        t: 0.,
    };
    rays.set(WorkItem { item: ray, idx }, idx, wavefront_size);

    // rays.push(WorkItem { item: ray, idx });
}

#[spirv(ray_generation)]
pub fn intersect_closest(
    #[spirv(launch_id)] pos: UVec3,
    #[spirv(launch_size)] size: UVec3,
    #[spirv(uniform_constant, descriptor_set = 1, binding = 0)] accel: &AccelerationStructure,
    #[spirv(storage_buffer, descriptor_set = 1, binding = 1)] ray_items: &[WorkItem<Ray3f>],
    #[spirv(storage_buffer, descriptor_set = 1, binding = 2)] surface_interactions: &mut WorkQueue<
        Ray3f,
    >,
) {
}
//
// #[spirv(ray_generation)]
// pub fn path_trace(
//     #[spirv(launch_id)] pos: UVec3,
//     #[spirv(launch_size)] size: UVec3,
//     // #[spirv(push_constant)] push_constant: &PathTracePushConstant,
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] indices: &[u32],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] positions: &[Vec3],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] normals: &[Vec3],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] uvs: &[Vec2],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 4)] instances: &[Instance],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 5)] meshes: &[Mesh],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 6)] emitters: &[Emitter],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 7)] materials: &[Material],
//     #[spirv(storage_buffer, descriptor_set = 0, binding = 8)] cammeras: &[Camera],
//     #[spirv(uniform_constant, descriptor_set = 0, binding = 10)] accel: &AccelerationStructure,
//     // #[spirv(uniform_constant, descriptor_set = 0, binding = 9)] textures: &RuntimeArray<
//     //     Image!(2D, format = rgba32f, sampled = false),
//     // >,
//     #[spirv(uniform_constant, descriptor_set = 1, binding = 0)] color: &Image!(
//         2D,
//         format = rgba32f,
//         sampled = false
//     ),
//     #[spirv(uniform_constant, descriptor_set = 1, binding = 1)] normal: &Image!(
//         2D,
//         format = rgba32f,
//         sampled = false
//     ),
//     #[spirv(uniform_constant, descriptor_set = 1, binding = 2)] position: &Image!(
//         2D,
//         format = rgba32f,
//         sampled = false
//     ),
// ) {
//     let idx = size.x * pos.y + pos.x;
//     unsafe { color.write(pos.xy().as_ivec2().as_uvec2(), vec4(1., 0., 0., 0.)) };
// }
//
#[spirv(closest_hit)]
#[allow(unused_variables)]
pub fn rchit(
    #[spirv(incoming_ray_payload)] payload: &mut RayPayload,
    #[spirv(hit_attribute)] hit_co: &mut Vec2,
) {
}
//
#[spirv(miss)]
pub fn rmiss(#[spirv(incoming_ray_payload)] payload: &mut RayPayload) {}
//
#[spirv(miss)]
pub fn rmiss_shadow(#[spirv(incoming_ray_payload)] payload: &mut RayPayload) {}
