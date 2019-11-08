use nannou::prelude::*;

mod waveclock;
use crate::waveclock::Waveclock;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    message: Message,
    window_event: Option<WindowEvent>,
    waveclock: Waveclock,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Initialize,
    Clear,
    RenderReady,
}

fn model(app: &App) -> Model {
    let _window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    let message = Message::Initialize;
    let window_event = None;
    let waveclock = Waveclock::new();

    Model {
        _window,
        message,
        window_event,
        waveclock,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(event) = model.window_event.clone() {
        model.window_event = None;
        match event {
            WindowEvent::MousePressed(_button) => {
                model.message = Message::Clear;
                model.waveclock.recreate();
                return;
            }
            _ => {}
        }
    }

    match model.message.clone() {
        Message::Initialize => {
            model.message = Message::Clear;
        }
        Message::Clear => {
            model.message = Message::RenderReady;
        }
        Message::RenderReady => {
            model.waveclock.update();
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
            model.waveclock.draw(&draw);
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
