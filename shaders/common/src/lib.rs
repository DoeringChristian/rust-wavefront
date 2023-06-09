#![cfg_attr(target_arch = "spirv", no_std, feature(asm_experimental_arch,))]

mod workitems;
pub mod workqueue;
pub use workitems::*;

use bytemuck::*;
use spirv_std::glam::*;

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Mesh {
    pub indices: u32,
    pub indices_count: u32,
    pub positions: u32,
    pub normals: u32,
    pub uvs: u32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Instance {
    pub to_world: Mat4,
    pub mesh: u32,
    pub material: u32,
    pub emitter: i32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Emitter {
    pub irradiance: Texture,
    pub instance: u32,
    pub ty: u32,
}

impl Emitter {
    const TY_NONE: u32 = 0;
    const TY_ENV: u32 = 1;
    const TY_AREA: u32 = 2;
    pub fn env(irradiance: Texture) -> Self {
        Self {
            irradiance,
            instance: 0,
            ty: Self::TY_ENV,
        }
    }
    pub fn area(irradiance: Texture, instance: u32) -> Self {
        Self {
            irradiance,
            instance,
            ty: Self::TY_AREA,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct Texture {
    pub val: Vec3,
    pub texture: u32,
    pub ty: u32,
}

impl Default for Texture {
    fn default() -> Self {
        Self {
            val: vec3(0., 0., 0.),
            texture: 0,
            ty: Self::TY_CONSTANT,
        }
    }
}

impl Texture {
    const TY_CONSTANT: u32 = 0;
    const TY_IMAGE: u32 = 1;
    pub fn constant(val: Vec3) -> Self {
        Self {
            ty: Self::TY_CONSTANT,
            val,
            texture: 0,
        }
    }
    pub fn image(texture: u32) -> Self {
        Self {
            ty: Self::TY_IMAGE,
            val: Vec3::ZERO,
            texture,
        }
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Default, Clone, Copy)]
#[repr(C)]
pub struct Material {
    pub normal: Texture,
    pub base_color: Texture,
    pub metallic_roughness: Texture,
    pub transmission: Texture,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Camera {
    pub to_world: [[f32; 4]; 4],
    pub to_view: [[f32; 4]; 4],
    pub near_clip: f32,
    pub far_clip: f32,
}

impl Camera {
    pub fn perspective(
        to_world: Mat4,
        fov_y: f32,
        aspect_ratio: f32,
        near_clip: f32,
        far_clip: f32,
    ) -> Self {
        let to_view = Mat4::perspective_lh(fov_y, aspect_ratio, near_clip, far_clip);
        let to_view = Mat4::from_translation(vec3(1., 1., 0.)) * to_view;
        let to_view = Mat4::from_scale(vec3(0.5, 0.5, 1.)) * to_view;
        #[cfg(not(target_arch = "spirv"))]
        {
            //println!("{:#?}", to_view);
        }
        Self {
            to_world: to_world.to_cols_array_2d(),
            to_view: to_view.to_cols_array_2d(),
            near_clip,
            far_clip,
            //size: glam::uvec2(width, height),
        }
    }
    pub fn to_world(&self) -> Mat4 {
        Mat4::from_cols_array_2d(&self.to_world)
    }
    pub fn to_view(&self) -> Mat4 {
        Mat4::from_cols_array_2d(&self.to_view)
    }
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C, align(16))]
pub struct Ray3f {
    pub o: Vec4,
    pub d: Vec4,
    pub tmin: f32,
    pub tmax: f32,
    pub t: f32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceInteraction {
    pub p: Vec4,
    pub dist: f32,
    pub t: f32,
    pub instance: u32,
    pub primitive: u32,
    pub material: u32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct LoopState {
    pub L: Vec4,
    pub f: Vec4,
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct GenerateCameraRaysPc {
    pub camera: u32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
struct PushConstant {
    pub camera: u32,
    pub max_depth: u32,
    pub rr_depth: u32,
    pub seed: u32,
}
