mod infrastructure;
mod interface;

use macroquad::prelude::*;

use crate::interface::input::Input;
use crate::interface::key::Key;


#[macroquad::main("Rhytry")]
async fn main() {
    let box_width: f32 = 1280.;
    let box_height: f32 = 720.;

    let mut circle_x: f32 = 1280.;
    let mut circle_y: f32 = 360.;
    let texture = load_texture("./assets/don.png").await.unwrap();

    loop {
        clear_background(LIGHTGRAY);

        let delta: f32 = get_frame_time() * 1000.;
        if Key::LeftDon.down() {
            circle_x -= 0.1 * delta;
        }
        if circle_x < 0. {
            circle_x = 1280.;
        }

        draw_rectangle_lines(0., 0., box_width, box_height, 1., BLACK);
        draw_circle(circle_x, circle_y, 15.0, RED);
        draw_texture_ex(
            &texture,
            circle_x,
            circle_y,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(80.0, 80.0)),
                ..Default::default()
            },
        );
        next_frame().await
    }
}
