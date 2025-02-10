use nannou::prelude::*;

fn main (){
    nannou::app(model).update(update).run();
}

struct Model {
    movers: Vec<Mover>,
    attractor: Attractor,
}

struct Mover {
    pos: Point2,
    vel: Vec2,
    accel: Vec2,
    mass: f32
}

struct Attractor {
    mass: f32,
    pos: Point2,
    dragging: bool,
    roll_over: bool,
    drag_offset: Vec2
}

impl Attractor {
    const G: f32 = 1.0;
    fn new(rect: Rect) -> Attractor {
        let pos = rect.xy();
        let mass = 100.0;
        let drag_offset = vec2(0.0, 0.0);
        let dragging = false;
        let roll_over = false;
        
        Attractor{
            pos,
            mass,
            drag_offset,
            dragging,
            roll_over,
        }
    }
    fn attract(&self, mover: &Mover) -> Vec2 {
        // calculate a an attracion vector with strength and direction
        let force = self.pos - mover.pos;
        let d = force.length();
        let dis = d.max(5.).min(25.);
        let dir = force.normalize();
        let strength = Attractor::G * self.mass * mover.mass / (dis * dis);
        strength * dir
    }
    
    // Method to display
    fn display_attractor(&self, draw: &Draw) {
        let gray = if self.dragging {
            0.2
        } else if self.roll_over {
            1.0
        } else {
            0.75
        };
        draw.ellipse()
            .xy(self.pos)
            .w_h(self.mass/10. * 2.0, self.mass/10.0 * 2.0)
            .rgba(gray, gray, gray, 0.5)
            .stroke(BLACK)
            .stroke_weight(6.0);
    }

    // The methods below are for mouse interaction
    fn clicked(&mut self, mx: f32, my: f32) {
        let d = self.pos.distance(pt2(mx, my));
        if d < self.mass/10. {
            self.dragging = true;
            self.drag_offset.x = self.pos.x - mx;
            self.drag_offset.y = self.pos.y - my;
        }
    }
    fn hover(&mut self, mx: f32, my: f32) {
        let d = self.pos.distance(pt2(mx, my));
        if d < self.mass {
            self.roll_over = true;
        } else {
            self.roll_over = false;
        }
    }

    fn stop_dragging(&mut self) {
        self.dragging = false;
    }

    fn drag(&mut self, mx: f32, my: f32) {
        if self.dragging {
            self.pos.x = mx + self.drag_offset.x;
            self.pos.y = my + self.drag_offset.y;
        }
    }
}

impl Mover {
    fn new(x: f32, y: f32, m: f32)-> Mover{
        let pos = pt2(x, y);
        let vel = vec2(0.0, 0.0);
        let accel = vec2(0.0, 0.0);
        let mass = m;
        Mover { pos, vel, accel, mass}
    }

    fn apply_force(&mut self, force: Vec2){
        let f = force / self.mass;
        self.accel += f;
    }

    fn update_mover(&mut self){
        self.vel += self.accel;
        self.pos += self.vel;
        self.accel *= 0.0;
    }
    fn display_mover(&self, draw: &Draw){
        draw.ellipse()
            .xy(self.pos)
            .w_h(10.0, 10.0)
            .gray(0.3)
            .stroke(BLACK)
            .stroke_weight(2.0);
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(800,800).event(event).view(view).build().unwrap();
    let rect = app.window_rect();

    let movers = (0..10).map(|_|{
        Mover::new(
            random_range(rect.left(), rect.right()),
            random_range(rect.top(), rect.bottom()),
            random_range(0.1f32, 3.0)
        )
    }).collect();
    let attractor = Attractor::new(rect);
    Model { movers, attractor}
}

fn event(app: &App, m: &mut Model, event: WindowEvent) {
    match event {
        MousePressed(_button) => {
            m.attractor.clicked(app.mouse.x, app.mouse.y);
        }
        MouseReleased(_buttom) => {
            m.attractor.stop_dragging();
        }
        _other => (),
    }
}

fn update(app: &App, model: &mut Model, _update: Update){
    model.attractor.drag(app.mouse.x, app.mouse.y);
    model.attractor.hover(app.mouse.x, app.mouse.y);
    for i in 0..model.movers.len(){
        let force = model.attractor.attract(&model.movers[i]);
        model.movers[i].apply_force(force);
        model.movers[i].update_mover();
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHEAT);
    
    model.attractor.display_attractor(&draw);
    for m in 0..model.movers.len(){
        model.movers[m].display_mover(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}
