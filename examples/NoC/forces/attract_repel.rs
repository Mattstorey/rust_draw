use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).run();
}

struct Mover {
    position: Point2,
    velocity: Vec2,
    acceleration: Vec2,
    mass: f32,
}

// A type for a draggable attractive body in our world
struct Attractor {
    mass: f32,        // Mass, tied to size
    radius: f32,      // Radius of the attractor
    position: Point2, // position
    dragging: bool,   // Is the object being dragged?
    roll_over: bool,  // Is the mouse over the ellipse?
    drag: Vec2,       // holds the offset for when the object is clicked on
}

impl Attractor {
    fn new(rect: Rect)->Attractor{
        let mass = 100.0;
        let radius =  mass / 10.0;
        let position = rect.xy();
        let dragging = false;
        let roll_over = false;
        let drag = vec2(0.0, 0.0);
        Attractor { mass, radius, position, dragging, roll_over, drag }
    }

    fn attract(&self, mover: &Mover) -> Vec2{
        let force = self.position - mover.position; //vec pointing to self.pos
        let mut dist = force.length(); //f32
        dist = dist.max(5.).min(25.);
        let dir = force.normalize(); // just get direction, magnitute is not needed
        let strength = (self.mass * mover.mass) / (dist*dist);
        dir * strength
    }

    fn display(&self, draw: &Draw) {
        let gray = if self.dragging {
            0.2
        } else if self.roll_over {
            0.4
        } else {
            0.0
        };
        draw.ellipse()
            .xy(self.position)
            .w_h(self.radius * 2.0, self.radius * 2.0)
            .gray(gray);
    }
    
    fn clicked(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(pt2(mx, my));
        if d < self.radius {
            self.dragging = true;
            self.drag.x = self.position.x - mx;
            self.drag.y = self.position.y - my;
        }
    }

    fn rollover(&mut self, mx: f32, my: f32) {
        let d = self.position.distance(pt2(mx, my));
        if d < self.radius {
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
            self.position.x = mx + self.drag.x;
            self.position.y = my + self.drag.y;
        }
    }
}

impl Mover {
    fn new(m: f32, x: f32, y: f32) -> Self {
        let mass = m;
        let position = pt2(x, y);
        let velocity = vec2(0.0, 0.0);
        let acceleration = vec2(0.0, 0.0);
        Mover {
            position,
            velocity,
            acceleration,
            mass,
        }
    }

    fn apply_force(&mut self, force: Vec2){
        let f = force / self.mass;
        self.acceleration += f;
    }

    fn update(&mut self){
        self.velocity += self.acceleration;
        self.position += self.velocity;
        self.acceleration *= 0.0;
    }

    fn display_mover(&self, draw: &Draw) {
        draw.ellipse()
            .xy(self.position)
            .w_h(self.mass * 2.0, self.mass * 2.0)
            .rgba(0.6, 0.6, 0.6, 0.5);
    }

    fn repel(&self, mover: &Mover) -> Vec2{
        let force = self.position - mover.position;
        let d = force.length();
        let dist = d.max(0.1).min(1000.0);
        let dir = force.normalize();
        let strenght = self.mass * mover.mass / (dist * dist);
        dir * (strenght * -1.0)

    }
}

struct Model {
    movers: Vec<Mover>,
    attractor: Attractor,
}

fn model(app: &App) -> Model {
    let rect = Rect::from_w_h(640.0, 360.0);
    app.new_window()
        .size(rect.w() as u32, rect.h() as u32)
        .event(event)
        .view(view)
        .build()
        .unwrap();

    let attractor = Attractor::new(rect);
    let movers = (0..20).map(|_|{
        let m = random_range(4., 14.);
        let x = random_range(rect.left(), rect.right());
        let y = random_range(rect.bottom(), rect.top());
        Mover::new(m, x, y)
    }).collect();

    Model{movers, attractor}
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
    model.attractor.rollover(app.mouse.x, app.mouse.y);

    for i in 0..model.movers.len(){
        for j in 0..model.movers.len(){
            if i != j{
                let repel = model.movers[i].repel(&model.movers[j]);
                model.movers[i].apply_force(repel);
            }
        }
        let attract = model.attractor.attract(&model.movers[i]);
        model.movers[i].apply_force(attract);
        model.movers[i].update();
    } 
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHITE);

    model.attractor.display(&draw);

    for mover in &model.movers {
        mover.display_mover(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}