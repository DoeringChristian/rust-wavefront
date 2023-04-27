mod accel;
mod array;
mod integrator;
mod loaders;
mod pipelines;
mod renderer;
mod sbt;
mod scene;
mod workqueue;

use glam::*;
use screen_13::prelude::*;

use self::integrator::WavefrontPathIntegrator;
use self::loaders::Loader;
use self::renderer::PTRenderer;
use self::scene::Scene;

fn main() {
    pretty_env_logger::init();

    let entry_points = spirq::ReflectConfig::new()
        .spv(pipelines::SHADERS)
        // .ref_all_rscs(true)
        .reflect()
        .unwrap();

    // dbg!(&entry_points);
    for ep in entry_points {
        if ep.name == "generate_camera_rays" {
            dbg!(ep.vars);
        }
    }
    // dbg!(env!("rust_shaders.spv"));

    //     let sc13 = EventLoop::new()
    //         .debug(true)
    //         .ray_tracing(true)
    //         .build()
    //         .unwrap();
    //     let device = &sc13.device;
    //     let mut cache = HashPool::new(device);
    //
    //     let integrator = WavefrontPathIntegrator::new(device);
    //
    //     let mut scene = Scene::default();
    //     let loader = loaders::GltfLoader::default();
    //     loader.append("assets/cornell-box.gltf", &mut scene);
    //
    //     let mut graph = RenderGraph::new();
    //
    //     // scene.update(device, &mut cache, &mut graph);
    //
    //     integrator.render(&mut scene, uvec2(4, 4));
    //
    //     graph.resolve();
    //     unsafe { device.device_wait_idle().unwrap() };
}
