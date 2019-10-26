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
    circles: CirclesRecursive,
}

enum Message {
    Initialize,
    Clear,
    KeyPressed(Key),
    RenderReady,
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

struct Range {
    start: usize,
    end: usize,
}

struct CirclesRecursive {
    initial_circle: Circle,
    circles: Vec<Circle>,
    render_queue: Option<Range>,
}

impl CirclesRecursive {
    fn new(x: f32, y: f32, r: f32, level: usize) -> Self {
        let mut queue: VecDeque<Circle> = VecDeque::new();
        let initial_circle = Circle::new(x, y, r, level);
        queue.push_back(initial_circle);

        // BFS の順に Circle を格納する
        let mut circles: Vec<Circle> = Vec::new();

        // count 等を乱数で生成
        let rng = &mut rand::thread_rng();
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

    fn shuffle(&mut self) {
        let rng = &mut rand::thread_rng();
        self.circles.shuffle(rng);
    }

    fn reverse(&mut self) {
        self.circles.reverse();
    }

    fn update(&mut self) -> &mut Self {
        self.circles = Self::new(
            self.initial_circle.x,
            self.initial_circle.y,
            self.initial_circle.r,
            self.initial_circle.level,
        )
        .circles;
        self
    }

    fn draw(&self, draw: &app::Draw) {
        if let Some(range) = &self.render_queue {
            if let Some(circles) = self.circles.get(range.start..range.end) {
                for circle in circles {
                    circle.draw(draw);
                }
            }
        }
    }

    fn add_que(&mut self, count: usize) {
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

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(SIZE, SIZE)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    let message = Message::Initialize;
    let mut circles = CirclesRecursive::new(0.0, 0.0, RADIUS as f32, 4);
    circles.shuffle();

    Model {
        _window,
        message,
        circles,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.message {
        Message::Initialize => {
            model.message = Message::Clear;
        }
        Message::KeyPressed(key) => {
            model.message = Message::Clear;
            match key {
                Key::Key1 => {
                    model.circles.update();
                }
                Key::Key2 => {
                    model.circles.update().reverse();
                }
                Key::Key3 => {
                    model.circles.update().shuffle();
                }
                _ => {}
            };
        }
        Message::Clear => {
            model.message = Message::RenderReady;
        }
        Message::RenderReady => {
            model.circles.add_que(40);
            match model.circles.render_queue {
                None => {
                    model.message = Message::Nothing;
                }
                _ => {}
            }
        }
        _ => {
            model.message = Message::Nothing;
        }
    };
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(key) => model.message = Message::KeyPressed(key),
        KeyReleased(_key) => {}
        MouseMoved(_pos) => {}
        MousePressed(_button) => {}
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
        Message::Clear => {
            draw.background().color(WHITE);
        }
        Message::RenderReady => {
            model.circles.draw(&draw);
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
