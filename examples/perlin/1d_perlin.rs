use nannou::prelude::*;
use nannou::noise::{BasicMulti, NoiseFn};

const NOISE_STEP: f32 = 500.; //number of frames that the graph will pass through 0

fn main() {
    nannou::app(model).update(update).run();
}

struct Model{
    noise: BasicMulti,
    points: Vec<Vec2>,
    frame_start: u64,
}

fn model(app: &App) -> Model{
    // sets up the initial window
    let _window = app
        .new_window()
        .size(1200, 360)
        .view(view)
        .build()
        .unwrap();

    Model{noise: BasicMulti::new(), points: vec![], frame_start: 0}
}

fn update(app: &App, model: &mut Model, _update: Update){
    // get window size
    let wind_rect = app.window_rect();
    let current_step = (app.elapsed_frames() - model.frame_start) as f32 / NOISE_STEP;
    let y = model.noise.get([current_step.into(), 0.]);
    let mapped_y = map_range(y, -1.0, 1.0, wind_rect.bottom(), wind_rect.top());
    model.points.push(pt2(current_step, mapped_y));

}

fn view(app: &App, model: &Model, frame: Frame){

    let background = rgb(0.4, 0.3, 0.42);
    let foreground = rgb(0.8,0.3,0.6);

    let wind_rect = app.window_rect();
    let _win_pad = wind_rect.pad(25.0);
    
    // grab the draw instance - set up the canvis 
    let draw = app.draw();

    draw.background().color(background);

    draw.line()
        .start(Vec2::new(wind_rect.left(), 0.))
        .end(Vec2::new(wind_rect.right(), 0.))
        .color(rgb(0.25, 0.02, 0.28));

    draw.polyline().x(0. - model.points.len() as f32).weight(1.0)
        .points(
            model.points.iter().cloned().enumerate()
            .map(|(index, mut point)|{
                point.x = index as f32; // modify x to be index
                point
            }).collect::<Vec<Vec2>>()
        )
        .color(foreground);
        
    draw.to_frame(app, &frame).unwrap()
}


