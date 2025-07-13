use macroquad::prelude::*;

#[macroquad::main("Rhytry")]
async fn main() {
    loop {
        clear_background(BLACK);

        draw_text("Hello from macroquad!", 20.0, 40.0, 30.0, WHITE);

        next_frame().await;
    }
}
