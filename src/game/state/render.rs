use std::f32::consts::FRAC_PI_2;

use macroquad::{color::colors, prelude::*};
use uom::si::{
    angle::{radian, revolution},
    length::meter,
};

use crate::game::{GameState, Sector, utils::draw_circular_sector};

impl<'a> GameState<'a> {
    pub fn render(&mut self) {
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
                &mut self.gl,
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
