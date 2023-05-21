use common::*;
use glam::*;
use screen_13::prelude::*;
use std::sync::Arc;

use crate::array::Array;
use crate::pipelines::{CPipeline, RTPipeline};
use crate::scene::{Scene, SceneBinding};
use crate::workqueue::{ItemWorkQueue, WorkQueue};

fn buffer_size<T>(num: usize) -> u64 {
    (std::mem::size_of::<T>() * num) as _
}

pub struct WavefrontPathIntegrator {
    generate_camera_rays_ppl: CPipeline,
    put_film: CPipeline,
    update_state: CPipeline,
    intersect_closest_ppl: RTPipeline,
    device: Arc<Device>,
}

impl WavefrontPathIntegrator {
    pub fn new(device: &Arc<Device>) -> Self {
        Self {
            generate_camera_rays_ppl: CPipeline::new(device, "generate_camera_rays"),
            put_film: CPipeline::new(device, "put_film"),
            update_state: CPipeline::new(device, "update_state"),
            intersect_closest_ppl: RTPipeline::new(device, "intersect_closest", "rchit", "rmiss"),
            device: device.clone(),
        }
    }
    pub fn generate_camera_rays(
        &self,
        graph: &mut RenderGraph,
        scene: &SceneBinding,
        rays: AnyBufferNode,
        sample_pos: AnyBufferNode,
        size: UVec2,
    ) {
        let pc = GenerateCameraRaysPc { camera: 0 };

        let pass = graph
            .begin_pass("Generate Camera Rays Pass")
            .bind_pipeline(&*self.generate_camera_rays_ppl)
            .read_descriptor((0, 0), scene.cameras)
            .write_descriptor((0, 1), rays)
            .write_descriptor((0, 2), sample_pos)
            .record_compute(move |comp, _| {
                // comp.push_constants(bytemuck::cast_slice(&[pc]));
                comp.dispatch(size.x, size.y, 1);
            });
        pass.submit_pass();
    }
    pub fn update_state(
        &self,
        graph: &mut RenderGraph,
        L: AnyBufferNode,
        si: AnyBufferNode,
        wavefront_size: usize,
    ) {
        graph
            .begin_pass("Update State")
            .bind_pipeline(&*self.update_state)
            .read_descriptor((0, 0), L)
            .read_descriptor((0, 1), si)
            .record_compute(move |comp, _| {
                comp.dispatch(wavefront_size as _, 1, 1);
            })
            .submit_pass();
    }
    pub fn put_film(
        &self,
        graph: &mut RenderGraph,
        sample: AnyBufferNode,
        sample_pos: AnyBufferNode,
        img: AnyImageNode,
        wavefront_size: usize,
        image_size: UVec2,
    ) {
        let image_size = [image_size.x, image_size.y];
        graph
            .begin_pass("Put Film")
            .bind_pipeline(&*self.put_film)
            .read_descriptor((0, 0), sample)
            .read_descriptor((0, 1), sample_pos)
            .write_descriptor((0, 2), img)
            .record_compute(move |comp, _| {
                comp.push_constants(bytemuck::cast_slice(&image_size));
                comp.dispatch(wavefront_size as _, 1, 1);
            })
            .submit_pass();
    }
    pub fn intersect_closest(
        &self,
        graph: &mut RenderGraph,
        scene: &SceneBinding,
        rays: AnyBufferNode,
        si: AnyBufferNode,
        wavefront_size: usize,
    ) {
        let sbt_rgen = self.intersect_closest_ppl.sbt.rgen();
        let sbt_miss = self.intersect_closest_ppl.sbt.miss();
        let sbt_hit = self.intersect_closest_ppl.sbt.hit();
        let sbt_callable = self.intersect_closest_ppl.sbt.callable();

        let pass = graph
            .begin_pass("Intersect Closest Pass")
            .bind_pipeline(self.intersect_closest_ppl.ppl())
            .read_descriptor((0, 0), scene.accel)
            .read_descriptor((0, 1), rays)
            .write_descriptor((0, 2), si)
            .read_descriptor((0, 3), scene.instances)
            .record_ray_trace(move |rt, _| {
                rt.trace_rays(
                    &sbt_rgen,
                    &sbt_miss,
                    &sbt_hit,
                    &sbt_callable,
                    wavefront_size as _,
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

        let rays = Array::<Ray3f>::empty(&self.device, wavefront_size);
        let rays_node = graph.bind_node(rays.buf());

        let sample_pos = Array::<Vec2>::empty(&self.device, wavefront_size);
        let sample_pos_node = graph.bind_node(sample_pos.buf());

        let si = Array::<SurfaceInteraction>::empty(&self.device, wavefront_size);
        let si_node = graph.bind_node(si.buf());

        let L = Array::<Vec4>::empty(&self.device, wavefront_size);
        let L_node = graph.bind_node(L.buf());

        let throughput = Array::<Vec4>::empty(&self.device, wavefront_size);
        let throughput_node = graph.bind_node(throughput.buf());

        let img = Image::create(
            &self.device,
            ImageInfo::new_2d(
                vk::Format::R32G32B32A32_SFLOAT,
                size.x,
                size.y,
                vk::ImageUsageFlags::STORAGE
                    | vk::ImageUsageFlags::TRANSFER_DST
                    | vk::ImageUsageFlags::TRANSFER_SRC,
            ),
        )
        .unwrap();
        let img = Arc::new(img);
        let img_node = graph.bind_node(&img);
        let img_buf = Array::<[f32; 4]>::empty(&self.device, wavefront_size);
        let img_buf_node = graph.bind_node(img_buf.buf());

        self.generate_camera_rays(
            &mut graph,
            &scene_bindings,
            rays_node.into(),
            sample_pos_node.into(),
            size,
        );

        self.intersect_closest(
            &mut graph,
            &scene_bindings,
            rays_node.into(),
            si_node.into(),
            wavefront_size,
        );

        self.update_state(&mut graph, L_node.into(), si_node.into(), wavefront_size);

        self.put_film(
            &mut graph,
            L_node.into(),
            sample_pos_node.into(),
            img_node.into(),
            wavefront_size,
            size,
        );

        graph.copy_image_to_buffer(img_node, img_buf_node);

        graph.resolve().submit(&mut cache, 0).unwrap();
        unsafe { self.device.device_wait_idle().unwrap() };

        println!("{:#?}", rays.map());
        println!("{:#?}", si.map());
        println!("{:#?}", L.map());

        image::save_buffer(
            "out/img.exr",
            img_buf.map_u8(),
            size.x,
            size.y,
            image::ColorType::Rgba32F,
        )
        .unwrap();

        // rays.unbind(&mut graph).info

        // let img_node = graph.bind_node(img);
        // let img_buf_node = graph.bind_node(img_buf.buf());
        // graph.copy_image_to_buffer(img_node, img_buf_node);
        //
        // graph.resolve().submit(&mut cache, 0).unwrap();
        // unsafe { self.device.device_wait_idle().unwrap() };
        // let mut graph = RenderGraph::new();
        // let scene_bindings = scene.bind(&mut graph);
        //
        // image::save_buffer(
        //     "out/img.exr",
        //     img_buf.map_u8(),
        //     size.x,
        //     size.y,
        //     image::ColorType::Rgba32F,
        // )
        // .unwrap();
        //
        // dbg!(material_eval_queue.items());
        // dbg!(material_eval_queue.len());
    }
}
