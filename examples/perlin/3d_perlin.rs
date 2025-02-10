use nannou::prelude::*;
use nannou::noise::{BasicMulti, NoiseFn};
use nannou::wgpu::Texture;
use nannou::image::{DynamicImage, ImageBuffer, Rgb};

const NOISE_FREQ: f64 = 2.; 
const LOW_RES: u32 = 1200;

fn main() {
    nannou::app(model).update(update).run();
}

struct Model {
    noise: BasicMulti,
    time_offset: f64,
    texture: Option<Texture>,
}

fn model(app: &App) -> Model {
    app.new_window().size(1200, 1200).view(view).build().unwrap();
    Model {
        noise: BasicMulti::new(),
        time_offset: 0.0,
        texture: None,
    }
}

fn update(app: &App, model: &mut Model, _update: Update) {
    model.time_offset += 0.005; // smaller for smoother motion

    // scale to lower res to allow smoother render 
    let mut imgbuf = ImageBuffer::new(LOW_RES, LOW_RES);
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        let nx = x as f64 / LOW_RES as f64 * NOISE_FREQ;
        let ny = y as f64 / LOW_RES as f64 * NOISE_FREQ;
        let noise_val = model.noise.get([nx, ny, model.time_offset]);
        // map noise val to brighness
        let brightness = map_range(noise_val, -1.0, 1.0, 0.0, 255.0) as u8;
        *pixel = Rgb([brightness, brightness, brightness]);
    }

    let dyn_img = DynamicImage::ImageRgb8(imgbuf);
    let _window = app.main_window();
    model.texture = Some(Texture::from_image(app, &dyn_img));
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);
    if let Some(ref texture) = model.texture {
        // Draw the low-res texture scaled to fill the window
        draw.texture(texture);
    }
    draw.to_frame(app, &frame).unwrap();
}