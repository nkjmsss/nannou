use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    message: Message,
}

#[derive(Debug, Clone)]
enum Message {
    Initialize,
    Clear,
    WindowEvent(WindowEvent),
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
    Model { _window, message }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    match model.message.clone() {
        Message::Initialize => {
            model.message = Message::Clear;
        }
        Message::WindowEvent(ref event) => {
            model.message = Message::Clear;
            match event {
                _ => {}
            };
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
    model.message = Message::WindowEvent(event);
}

fn view(app: &App, model: &Model, frame: &Frame) {
    let draw = app.draw();

    match model.message {
        Message::Clear => {
            draw.background().color(WHITE);
        }
        Message::RenderReady => {
            // render some staff
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
