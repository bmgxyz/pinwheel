use macroquad::color::Color;
use uom::si::{
    angle::degree,
    angular_velocity::degree_per_second,
    f32::{Angle, AngularVelocity, Length, Time, Velocity},
    length::centimeter,
    velocity::centimeter_per_second,
};

pub(crate) struct GameState {
    spinner: Spinner,
    pin_gun: PinGun,
    flying_pins: Vec<PinFlying>,
    fire_pin: bool,
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            spinner: Spinner {
                sectors: Vec::new(),
                angular_position: Angle::new::<degree>(0.),
                angular_velocity: AngularVelocity::new::<degree_per_second>(0.),
                pins: Vec::new(),
            },
            pin_gun: PinGun { pins: Vec::new() },
            flying_pins: Vec::new(),
            fire_pin: false,
        }
    }
}

impl GameState {
    pub(crate) fn step(&mut self, dt: Time) {
        // fire a pin if the player asked to
        if self.fire_pin {
            if let Some(next_pin) = self.pin_gun.pins.pop() {
                self.flying_pins.push(next_pin.into());
            }
            self.fire_pin = false;
        }

        // spin the spinner
        let d_theta: Angle = (self.spinner.angular_velocity * dt).into();
        self.spinner.angular_position += d_theta;

        // advance flying pins
        for flying_pin in self.flying_pins.iter_mut().filter(|p| p.alive) {
            let dy: Length = (flying_pin.vertical_velocity * dt).into();
            flying_pin.vertical_position += dy;
        }

        // check for pin collisions
        for flying_pin in self.flying_pins.iter_mut() {
            // sector collisions
            // TODO

            // pin collisions
            // TODO
        }
    }
}

struct Spinner {
    sectors: Vec<Sector>,
    angular_position: Angle,
    angular_velocity: AngularVelocity,
    pins: Vec<PinOnSpinner>,
}

struct Sector {
    color: Color,
    angle_start: Angle,
    angle_stop: Angle,
    radius: f32,
}

struct PinGun {
    pins: Vec<PinInGun>,
}

struct PinInGun {
    color: Color,
}

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
            vertical_position: Length::new::<centimeter>(0.),
            vertical_velocity: Velocity::new::<centimeter_per_second>(0.),
            alive: true,
        }
    }
}

struct PinOnSpinner {
    color: Color,
    angular_position: Angle,
}
