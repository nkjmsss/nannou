use nannou::prelude::*;

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

    render_circles(&draw, 0.0, 0.0, RADIUS as f32, 4);

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}

fn render_circles(draw: &app::Draw, x: f32, y: f32, r: f32, level: u8) {
    if level <= 0 {
        return;
    }

    let stroke_color = if level == 2 {
        rgba(255.0, 255.0, 255.0, 0.8)
    } else {
        rgba(0.0, 0.0, 0.0, 0.5)
    };

    let stroke_weight = if level == 2 { 1.5 } else { 1.0 };

    // draw current circle
    draw.ellipse()
        .x(x)
        .y(y)
        .radius(r)
        .color(rgba(0.0, 0.0, 0.0, 0.0))
        .stroke(stroke_color)
        .stroke_weight(stroke_weight);

    let count = 16;
    for i in 0..count {
        let angle = 360.0 / (count as f32) * (i as f32);
        let nx = x + angle.to_radians().cos() * r / 2.0;
        let ny = y + angle.to_radians().sin() * r / 2.0;

        render_circles(draw, nx, ny, r / 2.0, level - 1);
    }
}
