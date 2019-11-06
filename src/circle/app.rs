use nannou::prelude::*;
mod circle;
use crate::circle::CirclesRecursive;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    message: Message,
    window_event: Option<WindowEvent>,
    circles: CirclesRecursive,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Initialize,
    Clear,
    RenderReady,
    Nothing,
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
    let mut circles = CirclesRecursive::new(0.0, 0.0, 320.0, 4);
    circles.reverse().shuffle();

    Model {
        _window,
        message,
        window_event,
        circles,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(event) = model.window_event.clone() {
        model.window_event = None;
        match event {
            WindowEvent::KeyPressed(key) => {
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
                    Key::Key4 => {
                        model.circles.update().reverse().shuffle();
                    }
                    _ => {}
                }
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
            model.circles.add_que(40);
            if model.circles.is_que_empty() {
                model.message = Message::Nothing;
            }
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
            model.circles.draw(&draw);
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
