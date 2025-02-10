use nannou::prelude::*;
use nannou::noise::{BasicMulti, NoiseFn};
use nannou::wgpu::Texture;
use nannou::image::DynamicImage;
use itertools::Itertools;

const NOISE_STEP: f64 = 500.;
const FRAME_SCALE: f64 = 0.5; // adjust for smoother motion

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    image_window: WindowId,
    noise: BasicMulti,
    image: DynamicImage,
    size: Vec2,
}

impl Model {
    fn new(noise: Option<BasicMulti>, image_window: WindowId) -> Model {
        let image = DynamicImage::new_rgb8(1000, 1000);
        Model {
            image_window,
            noise: noise.unwrap_or(BasicMulti::new()),
            image,
            size: Vec2::new(10., 1000.),
        }
    }
    fn update_size(&mut self, size: Vec2) {
        self.image = DynamicImage::new_rgb8(size.x.floor() as u32, size.y.floor() as u32);
        self.size = size;
    }
    fn redraw_image(&mut self, frame_num: u64) {
        for (x, y) in (0..self.size.x.floor() as u32)
            .cartesian_product(0..self.size.y.floor() as u32)
        {
            let noise_value = self.noise.get([
                x as f64 / NOISE_STEP,
                y as f64 / NOISE_STEP,
                frame_num as f64 * FRAME_SCALE,
            ]);
            let mapped = map_range(noise_value, -1.0, 1.0, 0.0, 255.0) as u16;
            if let Some(buffer) = self.image.as_mut_rgb16() {
                buffer.put_pixel(x, y, nannou::image::Rgb([mapped, mapped, mapped]));
            }
        }
    }
}

fn model(app: &App) -> Model {
    let image_window = app.new_window().size(1200, 10).view(view).build().unwrap();
    Model::new(None, image_window)
}

fn update(app: &App, model: &mut Model, _update: Update) {
    let win_rect = app.window(model.image_window).unwrap().rect();
    // if model.size != win_rect.wh() {
    //     model.update_size(win_rect.wh());

    // }
    model.update_size(win_rect.wh());
    model.redraw_image(app.elapsed_frames());
}

fn view(app: &App, model: &Model, frame: Frame) {
    let background = rgb(0.439, 0.039, 0.467);
    let draw = app.draw();
    draw.background().color(background);
    let texture = Texture::from_image(app, &model.image);
    draw.texture(&texture);
    
    draw.to_frame(app, &frame).unwrap();
}
