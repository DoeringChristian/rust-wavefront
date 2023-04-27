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
    // intersect_closest_ppl: RTPipeline,
    device: Arc<Device>,
}

impl WavefrontPathIntegrator {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            generate_camera_rays_ppl: CPipeline::new(device, "generate_camera_rays"),
            // intersect_closest_ppl: RTPipeline::new(device, "intersect_closest"),
            device: device.clone(),
        }
    }
    pub fn generate_camera_rays(
        &self,
        scene: SceneBinding,
        graph: &mut RenderGraph,
        rays: &ItemWorkQueue<Ray3f>,
        size: UVec2,
    ) {
        let rays = graph.bind_node(rays.buf());
        // let counter_node = graph.bind_node(rays.counter.buf());

        let pc = GenerateCameraRaysPc { camera: 0 };

        let pass = graph
            .begin_pass("Generate Camera Rays Pass")
            .bind_pipeline(&*self.generate_camera_rays_ppl)
            .write_descriptor((0, 0), scene.cameras)
            .write_descriptor((0, 1), rays)
            .record_compute(move |comp, _| {
                // comp.push_constants(bytemuck::cast_slice(&[pc]));
                comp.dispatch(size.x, size.y, 1);
            });
        pass.submit_pass();
    }
    pub fn intersect_closest(
        &self,
        scene: &SceneBinding,
        rays: &WorkQueue<Ray3f>,
        surface_interactions: &ItemWorkQueue<SurfaceInteraction>,
    ) {
        // let pass = graph.begin_pass("Intersect Closest Pass");
    }
    pub fn render(&self, scene: &mut Scene, size: UVec2) {
        let mut graph = RenderGraph::new();
        let mut cache = HashPool::new(&self.device);

        scene.update(&self.device, &mut cache, &mut graph);

        // dbg!(&scene.cameras);
        let scene = scene.bind(&mut graph);

        let rays = ItemWorkQueue::new(&self.device, (size.x * size.y) as _);
        // println!("{}", rays.len());
        self.generate_camera_rays(scene, &mut graph, &rays, size);

        graph.resolve().submit(&mut cache, 0).unwrap();

        unsafe { self.device.device_wait_idle().unwrap() };

        // println!("{}", rays.len());

        dbg!(rays.items());
    }
}
