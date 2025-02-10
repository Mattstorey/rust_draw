use nannou::prelude::*;

fn main(){
    nannou::app(model).update(update).run();
}

struct Model{
    movers: Vec<Mover>
}
struct Mover{
    pos: Point2,
    vel: Vec2,
    accel: Vec2,
    mass: f32
}

impl Mover {
    fn new(m: f32, x: f32, y:f32)-> Self{
        let pos = vec2(x, y);
        let mass = m;
        let accel = vec2(0.0, 0.0);
        let vel = vec2(0.0, 0.0);
        Mover{pos, vel, accel, mass}
    }

    fn apply_force(&mut self, force: Vec2){
        let f  = force / self.mass;
        self.accel += f;
    }

    fn update_mover(&mut self){
        self.vel += self.accel;
        self.pos += self.vel;
        self.accel *= 0.0;
    }
    fn display(&self, draw: &Draw){
        draw.ellipse().xy(self.pos)
        .w_h(self.mass * 10.0, self.mass * 10.0)
        .rgba(0.0, 0.0, 0.0, 0.5)
        .stroke(BLACK)
        .stroke_weight(2.0);
    }
    fn check_edges(&mut self, rect: Rect){
        if self.pos.x < rect.left(){
            self.pos.x = rect.left();
            self.vel.x *= -1.0;
        } else if self.pos.x > rect.right() {
            self.vel.x *= -1.0;
            self.pos.x = rect.right();
            
        }
        if self.pos.y < rect.bottom(){
            
            // apply some bounce damping base on mass
            //let min_bounce = 0.8;
            //let bounce_factor = min_bounce + (self.mass.sqrt() / 2.) * (1. - min_bounce);
            //self.vel.y *= -bounce_factor;
            
            self.vel.y *= -1.0;
            self.pos.y = rect.bottom();
            
        }
    }
}

fn model(app: &App) -> Model{
    app.new_window().size(800,800).view(view).build().unwrap();
    let rect = app.window_rect();
    let movers = (0..30).map(|_| Mover::new(random_range(0.1, 4.0), rect.left(), rect.top())).collect();

    Model { movers }
}

fn update(app: &App, model: &mut Model, _update: Update){
    //loop, make wind, apply, update, edges
    let wind = vec2(0.01, 0.0);
    let c = 0.05;

    model.movers.iter_mut().for_each( |m| {
        let gravity = vec2(0., -0.1 * m.mass);
        let mut friction = m.vel;
        // apply some systematic friction
        if friction.length() > 0.0 {
            friction *= -1.0; // frcition in reverse direction to vel
            friction = friction.normalize_or_zero();
            friction *= c;
            m.apply_force(friction);
        }

        m.apply_force(wind);
        
        m.apply_force(gravity);
        m.update_mover();
        m.check_edges(app.window_rect());
    });
    
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITE);
    for m in &model.movers {
        m.display(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

