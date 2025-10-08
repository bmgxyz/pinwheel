use macroquad::prelude::*;
use uom::si::{angle::revolution, f32::Angle};

pub(crate) struct CircularSectorParams {
    pub(crate) n: u16,
    pub(crate) radius: f32,
    pub(crate) rotation: f32,
    pub(crate) arc: f32,
    pub(crate) color: Color,
}

pub(crate) fn draw_circular_sector(
    x: f32,
    y: f32,
    params: CircularSectorParams,
    gl: &mut InternalGlContext,
) {
    let CircularSectorParams {
        n,
        radius,
        rotation,
        arc,
        color,
    } = params;
    if arc <= 0. || radius <= 0. || n == 0 {
        return;
    }
    let step = arc / (n as f32);
    let center = Vertex::new(x, y, 0., 0., 0., color);
    let center_idx = 0;
    let mut vertices = vec![center];
    let mut indices = vec![center_idx];
    let mut previous_vertex_idx = None;
    let mut theta = rotation;
    let mut i = 0;
    while i <= n {
        let new_vertex = Vertex::new(
            radius * theta.cos() + x,
            radius * theta.sin() + y,
            0.,
            0.,
            0.,
            color,
        );
        vertices.push(new_vertex);
        let new_vertex_idx = (vertices.len() - 1) as u16;
        if let Some(previous_vertex_idx) = previous_vertex_idx {
            indices.extend_from_slice(&[center_idx, new_vertex_idx, previous_vertex_idx]);
        }
        previous_vertex_idx = Some(new_vertex_idx);
        theta += step;
        i += 1;
    }
    gl.flush();
    gl.quad_gl.texture(None);
    gl.quad_gl.draw_mode(DrawMode::Triangles);
    gl.quad_gl.geometry(&vertices, &indices);
    gl.flush();
}

pub(crate) fn normalize_angle(angle: &Angle) -> Angle {
    let mut new_angle = *angle;
    while new_angle < Angle::new::<revolution>(0.) {
        new_angle += Angle::new::<revolution>(1.);
    }
    while new_angle > Angle::new::<revolution>(1.) {
        new_angle -= Angle::new::<revolution>(1.);
    }
    new_angle
}

pub(crate) fn draw_text_ex_center(
    text: &str,
    x: f32,
    y: f32,
    params: TextParams,
) -> TextDimensions {
    let TextParams {
        font,
        font_size,
        font_scale,
        ..
    } = params;
    let dimensions = measure_text(text, font, font_size, font_scale);
    let new_x = x - dimensions.width / 2.;
    let new_y = y - dimensions.height / 2.;
    draw_text_ex(text, new_x, new_y, params)
}

pub(crate) fn use_white_text(background: Color) -> bool {
    let Color { r, g, b, .. } = background;
    // https://stackoverflow.com/a/3943023
    r * 0.299 + g * 0.587 + b * 0.114 > 165.
}
