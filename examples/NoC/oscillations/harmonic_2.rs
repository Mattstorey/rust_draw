use core::f32;

use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}


struct Model{
    angle: f32,
    a_velocity: f32
}

fn model(app: &App) -> Model {
    let angle = 0.0;
    let a_velocity = 0.03;

    app.new_window().size(640, 360).view(view).build().unwrap();
    Model { angle, a_velocity }
}


fn update(_app: &App, model: &mut Model, _update: Update) {
    model.angle += model.a_velocity;
}


fn view(app: &App, model: &Model, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    let amplitude = 250.0;

    let x = amplitude * model.angle.sin() as f32;

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(x, 0.0))
        .rgb(0.0, 0.0, 0.0)
        .stroke_weight(2.0);

    draw.ellipse()
        .x_y(x, 0.0)
        .w_h(50.0, 50.0)
        .rgba(0.5, 0.5, 0.5, 1.0)
        .stroke(BLACK)
        .stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();

}
