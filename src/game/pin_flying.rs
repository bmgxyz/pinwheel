use uom::si::{
    f32::{Length, Velocity},
    length::meter,
    velocity::meter_per_second,
};

use crate::game::{PinFlying, PinInGun};

impl From<PinInGun> for PinFlying {
    fn from(value: PinInGun) -> Self {
        PinFlying {
            color: value.color,
            vertical_position: Length::new::<meter>(-5.),
            vertical_velocity: Velocity::new::<meter_per_second>(15.),
        }
    }
}
