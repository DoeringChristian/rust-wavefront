use core::ops::Index;

use spirv_std::arch::{atomic_i_add, atomic_i_increment};
use spirv_std::{glam::*, RuntimeArray};

use crate::{Ray3f, SurfaceInteraction};

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WorkItem<T: Copy> {
    pub item: T,
    pub idx: UVec2, // Position of the original pixel
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct PixelSampleState {
    pub pixel: UVec2,
    pub radiance: Vec4,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct RayWorkItem {
    pub ray: Ray3f,
    pub beta: Vec4,
    pub pixel_idx: u32,
}

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct MaterialEvalWorkItem {
    pub si: SurfaceInteraction,
    pub pixel_idx: u32,
}
