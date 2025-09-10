use std::f32::consts::FRAC_PI_2;

use macroquad::{color::colors, prelude::*};
use serde::Deserialize;
use uom::si::{
    angle::{degree, radian, revolution},
    angular_velocity::degree_per_second,
    f32::{Angle, AngularVelocity, Length, Time, Velocity},
    length::meter,
    velocity::meter_per_second,
};

use crate::level::{Level, SerdeColor};
use crate::utils::{draw_circular_sector, normalize_angle};

#[derive(Debug, PartialEq, Default)]
enum LevelState {
    #[default]
    Playing,
    Won,
    Lost,
}

#[derive(Debug, Default)]
pub(crate) struct GameState {
    spinner: Spinner,
    pin_gun: PinGun,
    flying_pins: Vec<PinFlying>,
    levels: Vec<Level>,
    level_idx: usize,
    level_state: LevelState,
}

impl GameState {
    pub(crate) fn load_levels(&mut self, levels: &[Level]) {
        self.levels = levels.to_vec();
        self.level_idx = 0;
        if levels.is_empty() {
            *self = GameState::default();
        } else {
            let level = self.levels[self.level_idx].clone();
            self.load_level(&level);
        }
    }
    fn load_level(&mut self, level: &Level) {
        self.spinner = level.spinner.clone();
        self.pin_gun.pins = level.pins_in_gun.clone();
        self.flying_pins.clear();
        self.level_state = LevelState::Playing;
    }
    pub(crate) fn step(&mut self, dt: Time) {
        clear_background(Color::from_hex(0x3CA7D5));

        // check win condition
        if self.pin_gun.pins.is_empty() && self.flying_pins.is_empty() {
            self.level_state = LevelState::Won;
        }

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            match self.level_state {
                LevelState::Playing => {
                    // fire a pin if the player asked to
                    if let Some(next_pin) = self.pin_gun.pins.pop() {
                        self.flying_pins.push(next_pin.into());
                    }
                }
                LevelState::Won => {
                    if self.level_idx < self.levels.len() - 1 {
                        self.level_idx += 1;
                        let level = self.levels[self.level_idx].clone();
                        self.load_level(&level);
                    }
                }
                LevelState::Lost => {
                    let level = self.levels[self.level_idx].clone();
                    self.load_level(&level);
                }
            }
        }

        // early return if the game has ended
        match self.level_state {
            LevelState::Won | LevelState::Lost => return,
            _ => (),
        }

        // spin the spinner
        let d_theta: Angle = (self.spinner.angular_velocity * dt).into();
        self.spinner.angular_position = normalize_angle(&(self.spinner.angular_position + d_theta));

        // advance flying pins
        for flying_pin in self.flying_pins.iter_mut() {
            flying_pin.vertical_position += flying_pin.vertical_velocity * dt;
        }

        // check for collisions
        let mut new_spinner_pin_idxs = Vec::new();
        for (idx, flying_pin) in self.flying_pins.iter_mut().enumerate() {
            // sector collisions
            for sector in self.spinner.sectors.iter() {
                if self.spinner.pin_sector_collision(flying_pin, sector) {
                    if flying_pin.color == sector.color {
                        new_spinner_pin_idxs.push(idx);
                    } else {
                        self.level_state = LevelState::Lost;
                    }
                }
            }

            // pin collisions
            for spinner_pin in self.spinner.pins.iter() {
                if self.spinner.pin_pin_collision(flying_pin, spinner_pin) {
                    self.level_state = LevelState::Lost;
                }
            }
        }

        // move pins that have landed safely into the spinner
        for idx in new_spinner_pin_idxs.into_iter().rev() {
            let new_spinner_pin = self.flying_pins.remove(idx);
            self.spinner.take_pin(new_spinner_pin);
        }
    }
    pub(crate) fn render(&self, gl: &mut InternalGlContext) {
        // spinner sectors
        for sector in self.spinner.sectors.iter() {
            let n = ((sector.angle_stop - sector.angle_start).get::<revolution>()
                * Sector::TRIANGLES_PER_TURN as f32) as u16;
            draw_circular_sector(
                0.,
                0.,
                n,
                self.spinner.radius.get::<meter>(),
                (sector.angle_start + self.spinner.angular_position).get::<radian>(),
                (sector.angle_stop - sector.angle_start).get::<radian>(),
                sector.color,
                gl,
            );
        }

        // spinner pins
        for spinner_pin in self.spinner.pins.iter() {
            let x = (self.spinner.radius
                * (spinner_pin.angular_position + self.spinner.angular_position).cos())
            .get::<meter>();
            let y = (self.spinner.radius
                * (spinner_pin.angular_position + self.spinner.angular_position).sin())
            .get::<meter>();
            draw_rectangle_ex(
                x,
                y,
                0.2,
                spinner_pin.length.get::<meter>(),
                DrawRectangleParams {
                    offset: vec2(0.5, 0.),
                    rotation: (spinner_pin.angular_position + self.spinner.angular_position)
                        .get::<radian>()
                        - FRAC_PI_2,
                    color: spinner_pin.color,
                },
            );
        }

        // pin gun
        draw_rectangle(-0.5, -10., 1., 5., colors::GRAY);
        for (pin_idx, pin_in_gun) in self.pin_gun.pins.iter().rev().take(5).enumerate() {
            let y = -5.5 - (pin_idx as f32 * 1.);
            draw_circle(0., y, 0.25, pin_in_gun.color);
        }

        // flying pins
        for flying_pin in self.flying_pins.iter() {
            draw_rectangle(
                -0.1,
                flying_pin.vertical_position.get::<meter>(),
                0.2,
                -1.,
                flying_pin.color,
            );
        }
    }
}

#[derive(Clone, Debug, Deserialize)]
pub(crate) struct Spinner {
    sectors: Vec<Sector>,
    angular_position: Angle,
    angular_velocity: AngularVelocity,
    pins: Vec<PinOnSpinner>,
    radius: Length,
}

impl Default for Spinner {
    fn default() -> Self {
        Spinner {
            sectors: vec![Sector {
                color: colors::BLACK,
                angle_start: Angle::new::<revolution>(0.),
                angle_stop: Angle::new::<revolution>(1.),
            }],
            angular_position: Angle::new::<revolution>(0.),
            angular_velocity: AngularVelocity::new::<degree_per_second>(60.),
            pins: Vec::new(),
            radius: Length::new::<meter>(2.),
        }
    }
}

impl Spinner {
    fn pin_sector_collision(&self, pin: &PinFlying, sector: &Sector) -> bool {
        let inside_sector_radius = pin.vertical_position.abs() < self.radius;
        let sector_angle_start = normalize_angle(&(self.angular_position + sector.angle_start));
        let sector_angle_stop = normalize_angle(&(self.angular_position + sector.angle_stop));
        let sector_is_facing_down = (sector_angle_start.get::<revolution>()
            ..sector_angle_stop.get::<revolution>())
            .contains(&0.75);
        inside_sector_radius && sector_is_facing_down
    }
    fn pin_pin_collision(&self, flying_pin: &PinFlying, spinner_pin: &PinOnSpinner) -> bool {
        let inside_pin_radius =
            flying_pin.vertical_position.abs() < self.radius + spinner_pin.length;
        let pin_angle_start =
            self.angular_position + spinner_pin.angular_position - spinner_pin.width / 2.;
        let pin_angle_stop =
            self.angular_position + spinner_pin.angular_position + spinner_pin.width / 2.;
        let spinner_pin_facing_down = (pin_angle_start.get::<revolution>()
            ..pin_angle_stop.get::<revolution>())
            .contains(&0.75);
        inside_pin_radius && spinner_pin_facing_down
    }
    fn take_pin(&mut self, pin: PinFlying) {
        self.pins.push(PinOnSpinner {
            color: pin.color,
            angular_position: Angle::new::<revolution>(0.75) - self.angular_position,
            length: Length::new::<meter>(1.),
            width: Angle::new::<degree>(5.),
        })
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct Sector {
    #[serde(with = "SerdeColor")]
    color: Color,
    angle_start: Angle,
    angle_stop: Angle,
}

impl Sector {
    const TRIANGLES_PER_TURN: u16 = 90;
}

#[derive(Debug, Default)]
struct PinGun {
    pins: Vec<PinInGun>,
}

#[derive(Clone, Copy, Debug, Deserialize)]
pub(crate) struct PinInGun {
    #[serde(with = "SerdeColor")]
    color: Color,
}

#[derive(Debug)]
struct PinFlying {
    color: Color,
    vertical_position: Length,
    vertical_velocity: Velocity,
}

impl From<PinInGun> for PinFlying {
    fn from(value: PinInGun) -> Self {
        PinFlying {
            color: value.color,
            vertical_position: Length::new::<meter>(-5.),
            vertical_velocity: Velocity::new::<meter_per_second>(15.),
        }
    }
}

#[derive(Clone, Copy, Debug, Deserialize)]
struct PinOnSpinner {
    #[serde(with = "SerdeColor")]
    color: Color,
    angular_position: Angle,
    length: Length,
    width: Angle,
}
