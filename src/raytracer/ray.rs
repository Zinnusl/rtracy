#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Ray
{
    pub origin: (f32, f32, f32),
    pub direction: (f32, f32, f32),
}

pub type RayHit = (f32, crossterm::style::Color);

