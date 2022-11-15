use crate::raytracer::ray::Ray;
use crate::raytracer::ray::RayHit;
use crate::renderer::shape::Shape;
use crossterm::style::Color;

const CONSOLE_CHAR_RATIO: f32 = 0.25;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Sphere
{
    pub center: (f32, f32, f32),
    pub radius: f32,
    pub color: (u8, u8, u8),
}

// fn dot(a: (f32, f32, f32), b: (f32, f32, f32)) -> f32
// {
//     a.0 * b.0 + a.1 * b.1 + a.2 * b.2
// }

impl Shape for Sphere
{
    fn intersects(&self, ray: &Ray) -> RayHit
    {
        let dx = (ray.origin.0 - self.center.0).abs();
        let dy = (ray.origin.1 - self.center.1).abs();
        let sqr_dist = (dx * dx * CONSOLE_CHAR_RATIO + dy * dy).sqrt();

        if sqr_dist < self.radius {
            let falloff = 1.0 - 1.0/(sqr_dist - self.radius).abs();
            return (falloff,
                Color::Rgb{r: (255.0 - sqr_dist.abs() * self.color.0 as f32) as u8, g: (255.0 - sqr_dist.abs() * self.color.1 as f32) as u8, b: (255.0 - sqr_dist.abs() * self.color.2 as f32) as u8}
            );
        }

        (0.0, Color::Black)
    }
}
