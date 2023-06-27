use crate::pipelines::CPipeline;
use crate::render::BSDF;
use screen_13::prelude::*;

pub struct DiffuseBsdf {
    sample: CPipeline,
    eval: CPipeline,
    // wavefront_size: usize,
}

impl BSDF for DiffuseBsdf {
    fn sample(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        si: &AnyBufferNode,
        sample1: &AnyBufferNode,
        sample2: &AnyBufferNode,
    ) -> (AnyBufferNode, AnyBufferNode) {
        todo!()
    }

    fn eval(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        si: &AnyBufferNode,
        wo: &AnyBufferNode,
    ) -> AnyBufferNode {
        todo!()
    }
}
