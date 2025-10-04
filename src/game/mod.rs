use std::{
    fmt::Debug,
    ops::{Deref, DerefMut},
};

use macroquad::prelude::*;
use serde::Deserialize;
use uom::si::f32::{Angle, AngularVelocity, Length, Velocity};

mod pin_flying;
mod sector;
mod spinner;
mod state;
mod utils;

#[derive(Debug)]
pub struct GameState<'a> {
    gl: GlWrapper<'a>,
    startup_complete: bool,
    text_params: TextParams<'a>,
    spinner: Spinner,
    pin_gun: PinGun,
    flying_pins: Vec<PinFlying>,
    levels: Vec<Level>,
    level_idx: usize,
    level_state: LevelState,
}

struct GlWrapper<'a>(InternalGlContext<'a>);

impl<'a> Debug for GlWrapper<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "InternalGlContext")
    }
}

impl<'a> Deref for GlWrapper<'a> {
    type Target = InternalGlContext<'a>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> DerefMut for GlWrapper<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Deserialize)]
struct Level {
    spinner: Spinner,
    pins_in_gun: Vec<PinInGun>,
}

#[derive(Deserialize)]
#[serde(remote = "Color")]
pub struct SerdeColor {
    r: f32,
    g: f32,
    b: f32,
    a: f32,
}

#[derive(Debug, PartialEq, Default)]
enum LevelState {
    #[default]
    Playing,
    Won,
    Lost,
}

#[derive(Clone, Debug, Deserialize)]
struct Spinner {
    sectors: Vec<Sector>,
    angular_position: Angle,
    angular_velocity: AngularVelocity,
    pins: Vec<PinOnSpinner>,
    radius: Length,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Sector {
    #[serde(with = "SerdeColor")]
    color: Color,
    angle_start: Angle,
    angle_stop: Angle,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct PinOnSpinner {
    #[serde(with = "SerdeColor")]
    color: Color,
    angular_position: Angle,
    length: Length,
    width: Angle,
}

#[derive(Debug)]
struct PinFlying {
    color: Color,
    vertical_position: Length,
    vertical_velocity: Velocity,
}

#[derive(Debug, Default)]
struct PinGun {
    pins: Vec<PinInGun>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct PinInGun {
    #[serde(with = "SerdeColor")]
    color: Color,
}
