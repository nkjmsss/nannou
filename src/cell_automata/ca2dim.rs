#![allow(dead_code)]
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};

const COUNT: usize = 121;

pub struct CA2dim {
    state: Vec<Vec<usize>>,
    mod_: usize,
}

impl CA2dim {
    pub fn new(mod_: usize) -> Self {
        let mut state = vec![vec![0; COUNT]; COUNT];
        state[COUNT / 2][COUNT / 2] = 1;

        CA2dim { state, mod_ }
    }

    pub fn initialize(&mut self) {
        let mut state = vec![vec![0; COUNT]; COUNT];
        state[COUNT / 2][COUNT / 2] = 1;

        self.state = state;
    }

    pub fn update(&mut self) {
        let mut next_state = vec![vec![0; COUNT]; COUNT];
        for i in 0..COUNT {
            for j in 0..COUNT {
                let v = self.state[(i - 1 + COUNT) % COUNT][j]
                    + self.state[i][(j - 1 + COUNT) % COUNT]
                    + self.state[i][j]
                    + self.state[i][(j + 1) % COUNT]
                    + self.state[(i + 1) % COUNT][j];
                next_state[i][j] = v % self.mod_;
            }
        }

        self.state = next_state;
    }

    pub fn draw(&self, draw: &app::Draw, window_size: (f32, f32)) {
        let (width, height) = window_size;
        let x_step = width / (COUNT as f32);
        let y_step = height / (COUNT as f32);
        let half_count = (COUNT as f32) / 2.0 - 0.5;

        for i in 0..COUNT {
            for j in 0..COUNT {
                let x = ((j as f32) - half_count) * x_step;
                let y = ((i as f32) - half_count) * y_step;
                let v = (self.state[i][j] as f32) / (self.mod_ as f32);

                draw.rect()
                    .x(x)
                    .y(y)
                    .width(x_step)
                    .height(y_step)
                    .color(hsv(v, v, 1.0));
            }
        }
    }

    pub fn set_mod(&mut self, mod_: usize) {
        println!("mod: {}", mod_);
        self.mod_ = mod_;
    }

    pub fn set_mod_rand(&mut self) {
        let mut rng = thread_rng();
        self.set_mod(rng.gen_range(4, 40));
    }
}
