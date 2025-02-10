use nannou::prelude::*;
// same as motion.rs with added rand accel and topspeed
fn main(){
    nannou::app(model).update(update).run();
}

struct Model {
    movers: Vec<Mover>
}
struct Mover {
    vel: Vec2,
    pos: Point2,
    accel: Vec2,
    top_speed: f32
}

impl Mover {
    fn new(rect: Rect) -> Mover {
        let pos = pt2(random_range(rect.left(), rect.right()), random_range(rect.top(), rect.bottom()));
        let vel = vec2(0.0, 0.0);
        let accel = vec2(0.0, 0.0);
        let top_speed = 4.;
        Mover{vel, pos, accel, top_speed}
    }
    fn update_mover(&mut self, mouse: Point2){
        self.accel = mouse - self.pos;
        self.accel = self.accel.normalize_or_zero() * 0.1;
        self.vel += self.accel;
        
        self.vel = self.vel.clamp_length_max(self.top_speed) * (1.45 - random_f32());
        
        self.pos += self.vel;
    }

    fn _check_edge(&mut self, rect: Rect){
        
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
    
    fn display(&self, draw: &Draw) {
        let rainbow = [
            rgb(1.0, 0.0, 0.0),    // red
            rgb(1.0, 0.5, 0.0),    // orange
            rgb(1.0, 1.0, 0.0),    // yellow
            rgb(0.0, 1.0, 0.0),    // green
            rgb(0.0, 0.0, 1.0),    // blue
            rgb(0.29, 0.0, 0.51),  // indigo
            rgb(0.56, 0.0, 1.0),   // violet
        ];
        let idx = (random::<f32>() * rainbow.len() as f32) as usize;
        draw.ellipse()
            .xy(self.pos)
            .w_h(5.0, 5.0)
            .color(rainbow[idx])
            .stroke(BLACK);
    }
}


fn model(app: &App) -> Model {
    app.new_window().size(800, 800).view(view).build().unwrap();
    let rect = app.window_rect();
    let movers = (0..150).map(|_| Mover::new(rect)).collect();
    Model{movers}
}

fn update(app: &App, model: &mut Model, _update: Update){
    for mover in &mut model.movers{
        mover.update_mover(pt2(app.mouse.x, app.mouse.y));
    }
    //model.mover.update_mover(pt2(app.mouse.x, app.mouse.x));
    //model.mover.check_edge(app.window_rect());
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.rect().wh(app.window_rect().wh()).rgba(1., 1., 1., 0.03);
    
    model.movers.iter().for_each(|mover| {
        mover.display(&draw);
    });
    

    draw.to_frame(app, &frame).unwrap()
}