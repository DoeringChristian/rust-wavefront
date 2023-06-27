use std::sync::Arc;

use crate::pipelines::CPipeline;
use crate::render::Sampler;
use screen_13::prelude::*;

pub struct IndependentSampler {
    next_1d: CPipeline,
    next_2d: CPipeline,
    seed: CPipeline,
    state: Option<Arc<Lease<Buffer>>>,
    wavefront_size: usize,
    // device: Arc<Device>,
}

impl IndependentSampler {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            next_1d: CPipeline::new(device, "sampler::independent::next_1d"),
            next_2d: CPipeline::new(device, "sampler::independent::next_2d"),
            seed: CPipeline::new(device, "sampler::independent::seed"),
            state: None,
            wavefront_size: 0,
            // device: device.clone(),
        }
    }
}

impl Sampler for IndependentSampler {
    fn next_1d(
        &self,
        graph: &mut screen_13::prelude::RenderGraph,
        cache: &mut screen_13::prelude::HashPool,
    ) -> screen_13::prelude::AnyBufferNode {
        let wavefront_size = self.wavefront_size;

        let sample = cache
            .lease(BufferInfo::new(
                (wavefront_size * std::mem::size_of::<f32>()) as _,
                vk::BufferUsageFlags::STORAGE_BUFFER,
            ))
            .unwrap();

        let sample = graph.bind_node(sample);
        let state = graph.bind_node(self.state.as_ref().unwrap());

        graph
            .begin_pass("IndependentSampler::next_1d")
            .bind_pipeline(self.next_1d.ppl())
            .write_descriptor((0, 0), state)
            .write_descriptor((0, 1), sample)
            .record_compute(move |comp, _| {
                comp.dispatch(wavefront_size as _, 1, 1);
            });

        AnyBufferNode::from(sample)
    }

    fn next_2d(
        &self,
        graph: &mut screen_13::prelude::RenderGraph,
        cache: &mut screen_13::prelude::HashPool,
    ) -> screen_13::prelude::AnyBufferNode {
        let wavefront_size = self.wavefront_size;

        let sample = cache
            .lease(BufferInfo::new(
                (wavefront_size * std::mem::size_of::<common::Vec2>()) as _,
                vk::BufferUsageFlags::STORAGE_BUFFER,
            ))
            .unwrap();

        let sample = graph.bind_node(sample);
        let state = graph.bind_node(self.state.as_ref().unwrap());

        graph
            .begin_pass("IndependentSampler::next_2d")
            .bind_pipeline(self.next_2d.ppl())
            .write_descriptor((0, 0), state)
            .write_descriptor((0, 1), sample)
            .record_compute(move |comp, _| {
                comp.dispatch(wavefront_size as _, 1, 1);
            });

        AnyBufferNode::from(sample)
    }

    fn seed(
        &mut self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        seed: u64,
        wavefront_size: usize,
    ) {
        self.wavefront_size = wavefront_size;
        self.state = Some(Arc::new(
            cache
                .lease(BufferInfo::new(
                    (wavefront_size
                        * std::mem::size_of::<common::sampler::independent::IndependentSampler>())
                        as _,
                    vk::BufferUsageFlags::STORAGE_BUFFER,
                ))
                .unwrap(),
        ));

        let state = graph.bind_node(self.state.as_ref().unwrap().clone());
        graph
            .begin_pass("IndependentSampler::seed")
            .bind_pipeline(self.seed.ppl())
            .write_descriptor((0, 0), state)
            .record_compute(move |comp, _| {
                comp.push_constants(bytemuck::cast_slice(&[seed]));
                comp.dispatch(wavefront_size as _, 1, 1);
            });
    }
}
