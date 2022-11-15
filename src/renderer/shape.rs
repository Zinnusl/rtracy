use crate::raytracer::ray::Ray;
use crate::raytracer::ray::RayHit;

pub trait Shape: Send + Sync
{
    fn intersects(&self, ray: &Ray) -> RayHit;
}

// dyn_clone::clone_trait_object!(Shape);
