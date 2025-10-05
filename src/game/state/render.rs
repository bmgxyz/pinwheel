use std::f32::consts::FRAC_PI_2;

use macroquad::{color::colors, miniquad::window::screen_size, prelude::*};
use uom::si::{
    angle::{radian, revolution},
    f32::Angle,
    length::meter,
};

use crate::game::{
    GameState, LevelState, Sector,
    utils::{draw_circular_sector, draw_text_ex_center, use_white_text},
};

impl<'a> GameState<'a> {
    const TARGET_HEIGHT: f32 = 720.;
    const TARGET_WIDTH: f32 = 480.;
    const TARGET_ASPECT_RATIO: f32 = Self::TARGET_HEIGHT / Self::TARGET_WIDTH;
    const TARGET_BOUNDING_BOX_METERS: Rect = Rect::new(-5., -10., 10., 15.);

    const SKY_BLUE: Color = Color::from_hex(0x3CA7D5);

    pub fn render(&mut self) {
        if !self.startup_complete {
            request_new_screen_size(Self::TARGET_WIDTH, Self::TARGET_HEIGHT);
            self.startup_complete = true;
            return;
        }

        // maintain camera aspect ratio regardless of screen dimensions
        let (width, height) = screen_size();
        let actual_aspect_ratio = height / width;
        let world_bounding_box_meters = if actual_aspect_ratio > Self::TARGET_ASPECT_RATIO {
            let new_height = Self::TARGET_BOUNDING_BOX_METERS.w * actual_aspect_ratio;
            let new_y = Self::TARGET_BOUNDING_BOX_METERS.y
                - (new_height - Self::TARGET_BOUNDING_BOX_METERS.h) / 2.;
            Rect::new(
                Self::TARGET_BOUNDING_BOX_METERS.x,
                new_y,
                Self::TARGET_BOUNDING_BOX_METERS.w,
                new_height,
            )
        } else if actual_aspect_ratio < Self::TARGET_ASPECT_RATIO {
            let new_width = Self::TARGET_BOUNDING_BOX_METERS.h / actual_aspect_ratio;
            let new_x = Self::TARGET_BOUNDING_BOX_METERS.x
                - (new_width - Self::TARGET_BOUNDING_BOX_METERS.w) / 2.;
            Rect::new(
                new_x,
                Self::TARGET_BOUNDING_BOX_METERS.y,
                new_width,
                Self::TARGET_BOUNDING_BOX_METERS.h,
            )
        } else {
            Self::TARGET_BOUNDING_BOX_METERS
        };
        let camera = Camera2D::from_display_rect(world_bounding_box_meters);
        set_camera(&camera);

        // set text parameters based on updated camera
        let (font_size, font_scale, font_aspect) = camera_font_scale(1.);
        self.text_params = TextParams {
            font_size,
            font_scale,
            // text appears upside down and backwards unless I include these tweaks, not sure why
            font_scale_aspect: -font_aspect,
            rotation: Angle::new::<revolution>(0.5).get::<radian>(),
            ..self.text_params
        };

        // clear screen so we can draw the next frame
        clear_background(Self::SKY_BLUE);

        if self.level_state == LevelState::Won && self.level_idx == self.levels.len() - 1 {
            // win message
            draw_text_ex_center(
                self.win_message,
                0.,
                4.,
                TextParams {
                    font_size: 24,
                    ..self.text_params
                },
            );
        } else {
            // level counter
            draw_text_ex_center(
                &format!("{} / {}", self.level_idx + 1, self.levels.len()),
                0.,
                4.,
                self.text_params.clone(),
            );
        }

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
            draw_text_ex_center(
                &format!("{}", self.pin_gun.pins.len() - pin_idx),
                0.,
                y,
                TextParams {
                    font_size: 14,
                    color: if use_white_text(pin_in_gun.color) {
                        colors::WHITE
                    } else {
                        colors::BLACK
                    },
                    ..self.text_params
                },
            );
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
