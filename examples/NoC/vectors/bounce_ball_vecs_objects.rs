use nannou::prelude::*;

fn main(){
    nannou::app(model).update(update).simple_window(view).size(300, 300).run();
}

struct Model{
    ball: Ball
}

struct Ball{
    position: Vec2,
    velocity: Point2
}

impl Ball {
    fn new()-> Ball {
        let velocity = pt2(2.5, 5.0);
        let position = vec2(100.0, 100.0);
        
        Ball { position, velocity }
    }

    fn update(&mut self, rect: Rect<f32>){
        self.position += self.velocity;

        if self.position.x < rect.left() || self.position.x > rect.right(){
            self.velocity.x = self.velocity.x * -1.0
        }
        if self.position.y > rect.top() || self.position.y < rect.bottom(){
            self.velocity.y = self.velocity.y * -1.0
        }
    }
    fn display(&self, draw: &Draw){
        draw.ellipse().w_h(5., 5.).x_y(self.position.x, self.position.y).gray(0.5).stroke(BLACK);
    }
}

fn model(_app: &App)-> Model{
    let ball = Ball::new();
    Model{ball}
}

fn update(app: &App, model: &mut Model, _update: Update){
    let rect = app.window_rect();
    model.ball.update(rect);
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.rect().wh(app.window_rect().wh()).rgba(1., 1., 1., 0.05);
    
    model.ball.display(&draw);
    draw.to_frame(app, &frame).unwrap()

}