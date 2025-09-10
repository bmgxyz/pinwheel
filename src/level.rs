use macroquad::color::Color;
use serde::Deserialize;

use crate::state::{PinInGun, Spinner};

#[derive(Deserialize)]
#[serde(remote = "Color")]
pub(crate) struct SerdeColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Level {
    pub(crate) spinner: Spinner,
    pub(crate) pins_in_gun: Vec<PinInGun>,
}
