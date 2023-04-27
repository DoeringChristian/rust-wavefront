use core::ops::Index;

use spirv_std::arch::{atomic_i_add, atomic_i_increment};
use spirv_std::{glam::*, RuntimeArray};

#[cfg_attr(not(target_arch = "spirv"), derive(Debug))]
#[derive(Clone, Copy)]
#[repr(C)]
pub struct WorkItem<T: Copy> {
    pub item: T,
    pub idx: u32,
}

#[derive(Clone, Copy)]
#[repr(C)]
pub struct SurfaceInteraction {
    pub p: Vec3,
    pub dist: f32,
    pub t: f32,
}
