mod accel;
mod array;
mod integrators;
mod loaders;
mod pipelines;
mod render;
mod samplers;
// mod renderer;
mod sbt;
mod scene;
// mod workqueue;

use glam::*;
use screen_13::prelude::*;

use self::loaders::Loader;
use self::scene::Scene;

fn main() {
    pretty_env_logger::init();

    let mut scene = Scene::default();
    let loader = loaders::GltfLoader::default();
    loader.append("assets/cornell-box.gltf", &mut scene);
}
