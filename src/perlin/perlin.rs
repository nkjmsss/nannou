#![allow(dead_code)]
use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
use rand::{thread_rng, Rng};

#[derive(Debug, Copy, Clone)]
pub struct PerlinRect {
    perlin_gen: Perlin,
}

impl PerlinRect {
    pub fn new() -> Self {
        let perlin_gen = Perlin::new();
        PerlinRect { perlin_gen }
    }

    pub fn draw(&self, draw: &app::Draw, window_size: (u32, u32)) {
        let mesh_count = 500.0;
        let perlin_range = 12.0;
        let (window_width, window_height) = window_size;
        let w = (window_width as f32) / mesh_count / 2.0;
        let h = (window_height as f32) / mesh_count / 2.0;

        for x in (0..).take(mesh_count as usize).map(|i| (i + 1) as f32) {
            for y in (0..).take(mesh_count as usize).map(|i| (i + 1) as f32) {
                let color = (self.perlin_gen.get([
                    (x / mesh_count * perlin_range) as f64,
                    (y / mesh_count * perlin_range) as f64,
                ]) + 1.0)
                    / 2.0;
                draw.rect()
                    .w(w)
                    .h(h)
                    .color(rgb(color, color, color))
                    // .color(hsl(color as f32, 0.7, 0.7))
                    .x((x - mesh_count / 2.0 - 0.5) * w)
                    .y((y - mesh_count / 2.0 - 0.5) * h);
            }
        }
    }

    pub fn set_seed(&mut self, seed: u32) -> Self {
        self.perlin_gen = Perlin::new().set_seed(seed);
        *self
    }

    pub fn set_seed_rand(&mut self) -> Self {
        let mut rng = thread_rng();
        self.set_seed(rng.gen())
    }
}
