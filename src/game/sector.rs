use crate::game::Sector;
use macroquad::color::colors;
use uom::si::{angle::revolution, f32::Angle};

impl Sector {
    pub const TRIANGLES_PER_TURN: u16 = 90;
}

impl Default for Sector {
    fn default() -> Self {
        Sector {
            color: colors::BLACK,
            angle_start: Angle::new::<revolution>(0.),
            angle_stop: Angle::new::<revolution>(1.),
        }
    }
}
