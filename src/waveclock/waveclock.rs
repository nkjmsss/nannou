use nannou::noise::{NoiseFn, Perlin, Seedable};
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};

struct Coordinate<T> {
    x: T,
    y: T,
}

struct State<T> {
    color: T,
    center: Coordinate<T>,
    length: T,
    angle: T,
}

impl State<Perlin> {
    fn new() -> Self {
        let mut rng = thread_rng();
        State {
            color: Perlin::new().set_seed(rng.gen()),
            center: Coordinate {
                x: Perlin::new().set_seed(rng.gen()),
                y: Perlin::new().set_seed(rng.gen()),
            },
            length: Perlin::new().set_seed(rng.gen()),
            angle: Perlin::new().set_seed(rng.gen()),
        }
    }
}

impl State<f64> {
    fn new() -> Self {
        State {
            color: 0.0,
            center: Coordinate { x: 0.0, y: 0.0 },
            length: 0.0,
            angle: 0.0,
        }
    }
}

pub struct Waveclock {
    /// -255.0 ~ 255.0
    /// value of RGB
    color: f64,

    /// clock center position
    center: Point2,

    /// line length
    length: f64,

    /// line angle
    angle: f64,

    /// perlin noise generators
    perlin_gen: State<Perlin>,

    /// perlin noise step
    perlin_step: State<f64>,
}

impl Waveclock {
    pub fn new() -> Self {
        let color = 0.0;
        let center = Point2::from((0.0, 0.0));
        let length = 100.0;
        let angle = 0.0;
        let perlin_gen = State::<Perlin>::new();
        let perlin_step = State::<f64>::new();

        Waveclock {
            color,
            center,
            length,
            angle,
            perlin_gen,
            perlin_step,
        }
    }

    pub fn recreate(&mut self) {
        self.perlin_gen = State::<Perlin>::new();
        self.perlin_step = State::<f64>::new();
        self.update();
        self.angle = 0.0;
    }

    pub fn update(&mut self) {
        self.update_color();
        self.update_center();
        self.update_length();
        self.update_angle();
    }

    fn update_color(&mut self) {
        self.color = self.perlin_gen.color.get([self.perlin_step.color, 0.0]);
        self.perlin_step.color += 1.0 / 100.0;
    }

    fn update_center(&mut self) {
        let amplitude = 90.0;
        let x_rand = self
            .perlin_gen
            .center
            .x
            .get([self.perlin_step.center.x, 0.0]) as f32;
        let y_rand = self
            .perlin_gen
            .center
            .y
            .get([self.perlin_step.center.y, 0.0]) as f32;
        self.center = Point2::from((x_rand * amplitude, y_rand * amplitude));
        self.perlin_step.center.x += 1.0 / 173.0;
        self.perlin_step.center.y += 1.0 / 239.0;
    }

    fn update_length(&mut self) {
        let base_length = 700.0;
        let len_rand = self.perlin_gen.length.get([self.perlin_step.length, 0.0]);
        self.length = base_length * (len_rand + 1.0) / 2.0;
        self.perlin_step.length += 1.0 / 517.0;
    }

    fn update_angle(&mut self) {
        let speed = 2.0;
        let default_rotation = -0.7;
        let angle_rand = self.perlin_gen.angle.get([self.perlin_step.angle, 0.0]);
        self.angle += angle_rand * speed + default_rotation;
        self.perlin_step.angle += 1.0 / 131.0;
    }

    fn position(&self) -> (Point2, Point2) {
        let r = self.length / 2.0;
        let rad = self.angle.to_radians();
        let opprad = (self.angle + 180.0).to_radians();
        let center_x = self.center.x as f64;
        let center_y = self.center.y as f64;

        let start = Point2::from((
            (r * rad.cos() + center_x) as f32,
            (r * rad.sin() + center_y) as f32,
        ));
        let end = Point2::from((
            (r * opprad.cos() + center_x) as f32,
            (r * opprad.sin() + center_y) as f32,
        ));
        (start, end)
    }

    fn color(&self) -> Rgba<f64> {
        let v = self.color.abs();
        rgba(v, v, v, 0.5)
    }

    pub fn draw(&self, draw: &app::Draw) {
        let (start, end) = self.position();

        draw.line()
            .stroke_weight(0.5)
            .points(start, end)
            .color(self.color());
    }
}
