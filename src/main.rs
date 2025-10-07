use macroquad::prelude::*;

use crate::game::GameState;

mod game;

#[macroquad::main("Pinwheel")]
async fn main() {
    let gl = unsafe { get_internal_gl() };
    let mut game = GameState::new(gl).await;
    game.run().await;
}
