use macroquad::color::Color;
use uom::si::{
    angle::degree,
    angular_velocity::degree_per_second,
    f32::{Angle, AngularVelocity, Length, Time, Velocity},
    length::meter,
    velocity::meter_per_second,
};

#[derive(Debug)]
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
                radius: Length::new::<meter>(2.),
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
        let inside_sector_radius = pin.vertical_position < self.radius;
        let sector_angle_start = self.angular_position + sector.angle_start;
        let sector_angle_stop = self.angular_position + sector.angle_stop;
        let sector_is_facing_down =
            (sector_angle_start.get::<degree>()..sector_angle_stop.get::<degree>()).contains(&270.);
        inside_sector_radius && sector_is_facing_down
    }
    fn pin_pin_collision(&self, flying_pin: &PinFlying, spinner_pin: &PinOnSpinner) -> bool {
        let inside_pin_radius = flying_pin.vertical_position < self.radius + spinner_pin.length;
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
            _color: pin.color,
            angular_position: self.angular_position,
            length: Length::new::<meter>(2.),
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
            vertical_position: Length::new::<meter>(10.),
            vertical_velocity: Velocity::new::<meter_per_second>(-50.),
            alive: true,
        }
    }
}

#[derive(Debug)]
struct PinOnSpinner {
    _color: Color,
    angular_position: Angle,
    length: Length,
    width: Angle,
}
