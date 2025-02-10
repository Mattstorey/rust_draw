use nannou::prelude::*;

fn main(){
    nannou::app(model).update(update).run();
}

struct Model {
    mover: Mover
}
struct Mover {
    vel: Vec2,
    pos: Point2
}

impl Mover {
    fn new(rect: Rect) -> Mover {
        let pos = pt2(random_range(rect.left(), rect.right()), random_range(rect.top(), rect.bottom()));
        let vel = vec2(random_range(0., 2.0), random_range(0., 2.0));
        Mover{vel, pos}
    }
    fn update_mover(&mut self){
        self.pos += self.vel
    }

    fn check_edge(&mut self, rect: Rect){
        
        if self.pos.x > rect.right(){
            self.pos.x = rect.left();
        } else if self.pos.x < rect.left(){
            self.pos.x = rect.right();
        }

        if self.pos.y < rect.bottom(){
            self.pos.y = rect.top();
        }
        else if self.pos.y > rect.top(){
            self.pos.y = rect.bottom();
        }
    }
    fn display(&self, draw: &Draw){
        draw.ellipse().xy(self.pos).w_h(20., 20.).gray(0.5).stroke(gray(0.0));
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(400, 400).view(view).build().unwrap();
    let rect = app.window_rect();
    let mover: Mover = Mover::new(rect);
    Model{mover}
}

fn update(app: &App, model: &mut Model, _update: Update){
    model.mover.update_mover();
    model.mover.check_edge(app.window_rect());
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.rect().wh(app.window_rect().wh()).rgba(1., 1., 1., 0.03);
    model.mover.display(&draw);

    draw.to_frame(app, &frame).unwrap()
}