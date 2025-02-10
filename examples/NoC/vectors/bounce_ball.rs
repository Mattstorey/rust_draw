use nannou::prelude::*;

fn main(){
    nannou::app(model).update(update).run();
}

struct Model{
    x: f32,
    y: f32,
    x_speed: f32,
    y_speed: f32,
}

fn model(app: &App) -> Model{
    let x = 100.;
    let y = 100.;
    let x_speed = 2.5;
    let y_speed = 0.2;

    let _window = app.new_window().size(400, 400).view(view).build().unwrap();
    Model{
        x,
        y,
        x_speed,
        y_speed
    }
}

fn update(app :&App, model: &mut Model, _update: Update){
    // add speed to pos
    model.x = model.x + model.x_speed;
    model.y = model.y + model.y_speed;

    // get window
    let wind_rec = app.window_rect();

    // bounce of walls
    if (model.x > wind_rec.right()) || (model.x < wind_rec.left()){
        model.x_speed = model.x_speed * -1.1
    }
    if (model.y > wind_rec.top()) || (model.y < wind_rec.bottom()){
        model.y_speed = model.y_speed * -1.1
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITE);

    draw.ellipse()
        .x_y(model.x, model.y)
        .w_h(5., 5.)
        .rgba(0.5, 0.5, 0.5, 1.)
        .stroke(BLACK);

    draw.to_frame(app, &frame).unwrap()

}