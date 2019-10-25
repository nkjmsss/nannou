// M_2_3_01
//
// Generative Gestaltung – Creative Coding im Web
// ISBN: 978-3-87439-902-9, First Edition, Hermann Schmidt, Mainz, 2018
// Benedikt Groß, Hartmut Bohnacker, Julia Laub, Claudius Lazzeroni
// with contributions by Joey Lee and Niels Poldervaart
// Copyright 2018
//
// http://www.generative-gestaltung.de
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at http://www.apache.org/licenses/LICENSE-2.0
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

/**
 * draws an amplitude modulated oscillator
 *
 * KEYS
 * i                 : toggle draw info signal
 * c                 : toggle draw carrier signal
 * 1/2               : info signal frequency -/+
 * arrow left/right  : info signal phi -/+
 * 7/8               : carrier signal frequency -/+ (modulation frequency)
 * s                 : save png
 */
use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    point_count: usize,
    freq: f32,
    phi: f32,
    mod_freq: f32,
    draw_frequency: bool,
    draw_modulation: bool,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(1000, 400)
        .view(view)
        .key_pressed(key_pressed)
        .build()
        .unwrap();

    Model {
        point_count: 1000,
        freq: 2.0,
        phi: 0.0,
        mod_freq: 12.0,
        draw_frequency: true,
        draw_modulation: true,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.point_count = app.window_rect().w() as usize;
}

fn view(app: &App, model: &Model, frame: &Frame) {
    // Begin drawing
    let draw = app.draw();
    let win = app.window_rect();

    draw.background().color(WHITE);

    // draw oscillator with freq and phi
    if model.draw_frequency {
        let vertices = (0..=model.point_count)
            .map(|i| {
                let angle = map_range(i, 0, model.point_count, 0.0, TAU);
                let mut y = (angle * model.freq + deg_to_rad(model.phi)).sin();
                y *= win.h() / 4.0;
                pt2(win.left() + i as f32, y)
            })
            .enumerate()
            .map(|(_i, p)| {
                let rgba = rgba(0.0, 0.0, 0.0, 1.0);
                (p, rgba)
            });
        draw.polyline().weight(1.0).colored_points(vertices);
    }

    // draw oscillator with mod_freq
    if model.draw_modulation {
        let vertices = (0..=model.point_count)
            .map(|i| {
                let angle = map_range(i, 0, model.point_count, 0.0, TAU);
                let mut y = (angle * model.mod_freq).cos();
                y *= win.h() / 4.0;
                pt2(win.left() + i as f32, y)
            })
            .enumerate()
            .map(|(_i, p)| {
                let rgba = rgba(0.0, 0.5, 0.64, 0.5);
                (p, rgba)
            });
        draw.polyline().weight(1.0).colored_points(vertices);
    }

    // draw both combined
    if model.draw_modulation {
        let vertices = (0..=model.point_count)
            .map(|i| {
                let angle = map_range(i, 0, model.point_count, 0.0, TAU);
                let info = (angle * model.freq + deg_to_rad(model.phi)).sin();
                let carrier = (angle * model.mod_freq).cos();
                let mut y = info * carrier;
                y *= win.h() / 4.0;
                pt2(win.left() + i as f32, y)
            })
            .enumerate()
            .map(|(_i, p)| {
                let rgba = rgba(0.30, 1.0, 0.64, 0.75);
                (p, rgba)
            });
        draw.polyline().weight(3.0).colored_points(vertices);
    }

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}

fn key_pressed(_app: &App, model: &mut Model, key: Key) {
    match key {
        Key::Key1 => {
            model.freq -= 1.0;
        }
        Key::Key2 => {
            model.freq += 1.0;
        }
        Key::Key7 => {
            model.mod_freq -= 1.0;
        }
        Key::Key8 => {
            model.mod_freq += 1.0;
        }
        Key::A => {
            model.draw_frequency = !model.draw_frequency;
        }
        Key::C => {
            model.draw_modulation = !model.draw_modulation;
        }
        Key::Left => {
            model.phi -= 15.0;
        }
        Key::Right => {
            model.phi += 15.0;
        }
        _other_key => {}
    }
    model.freq = model.freq.max(1.0);
    model.mod_freq = model.mod_freq.max(1.0);
}
