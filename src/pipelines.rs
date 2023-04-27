use screen_13::prelude::*;

use crate::sbt::{SbtBuffer, SbtBufferInfo};
use std::collections::HashMap;
use std::ops::Deref;
use std::path::Path;
use std::sync::Arc;

use serde::Deserialize;

#[derive(Deserialize)]
pub struct CompileResult {
    entry_to_module: HashMap<String, String>,
}

const SPV_DIR: &str = "./assets/spv";
static SHADERS: once_cell::sync::Lazy<CompileResult> = once_cell::sync::Lazy::new(|| {
    serde_json::from_str(
        std::fs::read_to_string("./assets/spv/shaders.json")
            .unwrap()
            .as_str(),
    )
    .unwrap()
});

fn load_spv(entry_name: &str) -> Vec<u8> {
    let path = Path::new(SPV_DIR).join(&SHADERS.entry_to_module[entry_name]);
    let spv = std::fs::read(path).unwrap();
    spv
}

pub struct RTPipeline {
    pub sbt: SbtBuffer,
    pub ppl: Arc<RayTracePipeline>,
}

impl RTPipeline {
    pub fn new(device: &Arc<Device>, rgen: &str, rchit: &str, rmiss: &str) -> Self {
        let ppl = Arc::new(
            RayTracePipeline::create(
                device,
                RayTracePipelineInfo::new()
                    .max_ray_recursion_depth(2)
                    .build(),
                [
                    Shader::new_ray_gen(load_spv(rgen)).entry_name(rgen.into()),
                    Shader::new_closest_hit(load_spv("rchit")).entry_name("rchit".into()),
                    Shader::new_miss(load_spv("rmiss")).entry_name("rmiss".into()),
                    // Shader::new_miss(load_spv("rmiss_shadow")).entry_name("rmiss_shadow".into()),
                ],
                [
                    RayTraceShaderGroup::new_general(0),
                    RayTraceShaderGroup::new_triangles(1, None),
                    RayTraceShaderGroup::new_general(2),
                    // RayTraceShaderGroup::new_general(3),
                ],
            )
            .unwrap(),
        );
        let sbt_info = SbtBufferInfo {
            rgen_index: 0,
            hit_indices: &[1],
            miss_indices: &[2],
            callable_indices: &[],
        };
        let sbt = SbtBuffer::create(device, sbt_info, &ppl).unwrap();
        Self { sbt, ppl }
    }
    pub fn ppl(&self) -> &Arc<RayTracePipeline> {
        &self.ppl
    }
}

pub struct CPipeline(Arc<ComputePipeline>);
impl CPipeline {
    pub fn new(device: &Arc<Device>, fname: &str) -> Self {
        Self(Arc::new(
            ComputePipeline::create(
                device,
                ComputePipelineInfo::default(),
                Shader::new_compute(load_spv(fname)).entry_name(fname.into()),
            )
            .unwrap(),
        ))
    }
    pub fn ppl(&self) -> &Arc<ComputePipeline> {
        &self.0
    }
}
impl Deref for CPipeline {
    type Target = Arc<ComputePipeline>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
