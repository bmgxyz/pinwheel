use macroquad::prelude::*;
use uom::si::{f32::Time, time::second};

use crate::state::GameState;

mod state;
mod utils;

#[macroquad::main("Pinwheel")]
async fn main() {
    let mut state = GameState::default();
    let mut gl = unsafe { get_internal_gl() };
    request_new_screen_size(480., 960.);
    let camera = Camera2D::from_display_rect(Rect::new(-5.0, -15.0, 10.0, 20.0));
    set_camera(&camera);
    loop {
        clear_background(Color::from_hex(0x3CA7D5));
        state.step(Time::new::<second>(get_frame_time()));
        state.render(&mut gl);
        next_frame().await
    }
}
