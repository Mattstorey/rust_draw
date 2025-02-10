use nannou::prelude::*;

fn main(){
    nannou::app(model).update(update).run();
}

struct Model{mover: Mover}
struct Mover{pos: Point2, vel: Vec2, accel: Vec2, mass: f32}

impl Mover {
    fn new(rect: Rect) -> Mover {
        let pos = pt2(rect.left() + 30., rect.top() - 30.);
        let vel = vec2(0., 0.);
        let accel = vec2(0., 0.);
        let mass = 1.0;
        Mover { pos, vel, accel, mass }
    }
    fn apply_force(&mut self, force: Vec2){
        let f = force/self.mass;
        self.accel += f;
    }

    fn update_mover(&mut self){
        self.vel += self.accel;
        self.pos += self.vel;
        self.accel *= 0.0; //reset accel to 0
    }

    fn check_edges(&mut self, rect: Rect){
        if self.pos.x > rect.right() {
            self.pos.x = rect.right();
            self.vel.x *= -1.0;
        } else if self.pos.x < rect.left() {
            self.vel.x *= -1.0;
            self.pos.x = rect.left();
        }
        if self.pos.y < rect.bottom() {
            self.vel.y *= -1.0;
            self.pos.y = rect.bottom();
        }
    }

    fn display(&self, draw: &Draw){
        draw.ellipse()
            .xy(self.pos)
            .w_h(48.0, 48.0)
            .gray(0.3)
            .stroke(BLACK)
            .stroke_weight(2.0);
        }
    }

// Model constructor
fn model(app: &App) -> Model {
    // set up app window - needs to call view on new_window
    app.new_window().size(800, 800).view(view).build().unwrap();
    let rect = app.window_rect();
    let mover = Mover::new(rect);
    Model { mover }
}

fn update(app: &App, model: &mut Model, _update: Update){
    // applied just before view
    let gravity = vec2(0., -0.1);
    let wind = vec2(0.01, 0.);
    model.mover.apply_force(wind);
    model.mover.apply_force(gravity);
    model.mover.update_mover();
    model.mover.check_edges(app.window_rect());
}

fn view(app: &App, model: &Model, frame: Frame){
    // get draw instance
    let draw = app.draw();
    // draw the BG
    draw.background().color(WHITE);
    // display the mover
    model.mover.display(&draw);
    // draw to the window
    draw.to_frame(app, &frame).unwrap();

}