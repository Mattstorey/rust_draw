use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Particle{
    positon: Point2,
    velocity: Vec2,
    accel: Vec2,
    life_span: f32
}

impl Particle {
    fn new(l: Point2)-> Particle{
        Particle{
            positon: l,
            velocity: vec2(random_f32() * 2. - 1., random_f32() -1.),
            accel: vec2(0.0, 0.05),
            life_span: 255.0
        }
    }
    fn update(&mut self){
        self.velocity += self.accel;
        self.positon -= self.velocity;
        self.life_span -= 1.0;
    }
    fn display(&self, draw: &Draw){
        draw.ellipse()
            .xy(self.positon)
            .w_h(2., 2.)
            .rgba(0.5, 0.5, 0.5, self.life_span / 255.)
            .stroke(rgba(0.5, 0.5, 0.5, self.life_span / 255.))
            .stroke_weight(1.);
    }
    fn is_dead(&self)-> bool {
        if self.life_span < 0.{
            true
        } else {
            false
        }
    }
}

struct Model{
    p: Particle,
    mouse_down: bool
}
impl Model {
    fn new(p: Particle) -> Model{
        Model{p, mouse_down: false}
    }
}

fn model(app: &App) -> Model{
    app.new_window()
        .size(600, 600)
        .view(view)
        .mouse_pressed(mouse_pressed)
        .mouse_released(mouse_released)
        .build().unwrap();

        let p = Particle::new(pt2(0.0, app.window_rect().top() - 20.0));
    Model::new(p)
}


fn update(app: &App, model: &mut Model, _update: Update){
    if model.mouse_down{
        model.p.update();
        if model.p.is_dead(){
            model.p = Particle::new(pt2(0.0, app.window_rect().top() - 20.0));
        }
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    
    if app.elapsed_frames() == 1{
        draw.background().color(WHITE);
    }
    if model.mouse_down{
        draw.rect()
            .wh(app.window_rect().wh())
            .rgba(1.0, 1.0, 1.0, 0.03);
        model.p.display(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn mouse_pressed(_app: &App, m: &mut Model, _button: MouseButton) {
    m.mouse_down = true;
}

fn mouse_released(_app: &App, m: &mut Model, _button: MouseButton) {
    m.mouse_down = false;
}