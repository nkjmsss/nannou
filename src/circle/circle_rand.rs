use nannou::prelude::*;
use rand::prelude::*;
use std::collections::VecDeque;

const RADIUS: u32 = 320;
const SIZE: u32 = 700;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    message: Message,
}

enum Message {
    UpdateReady,
    Update,
    Nothing,
}

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

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(SIZE, SIZE)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    let message = Message::UpdateReady;

    Model { _window, message }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.message = match model.message {
        Message::UpdateReady => Message::Update,
        Message::Update => Message::Nothing,
        _ => Message::Nothing,
    };
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {
            model.message = Message::UpdateReady;
        }
        MouseReleased(_button) => {}
        MouseEntered => {}
        MouseExited => {}
        MouseWheel(_amount, _phase) => {}
        Moved(_pos) => {}
        Resized(_size) => {}
        Touch(_touch) => {}
        TouchPressure(_pressure) => {}
        HoveredFile(_path) => {}
        DroppedFile(_path) => {}
        HoveredFileCancelled => {}
        Focused => {}
        Unfocused => {}
        Closed => {}
    }
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    match model.message {
        Message::Update => {
            draw.background().color(WHITE);
            render_circles(&draw, Circle::new(0.0, 0.0, RADIUS as f32, 4));
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn render_circles(draw: &app::Draw, first_circle: Circle) {
    let mut queue: VecDeque<Circle> = VecDeque::new();
    queue.push_back(first_circle);

    // BFS の順に Circle を格納する
    let mut result: Vec<Circle> = Vec::new();

    // count 等を乱数で生成
    let mut rng = rand::thread_rng();
    let counts = (0..)
        .take(first_circle.level as usize)
        .map(|_| rng.gen_range(10, 20))
        .collect::<Vec<_>>();
    let colors = (0..)
        .take(first_circle.level as usize)
        .map(|_| {
            let v = rng.gen_range(0.0, 1.0);
            let alpha = rng.gen_range(0.5, 1.0);
            rgba(v, v, v, alpha)
        })
        .collect::<Vec<_>>();
    let stroke_weights = (0..)
        .take(first_circle.level as usize)
        .map(|_| rng.gen_range(0.5, 2.0))
        .collect::<Vec<_>>();

    while let Some(mut cur) = queue.pop_front() {
        if cur.level > 0 {
            cur.child_count(counts[cur.level - 1]);
            cur.color(colors[cur.level - 1]);
            cur.stroke_weight(stroke_weights[cur.level - 1]);
            result.push(cur);

            for i in 0..cur.child_count {
                let angle = 360.0 / (cur.child_count as f32) * (i as f32);
                let nx = cur.x + angle.to_radians().cos() * cur.r / 2.0;
                let ny = cur.y + angle.to_radians().sin() * cur.r / 2.0;
                let nr = cur.r / 2.0;

                queue.push_back(Circle::new(nx, ny, nr, cur.level - 1));
            }
        }
    }

    for circle in result.iter().rev() {
        circle.draw(draw);
    }
}
