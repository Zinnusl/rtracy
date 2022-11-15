use crate::renderer::Renderer;
use crate::renderer::scene;
use crate::renderer::random_scene;
use crate::renderer::shape;

use std::time::Duration;
use crossterm::style::Stylize;
use crossterm::style;                                      
use rusty_pool::ThreadPool;
use std::sync::Arc;
use ray::RayHit;
use crossterm::style::Color;
use std::pin::Pin;

pub mod ray;

pub struct RayTracer
{
    scene: Box<dyn scene::Scene>,
    pool: Box<ThreadPool>,
}

impl RayTracer
{
    pub fn new() -> RayTracer
    {
        RayTracer {
            scene: Box::new(random_scene::RandomScene::new()),
            pool: Box::new(ThreadPool::new(16, 16, Duration::new(10, 0))),
        }
    }

    pub fn render_scene(&self)
    {
        return self.render(self.scene.as_ref());
    }

    fn trace_ray(ray: &ray::Ray, shapes: Vec<Arc<dyn shape::Shape>>) -> RayHit
    {
        // shapes.iter().map(|shape| {
        //     shape.intersects(ray)
        // }).find(|hit| {
        //     hit.0 > 0.0
        // }).unwrap_or((0.0, crossterm::style::Color::Black))

        let len = shapes.len() as f32;
        let res = shapes.iter().map(|shape| {
            shape.intersects(ray)
        }).fold((0.0, (0.0, 0.0, 0.0)), |cum, hit_tup| {
            let mut hit = cum.0;
            if hit_tup.0 > 0.0 {
                hit = hit_tup.0;
            }
            match hit_tup.1 {
                Color::Rgb{r, g, b} => {
                    (hit, (cum.1.0 + r as f32, cum.1.1 + g as f32, cum.1.2 + b as f32))
                },
                _ => {
                    (hit, cum.1)
                }
            }
        });
        (res.0, Color::Rgb{r: res.1.0.min(255.0) as u8, g: res.1.1.min(255.0) as u8, b: res.1.2.min(255.0) as u8})
    }

    fn render_task(shapes: Vec<Arc<dyn shape::Shape>>, xy: (f32, f32)) -> RayHit {
        RayTracer::trace_ray(&ray::Ray {
            origin: (xy.0, xy.1, 0.0),
            direction: (xy.0, xy.1, 1.0)
        }, shapes)
    }
}

impl Renderer for RayTracer
{
    fn render(&self, scene: &dyn scene::Scene)
    {
        let (width, height) = crossterm::terminal::size().unwrap();

        let mut futures: Vec<Pin<Box<dyn std::future::Future<Output = RayHit>>>> = Vec::new();
        // let mut futures: Vec<rusty_pool::JoinHandle<RayHit>> = Vec::new();
        let shapes: Vec<Arc<dyn shape::Shape>> = scene.get_shapes().to_vec();

        for y in 0..height {
            for x in 0..width {
                let copy = shapes.to_vec();
                futures.push(Box::pin(async move {
                    RayTracer::render_task(copy, (x as f32, y as f32))
                }));
                // futures.push(self.pool.complete(async move {
                //     RayTracer::render_task(copy, (x as f32, y as f32))
                // }))
            }
        }

        // let result = futures.into_iter().map(|f| {
        //     f.await_complete()
        // }).collect::<Vec<RayHit>>();
        let result = futures::executor::block_on(futures::future::join_all(futures));

        result.iter().for_each(|&hit| {
            let (amount, colour) = hit;
            let out_str = match amount {
                x if x == std::f32::INFINITY => " ",
                x if x < 0.0001 => " ",
                x if x < 0.5 => "ʿ",
                x if x < 0.6 => "Ϟ",
                x if x < 0.7 => "‰",
                x if x < 0.8 => "░",
                x if x < 0.9 => "▒",
                _ => "▓",
            };

            print!("{}", style::style(out_str).with(colour));
        });
    }
}
