mod renderer;
mod raytracer;

fn main() {
    let renderer = raytracer::RayTracer::new();
    renderer.render_scene();
}
