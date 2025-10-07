use uom::si::{
    angle::{degree, revolution},
    angular_velocity::degree_per_second,
    f32::{Angle, AngularVelocity, Length},
    length::meter,
};

use crate::game::{PinFlying, PinOnSpinner, Sector, Spinner, utils::normalize_angle};

impl Default for Spinner {
    fn default() -> Self {
        Spinner {
            sectors: vec![Sector::default()],
            angular_position: Angle::new::<revolution>(0.),
            angular_velocity: AngularVelocity::new::<degree_per_second>(60.),
            pins: Vec::new(),
            radius: Length::new::<meter>(2.),
        }
    }
}

impl Spinner {
    pub fn pin_sector_collision(&self, pin: &PinFlying, sector: &Sector) -> bool {
        let inside_sector_radius = pin.vertical_position.abs() < self.radius;
        let sector_angle_start_absolute =
            normalize_angle(&(self.angular_position + sector.angle_start));
        let sector_angle_stop_absolute =
            sector_angle_start_absolute + (sector.angle_stop - sector.angle_start);
        let sector_is_facing_down = (sector_angle_start_absolute.get::<revolution>()
            ..sector_angle_stop_absolute.get::<revolution>())
            .contains(&0.75);
        inside_sector_radius && sector_is_facing_down
    }
    pub fn pin_pin_collision(&self, flying_pin: &PinFlying, spinner_pin: &PinOnSpinner) -> bool {
        let inside_pin_radius =
            flying_pin.vertical_position.abs() < self.radius + spinner_pin.length;
        let pin_angle_start = normalize_angle(
            &(self.angular_position + spinner_pin.angular_position - spinner_pin.width / 2.),
        );
        let pin_angle_stop = pin_angle_start + spinner_pin.width;
        let spinner_pin_facing_down = (pin_angle_start.get::<revolution>()
            ..pin_angle_stop.get::<revolution>())
            .contains(&0.75);
        inside_pin_radius && spinner_pin_facing_down
    }
    pub fn take_pin(&mut self, pin: PinFlying) {
        self.pins.push(PinOnSpinner {
            color: pin.color,
            angular_position: Angle::new::<revolution>(0.75) - self.angular_position,
            length: Length::new::<meter>(1.),
            width: Angle::new::<degree>(8.),
        })
    }
}
