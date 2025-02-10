use nannou::prelude::*;

fn main() {
    nannou::sketch(view).size(640, 360).run();
}

fn view(app: &App, frame: Frame) {
    // Begin drawing
    let draw = app.draw();
    draw.background().color(WHITE);

    let period = 2.;
    let amplitude = 250.0;

    let two_pi = std::f64::consts::PI;
    let x = amplitude * (two_pi * app.duration.since_start.secs() * period).sin() as f32;

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
