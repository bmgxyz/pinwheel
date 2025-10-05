use std::sync::LazyLock;

use macroquad::{
    color::colors,
    miniquad::date::now,
    prelude::{rand::rand, *},
    rand::srand,
};
use uom::si::{f32::Time, time::second};

use crate::game::{GameState, GlWrapper, Level, LevelState, PinGun, Spinner};

mod render;
mod step;

static FONT: LazyLock<Font> = LazyLock::new(|| {
    let font_bytes = include_bytes!("../../../AlanSans-Medium.ttf");
    load_ttf_font_from_bytes(font_bytes).unwrap()
});

impl<'a> GameState<'a> {
    const WIN_MESSAGES: [&'static str; 8] = [
        "you done did it",
        "good shootin son",
        "wow okay nice",
        "hell yeah brother",
        "it's good to be the king",
        "a glorious triumph",
        "you have mastered this game",
        "you are victorious",
    ];
    pub fn new(gl: InternalGlContext) -> GameState {
        let levels_str = include_str!("../../../levels.json");
        // TODO handle error instead of unwrapping
        let levels = serde_json::from_str::<Vec<Level>>(levels_str).unwrap();
        // TODO check that there is at least one level
        srand((now() * 1000.) as u64);
        let mut game = GameState {
            gl: GlWrapper(gl),
            startup_complete: false,
            text_params: TextParams {
                font: Some(&FONT),
                color: colors::WHITE,
                ..Default::default()
            },
            win_message: Self::WIN_MESSAGES[rand() as usize % Self::WIN_MESSAGES.len()],
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
