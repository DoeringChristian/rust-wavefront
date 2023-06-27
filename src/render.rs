use screen_13::prelude::*;

// use crate::array::Array;
use crate::scene::SceneBinding;

pub trait Sampler {
    fn next_1d(&self, graph: &mut RenderGraph, cache: &mut HashPool) -> AnyBufferNode;
    fn next_2d(&self, graph: &mut RenderGraph, cache: &mut HashPool) -> AnyBufferNode;
    fn seed(
        &mut self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        seed: u64,
        wavefront_size: usize,
    );
    // fn seed(&mut self, seed: u64, wavefront_size: usize);
}

pub trait Sensor {
    fn sample_ray(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        sample1: &AnyBufferNode,
        sample2: &AnyBufferNode,
        sample3: &AnyBufferNode,
    ) -> (AnyBufferNode, AnyBufferNode);
    fn film(&self) -> &dyn Film;
    fn sampler(&self) -> &dyn Sampler;
}

pub trait Film {
    fn put(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        pos: &AnyBufferNode,
        value: &AnyBufferNode,
    );
}

pub trait BSDF {
    fn sample(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        si: &AnyBufferNode,
        sample1: &AnyBufferNode,
        sample2: &AnyBufferNode,
    ) -> (AnyBufferNode, AnyBufferNode);
    fn eval(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        si: &AnyBufferNode,
        wo: &AnyBufferNode,
    ) -> AnyBufferNode;
}

pub trait Integrator {
    fn render(
        &self,
        graph: &mut RenderGraph,
        cache: &mut HashPool,
        scene: &SceneBinding,
        sensor: &dyn Sensor,
        seed: u32,
        spp: u32,
    ) -> AnyImageNode;
}
