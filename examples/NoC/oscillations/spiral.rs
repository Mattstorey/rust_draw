use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    r: f32,
    theta: f32,
}

fn model(app: &App) -> Model{
    let rect = Rect::from_w_h(800.0, 200.0);
    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .view(view)
        .build()
        .unwrap();

    let r = rect.h() * 0.6;
    let theta = 0.0;

    Model{r, theta}
}

fn update(_app: &App, model: &mut Model, _update: Update){
    model.theta += 0.02;
    
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    
    draw.rect()
        .wh(app.window_rect().wh())
        .rgba(1.0, 1.0, 1.0, 0.03);

    
    if model.theta <= 0.01{
        draw.background().color(WHEAT);
    }

    let x = model.r * model.theta.cos();
    let y = model.r * -model.theta.sin();

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(x, y))
        .rgb(0.0, 0.0, 0.0)
        .stroke_weight(2.0);

    draw.ellipse()
        .x_y(x, y)
        .w_h(48.0, 48.0)
        .gray(0.5)
        .stroke(BLACK)
        .stroke_weight(2.0);

    draw.to_frame(app, &frame).unwrap();
}