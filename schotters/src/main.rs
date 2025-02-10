use nannou::prelude::*;
use nannou::rand::rngs::StdRng;
use nannou::rand::{Rng, SeedableRng};

const ROWS: u32 = 44;
const COLS: u32 = 24;
const SIZE: u32 = 15; // size of each square
const MARGIN: u32 = 25;
const HEIGHT: u32 = COLS * SIZE + 2 * MARGIN;
const WIDTH: u32 = ROWS * SIZE + 2 * MARGIN;
const LINE_WIDTH: f32 = 0.06;

fn main() {
    nannou::app(model)
        .update(update)
        .loop_mode(LoopMode::wait())
        .run();
}

struct Stone {
    x: f32,
    y: f32,
    x_off: f32,
    y_off: f32,
    rotation: f32,
}

impl Stone {
    fn new(x: f32, y: f32) -> Self {
        let x_off = 0.0;
        let y_off = 0.0;
        let rotation = 0.0;
        Stone {
            x,
            y,
            x_off,
            y_off,
            rotation,
        }
    }
}

struct Model {
    random_seed: u64,
    disp_adj: f32,
    rot_adj: f32,
    stones: Vec<Stone>,
}

impl Model {
    fn new() -> Self {
        let disp_adj = 1.0;
        let rot_adj = 1.0;
        let random_seed = random_range(0, 1000000);
        let stones = (0..COLS)
            .flat_map(|x| (0..ROWS).map(move |y| Stone::new(x as f32, y as f32)))
            .collect();

        Model {
            random_seed,
            disp_adj,
            rot_adj,
            stones,
        }
    }
    fn new_seed(&mut self) {
        let random_seed = random_range(0, 1000000);
        self.random_seed = random_seed;
    }
}

fn key_pressed(app: &App, model: &mut Model, key: Key) {
    match key {
        Key::R => model.new_seed(),
        Key::S => {
            app.main_window()
                .capture_frame(app.exe_name().unwrap() + ".png");
        }
        Key::Up => {
            model.disp_adj += 0.1;
        }
        Key::Down => {
            if model.disp_adj > 0.0 {
                model.disp_adj -= 0.1;
            }
        }
        Key::Right => {
            model.rot_adj += 0.1;
        }
        Key::Left => {
            if model.rot_adj > 0.0 {
                model.rot_adj -= 0.1;
            }
        }
        _ => (),
    }
}

fn model(app: &App) -> Model {
    app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH, HEIGHT)
        .key_pressed(key_pressed)
        .view(view)
        .build()
        .unwrap();
    Model::new()
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    let mut rng = StdRng::seed_from_u64(model.random_seed);

    let _ = model
        .stones
        .iter_mut()
        .map(|stone| {
            let y_fctr = stone.y as f32 / ROWS as f32;
            let disp_fctr = y_fctr * model.disp_adj;
            let rot_fctr = y_fctr * model.rot_adj;
            stone.x_off = disp_fctr * rng.gen_range(-0.5..0.5) * 0.65;
            stone.y_off = disp_fctr * rng.gen_range(-0.5..0.5) * 0.65;
            stone.rotation = rot_fctr * rng.gen_range(-PI / 4.0..PI / 4.0) * 0.75;
        })
        .collect::<Vec<_>>();
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(SNOW);

    let scalled_draw = draw
        .scale(SIZE as f32) // draws unit is now size of a square
        .scale_y(-1.0) // flip y direction
        .x_y(COLS as f32 / -2.0 + 0.5, ROWS as f32 / -2.0 + 0.5); // translast to top left square

    // draw a grid of squares
    for stone in &model.stones {
        scalled_draw
            .rect()
            .no_fill()
            .stroke(BLACK)
            .stroke_weight(LINE_WIDTH)
            .w_h(1.0, 1.0)
            .x_y(stone.x as f32 + stone.x_off, stone.y as f32 + stone.y_off)
            .rotate(stone.rotation);
    }
    draw.to_frame(app, &frame).unwrap();
}
