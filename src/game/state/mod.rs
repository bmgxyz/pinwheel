use macroquad::prelude::*;
use uom::si::{f32::Time, time::second};

use crate::game::{GameState, GlWrapper, Level, LevelState, PinGun, Spinner};

mod render;
mod step;

impl<'a> GameState<'a> {
    pub fn new(gl: InternalGlContext) -> GameState {
        let levels_str = include_str!("../../../levels.json");
        // TODO handle error instead of unwrapping
        let levels = serde_json::from_str::<Vec<Level>>(levels_str).unwrap();
        // TODO check that there is at least one level
        let mut game = GameState {
            gl: GlWrapper(gl),
            startup_complete: false,
            text_params: TextParams::default(),
            spinner: Spinner::default(),
            pin_gun: PinGun::default(),
            flying_pins: vec![],
            levels,
            level_idx: 0,
            level_state: LevelState::default(),
        };
        game.load_level(0);
        game
    }
    pub(crate) fn load_level(&mut self, level_idx: usize) {
        let level = &self.levels[level_idx];
        self.spinner = level.spinner.clone();
        self.pin_gun.pins = level.pins_in_gun.clone();
        self.flying_pins.clear();
        self.level_state = LevelState::Playing;
    }
    pub async fn run(&mut self) -> ! {
        loop {
            self.step(Time::new::<second>(get_frame_time()));
            self.render();
            next_frame().await
        }
    }
}
