use nannou::prelude::*;
use std::collections::VecDeque;

const RADIUS: u32 = 300;
const SIZE: u32 = RADIUS * 2 + 100;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(SIZE, SIZE)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    Model { _window }
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn window_event(_app: &App, _model: &mut Model, event: WindowEvent) {
    match event {
        KeyPressed(_key) => {}
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

fn view(app: &App, _model: &Model, frame: &Frame) {
    let draw = app.draw();
    draw.background().color(WHITE);

    render_circles(&draw, Circle::new(0.0, 0.0, RADIUS as f32, 4));

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

#[derive(Copy, Clone, Debug)]
struct Circle {
    x: f32,
    y: f32,
    r: f32,
    level: u8,
    child_count: u8,
}

impl Circle {
    fn new(x: f32, y: f32, r: f32, level: u8) -> Circle {
        let child_count = 16;

        Circle {
            x,
            y,
            r,
            level,
            child_count,
        }
    }

    #[allow(dead_code)]
    fn child_count(&mut self, count: u8) -> Self {
        self.child_count = count;
        *self
    }

    fn stroke_color(&self) -> Rgba<f32> {
        match self.level {
            2 => rgba(255.0, 255.0, 255.0, 0.5),
            _ => rgba(0.0, 0.0, 0.0, 0.5),
        }
    }

    fn stroke_weight(&self) -> f32 {
        match self.level {
            2 => 0.8,
            _ => 1.0,
        }
    }

    fn draw(&self, draw: &app::Draw) {
        draw.ellipse()
            .x(self.x)
            .y(self.y)
            .radius(self.r)
            .color(rgba(0.0, 0.0, 0.0, 0.0))
            .stroke(self.stroke_color())
            .stroke_weight(self.stroke_weight());
    }
}

fn render_circles(draw: &app::Draw, first_circle: Circle) {
    let mut queue: VecDeque<Circle> = VecDeque::new();
    queue.push_back(first_circle);

    // BFS の順に Circle を格納する
    let mut result: Vec<Circle> = Vec::new();

    while let Some(cur) = queue.pop_front() {
        if cur.level > 0 {
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
