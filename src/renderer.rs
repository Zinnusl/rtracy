pub mod scene;
pub mod simple_scene;
pub mod random_scene;
pub mod sphere;
pub mod shape;

pub trait Renderer {
    fn render(&self, scene: &dyn scene::Scene);
}
