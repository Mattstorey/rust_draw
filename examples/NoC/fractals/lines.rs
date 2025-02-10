use nannou::prelude::*;

fn main() {
    nannou::app(model).update(update).view(view).run();
}

struct Model;

fn model(app: &App) -> Model {
    // Create a window with default loop mode (animating continuously)
    app.new_window().size(1400, 1200).view(view).build().unwrap();
    Model
}

fn update(_app: &App, _model: &mut Model, _update: Update) {}

fn view(app: &App, _model: &Model, frame: Frame) {
    let draw = app.draw();
    // Dark background for contrast.
    draw.background().color(BLACK);
    let win = app.window_rect();

    let num_lines = 100;
    let spacing = win.h() / num_lines as f32;
    let base_angle_inc = 0.01; // slight angle change per line
    let time_phase = app.time;

    // Base set: thin white lines.
    for i in 0..num_lines {
        let y = win.top() - i as f32 * spacing * 1.5;
        let angle = 0.0 + i as f32 * base_angle_inc;
        let center = pt2(0.0, y);
        let half_length = win.w() * 0.7;
        let dx = half_length * angle.cos();
        let dy = half_length * angle.sin();
        // Thinner weight: from 0.5 to 1.0
        let weight = map_range(i as f32, 0.0, num_lines as f32, 0.5, 1.0);
        draw.line()
            .start(pt2(center.x - dx, center.y - dy))
            .end(pt2(center.x + dx, center.y + dy))
            .weight(weight)
            .color(BLUEVIOLET);
    }

    // next layer
    let iterations = 2;
    for j in 0..iterations {
        // dynamic tilt offset varying over time.
        let base_offset = 0.04; // smaller base offset than before
        let tilt_step = 0.003;
        let dynamic_offset = (time_phase + j as f32 * 0.1).sin() * 0.15;
        let tilt_offset = base_offset + j as f32 * tilt_step + dynamic_offset;
        for i in 0..num_lines {
            let y = win.top() - i as f32 * spacing * 1.7;
            let angle = tilt_offset + i as f32 * base_angle_inc;
            let center = pt2(j as f32, y);
            let half_length = win.w() * 0.6;
            let dx = half_length * angle.cos();
            let dy = half_length * angle.sin();
            // Use modest weights for the overlay: 0.8 to 1.5.
            let weight = map_range(i as f32, 0.0, num_lines as f32, 0.8, 1.5);
            draw.line()
                .start(pt2(center.x - dx, center.y - dy))
                .end(pt2(center.x + dx, center.y + dy))
                .weight(weight)
                .color(RED);
        }
    }

    draw.to_frame(app, &frame).unwrap();
}
