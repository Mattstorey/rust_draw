use nannou::prelude::*;

fn main(){
    nannou::sketch(view).size(300, 600).run();
}

fn view(app: &App, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITE);

    let mouse = app.mouse.position();
    let centre = vec2(0.0, 0.0);

    // this subtraction was in the example, not sure why
    //mouse -= centre;
    draw.line().weight(2.0).color(BEIGE).points(centre, mouse);
    draw.to_frame(app, &frame).unwrap();

}