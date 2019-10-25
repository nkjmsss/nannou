// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 1-3: Vector Subtraction
use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    app.main_window().set_inner_size_points(640.0, 360.0);

    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    let mut mouse = app.mouse.position();
    let center = vec2(0.0, 0.0);
    mouse -= center;

    draw.line().weight(2.0).color(BLACK).points(center, mouse);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
