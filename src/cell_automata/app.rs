use nannou::prelude::*;
mod ca2dim;
use crate::ca2dim::CA2dim;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    window: window::Id,
    message: Message,
    window_event: Option<WindowEvent>,
    ca2dim: CA2dim,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Initialize,
    Clear,
    RenderReady,
}

fn model(app: &App) -> Model {
    app.set_loop_mode(LoopMode::rate_fps(4.0));
    let window = app
        .new_window()
        .with_dimensions(720, 720)
        .view(view)
        .event(window_event)
        .build()
        .unwrap();
    let message = Message::Initialize;
    let window_event = None;
    let ca2dim = CA2dim::new(4);

    Model {
        window,
        message,
        window_event,
        ca2dim,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(event) = model.window_event.clone() {
        model.window_event = None;
        match event {
            WindowEvent::MousePressed(_button) => {
                model.ca2dim.initialize();
                model.ca2dim.set_mod_rand();
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
            model.ca2dim.update();
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
                .ca2dim
                .draw(&draw, app.window(model.window).unwrap().inner_size_pixels());
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
