use crate::renderer::scene::Scene;
use crate::renderer::sphere::Sphere;
use crate::renderer::shape::Shape;

use std::sync::Arc;

pub struct SimpleScene {
    shapes: Vec<Arc<dyn Shape>>
}

impl SimpleScene {
    pub fn new() -> SimpleScene {
        let shapes: Vec<Arc<dyn Shape>> = vec![
            Arc::new(Sphere {
                center: (10.0, 15.0, 6.0),
                radius: 3.5,
                color: (251u8, 0u8, 7u8)
            }),
            Arc::new(Sphere {
                center: (105.0, 20.5, 9.0),
                radius: 10.0,
                color: (0u8, 0u8, 7u8)
            }),
            Arc::new(Sphere {
                center: (245.0, 50.5, 2.0),
                radius: 30.0,
                color: (251u8, 55u8, 7u8)
            }),
            Arc::new(Sphere {
                center: (45.0, 50.5, 2.0),
                radius: 5.0,
                color: (21u8, 99u8, 78u8)
            }),
            Arc::new(Sphere {
                center: (545.0, 83.5, 2.0),
                radius: 5.0,
                color: (21u8, 99u8, 78u8)
            }),
        ];

        SimpleScene {
            shapes
        }
    }
}

impl Scene for SimpleScene {
    fn get_shapes(&self) -> &Vec<Arc<dyn Shape>> {
        &self.shapes
    }
}
