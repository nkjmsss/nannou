#![allow(dead_code)]
use nannou::prelude::*;
use nannou::rand::seq::SliceRandom;
use nannou::rand::{thread_rng, Rng};
use std::collections::VecDeque;

const RADIUS: u32 = 320;
const SIZE: u32 = 700;

#[derive(Copy, Clone, Debug)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
    level: usize,
    child_count: u8,
    color: Rgba<f32>,
    stroke_weight: f32,
}

impl Circle {
    fn new(x: f32, y: f32, r: f32, level: usize) -> Self {
        let child_count = 16;
        let color = rgba(0.0, 0.0, 0.0, 0.5);
        let stroke_weight = 1.0;

        Circle {
            x,
            y,
            r,
            level,
            child_count,
            color,
            stroke_weight,
        }
    }

    fn child_count(&mut self, count: u8) -> Self {
        self.child_count = count;
        *self
    }

    fn color(&mut self, color: Rgba<f32>) -> Self {
        self.color = color;
        *self
    }

    fn stroke_weight(&mut self, stroke_weight: f32) -> Self {
        self.stroke_weight = stroke_weight;
        *self
    }

    fn draw(&self, draw: &app::Draw) {
        draw.ellipse()
            .x(self.x)
            .y(self.y)
            .radius(self.r)
            .color(rgba(0.0, 0.0, 0.0, 0.0))
            .stroke(self.color)
            .stroke_weight(self.stroke_weight);
    }
}

struct Range {
    start: usize,
    end: usize,
}

pub struct CirclesRecursive {
    initial_circle: Circle,
    circles: Vec<Circle>,
    render_queue: Option<Range>,
}

impl CirclesRecursive {
    pub fn new(x: f32, y: f32, r: f32, level: usize) -> Self {
        let mut queue: VecDeque<Circle> = VecDeque::new();
        let initial_circle = Circle::new(x, y, r, level);
        queue.push_back(initial_circle);

        // BFS の順に Circle を格納する
        let mut circles: Vec<Circle> = Vec::new();

        // count 等を乱数で生成
        let rng = &mut thread_rng();
        let counts = (0..)
            .take(level)
            .map(|_| rng.gen_range(10, 20))
            .collect::<Vec<_>>();
        let colors = (0..)
            .take(level)
            .map(|_| {
                let v = rng.gen_range(0.0, 1.0);
                let alpha = rng.gen_range(0.5, 1.0);
                rgba(v, v, v, alpha)
            })
            .collect::<Vec<_>>();
        let stroke_weights = (0..)
            .take(level)
            .map(|_| rng.gen_range(0.5, 2.0))
            .collect::<Vec<_>>();

        while let Some(mut cur) = queue.pop_front() {
            if cur.level > 0 {
                cur.child_count(counts[cur.level - 1]);
                cur.color(colors[cur.level - 1]);
                cur.stroke_weight(stroke_weights[cur.level - 1]);
                circles.push(cur);

                for i in 0..cur.child_count {
                    let angle = 360.0 / (cur.child_count as f32) * (i as f32);
                    let nx = cur.x + angle.to_radians().cos() * cur.r / 2.0;
                    let ny = cur.y + angle.to_radians().sin() * cur.r / 2.0;
                    let nr = cur.r / 2.0;

                    queue.push_back(Circle::new(nx, ny, nr, cur.level - 1));
                }
            }
        }

        let render_queue = None;

        CirclesRecursive {
            initial_circle,
            circles,
            render_queue,
        }
    }

    fn partition_by_level(&self) -> Vec<(usize, usize)> {
        let len = self
            .circles
            .iter()
            .enumerate()
            .scan::<Option<Circle>, Option<usize>, _>(None, |prev, (i, circle)| {
                let res = match *prev {
                    Some(p) => {
                        if p.level != circle.level {
                            Some(i)
                        } else {
                            None
                        }
                    }
                    None => Some(i),
                };
                *prev = Some(*circle);
                Some(res)
            })
            .filter_map(|i| i)
            .collect::<Vec<_>>();

        let mut idx = len
            .iter()
            .rev()
            .scan(self.circles.len(), |prev, l| {
                let res = (*l, *prev);
                *prev = *l;
                Some(res)
            })
            .collect::<Vec<_>>();
        idx.reverse();
        idx
    }

    pub fn shuffle(&mut self) -> &mut Self {
        let mut rng = thread_rng();

        self.partition_by_level().iter().for_each(|(start, end)| {
            let maybe_arr = self.circles.get_mut(*start..*end);
            if let Some(arr) = maybe_arr {
                arr.shuffle(&mut rng);
            }
        });

        self
    }

    pub fn reverse(&mut self) -> &mut Self {
        self.circles.reverse();
        self
    }

    pub fn update(&mut self) -> &mut Self {
        self.circles = Self::new(
            self.initial_circle.x,
            self.initial_circle.y,
            self.initial_circle.r,
            self.initial_circle.level,
        )
        .circles;
        self
    }

    pub fn draw(&self, draw: &app::Draw) {
        if let Some(range) = &self.render_queue {
            match self.circles.get(range.start..range.end) {
                Some(circles) => {
                    for circle in circles {
                        circle.draw(draw);
                    }
                }
                None => {
                    for circle in self.circles.get(range.start..).unwrap() {
                        circle.draw(draw);
                    }
                }
            }
        }
    }

    pub fn is_que_empty(&self) -> bool {
        match self.render_queue {
            None => true,
            _ => false,
        }
    }

    pub fn add_que(&mut self, count: usize) {
        let start = match &self.render_queue {
            Some(range) => range.end + 1,
            None => 0,
        };
        if start < self.circles.len() {
            self.render_queue = Some(Range {
                start: start,
                end: start + count,
            });
        } else {
            self.render_queue = None;
        }
    }
}
