use screen_13::prelude::*;

use crate::sbt::{SbtBuffer, SbtBufferInfo};
use std::ops::Deref;
use std::sync::Arc;

pub const SHADERS: &[u8] = include_bytes!(env!("rust_shaders.spv"));

pub struct RTPipeline {
    pub sbt: SbtBuffer,
    pub ppl: Arc<RayTracePipeline>,
}

impl RTPipeline {
    pub fn new(device: &Arc<Device>, rgen: &str) -> Self {
        let ppl = Arc::new(
            RayTracePipeline::create(
                device,
                RayTracePipelineInfo::new()
                    .max_ray_recursion_depth(2)
                    .build(),
                [
                    Shader::new_ray_gen(SHADERS).entry_name(rgen.into()),
                    Shader::new_closest_hit(SHADERS).entry_name("rchit".into()),
                    Shader::new_miss(SHADERS).entry_name("rmiss".into()),
                    Shader::new_miss(SHADERS).entry_name("rmiss_shadow".into()),
                ],
                [
                    RayTraceShaderGroup::new_general(0),
                    RayTraceShaderGroup::new_triangles(1, None),
                    RayTraceShaderGroup::new_general(2),
                    RayTraceShaderGroup::new_general(3),
                ],
            )
            .unwrap(),
        );
        let sbt_info = SbtBufferInfo {
            rgen_index: 0,
            hit_indices: &[1],
            miss_indices: &[2, 3],
            callable_indices: &[],
        };
        let sbt = SbtBuffer::create(device, sbt_info, &ppl).unwrap();
        Self { sbt, ppl }
    }
}

pub struct CPipeline(Arc<ComputePipeline>);
impl CPipeline {
    pub fn new(device: &Arc<Device>, fname: &str) -> Self {
        Self(Arc::new(
            ComputePipeline::create(
                device,
                ComputePipelineInfo::default(),
                Shader::new_compute(SHADERS).entry_name(fname.into()),
            )
            .unwrap(),
        ))
    }
}
impl Deref for CPipeline {
    type Target = Arc<ComputePipeline>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
