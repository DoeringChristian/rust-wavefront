mod accel;
mod array;
mod bsdf;
mod film;
mod integrators;
mod loaders;
mod pipelines;
mod render;
mod sampler;
// mod renderer;
mod sbt;
mod scene;
// mod workqueue;

use screen_13::prelude::*;

use self::film::hdrfilm::HdrFilm;
use self::loaders::Loader;
use self::render::Film;
use self::scene::Scene;

fn main() {
    pretty_env_logger::init();

    let mut scene = Scene::default();
    let loader = loaders::GltfLoader::default();
    loader.append("assets/cornell-box.gltf", &mut scene);

    let sc13 = EventLoop::new()
        .ray_tracing(true)
        .debug(true)
        .build()
        .unwrap();
    let mut cache = HashPool::new(&sc13.device);

    let mut film = HdrFilm::new(&sc13.device, 100, 100);
    let presenter = screen_13_fx::prelude::GraphicPresenter::new(&sc13.device).unwrap();

    sc13.run(|frame| {
        film.prepare(frame.render_graph, &mut cache);
        let img = film.develop(frame.render_graph, &mut cache);
        presenter.present_image(frame.render_graph, img, frame.swapchain_image);
    })
    .unwrap();
}
