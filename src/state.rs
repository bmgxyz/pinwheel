use std::f32::consts::FRAC_PI_2;

use macroquad::{color::colors, prelude::*};
use uom::si::{
    angle::{degree, radian, revolution},
    angular_velocity::degree_per_second,
    f32::{Angle, AngularVelocity, Length, Time, Velocity},
    length::meter,
    velocity::meter_per_second,
};

use crate::utils::{draw_circular_sector, normalize_angle};

#[derive(Debug)]
pub(crate) struct GameState {
    spinner: Spinner,
    pin_gun: PinGun,
    flying_pins: Vec<PinFlying>,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            spinner: Spinner {
                // TODO verify no overlap and entire circle covered
                sectors: vec![
                    Sector {
                        color: colors::GREEN,
                        angle_start: Angle::new::<revolution>(0.),
                        angle_stop: Angle::new::<revolution>(0.25),
                    },
                    Sector {
                        color: colors::YELLOW,
                        angle_start: Angle::new::<revolution>(0.25),
                        angle_stop: Angle::new::<revolution>(0.5),
                    },
                    Sector {
                        color: colors::PURPLE,
                        angle_start: Angle::new::<revolution>(0.5),
                        angle_stop: Angle::new::<revolution>(0.75),
                    },
                    Sector {
                        color: colors::RED,
                        angle_start: Angle::new::<revolution>(0.75),
                        angle_stop: Angle::new::<revolution>(1.),
                    },
                ],
                angular_position: Angle::new::<degree>(0.),
                angular_velocity: AngularVelocity::new::<degree_per_second>(60.),
                pins: vec![
                    PinOnSpinner {
                        color: colors::RED,
                        angular_position: Angle::new::<revolution>(0.),
                        length: Length::new::<meter>(1.),
                        width: Angle::new::<degree>(5.),
                    },
                    PinOnSpinner {
                        color: colors::PURPLE,
                        angular_position: Angle::new::<revolution>(0.666),
                        length: Length::new::<meter>(1.),
                        width: Angle::new::<degree>(5.),
                    },
                ],
                radius: Length::new::<meter>(2.),
            },
            pin_gun: PinGun {
                pins: vec![
                    PinInGun { color: colors::RED },
                    PinInGun { color: colors::RED },
                    PinInGun { color: colors::RED },
                    PinInGun { color: colors::RED },
                    PinInGun { color: colors::RED },
                ],
            },
            flying_pins: Vec::new(),
        }
    }
}

impl GameState {
    pub(crate) fn step(&mut self, dt: Time) {
        // fire a pin if the player asked to
        if is_key_pressed(KeyCode::Space) {
            if let Some(next_pin) = self.pin_gun.pins.pop() {
                self.flying_pins.push(next_pin.into());
            }
        }

        // spin the spinner
        let d_theta: Angle = (self.spinner.angular_velocity * dt).into();
        self.spinner.angular_position = normalize_angle(&(self.spinner.angular_position + d_theta));

        // advance flying pins
        for flying_pin in self.flying_pins.iter_mut().filter(|p| p.alive) {
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
                        flying_pin.alive = false;
                    }
                }
            }

            // pin collisions
            for spinner_pin in self.spinner.pins.iter() {
                if self.spinner.pin_pin_collision(flying_pin, spinner_pin) {
                    flying_pin.alive = false;
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

#[derive(Debug)]
struct Spinner {
    sectors: Vec<Sector>,
    angular_position: Angle,
    angular_velocity: AngularVelocity,
    pins: Vec<PinOnSpinner>,
    radius: Length,
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
        let spinner_pin_facing_down =
            (pin_angle_start.get::<degree>()..pin_angle_stop.get::<degree>()).contains(&270.);
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

#[derive(Debug)]
struct Sector {
    color: Color,
    angle_start: Angle,
    angle_stop: Angle,
}

impl Sector {
    const TRIANGLES_PER_TURN: u16 = 360;
}

#[derive(Debug)]
struct PinGun {
    pins: Vec<PinInGun>,
}

#[derive(Debug)]
struct PinInGun {
    color: Color,
}

#[derive(Debug)]
struct PinFlying {
    color: Color,
    vertical_position: Length,
    vertical_velocity: Velocity,
    alive: bool,
}

impl From<PinInGun> for PinFlying {
    fn from(value: PinInGun) -> Self {
        PinFlying {
            color: value.color,
            vertical_position: Length::new::<meter>(-5.),
            vertical_velocity: Velocity::new::<meter_per_second>(10.),
            alive: true,
        }
    }
}

#[derive(Debug)]
struct PinOnSpinner {
    color: Color,
    angular_position: Angle,
    length: Length,
    width: Angle,
}
