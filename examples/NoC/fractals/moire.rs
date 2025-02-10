use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model {
    centre_1: Point2,
    centre_2: Point2,
}

fn model(app: &App) -> Model {
    app.new_window().size(1200, 600).view(view).build().unwrap();
    
    Model{centre_1: pt2(-450., 0.0), centre_2: pt2(450., 0.0)}
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.centre_1.x += 0.002;
    model.centre_2.x += 0.00168;

}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    let r = 250.;

    draw_rings(&draw, model, r);
    draw.to_frame(app, &frame).unwrap()
}

fn draw_rings(draw: &Draw, model: &Model, r: f32){
    let amp = 370.;
    draw.ellipse().x_y(model.centre_1.x.sin() * amp, 0.0)
        .radius(r)
        .hsva(map_range(r, 2.0, 250.0, 0.0, 1.0), 0.75, 1.0, 0.9)
        .stroke_weight(1.0)
        .stroke(BLACK);
    
        draw.ellipse().x_y(model.centre_2.x.sin() * amp, 0.0)
        .radius(r)
        .hsva(map_range(r, 2.0, 360.0, 0.0, 1.0), 0.75, 1.0, 0.9)
        .stroke_weight(1.0)
        .stroke(BLACK);

    if r > 3.0 {
        draw_rings(draw, model, r - 3.)
    }
}