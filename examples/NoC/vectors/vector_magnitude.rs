use nannou::prelude::*;

fn main(){
    nannou::sketch(view).size(640, 360).run();
}

fn view(app: &App, frame: Frame){
    let draw = app.draw();
    let win = app.window_rect();
    draw.background().color(WHITE);

    let mut mouse = app.mouse.position();
    
    let centre = vec2(0.0, 0.0);
    mouse -= centre;
    let m = mouse.length(); // len of vector
    let r = Rect::from_w_h(m, 10.).top_left_of(win);
    draw.rect().xy(r.xy()).wh(r.wh()).color(BLACK);

    draw.line().weight(2.0).color(BEIGE).points(centre, mouse);
    draw.to_frame(app, &frame).unwrap();

}