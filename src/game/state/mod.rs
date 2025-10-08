use std::{error::Error, sync::LazyLock};

use macroquad::{
    audio::play_sound_once,
    color::colors,
    miniquad::date::now,
    prelude::*,
    rand::{ChooseRandom, srand},
};
use uom::si::{f32::Time, time::second};

use crate::game::{GameState, GlWrapper, Level, LevelState, PinGun, SoundData, Sounds, Spinner};

mod render;
mod step;

static FONT: LazyLock<Font> = LazyLock::new(|| {
    let font_bytes = include_bytes!("../../../assets/AlanSans-Medium.ttf");
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
    pub async fn new(gl: InternalGlContext<'_>) -> Result<GameState<'_>, Box<dyn Error>> {
        let levels_str = include_str!("../../../assets/levels.json");
        let levels = match serde_json::from_str::<Vec<Level>>(levels_str) {
            Ok(l) => l,
            Err(e) => return Err(format!("Failed to parse level definitions: {e}").into()),
        };
        if levels.is_empty() {
            return Err("No levels found".into());
        }
        srand((now() * 1000.) as u64);
        let mut game = GameState {
            gl: GlWrapper(gl),
            startup_complete: false,
            text_params: TextParams {
                font: Some(&FONT),
                color: colors::WHITE,
                ..Default::default()
            },
            win_message: Self::WIN_MESSAGES.choose().unwrap(),
            spinner: Spinner::default(),
            pin_gun: PinGun::default(),
            flying_pins: vec![],
            levels,
            level_idx: 0,
            level_state: LevelState::default(),
            sound_data: SoundData::load().await,
        };
        game.load_level(game.level_idx);
        Ok(game)
    }
    fn load_level(&mut self, level_idx: usize) {
        let level = &self.levels[level_idx];
        self.spinner = level.spinner.clone();
        self.pin_gun.pins = level.pins_in_gun.clone();
        self.flying_pins.clear();
        self.level_state = LevelState::Playing;
    }
    fn play_sound(&self, sound: Sounds) {
        match sound {
            Sounds::PinFire => play_sound_once(&self.sound_data.pin_fire),
            Sounds::PinLand => play_sound_once(&self.sound_data.pin_land),
            Sounds::LoseLevel => play_sound_once(&self.sound_data.lose_level),
            Sounds::NextLevel => play_sound_once(&self.sound_data.next_level),
            Sounds::WinLevel => play_sound_once(&self.sound_data.win_level),
            Sounds::WinGame => play_sound_once(&self.sound_data.win_game),
        };
    }
    pub async fn run(&mut self) -> ! {
        loop {
            self.step(Time::new::<second>(get_frame_time()));
            self.render();
            next_frame().await
        }
    }
}
