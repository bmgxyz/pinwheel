use macroquad::prelude::*;
use uom::si::{f32::Time, time::second};

use crate::{level::Level, state::GameState};

mod level;
mod state;
mod utils;

#[macroquad::main("Pinwheel")]
async fn main() {
    let levels_str = include_str!("../levels.json");
    let levels = serde_json::from_str::<Vec<Level>>(&levels_str).unwrap();
    let mut state = GameState::default();
    state.load_levels(&levels);
    let mut gl = unsafe { get_internal_gl() };
    request_new_screen_size(480., 720.);
    let camera = Camera2D::from_display_rect(Rect::new(-5., -10., 10., 15.));
    set_camera(&camera);
    loop {
        state.step(Time::new::<second>(get_frame_time()));
        state.render(&mut gl);
        next_frame().await
    }
}
