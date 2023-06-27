use crate::render::Integrator;

pub struct PathIntegrator {}

impl Integrator for PathIntegrator {
    fn render(
        &self,
        graph: &mut screen_13::prelude::RenderGraph,
        cache: &mut screen_13::prelude::HashPool,
        scene: &crate::scene::SceneBinding,
        sensor: &dyn crate::render::Sensor,
        seed: u32,
        spp: u32,
    ) -> screen_13::prelude::AnyImageNode {
        todo!()
    }
}
