use macroquad::{color::colors, prelude::*};
use uom::si::{f32::Time, time::second};

use crate::state::GameState;

mod state;

#[macroquad::main("Pinwheel")]
async fn main() {
    let mut state = GameState::default();
    clear_background(colors::BLACK);
    loop {
        state.step(Time::new::<second>(get_frame_time()));
        next_frame().await
    }
}
