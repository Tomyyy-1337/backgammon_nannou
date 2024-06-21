mod backgammon;
mod model;
mod dice;

use model::Model;
use nannou::{event::Update, glam::Vec2, App, Frame};

fn main() {
    nannou::app(model)
        .update(update)
        .run();
}

fn model(app: &App) -> Model {
    let width = 1000;
    let height = 800;
    app.new_window()
        .size(width, height)
        .view(view)
        .resizable(false)
        .build()
        .unwrap();

    Model::new(width, height)
}

fn update(app: &App, model: &mut Model, update: Update) {
    let x = app.mouse.x;
    let y = app.mouse.y;
    model.upate_mouse_pos(x, y);

    
}

fn view(app: &App, model: &Model, frame: Frame) {
    let mut draw = app.draw();
    
    model.draw(&mut draw);
    
    draw.to_frame(app, &frame).unwrap();
}