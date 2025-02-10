use nannou::prelude::*;

fn main(){
    nannou::app(model).update(update).run();
}

struct Model{
    theta: f32
}

fn model(app: &App) -> Model{
    let _window = app.new_window().size(600, 600).view(view).build().unwrap();
    Model{theta: 0.0}
}

fn update(app: &App, model: &mut Model, _update: Update){
    let rect = app.window_rect();
    model.theta = map_range(app.mouse.x, rect.left(), rect.right(), 0., PI / 2.)
}

fn view(app: &App, model: &Model, frame: Frame){
    frame.clear(WHITE);
    
    let win = app.window_rect();
    let draw = app.draw().x_y(0.0, win.bottom());

    branch(&draw, 160.0, model.theta);

    draw.to_frame(app, &frame).unwrap()
}

fn branch(draw: &Draw, len: f32, theta: f32){
    let mut length = len;

    let sw = map_range(length, 2.0, 120., 1.0, 10.);
    let mut color = if len > 4.0 { BURLYWOOD } else { GREEN };
    if theta == 0.0 {color = BLACK }

    draw.line()
        .start(pt2(0.0, 0.0))
        .end(pt2(0.0, length))
        .weight(sw)
        .color(color);

    let draw = draw.x_y(0.0, length);

    length *= 0.72;

    if len > 2. {
        let draw2 = draw.rotate(theta);
        branch(&draw2, length, theta);

        let draw3 = draw.rotate(-theta);
        branch(&draw3, length, theta);
    }
}