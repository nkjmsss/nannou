#![allow(dead_code)]
extern crate rotation;
use crate::camera::Camera;
use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};
use rotation::rotate;

#[derive(Copy, Clone, Debug)]
struct State<T> {
    color: T,
    rotation: T,
}

impl State<f64> {
    fn new() -> Self {
        let color = 0.0;
        let rotation = 0.0;

        Self { color, rotation }
    }
}

impl State<Perlin> {
    fn new() -> Self {
        let mut rng = thread_rng();

        let color = Perlin::new().set_seed(rng.gen());
        let rotation = Perlin::new().set_seed(rng.gen());

        Self { color, rotation }
    }
}

#[derive(Copy, Clone, Debug)]
pub struct SoapBubble {
    r: f32,
    position: Point3,
    perlin_gen: State<Perlin>,
    perlin_step: State<f64>,
    update_count: f32,
    twists: f32,
    speed: f32,
    color_range: (f32, f32),
    fade: bool,
}

impl SoapBubble {
    pub fn new(r: f32, position: Point3) -> Self {
        let perlin_gen = State::<Perlin>::new();
        let perlin_step = State::<f64>::new();
        let update_count = 0.0;
        let twists = 3.0;
        let speed = 1.0;
        let color_range = (0.5, 0.8);
        let fade = true;

        Self {
            r,
            position,
            perlin_gen,
            perlin_step,
            update_count,
            twists,
            speed,
            color_range,
            fade,
        }
    }

    /// default to 3.0
    pub fn twists(&mut self, twists: f32) -> Self {
        self.twists = twists;
        *self
    }

    /// default to 1.0
    pub fn speed(&mut self, speed: f32) -> Self {
        self.speed = speed;
        *self
    }

    /// default to (0.5, 0.8)
    ///
    /// First, map the value into this range.
    /// Then, remap into (0.0, 1.0) by getting the surplus by 1.0.
    pub fn color_range(&mut self, (min, max): (f32, f32)) -> Self {
        self.color_range = (min, max);
        *self
    }

    /// default to true
    pub fn fade(&mut self, fade: bool) -> Self {
        self.fade = fade;
        *self
    }

    pub fn update(&mut self) {
        self.perlin_step.color += 1.0 / 230.0;
        self.perlin_step.rotation += 1.0 / 277.0;
        self.update_count += 1.0;
    }

    fn color(&self) -> Hsva {
        let (min, max) = self.color_range;
        assert!(min < max, "got the invalid color range");
        let v = map_range(
            self.perlin_gen.color.get([self.perlin_step.color, 0.0]) as f32,
            -1.0,
            1.0,
            min,
            max,
        );
        let hue = if v > 0.0 { v % 1.0 } else { v % 1.0 + 1.0 };
        hsva(hue, 0.7, 1.0, 0.05)
    }

    fn degrees(&self) -> Vector3 {
        let v = self
            .perlin_gen
            .rotation
            .get([self.perlin_step.rotation, 0.0]) as f32;
        Vector3::from((
            (self.update_count * 0.5 + v * 40.0),
            self.update_count * 0.7,
            self.update_count * 0.7 + v * 20.0,
        )) * self.speed
    }

    pub fn draw(&self, draw: &app::Draw, camera: &Camera) {
        let count = 1000;
        let vertices = (0..count).map(|i| {
            let theta = map_range(i, 0, count, 0.0, PI);
            let phi = map_range(i, 0, count, 0.0, PI * self.twists);
            let x = self.r * theta.cos() * phi.sin();
            let y = self.r * theta.sin() * phi.sin();
            let z = self.r * phi.cos();

            let v = camera.projection(rotate(pt3(x, y, z), self.degrees(), self.position));
            pt2(v.x, v.y)
        });

        if self.fade {
            draw.rect()
                .w_h(camera.get_window_w(), camera.get_window_h())
                .color(rgba(0.0, 0.0, 0.0, 0.002));
        }

        draw.polyline()
            .weight(self.r / 300.0)
            .points(vertices)
            .color(self.color());
    }
}
