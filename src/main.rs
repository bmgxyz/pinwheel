use std::time::Duration;

use macroquad::{color::colors, prelude::*};

use crate::state::GameState;

mod state;

#[macroquad::main("Pinwheel")]
async fn main() {
    let mut state = GameState::default();
    clear_background(colors::BLACK);
    loop {
        state.step(Duration::from_secs_f32(get_frame_time()));
        next_frame().await
    }
}
