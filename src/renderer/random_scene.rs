use crate::renderer::scene::Scene;
use crate::renderer::sphere::Sphere;
use crate::renderer::shape::Shape;

use rand::Rng;

use std::sync::Arc;

pub struct RandomScene {
    shapes: Vec<Arc<dyn Shape>>
}

impl RandomScene {
    pub fn new() -> RandomScene {
        let mut shapes: Vec<Arc<dyn Shape>> = vec!();
        let mut rng = rand::thread_rng();

        let mut i = 0;
        let (num, _) = rng.gen::<(u8, bool)>();
        while i < num {
            shapes.push(Arc::new(Sphere {
                center: (rng.gen_range(10.0..500.0), rng.gen_range(10.0..500.0)/4.0, rng.gen_range(10.0..500.0)),
                radius: rng.gen_range(0.1..20.0),
                color: (rng.gen_range(0u8..255u8), rng.gen_range(0u8..255u8), rng.gen_range(0u8..255u8))
            }));
            i += 1;
        }

        RandomScene {
            shapes
        }
    }
}

impl Scene for RandomScene {
    fn get_shapes(&self) -> &Vec<Arc<dyn Shape>> {
        &self.shapes
    }
}
