#![allow(unused)]
use nannou::color::white_point::A;
use nannou::prelude::*;
use std::cell::RefCell;
use std::ops::DerefMut;
use std::rc::{Rc, Weak};


#[derive(Debug)]
pub struct Node {
    pos: Point2,
    vel: Vec2,
    accel: Vec2,
    mass: f32
}

impl Node {
    fn new(pos: Point2)-> Self{
        let vel = pt2(0., 0.);
        let accel = pt2(0., 0.);
        let mass = 1.1;
        Node{pos, vel, accel, mass}
    }
    fn apply_force(&mut self, force: Vec2){
        self.accel += force / self.mass;
    }
    fn update_node(&mut self){
        self.vel += self.accel;
        self.pos += self.vel;
        self.accel *= 0.0;
    }
    fn display_node(&self, draw: &Draw){
        draw.ellipse()
            .xy(self.pos)
            .w_h(5.0, 5.0)
            .gray(0.3)
            .stroke(BLACK)
            .stroke_weight(2.0);
        }
}

#[derive(Debug)]
pub struct Edge {
    pub k: f32, // spring constant
    pub rest_len: f32,
    pub node_a: Weak<RefCell<Node>>,
    pub node_b: Weak<RefCell<Node>>,
    
}

impl Edge {
    pub fn new(rest_len: f32, a: &Rc<RefCell<Node>>, b: &Rc<RefCell<Node>>, ) -> Self {
        let k = 0.02;
        let node_a = Rc::downgrade(a); //make it a weak pointer
        let node_b = Rc::downgrade(b);
        Self { k, rest_len, node_a, node_b }
    }
    fn apply_force(&self){
        if let (Some(a_rc), Some(b_rc)) = (self.node_a.upgrade(), self.node_b.upgrade()){
            let dist = a_rc.borrow().pos - b_rc.borrow().pos;
            let stretch = dist.length() - self.rest_len;
            let dir = dist.normalize();
            let spring_force = -self.k * stretch * dir;
            a_rc.borrow_mut().apply_force(spring_force);
            b_rc.borrow_mut().apply_force(-spring_force);
        }
    }
    fn display_spring(&self, draw: &Draw){
        if let (Some(a_rc), Some(b_rc)) = (self.node_a.upgrade(), self.node_b.upgrade()){
            draw.line()
                .start(a_rc.borrow().pos)
                .end(b_rc.borrow().pos)
                .color(BLACK)
                .stroke_weight(2.0);
        }
    }

}

#[derive(Debug)]
pub struct Model {
    pub nodes: Vec<Rc<RefCell<Node>>>,
    pub edges: Vec<Rc<RefCell<Edge>>>,
    pub num_nodes: usize,
    pub spacing: f32,
}

impl Model {
    
    pub fn new(num_nodes: usize, spacing: f32) -> Self {
        let mut nodes = Vec::with_capacity(num_nodes);
        let mut edges = Vec::with_capacity(num_nodes.saturating_sub(1));

        let mut current_y = 0.0;
        for _ in 0..num_nodes {
            current_y -= spacing;
            let pos = vec2(0.0, current_y) ;
            nodes.push(Rc::new(RefCell::new(Node::new(pos))));
        }

        for i in 0..nodes.len().saturating_sub(1) {
            let edge = Edge::new(spacing, &nodes[i], &nodes[i + 1]);
            edges.push(Rc::new(RefCell::new(edge)));
        }
        if let Some(first_node) = nodes.first(){
            first_node.borrow_mut().mass = 2.0;
        }

        Self {
            nodes,
            edges,
            num_nodes,
            spacing,
        }
    }
}

fn model(app: &App) -> Model {
    app.new_window().size(800,800).view(view).build().unwrap();
    let num_nodes = 150;
    let spacing = 4.0;
    Model::new(num_nodes, spacing)
}

fn update(app: &App, model: &mut Model, _update: Update){
    for mut e in &model.edges{e.borrow_mut().apply_force()}

    if let Some(last_node) = model.nodes.last(){
        last_node.borrow_mut().pos = pt2(app.mouse.x, app.mouse.y)
    }
    
    for n in &model.nodes{
        n.borrow_mut().vel *= 0.999;
        n.borrow_mut().update_node();
    }
}

fn view(app: &App, model: &Model, frame: Frame){
    let draw = app.draw();
    draw.background().color(WHEAT);
    for s in &model.edges{
        s.borrow().display_spring(&draw);
    }
    draw.to_frame(app, &frame).unwrap();
}

fn main() {
    nannou::app(model).update(update).run();
}
