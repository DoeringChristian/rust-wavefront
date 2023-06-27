use std::sync::Arc;

use crate::pipelines::CPipeline;
use crate::render::Film;
use screen_13::prelude::*;

pub struct HdrFilm {
    put: CPipeline,
    data: Option<Arc<Lease<Image>>>,
    width: usize,
    height: usize,
}

impl HdrFilm {
    pub fn new(device: &Arc<Device>, width: usize, height: usize) -> Self {
        Self {
            put: CPipeline::new(device, "film::hdrfilm::put"),
            data: None,
            width,
            height,
        }
    }
}

impl Film for HdrFilm {
    fn put(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        wavefront_size: usize,
        pos: &AnyBufferNode,
        value: &AnyBufferNode,
    ) {
        let data = graph.bind_node(self.data.as_ref().unwrap());
        graph
            .begin_pass("film::hdrfilm::put")
            .bind_pipeline(self.put.ppl())
            .read_descriptor((0, 0), value.clone())
            .read_descriptor((0, 1), pos.clone())
            .write_descriptor((0, 2), data)
            .record_compute(move |comp, _| {
                comp.dispatch(wavefront_size as _, 1, 1);
            });
    }

    fn prepare(&mut self, graph: &mut RenderGraph, cache: &mut HashPool) {
        self.data = Some(Arc::new(
            cache
                .lease(ImageInfo::new_2d(
                    vk::Format::R32G32B32A32_SFLOAT,
                    self.width as _,
                    self.height as _,
                    vk::ImageUsageFlags::STORAGE | vk::ImageUsageFlags::SAMPLED,
                ))
                .unwrap(),
        ));
    }

    fn develop(&mut self, graph: &mut RenderGraph, cache: &mut HashPool) -> AnyImageNode {
        AnyImageNode::from(graph.bind_node(self.data.as_ref().unwrap()))
    }
}
