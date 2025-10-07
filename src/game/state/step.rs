use macroquad::prelude::*;
use uom::si::f32::{Angle, Time};

use crate::game::{GameState, LevelState, Sounds, utils::normalize_angle};

impl<'a> GameState<'a> {
    pub(crate) fn step(&mut self, dt: Time) {
        // check win condition
        if self.pin_gun.pins.is_empty() && self.flying_pins.is_empty() {
            if self.level_state == LevelState::Playing {
                if self.level_idx == self.levels.len() - 1 {
                    self.play_sound(Sounds::WinGame);
                } else {
                    self.play_sound(Sounds::WinLevel);
                }
            }
            self.level_state = LevelState::Won;
        }

        if is_key_pressed(KeyCode::Space) || is_mouse_button_pressed(MouseButton::Left) {
            match self.level_state {
                LevelState::Playing => {
                    // fire a pin if the player asked to
                    if let Some(next_pin) = self.pin_gun.pins.pop() {
                        self.flying_pins.push(next_pin.into());
                        self.play_sound(Sounds::PinFire);
                    }
                }
                LevelState::Won => {
                    if self.level_idx < self.levels.len() - 1 {
                        self.play_sound(Sounds::NextLevel);
                        self.level_idx += 1;
                        self.load_level(self.level_idx);
                    }
                }
                LevelState::Lost => {
                    self.play_sound(Sounds::NextLevel);
                    self.load_level(self.level_idx);
                }
            }
        }

        // early return if the level has ended
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
        for (idx, flying_pin) in self.flying_pins.iter().enumerate() {
            // sector collisions
            for sector in self.spinner.sectors.iter() {
                if self.spinner.pin_sector_collision(flying_pin, sector) {
                    if flying_pin.color == sector.color {
                        new_spinner_pin_idxs.push(idx);
                    } else {
                        self.play_sound(Sounds::LoseLevel);
                        self.level_state = LevelState::Lost;
                    }
                }
            }

            // pin collisions
            for spinner_pin in self.spinner.pins.iter() {
                if self.spinner.pin_pin_collision(flying_pin, spinner_pin) {
                    self.play_sound(Sounds::LoseLevel);
                    self.level_state = LevelState::Lost;
                }
            }
        }

        // move pins that have landed safely into the spinner
        for idx in new_spinner_pin_idxs.into_iter().rev() {
            if !self.pin_gun.pins.is_empty() {
                self.play_sound(Sounds::PinLand);
            }
            let new_spinner_pin = self.flying_pins.remove(idx);
            self.spinner.take_pin(new_spinner_pin);
        }
    }
}
