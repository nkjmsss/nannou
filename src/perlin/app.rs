use nannou::prelude::*;

mod perlin;
use crate::perlin::PerlinRect;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    message: Message,
    window_event: Option<WindowEvent>,
    perlin_rect: PerlinRect,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Initialize,
    Clear,
    RenderReady,
    Nothing,
}

fn model(app: &App) -> Model {
    let window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    let message = Message::Initialize;
    let window_event = None;
    let perlin_rect = PerlinRect::new();

    Model {
        window,
        message,
        window_event,
        perlin_rect,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(event) = model.window_event.clone() {
        model.window_event = None;
        match event {
            WindowEvent::KeyPressed(_key) => {
                model.perlin_rect.set_seed_rand();
                model.message = Message::Clear;
                return;
            }
            _ => {}
        }
    }

    match model.message {
        Message::Initialize => {
            model.message = Message::Clear;
        }
        Message::Clear => {
            model.message = Message::RenderReady;
        }
        Message::RenderReady => {
            model.message = Message::Nothing;
        }
        _ => {
            model.message = Message::Nothing;
        }
    };
}

fn window_event(_app: &App, model: &mut Model, event: WindowEvent) {
    model.window_event = Some(event);
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    match model.message {
        Message::Clear => {
            draw.background().color(WHITE);
        }
        Message::RenderReady => {
            model
                .perlin_rect
                .draw(&draw, app.window(model.window).unwrap().rect().w_h());
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
