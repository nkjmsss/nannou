// The Nature of Code
// Daniel Shiffman
// http://natureofcode.com
//
// Example 1-4: Vector Multiplication
use nannou::prelude::*;

fn main() {
    nannou::sketch(view);
}

fn view(app: &App, frame: &Frame) {
    app.main_window().set_inner_size_points(640.0, 360.0);

    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    let mut mouse = vec2(app.mouse.x, app.mouse.y);
    let center = vec2(0.0, 0.0);

    // Multiplying a vector! The vector is now half its original size (multilies by 0.5)
    mouse *= 0.5;

    draw.line().weight(2.0).color(BLACK).points(center, mouse);

    // Write the result of our drawing to the window's frame.
    draw.to_frame(app, &frame).unwrap();
}
