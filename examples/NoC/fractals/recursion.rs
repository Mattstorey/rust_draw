use nannou::prelude::*;


fn main(){
    nannou::app(model).run();
}

struct Model;

fn model(app: &App)-> Model {
    app.set_loop_mode(LoopMode::loop_once());
    app.new_window().size(1400, 1200).view(view).build().unwrap();
    Model
}

fn draw_circle(draw: &Draw, x:f32, y:f32, r:f32){
    draw.ellipse()
        .x_y(x, y)
        .radius(r)
        .hsva(map_range(r, 2.0, 360.0, 0.0, 1.0), 0.75, 1.0, 0.25)
        .stroke(BLACK);

    if r > 3. {
        draw_circle(&draw, x + r, y, r / 2.);
        draw_circle(&draw, x - r, y, r / 2.);
        draw_circle(&draw, x, y - r, r / 1.8);
        //draw_circle(&draw, x, y + r, r / 2.);
    }
}

fn view(app: &App, _model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(BLACK);

    draw_circle(&draw, 0.0, 250.0, 500.);

    draw_circle(&draw, 0.0, 200.0, 400.);
    draw.to_frame(app, &frame).unwrap();
}