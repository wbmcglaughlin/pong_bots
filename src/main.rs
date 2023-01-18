mod pong;
use std::fmt::format;
use nannou::prelude::*;
use crate::pong::*;

pub const WINDOW_SCALE: f32 = 2.0;
pub const WIDTH: f32 = 1920.0 / WINDOW_SCALE;
pub const HEIGHT: f32 = 1080.0 / WINDOW_SCALE;
pub const FPS: f64 = 60.0;

fn main() {
    nannou::app(model).update(update).loop_mode(LoopMode::rate_fps(FPS)).run()
}

struct Model {
    pong: Pong
}

fn model(app: &App) -> Model {
    let _window = app.new_window()
        .title(app.exe_name().unwrap())
        .size(WIDTH as u32, HEIGHT as u32)
        .view(view)
        .build()
        .unwrap();

    Model {
        pong: Pong::new(),
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.pong.update(1.0 / FPS as f32);
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();

    draw.background().color(SNOW);

    draw.rect()
        .color(BLACK)
        .wh(model.pong.dim);

    draw.ellipse()
        .xy(model.pong.ball.pos)
        .color(SNOW)
        .radius(model.pong.ball.rad);

    for (i, player) in model.pong.players.iter().enumerate() {
        draw.rect()
            .xy(player.pos)
            .color(SNOW)
            .h(PLAYER_LENGTH)
            .w(PLAYER_THICKNESS);
    }

    draw.to_frame(app, &frame).unwrap();

    // Capture the frame!
    // let file_path = captured_frame_path(app, &frame);
    // println!("{:?}", file_path);
    // app.main_window().capture_frame(file_path);
}

fn captured_frame_path(app: &App, frame: &Frame) -> std::path::PathBuf {
    // Create a path that we want to save this frame to.
    app.project_path()
        .expect("failed to locate `project_path`")
        // Capture all frames to a directory called `/<path_to_nannou>/nannou/simple_capture`.
        .join(app.exe_name().unwrap())
        // Name each file after the number of the frame.
        .join(format!("{:04}", frame.nth()))
        // The extension will be PNG. We also support tiff, bmp, gif, jpeg, webp and some others.
        .with_extension("png")
}
