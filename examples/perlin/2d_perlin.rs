use nannou::prelude::*;
use nannou::noise::{BasicMulti, NoiseFn, Seedable};
use  nannou::wgpu::Texture;
use nannou::image::DynamicImage;
use itertools::Itertools;

const NOISE_STEP: f64 = 500.;


fn main(){
    nannou::app(model).update(update).run();
}

struct Model{
    image_window: WindowId,
    should_redraw: bool,

    noise: BasicMulti,
    image: DynamicImage,
    size: Vec2,
}

impl Model {
    fn new(noise: Option<BasicMulti>, image_window: WindowId) -> Model{
        let image = DynamicImage::new_rgb8(10, 10);

        Model{
            should_redraw: false,
            image_window,
            noise: noise.unwrap_or(BasicMulti::new()),
            image: image,
            size: Vec2::new(10., 10.)

        }
    }
    // fn update_noise(&mut self, noise: BasicMulti){
    //     self.noise = noise;
    //     self.redraw_image();
    // }
    fn update_size(&mut self, size: Vec2){
        let image = DynamicImage::new_rgb8(
            size.x.floor() as u32, size.y.floor() as u32);
        self.image = image;
        self.size = size;
        self.redraw_image();
    }
    fn redraw_image(&mut self){
        for (x, y) in (0..self.size.x.floor() as u32).cartesian_product(0..self.size.y.floor() as u32){
            // val -1 to 1 this point
            let value = self.noise.get([
                x as f64 / NOISE_STEP,
                y as f64 / NOISE_STEP
                ]);
            // map to brightness
            let mapped_value = map_range(value, -1., 1., 0., 255.);
            if let Some(buffer) = self.image.as_mut_rgb8(){
                buffer
                .put_pixel(
                    x, 
                    y, 
                    nannou::image::Rgb::from([
                        mapped_value as u8,
                        mapped_value as u8,
                        mapped_value as u8,
                    ]),
                );
            }
        }
    }

}

fn model(app: &App) -> Model{
    let image_window = app
        .new_window()
        .size(1200, 600)
        .view(view)
        .build()
        .unwrap();

    Model::new(None, image_window)
}

fn update(app: &App, model: &mut Model, _update: Update) {
        let win_rect =
            app.window(model.image_window).unwrap().rect();
    
        if model.size != win_rect.wh() || model.should_redraw {
            model.update_size(win_rect.wh());
            model.redraw_image();
            model.should_redraw = false;
        };

    }

fn view(app: &App, model: &Model, frame: Frame) {
        let background = rgb(0.439, 0.039, 0.467);
        //let foreground = rgb(0.855, 0.31, 0.671);
    
        // set up containing rectangles
        let win_rect = app.window_rect();
        let win_p = win_rect.pad(25.0);
    
        let draw = app.draw();
        draw.background().color(background);
    
        let texture = Texture::from_image(app, &model.image);
        draw.texture(&texture);
    
        // display noise seed
        let seed = model.noise.seed();
        let seed_bytes = seed.to_be_bytes();
        let seed_display =
            std::str::from_utf8(&seed_bytes).unwrap();
    
        draw.text(&format!("{}", seed_display))
            .font_size(48)
            .wh(win_p.wh())
            .right_justify()
            .align_text_bottom()
            .color(background);
    
        // draw to frame
        draw.to_frame(app, &frame).unwrap();
    }