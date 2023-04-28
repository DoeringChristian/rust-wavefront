use common::*;
use glam::*;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::array::Array;
use crate::pipelines::{CPipeline, RTPipeline};
use crate::scene::{Scene, SceneBinding};
use crate::workqueue::{ItemWorkQueue, WorkQueue};

pub struct WavefrontPathIntegrator {
    generate_camera_rays_ppl: CPipeline,
    intersect_closest_ppl: RTPipeline,
    device: Arc<Device>,
}

impl WavefrontPathIntegrator {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            generate_camera_rays_ppl: CPipeline::new(device, "generate_camera_rays"),
            intersect_closest_ppl: RTPipeline::new(device, "intersect_closest", "rchit", "rmiss"),
            device: device.clone(),
        }
    }
    pub fn generate_camera_rays(
        &self,
        scene: SceneBinding,
        graph: &mut RenderGraph,
        rays: &WorkQueue<RayWorkItem>,
        pixel_states: &Array<PixelSampleState>,
        size: UVec2,
    ) {
        let rays = graph.bind_node(rays.buf());
        let pixel_states = graph.bind_node(pixel_states.buf());
        // let counter_node = graph.bind_node(rays.counter.buf());

        let pc = GenerateCameraRaysPc { camera: 0 };

        let pass = graph
            .begin_pass("Generate Camera Rays Pass")
            .bind_pipeline(&*self.generate_camera_rays_ppl)
            .read_descriptor((0, 0), scene.cameras)
            .write_descriptor((0, 1), rays)
            .write_descriptor((0, 2), pixel_states)
            .record_compute(move |comp, _| {
                // comp.push_constants(bytemuck::cast_slice(&[pc]));
                comp.dispatch(size.x, size.y, 1);
            });
        pass.submit_pass();
    }
    pub fn intersect_closest(
        &self,
        scene: &SceneBinding,
        graph: &mut RenderGraph,
        rays: &WorkQueue<RayWorkItem>,
        surface_interactions: &WorkQueue<MaterialEvalWorkItem>,
    ) {
        let size = rays.len();
        let rays = graph.bind_node(rays.buf());
        let material_eval_queue = graph.bind_node(surface_interactions.buf());

        let sbt_rgen = self.intersect_closest_ppl.sbt.rgen();
        let sbt_miss = self.intersect_closest_ppl.sbt.miss();
        let sbt_hit = self.intersect_closest_ppl.sbt.hit();
        let sbt_callable = self.intersect_closest_ppl.sbt.callable();

        let pass = graph
            .begin_pass("Intersect Closest Pass")
            .bind_pipeline(self.intersect_closest_ppl.ppl())
            .read_descriptor((0, 0), scene.accel)
            .read_descriptor((0, 1), rays)
            .write_descriptor((0, 2), material_eval_queue)
            .record_ray_trace(move |rt, _| {
                rt.trace_rays(
                    &sbt_rgen,
                    &sbt_miss,
                    &sbt_hit,
                    &sbt_callable,
                    size as _,
                    1,
                    1,
                );
            });
        pass.submit_pass();
    }
    pub fn render(&self, scene: &mut Scene, size: UVec2) {
        let mut graph = RenderGraph::new();
        let mut cache = HashPool::new(&self.device);

        scene.update(&self.device, &mut cache, &mut graph);

        let wavefront_size = (size.x * size.y) as usize;

        let scene_bindings = scene.bind(&mut graph);
        let current = WorkQueue::new(&self.device, wavefront_size);
        // let next = WorkQueue::new(&self.device, wavefront_size);
        let pixel_states = Array::empty(&self.device, wavefront_size);
        let material_eval_queue = WorkQueue::new(&self.device, wavefront_size);

        self.generate_camera_rays(scene_bindings, &mut graph, &current, &pixel_states, size);

        graph.resolve().submit(&mut cache, 0).unwrap();
        unsafe { self.device.device_wait_idle().unwrap() };
        let mut graph = RenderGraph::new();
        let scene_bindings = scene.bind(&mut graph);

        self.intersect_closest(&scene_bindings, &mut graph, &current, &material_eval_queue);

        graph.resolve().submit(&mut cache, 0).unwrap();
        unsafe { self.device.device_wait_idle().unwrap() };
        // let mut graph = RenderGraph::new();
        // let scene_bindings = scene.bind(&mut graph);

        dbg!(material_eval_queue.items());
        dbg!(material_eval_queue.len());
    }
}
