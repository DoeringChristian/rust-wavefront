#![no_std]

mod independent;

use common::sampler::IndependentSampler;
use common::*;
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
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] rays: &mut [Ray3f],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] _sample_pos: &mut [Vec2],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] _sampler: &mut [IndependentSampler],
) {
    let idx = (size.x * pos.y + pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    let wavefront_size = size.x * size.y;

    let mut sampler = IndependentSampler::new(0, idx as _);

    // let sample_pos = pos.as_vec3().xy() / size.as_vec3().xy();
    let sample_pos = (pos.as_vec3().xy() + sampler.next_2d()) / size.as_vec3().xy();
    // let sample_pos = sample_pos[idx];

    // cameras[0].near_clip = 0.5;
    let camera = cameras[0];

    let view2camera = camera.to_view().inverse();

    let near_p = (view2camera * sample_pos.extend(0.).extend(1.)).xyz();

    let o = Vec4::from(camera.to_world[3]).xyz();
    let d = near_p.normalize();

    let near_t = camera.near_clip / -d.z;
    let far_t = camera.far_clip / -d.z;

    let d = -(camera.to_world() * d.extend(0.)).xyz().normalize();

    rays[idx] = Ray3f {
        o: o.extend(1.),
        d: d.extend(1.),
        tmin: 0.001,
        tmax: 10000.,
        t: 0.,
    };
    _sample_pos[idx] = pos.xy().as_vec2();
    _sampler[idx] = sampler;
}

#[spirv(compute(threads(64)))]
pub fn put_film(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] sample: &[Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] sample_pos: &[Vec2],
    #[spirv(uniform_constant, descriptor_set = 0, binding = 2)] image: &Image!(
        2D,
        format = rgba32f,
        sampled = false
    ),
    // #[spirv(push_constant)] image_size: &[u32; 2],
) {
    let idx = (pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    // let wavefront_size = size.x * size.y;
    // let img_size: UVec2 = image.query_size();

    unsafe { image.write((sample_pos[idx]).as_uvec2(), sample[idx].xyz().extend(1.)) };
}

#[spirv(compute(threads(64)))]
pub fn smaple_bsdf(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
) {
    let idx = (size.x * pos.y + pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);
}

#[spirv(compute(threads(64)))]
pub fn update_state(
    #[spirv(global_invocation_id)] pos: glam::UVec3,
    #[spirv(num_workgroups)] size: glam::UVec3,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 0)] L: &mut [Vec4],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] si: &mut [SurfaceInteraction],
) {
    let idx = (pos.x) as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    L[idx] = vec4(si[idx].dist / 10., 0., 0., 1.);
}

#[spirv(ray_generation)]
pub fn intersect_closest(
    #[spirv(ray_payload)] payload: &mut RayPayload,
    #[spirv(launch_id)] pos: UVec3,
    #[spirv(launch_size)] size: UVec3,
    #[spirv(uniform_constant, descriptor_set = 0, binding = 0)] accel: &AccelerationStructure,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] rays: &[Ray3f],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] si: &mut [SurfaceInteraction],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 3)] instances: &[Instance],
) {
    let idx = pos.x as usize;
    assert!(pos.x < size.x);
    assert!(pos.y < size.y);
    assert!(pos.z < size.z);

    *payload = RayPayload::default();
    let ray = rays[idx];

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

    let instance = instances[payload.instance as usize];

    si[idx] = SurfaceInteraction {
        p: (ray.o.xyz() + ray.d.xyz() * payload.dist).extend(1.),
        dist: payload.dist,
        t: 1.,
        // t: ray.t,
        instance: payload.instance,
        primitive: payload.primitive,
        material: instance.material,
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
