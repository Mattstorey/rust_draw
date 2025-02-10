use nannou::prelude::*;

fn main(){
    nannou::sketch(view).size(480,480).run();
}

fn view(app: &App, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITESMOKE);
    
    let mut mouse = app.mouse.position();
    let centre = vec2(0., 0.);

    mouse -= centre;
    mouse = mouse.normalize();
    mouse *= 150.0;

    draw.line().weight(2.0).color(BLACK).points(centre, mouse);
    draw.to_frame(app, &frame).unwrap();

}