#![no_std]

use common::workqueue::WorkQueue;
use common::*;
use spirv_std::arch::atomic_i_add;
use spirv_std::glam::*;
use spirv_std::ray_tracing::{AccelerationStructure, RayFlags};
use spirv_std::*;

#[derive(Default)]
#[repr(C)]
pub struct RayPayload {
    valid: u32,
    uv: Vec2,
    instance: u32,
    primitive: u32,
    dist: f32,
}

#[spirv(compute(threads(64)))]
pub fn generate_camera_rays(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] cameras: &[Camera],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] rays: &mut WorkQueue<RayWorkItem>,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)]
    pixel_sample_states: &mut [PixelSampleState],
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
    rays.set(
        RayWorkItem {
            ray,
            throughput: vec4(1., 1., 1., 1.),
            pixel_idx: idx,
        },
        idx,
        wavefront_size,
    );
    pixel_sample_states[idx as usize] = PixelSampleState {
        pixel: pos.xy(),
        radiance: vec4(1., 0., 0., 0.),
    };
}

#[spirv(compute(threads(64)))]
pub fn update_film(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)]
    pixel_sample_states: &[PixelSampleState],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 1)] image: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
) {
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    let idx = size.x * pos.y + pos.x;
    let wavefront_size = size.x * size.y;

    let PixelSampleState { pixel, radiance } = pixel_sample_states[idx as usize];

    unsafe { image.write(pixel, radiance.xyz().extend(1.)) };
}

#[spirv(ray_generation)]
pub fn intersect_closest(
    #[spirv(ray_payload)] payload: &mut RayPayload,
    #[spirv(launch_id)] pos: UVec3,
    #[spirv(launch_size)] size: UVec3,
    #[spirv(uniform_constant, descriptor_set = 0, binding = 0)] accel: &AccelerationStructure,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] rays: &WorkQueue<RayWorkItem>,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] material_eval_queue: &mut WorkQueue<
        MaterialEvalWorkItem,
    >,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] instances: &[Instance],
) {
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);
    // let WorkItem { item: ray, idx } = *rays.item(pos.x);
    let RayWorkItem {
        ray,
        throughput,
        pixel_idx,
    } = *rays.item(pos.x);

    *payload = RayPayload::default();

    unsafe {
        accel.trace_ray(
            RayFlags::OPAQUE,
            0xff,
            0,
            0,
            0,
            ray.o.xyz(),
            ray.tmin,
            ray.d.xyz(),
            ray.tmax,
            payload,
        )
    };

    if payload.valid != 0 {
        let material = instances[payload.instance as usize].material;
        material_eval_queue.push(MaterialEvalWorkItem {
            pixel_idx,
            si: SurfaceInteraction {
                p: (ray.o.xyz() + ray.d.xyz() * payload.dist).extend(1.),
                dist: payload.dist,
                t: ray.t,
                instance: payload.instance,
                primitive: payload.primitive,
                material,
            },
        });
    }
}
#[spirv(closest_hit)]
#[allow(unused_variables)]
pub fn rchit(
    #[spirv(incoming_ray_payload)] payload: &mut RayPayload,
    #[spirv(hit_attribute)] hit_co: &mut Vec2,
    #[spirv(instance_id)] instance: u32,
    #[spirv(primitive_id)] primitive: u32,
    #[spirv(ray_tmax)] dist: f32,
) {
    payload.uv = *hit_co;
    payload.valid = 1;
    payload.instance = instance;
    payload.primitive = primitive;
    payload.dist = dist;
}
//
#[spirv(miss)]
pub fn rmiss(#[spirv(incoming_ray_payload)] payload: &mut RayPayload) {}
//
#[spirv(miss)]
pub fn rmiss_shadow(#[spirv(incoming_ray_payload)] payload: &mut RayPayload) {}
