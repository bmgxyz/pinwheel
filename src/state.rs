use std::time::Duration;

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
                angular_velocity: 0.,
                pins: Vec::new(),
            },
            pin_gun: PinGun { pins: Vec::new() },
            flying_pins: Vec::new(),
            fire_pin: false,
        }
    }
}

impl GameState {
    pub(crate) fn step(&mut self, dt: Duration) {
        // fire a pin if the player asked to
        if self.fire_pin {
            // TODO move next pin from gun to flying
            self.fire_pin = false;
        }

        // spin the spinner
        // TODO

        // advance flying pins
        // TODO

        // check for pin collisions
        for pin in self.flying_pins.iter_mut() {
            // if the pin has collided with the sector of its own color, then move it to the spinner
            // TODO

            // if the pin has collided with anything else on the spinner, then game over
            // TODO
        }
    }
}

struct Spinner {
    sectors: Vec<Sector>,
    angular_velocity: f32,
    pins: Vec<PinOnSpinner>,
}

struct Sector {
    color: Color,
    angle_start: Angle,
    angle_stop: Angle,
    radius: f32,
}

struct Angle {
    angle: f32,
}

struct PinGun {
    pins: Vec<PinInGun>,
}

struct PinInGun {
    color: Color,
}

struct PinFlying {
    color: Color,
    vertical_position: PinVerticalPosition,
    alive: bool,
}

struct PinVerticalPosition {
    position: f32,
}

struct PinOnSpinner {
    color: Color,
    angular_position: f32,
}

enum Color {
    Black,
    Blue,
    Green,
    Red,
}
