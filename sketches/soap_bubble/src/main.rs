extern crate camera;
mod soap_bubble;

use camera::Camera;
use nannou::prelude::*;
use nannou::rand::{thread_rng, Rng};
use soap_bubble::SoapBubble;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    _window: window::Id,
    message: Message,
    window_event: Option<WindowEvent>,
    soap_bubble: SoapBubble,
    camera: Camera,
}

#[derive(Debug, Copy, Clone)]
enum Message {
    Initialize,
    Clear,
    RenderReady,
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
    let soap_bubble = SoapBubble::new(200.0, pt3(0.0, 0.0, 0.0));
    let camera = Camera::new(app.window(window).unwrap().rect().w_h());

    Model {
        _window: window,
        message,
        window_event,
        soap_bubble,
        camera,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    if let Some(event) = model.window_event.clone() {
        model.window_event = None;
        match event {
            WindowEvent::KeyPressed(_key) => {
                let mut rng = thread_rng();
                let hue = rng.gen_range(0.0, 1.0) as f32;
                let range = rng.gen_range(0.2, 0.7) as f32;
                model.soap_bubble.color_range((hue, hue + range));
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
            model.soap_bubble.update();
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
            frame.clear(BLACK);
        }
        Message::RenderReady => {
            model.soap_bubble.draw(&draw, &model.camera);
        }
        _ => {}
    };

    // Write to the window frame.
    draw.to_frame(app, &frame).unwrap();
}
